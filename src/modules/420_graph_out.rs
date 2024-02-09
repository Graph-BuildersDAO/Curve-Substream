use std::{
    borrow::BorrowMut,
    collections::HashSet,
    ops::{Div, Mul},
};

use anyhow::anyhow;
use substreams::{
    errors::Error,
    pb::substreams::{store_delta::Operation, Clock},
    scalar::{BigDecimal, BigInt},
    store::{
        DeltaBigDecimal, DeltaInt64, DeltaProto, Deltas, StoreGet, StoreGetBigDecimal,
        StoreGetBigInt, StoreGetInt64, StoreGetProto, StoreGetString,
    },
};
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

use crate::{
    common::{
        format::{self, format_address_string},
        prices::get_token_usd_price,
        utils,
    },
    constants,
    key_management::{entity_key_manager::EntityKey, store_key_manager::StoreKey},
    network_config::DEFAULT_NETWORK,
    pb::{
        curve::types::v1::{
            events::{
                pool_event::{DepositEvent, SwapEvent, SwapUnderlyingEvent, Type, WithdrawEvent},
                PoolEvent,
            },
            Events, Pool, PoolFee, PoolFees, Pools, Token,
        },
        uniswap_pricing::v1::Erc20Price,
    },
    types::snapshot::SnapshotType,
};

// TODO: If this module gets too bulky, consider following an approach similar to Uniswap V2 SPS:
//       (https://github.com/messari/substreams/tree/1c148752f7eb6b75804542428630f7fa74bf6414/uniswap-v2/src/modules)
#[substreams::handlers::map]
pub fn graph_out(
    clock: Clock,
    pools: Pools,
    events: Events,
    pools_store: StoreGetProto<Pool>,
    pool_count_store: StoreGetInt64,
    pool_count_deltas: Deltas<DeltaInt64>,
    pool_addresses_store: StoreGetString,
    pool_fees_store: StoreGetProto<PoolFees>,
    pool_fees_deltas: Deltas<DeltaProto<PoolFees>>,
    tokens_store: StoreGetInt64,
    output_token_supply_store: StoreGetBigInt,
    input_token_balances_store: StoreGetBigInt,
    pool_volume_native_store: StoreGetBigInt,
    pool_volume_usd_store: StoreGetBigDecimal,
    pool_volume_usd_deltas: Deltas<DeltaBigDecimal>,
    protocol_volume_store: StoreGetBigDecimal,
    protocol_volume_deltas: Deltas<DeltaBigDecimal>,
    pool_tvl_store: StoreGetBigDecimal,
    pool_tvl_deltas: Deltas<DeltaBigDecimal>,
    protocol_tvl_store: StoreGetBigDecimal,
    store_current_time_snapshots: Deltas<DeltaInt64>,
    uniswap_prices: StoreGetProto<Erc20Price>,
    chainlink_prices: StoreGetBigDecimal,
) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    create_protocol_entity(&mut tables, &clock);

    // Create entities related to Pool contract deployments
    for pool in pools.pools {
        // TODO: Should we move the getting of pool fees to the functions that use them?
        // Pools must have related fees for the output data to be accurate and useful.
        let pool_fees = pool_fees_store.must_get_last(StoreKey::pool_fees_key(&pool.address));
        create_pool_entity(&mut tables, &pool, &pool_fees);
        create_pool_fee_entities(&mut tables, &pool_fees);
        create_pool_token_entities(&mut tables, &pool, &tokens_store)?;
    }

    for delta in pool_count_deltas.deltas.iter().last() {
        tables
            .update_row("DexAmmProtocol", EntityKey::protocol_key())
            .set("totalPoolCount", delta.new_value);
    }

    for delta in pool_fees_deltas.iter() {
        if delta.operation == Operation::Update {
            update_pool_fee_entities(&mut tables, &delta.new_value)
        }
    }

    for delta in pool_volume_usd_deltas.deltas.iter() {
        // Attempt to extract the pool address from the store key
        if let Some((pool_address, _, _)) = StoreKey::extract_parts_from_key(&delta.key) {
            if let Some(volume) =
                // If volume is found, update the corresponding row in the "LiquidityPool" table
                // with the cumulative volume in USD
                pool_volume_usd_store.get_last(StoreKey::pool_volume_usd_key(&pool_address))
            {
                tables
                    .update_row(
                        "LiquidityPool",
                        EntityKey::liquidity_pool_key(&pool_address),
                    )
                    .set("cumulativeVolumeUSD", volume);
            } else {
                substreams::log::info!("No volume data found for pool: {}", pool_address);
            }
        }
    }

    if !protocol_volume_deltas.deltas.is_empty() {
        if let Some(volume) = protocol_volume_store.get_last(StoreKey::protocol_volume_usd_key()) {
            tables
                .update_row("DexAmmProtocol", EntityKey::protocol_key())
                .set("cumulativeVolumeUSD", volume);
        }
    }

    // Start - Pool TVL weights updates
    if !pool_tvl_deltas.deltas.is_empty() {
        // Initialize a HashSet to store unique pool addresses
        let mut unique_pool_addresses = HashSet::new();

        // Filter and extract unique pool addresses
        for delta in pool_tvl_deltas.deltas {
            if delta.key.starts_with("PoolTvl:") {
                // Extract the pool address from the key
                if let Some((pool_address, _, _)) = StoreKey::extract_parts_from_key(&delta.key) {
                    unique_pool_addresses.insert(pool_address);
                }
            }
        }
        for pool_address in unique_pool_addresses.iter() {
            let pool = pools_store.must_get_last(StoreKey::pool_key(pool_address));
            let input_token_weights = get_input_token_weights(&pool, &pool_tvl_store);
            tables
                .update_row("LiquidityPool", EntityKey::liquidity_pool_key(pool_address))
                .set("inputTokenWeights", input_token_weights);
        }
    }

    // Create entities related to Pool events
    create_pool_events_entities(
        &mut tables,
        events.pool_events,
        &pools_store,
        &output_token_supply_store,
        &input_token_balances_store,
        &pool_tvl_store,
        &protocol_tvl_store,
    );

    for delta in store_current_time_snapshots.deltas {
        if delta.operation == Operation::Create {
            continue;
        }
        // This condition indicates the delta relates to a new day.
        if delta.key == StoreKey::current_day_id_key()
            && delta.operation == Operation::Update
            && delta.old_value != delta.new_value
        {
            let day_id = delta.old_value;
            // We have identified a new day, create snapshots for the previous day.
            create_protocol_financials_daily_snapshot(
                &clock,
                &mut tables,
                &day_id,
                &protocol_tvl_store,
                &protocol_volume_store,
            );
            create_liquidity_pool_snapshots(
                &SnapshotType::Daily,
                &day_id,
                &mut tables,
                &clock,
                &pool_count_store,
                &pool_addresses_store,
                &pools_store,
                &pool_tvl_store,
                &pool_volume_usd_store,
                &pool_volume_native_store,
                &input_token_balances_store,
                &output_token_supply_store,
                &uniswap_prices,
                &chainlink_prices,
            );
        }
        // This condition indicates the delta relates to a new hour
        else if delta.key == StoreKey::current_hour_id_key()
            && delta.operation == Operation::Update
            && delta.old_value != delta.new_value
        {
            let hour_id = delta.old_value;
            create_liquidity_pool_snapshots(
                &SnapshotType::Hourly,
                &hour_id,
                &mut tables,
                &clock,
                &pool_count_store,
                &pool_addresses_store,
                &pools_store,
                &pool_tvl_store,
                &pool_volume_usd_store,
                &pool_volume_native_store,
                &input_token_balances_store,
                &output_token_supply_store,
                &uniswap_prices,
                &chainlink_prices,
            );
        }
    }

    Ok(tables.to_entity_changes())
}

fn create_liquidity_pool_snapshots(
    snapshot_type: &SnapshotType,
    time_frame_id: &i64,
    tables: &mut Tables,
    clock: &Clock,
    pool_count_store: &StoreGetInt64,
    pool_addresses_store: &StoreGetString,
    pools_store: &StoreGetProto<Pool>,
    pool_tvl_store: &StoreGetBigDecimal,
    pool_volume_usd_store: &StoreGetBigDecimal,
    pool_volume_native_store: &StoreGetBigInt,
    input_token_balances_store: &StoreGetBigInt,
    output_token_supply_store: &StoreGetBigInt,
    uniswap_prices: &StoreGetProto<Erc20Price>,
    chainlink_prices: &StoreGetBigDecimal,
) {
    // Create Liquidity Pools daily snapshots
    let pool_count = pool_count_store
        .get_last(StoreKey::protocol_pool_count_key())
        .unwrap();

    for i in 1..=pool_count {
        let pool_address = pool_addresses_store
            .get_last(StoreKey::pool_address_key(&i))
            .unwrap();

        let pool = pools_store
            .get_last(StoreKey::pool_key(&pool_address))
            .unwrap();

        let pool_tvl_usd = pool_tvl_store
            .get_last(StoreKey::pool_tvl_key(&pool.address))
            .unwrap();

        // Get the volume in the pool for a given timeframe (Daily/Hourly)
        let pool_volume = match snapshot_type {
            SnapshotType::Daily => pool_volume_usd_store
                .get_last(StoreKey::pool_volume_usd_daily_key(
                    &pool.address,
                    &time_frame_id,
                ))
                .unwrap(),
            SnapshotType::Hourly => pool_volume_usd_store
                .get_last(StoreKey::pool_volume_usd_hourly_key(
                    &pool.address,
                    &time_frame_id,
                ))
                .unwrap(),
        };

        // Get the volumes of each pool input token for a given timeframe (Daily/Hourly)
        let (volume_by_token_native, volume_by_token_usd) = get_pool_token_volumes_in_timeframe(
            &pool,
            time_frame_id,
            snapshot_type,
            pool_volume_native_store,
            pool_volume_usd_store,
        );

        let pool_cumulative_volume_usd = pool_volume_usd_store
            .get_last(StoreKey::pool_volume_usd_key(&pool.address))
            .unwrap();

        let input_token_balances = get_input_token_balances(
            &pool_address,
            &pool.input_tokens,
            &input_token_balances_store,
        );

        let input_token_weights = get_input_token_weights(&pool, &pool_tvl_store);

        let output_token_supply = output_token_supply_store
            .get_last(StoreKey::output_token_supply_key(&pool_address))
            .unwrap_or_else(|| BigInt::zero());

        let output_token_price =
            get_token_usd_price(pool.output_token_ref(), &uniswap_prices, &chainlink_prices);

        // Create the relevant timeframe snapshot
        match snapshot_type {
            SnapshotType::Daily => create_pool_daily_snapshot(
                tables,
                clock,
                time_frame_id,
                &pool_address,
                &pool_tvl_usd,
                &pool_volume,
                &volume_by_token_native,
                &volume_by_token_usd,
                &pool_cumulative_volume_usd,
                &input_token_balances,
                &input_token_weights,
                &output_token_supply,
                &output_token_price,
            ),
            SnapshotType::Hourly => create_pool_hourly_snapshot(
                tables,
                clock,
                time_frame_id,
                &pool_address,
                &pool_tvl_usd,
                &pool_volume,
                &volume_by_token_native,
                &volume_by_token_usd,
                &pool_cumulative_volume_usd,
                &input_token_balances,
                &input_token_weights,
                &output_token_supply,
                &output_token_price,
            ),
        }
    }
}

fn create_pool_daily_snapshot(
    tables: &mut Tables,
    clock: &Clock,
    day_id: &i64,
    pool_address: &str,
    pool_tvl_usd: &BigDecimal,
    pool_volume_daily: &BigDecimal,
    volume_by_token_native: &Vec<BigInt>,
    volume_by_token_usd: &Vec<BigDecimal>,
    pool_cumulative_volume_usd: &BigDecimal,
    input_token_balances: &Vec<BigInt>,
    input_token_weights: &Vec<BigDecimal>,
    output_token_supply: &BigInt,
    output_token_price: &BigDecimal,
) {
    tables
        .create_row(
            "LiquidityPoolDailySnapshot",
            EntityKey::pool_daily_snapshot_key(pool_address, day_id),
        )
        .set("protocol", EntityKey::protocol_key())
        .set("pool", EntityKey::liquidity_pool_key(pool_address))
        .set("blockNumber", BigInt::from(clock.number))
        .set(
            "timestamp",
            BigInt::from(clock.timestamp.clone().unwrap().seconds),
        )
        .set("totalValueLockedUSD", pool_tvl_usd)
        .set("dailyVolumeUSD", pool_volume_daily)
        .set("dailyVolumeByTokenAmount", volume_by_token_native)
        .set("dailyVolumeByTokenUSD", volume_by_token_usd)
        .set("cumulativeVolumeUSD", pool_cumulative_volume_usd)
        .set("inputTokenBalances", input_token_balances)
        .set("inputTokenWeights", input_token_weights)
        .set("outputTokenSupply", output_token_supply)
        .set("outputTokenPriceUSD", output_token_price);
}

fn create_pool_hourly_snapshot(
    tables: &mut Tables,
    clock: &Clock,
    hour_id: &i64,
    pool_address: &str,
    pool_tvl_usd: &BigDecimal,
    pool_volume_hourly: &BigDecimal,
    volume_by_token_native: &Vec<BigInt>,
    volume_by_token_usd: &Vec<BigDecimal>,
    pool_cumulative_volume_usd: &BigDecimal,
    input_token_balances: &Vec<BigInt>,
    input_token_weights: &Vec<BigDecimal>,
    output_token_supply: &BigInt,
    output_token_price: &BigDecimal,
) {
    tables
        .create_row(
            "LiquidityPoolHourlySnapshot",
            EntityKey::pool_hourly_snapshot_key(&pool_address, hour_id),
        )
        .set("protocol", EntityKey::protocol_key())
        .set("pool", EntityKey::liquidity_pool_key(pool_address))
        .set("blockNumber", BigInt::from(clock.number))
        .set(
            "timestamp",
            BigInt::from(clock.timestamp.clone().unwrap().seconds),
        )
        .set("totalValueLockedUSD", pool_tvl_usd)
        .set("hourlyVolumeUSD", pool_volume_hourly)
        .set("hourlyVolumeByTokenAmount", volume_by_token_native)
        .set("hourlyVolumeByTokenUSD", volume_by_token_usd)
        .set("cumulativeVolumeUSD", pool_cumulative_volume_usd)
        .set("inputTokenBalances", input_token_balances)
        .set("inputTokenWeights", input_token_weights)
        .set("outputTokenSupply", output_token_supply)
        .set("outputTokenPriceUSD", output_token_price);
}

fn get_pool_token_volumes_in_timeframe(
    pool: &Pool,
    time_frame_id: &i64,
    snapshot_type: &SnapshotType,
    pool_volume_native_store: &StoreGetBigInt,
    pool_volume_usd_store: &StoreGetBigDecimal,
) -> (Vec<BigInt>, Vec<BigDecimal>) {
    let mut pool_volume_by_token_native: Vec<BigInt> = Vec::new();
    let mut pool_volume_by_token_usd: Vec<BigDecimal> = Vec::new();

    for token in &pool.input_tokens {
        let native_volume_key = match snapshot_type {
            SnapshotType::Daily => StoreKey::pool_token_volume_native_daily_key(
                &pool.address,
                &token.address,
                &time_frame_id,
            ),
            SnapshotType::Hourly => StoreKey::pool_token_volume_native_hourly_key(
                &pool.address,
                &token.address,
                &time_frame_id,
            ),
        };
        let usd_volume_key = match snapshot_type {
            SnapshotType::Daily => StoreKey::pool_token_volume_usd_daily_key(
                &pool.address,
                &token.address,
                &time_frame_id,
            ),
            SnapshotType::Hourly => StoreKey::pool_token_volume_usd_hourly_key(
                &pool.address,
                &token.address,
                &time_frame_id,
            ),
        };

        // Fetch and push the native volume for the token
        let token_native_volume = pool_volume_native_store
            .get_last(&native_volume_key)
            .unwrap_or_else(|| BigInt::from(0)); // Default to 0 if not found
        pool_volume_by_token_native.push(token_native_volume);

        // Fetch and push the USD volume for the token
        let token_usd_volume = pool_volume_usd_store
            .get_last(&usd_volume_key)
            .unwrap_or_else(|| BigDecimal::from(0)); // Default to 0 if not found
        pool_volume_by_token_usd.push(token_usd_volume);
    }

    (pool_volume_by_token_native, pool_volume_by_token_usd)
}

fn create_protocol_financials_daily_snapshot(
    clock: &Clock,
    tables: &mut Tables,
    day_id: &i64,
    protocol_tvl_store: &StoreGetBigDecimal,
    protocol_volume_store: &StoreGetBigDecimal,
) {
    let tvl_usd = protocol_tvl_store
        .get_last(StoreKey::protocol_tvl_key())
        .unwrap();
    let daily_volume = protocol_volume_store
        .get_last(StoreKey::protocol_daily_volume_usd_key(&day_id))
        .unwrap();
    let cumulative_volume = protocol_volume_store
        .get_last(StoreKey::protocol_volume_usd_key())
        .unwrap();
    tables
        .create_row(
            "FinancialsDailySnapshot",
            EntityKey::protocol_daily_financials_key(&day_id),
        )
        .set("protocol", EntityKey::protocol_key())
        .set("totalValueLockedUSD", tvl_usd)
        .set("dailyVolumeUSD", daily_volume)
        .set("cumulativeVolumeUSD", cumulative_volume)
        .set("blockNumber", BigInt::from(clock.number))
        .set(
            "timestamp",
            BigInt::from(clock.timestamp.clone().unwrap().seconds),
        );
}

fn create_protocol_entity(tables: &mut Tables, clock: &Clock) {
    // TODO: We should pass in the start block once we provide multi-network support.
    if clock.number.eq(&9456293) {
        tables
            .create_row("DexAmmProtocol", utils::get_protocol_id())
            .set("name", constants::protocol::NAME)
            .set("slug", constants::protocol::SLUG)
            .set("schemaVersion", constants::protocol::SCHEMA_VERSION)
            .set("subgraphVersion", constants::protocol::SUBGRAPH_VERSION)
            .set(
                "methodologyVersion",
                constants::protocol::METHODOLOGY_VERSION,
            )
            .set("network", DEFAULT_NETWORK)
            .set("type", constants::protocol_type::EXCHANGE)
            .set("totalValueLockedUSD", BigDecimal::zero())
            .set("protocolControlledValueUSD", BigDecimal::zero())
            .set("cumulativeVolumeUSD", BigDecimal::zero())
            .set("cumulativeSupplySideRevenueUSD", BigDecimal::zero())
            .set("cumulativeProtocolSideRevenueUSD", BigDecimal::zero())
            .set("cumulativeTotalRevenueUSD", BigDecimal::zero())
            .set("cumulativeUniqueUsers", 0)
            .set("totalPoolCount", 0)
            .set("_poolIds", Vec::<String>::new());
    }
}

fn create_pool_entity(tables: &mut Tables, pool: &Pool, pool_fees: &PoolFees) {
    let input_token_addresses: Vec<String> = pool
        .input_tokens
        .iter()
        .map(|t| format::format_address_string(&t.address))
        .collect();
    let input_tokens_ordered: Vec<String> = pool
        .input_tokens_ordered
        .iter()
        .map(|t| format_address_string(&t))
        .collect();

    // There is no liquidity when a Pool is first deployed, so we set the balances and weights to zero.
    let input_token_balances = vec![BigInt::zero(); input_token_addresses.len()];
    let input_token_weights = vec![BigDecimal::zero(); input_token_addresses.len()];

    tables
        .create_row(
            "LiquidityPool",
            format::format_address_string(&pool.address),
        )
        .set("protocol", utils::get_protocol_id())
        .set("name", &pool.name)
        .set("symbol", &pool.symbol)
        .set("inputTokens", input_token_addresses)
        .set("_inputTokensOrdered", input_tokens_ordered)
        .set(
            "outputToken",
            format::format_address_string(&pool.output_token_ref().address),
        )
        .set("fees", pool_fees.string_ids())
        .set("isSingleSided", false)
        .set("createdTimestamp", BigInt::from(pool.created_at_timestamp))
        .set(
            "createdBlockNumber",
            BigInt::from(pool.created_at_block_number),
        )
        .set("totalValueLockedUSD", BigDecimal::zero())
        .set("cumulativeSupplySideRevenueUSD", BigDecimal::zero())
        .set("cumulativeProtocolSideRevenueUSD", BigDecimal::zero())
        .set("cumulativeTotalRevenueUSD", BigDecimal::zero())
        .set("cumulativeVolumeUSD", BigDecimal::zero())
        .set("inputTokenBalances", input_token_balances)
        .set("inputTokenWeights", input_token_weights)
        .set("outputTokenSupply", BigInt::zero())
        .set("outputTokenPriceUSD", BigDecimal::zero())
        .set("stakedOutputTokenAmount", BigInt::zero())
        .set(
            "_registryAddress",
            format::format_address_string(&pool.registry_address),
        )
        .set("_isMetapool", &pool.is_metapool);
}

fn create_pool_fee_entities(tables: &mut Tables, pool_fees: &PoolFees) {
    create_pool_fee_entity(tables, pool_fees.trading_fee());
    create_pool_fee_entity(tables, pool_fees.protocol_fee());
    create_pool_fee_entity(tables, pool_fees.lp_fee());
}

fn create_pool_fee_entity(tables: &mut Tables, fee: &PoolFee) {
    tables
        .create_row("LiquidityPoolFee", &fee.id)
        .set("feePercentage", fee.fee_percentage_big_decimal())
        .set("feeType", fee.fee_type().as_str_name());
}

fn update_pool_fee_entities(tables: &mut Tables, pool_fees: &PoolFees) {
    update_pool_fee_entity(tables, pool_fees.trading_fee());
    update_pool_fee_entity(tables, pool_fees.protocol_fee());
    update_pool_fee_entity(tables, pool_fees.lp_fee());
}

fn update_pool_fee_entity(tables: &mut Tables, fee: &PoolFee) {
    tables
        .update_row("LiquidityPoolFee", &fee.id)
        .set("feePercentage", fee.fee_percentage_big_decimal())
        .set("feeType", fee.fee_type().as_str_name());
}

fn create_pool_token_entities(
    tables: &mut Tables,
    pool: &Pool,
    tokens_store: &StoreGetInt64,
) -> Result<(), Error> {
    let pool_tokens: Vec<Token> = pool.get_all_tokens();

    for token in pool_tokens {
        let token_address = token.address;

        match tokens_store.get_at(pool.log_ordinal, StoreKey::token_key(&token_address)) {
            Some(count) => {
                // If count is one, this is the first time we have seen this token,
                // and we only need to create a token entity once.
                if count.eq(&1) {
                    tables
                        .create_row("Token", format::format_address_string(&token_address))
                        .set("name", token.name)
                        .set("symbol", token.symbol)
                        .set("decimals", token.decimals as i32)
                        .set("isBasePoolLpToken", token.is_base_pool_lp_token)
                        .set("lastPriceUSD", BigDecimal::zero())
                        .set("_totalSupply", BigInt::zero())
                        .set("_totalValueLockedUSD", BigDecimal::zero())
                        .set("_largePriceChangeBuffer", 0)
                        .set("_largeTVLImpactBuffer", 0);
                }
            }
            None => {
                return Err(anyhow!(
                    "Pool contains token with address {} that does not exist in the store",
                    token_address
                ));
            }
        }
    }
    Ok(())
}

fn create_pool_events_entities(
    tables: &mut Tables,
    pool_events: Vec<PoolEvent>,
    pools_store: &StoreGetProto<Pool>,
    output_token_supply_store: &StoreGetBigInt,
    input_token_balances_store: &StoreGetBigInt,
    pool_tvl_store: &StoreGetBigDecimal,
    protocol_tvl_store: &StoreGetBigDecimal,
) {
    for event in &pool_events {
        if let Some(event_type) = &event.r#type {
            match event_type {
                Type::DepositEvent(deposit) => {
                    if let Some(pool) =
                        pools_store.get_last(StoreKey::pool_key(&event.pool_address))
                    {
                        create_deposit_entity(tables, &pool, &event, &deposit);
                        update_pool_output_token_supply(tables, &event, output_token_supply_store);
                        update_input_token_balances(
                            tables,
                            &event,
                            &pool.input_tokens,
                            input_token_balances_store,
                            pool_tvl_store,
                        );
                    }
                }
                Type::WithdrawEvent(withdraw) => {
                    if let Some(pool) =
                        pools_store.get_last(StoreKey::pool_key(&event.pool_address))
                    {
                        create_withdraw_entity(tables, &pool, &event, &withdraw);
                        update_pool_output_token_supply(tables, &event, output_token_supply_store);
                        update_input_token_balances(
                            tables,
                            &event,
                            &pool.input_tokens,
                            input_token_balances_store,
                            pool_tvl_store,
                        );
                    }
                }
                Type::SwapEvent(swap) => {
                    if let Some(pool) =
                        pools_store.get_last(StoreKey::pool_key(&event.pool_address))
                    {
                        create_swap_entity(tables, &event, &swap);
                        update_input_token_balances(
                            tables,
                            &event,
                            &pool.input_tokens,
                            input_token_balances_store,
                            pool_tvl_store,
                        )
                    }
                }
                Type::SwapUnderlyingEvent(swap_underlying) => {
                    if let Some(pool) =
                        pools_store.get_last(StoreKey::pool_key(&event.pool_address))
                    {
                        create_swap_underlying_entity(tables, &event, swap_underlying);
                        update_input_token_balances(
                            tables,
                            &event,
                            &pool.input_tokens,
                            input_token_balances_store,
                            pool_tvl_store,
                        )
                    }
                }
            }
        }
    }

    if !pool_events.is_empty() {
        // Retrieve the latest TVL from the store
        if let Some(tvl) = protocol_tvl_store.get_last(StoreKey::protocol_tvl_key()) {
            // Update the DexAmmProtocol entity with the new `totalValueLockedUSD` value
            tables
                .update_row("DexAmmProtocol", EntityKey::protocol_key())
                .set("totalValueLockedUSD", tvl);
        }
    }
}

fn update_pool_output_token_supply(
    tables: &mut Tables,
    event: &PoolEvent,
    output_token_supply_store: &StoreGetBigInt,
) {
    let output_token_supply = output_token_supply_store
        .get_at(
            event.log_ordinal,
            StoreKey::output_token_supply_key(&event.pool_address),
        )
        .unwrap_or(BigInt::zero());
    tables
        .update_row(
            "LiquidityPool",
            EntityKey::liquidity_pool_key(&event.pool_address),
        )
        .set("outputTokenSupply", output_token_supply);
}

fn get_input_token_balances(
    pool_address: &str,
    input_tokens: &Vec<Token>,
    input_token_balances_store: &StoreGetBigInt,
) -> Vec<BigInt> {
    input_tokens
        .iter()
        .map(|token| {
            let input_token_balance_key =
                StoreKey::input_token_balance_key(&pool_address, &token.address);
            input_token_balances_store
                .get_last(input_token_balance_key)
                .unwrap_or_else(|| {
                    substreams::log::debug!(
                        "No input token balance found for pool {} and token {}",
                        pool_address,
                        token.address
                    );
                    BigInt::zero()
                })
        })
        .collect()
}

fn get_input_token_weights(pool: &Pool, pool_tvl_store: &StoreGetBigDecimal) -> Vec<BigDecimal> {
    if let Some(pool_tvl) = pool_tvl_store.get_last(StoreKey::pool_tvl_key(&pool.address)) {
        if pool_tvl == BigDecimal::zero() {
            vec![BigDecimal::zero(); pool.input_tokens.len()]
        } else {
            pool.input_tokens
                .iter()
                .map(|token| {
                    pool_tvl_store
                        .get_last(StoreKey::pool_token_tvl_key(&pool.address, &token.address))
                        .map_or(BigDecimal::zero(), |token_tvl| {
                            token_tvl.div(&pool_tvl).mul(BigDecimal::from(100))
                        })
                })
                .collect()
        }
    } else {
        vec![BigDecimal::zero(); pool.input_tokens.len()]
    }
}

fn update_input_token_balances(
    tables: &mut Tables,
    event: &PoolEvent,
    input_tokens: &Vec<Token>,
    input_token_balances_store: &StoreGetBigInt,
    pool_tvl_store: &StoreGetBigDecimal,
) {
    let input_token_balances = get_input_token_balances(
        &event.pool_address,
        input_tokens,
        input_token_balances_store,
    );

    let tvl = pool_tvl_store
        .get_last(StoreKey::pool_tvl_key(&event.pool_address))
        .unwrap_or(BigDecimal::zero());

    tables
        .update_row(
            "LiquidityPool",
            EntityKey::liquidity_pool_key(&event.pool_address),
        )
        .set("inputTokenBalances", input_token_balances)
        .set("totalValueLockedUSD", tvl);
}

fn create_deposit_entity(
    tables: &mut Tables,
    pool: &Pool,
    event: &PoolEvent,
    deposit: &DepositEvent,
) {
    let (input_tokens, input_token_amounts): (Vec<String>, Vec<BigInt>) = deposit
        .input_tokens
        .iter()
        .map(|t| {
            (
                format::format_address_string(&t.token_address),
                BigInt::from(t.amount.parse::<u64>().unwrap_or_default()),
            )
        })
        .unzip();
    let output_token_amount =
        BigInt::try_from(deposit.output_token.as_ref().unwrap().clone().amount).unwrap();
    tables
        .create_row(
            "Deposit",
            EntityKey::deposit_key(&event.transaction_hash, &event.log_index),
        )
        .set(
            "hash",
            format::format_address_string(&event.transaction_hash),
        )
        .set("logIndex", event.log_index as i32)
        .set("protocol", utils::get_protocol_id())
        .set("to", format::format_address_string(&event.to_address))
        .set("from", format::format_address_string(&event.from_address))
        .set("blockNumber", BigInt::from(event.block_number))
        .set("timestamp", BigInt::from(event.timestamp))
        .set("inputTokens", input_tokens)
        .set(
            "outputToken",
            format::format_address_string(&pool.output_token_ref().address),
        )
        .set("inputTokenAmounts", input_token_amounts)
        .set("outputTokenAmount", BigInt::from(output_token_amount))
        .set("amountUSD", BigDecimal::zero())
        .set("pool", format::format_address_string(&event.pool_address));
}

fn create_withdraw_entity(
    tables: &mut Tables,
    pool: &Pool,
    event: &PoolEvent,
    withdraw: &WithdrawEvent,
) {
    let (input_tokens, input_token_amounts): (Vec<String>, Vec<BigInt>) = withdraw
        .input_tokens
        .iter()
        .map(|t| {
            (
                format::format_address_string(&t.token_address),
                BigInt::from(t.amount.parse::<u64>().unwrap_or_default()),
            )
        })
        .unzip();
    let output_token_amount =
        BigInt::try_from(withdraw.output_token.as_ref().unwrap().amount.clone()).unwrap();
    tables
        .create_row(
            "Withdraw",
            EntityKey::withdraw_key(&event.transaction_hash, &event.log_index),
        )
        .set(
            "hash",
            format::format_address_string(&event.transaction_hash),
        )
        .set("logIndex", event.log_index as i32)
        .set("protocol", utils::get_protocol_id())
        .set("to", format::format_address_string(&event.to_address))
        .set("from", format::format_address_string(&event.from_address))
        .set("blockNumber", BigInt::from(event.block_number))
        .set("timestamp", BigInt::from(event.timestamp))
        .set("inputTokens", input_tokens)
        .set(
            "outputToken",
            format::format_address_string(&pool.output_token_ref().address),
        )
        .set("inputTokenAmounts", input_token_amounts)
        .set("outputTokenAmount", BigInt::from(output_token_amount))
        .set("amountUSD", BigDecimal::zero())
        .set("pool", format::format_address_string(&event.pool_address));
}

fn create_swap_entity(tables: &mut Tables, event: &PoolEvent, swap: &SwapEvent) {
    tables
        .create_row(
            "Swap",
            EntityKey::swap_key(&event.transaction_hash, &event.log_index),
        )
        .set(
            "hash",
            format::format_address_string(&event.transaction_hash),
        )
        .set("logIndex", event.log_index as i32)
        .set("protocol", utils::get_protocol_id())
        .set("to", format::format_address_string(&event.to_address))
        .set("from", format::format_address_string(&event.from_address))
        .set("blockNumber", BigInt::from(event.block_number))
        .set("timestamp", BigInt::from(event.timestamp))
        .set(
            "tokenIn",
            format::format_address_string(&swap.token_in.as_ref().unwrap().token_address),
        )
        .set(
            "amountIn",
            BigInt::from(
                swap.token_in
                    .as_ref()
                    .unwrap()
                    .amount
                    .parse::<u64>()
                    .unwrap_or_default(),
            ),
        )
        .set("amountInUSD", BigDecimal::zero())
        .set(
            "tokenOut",
            format::format_address_string(&swap.token_out.as_ref().unwrap().token_address),
        )
        .set(
            "amountOut",
            BigInt::from(
                swap.token_out
                    .as_ref()
                    .unwrap()
                    .amount
                    .parse::<u64>()
                    .unwrap_or_default(),
            ),
        )
        .set("amountOutUSD", BigDecimal::zero())
        .set("pool", format::format_address_string(&event.pool_address));
}

fn create_swap_underlying_entity(
    tables: &mut Tables,
    event: &PoolEvent,
    swap_underlying: &SwapUnderlyingEvent,
) {
    tables
        .create_row(
            "Swap",
            EntityKey::swap_key(&event.transaction_hash, &event.log_index),
        )
        .set(
            "hash",
            format::format_address_string(&event.transaction_hash),
        )
        .set("logIndex", event.log_index as i32)
        .set("protocol", utils::get_protocol_id())
        .set("to", format::format_address_string(&event.to_address))
        .set("from", format::format_address_string(&event.from_address))
        .set("blockNumber", BigInt::from(event.block_number))
        .set("timestamp", BigInt::from(event.timestamp))
        .set(
            "tokenIn",
            format::format_address_string(
                &swap_underlying.token_in.as_ref().unwrap().token_address,
            ),
        )
        .set(
            "amountIn",
            BigInt::from(
                swap_underlying
                    .token_in
                    .as_ref()
                    .unwrap()
                    .amount
                    .parse::<u64>()
                    .unwrap_or_default(),
            ),
        )
        .set("amountInUSD", BigDecimal::zero())
        .set(
            "tokenOut",
            format::format_address_string(
                &swap_underlying.token_out.as_ref().unwrap().token_address,
            ),
        )
        .set(
            "amountOut",
            BigInt::from(
                swap_underlying
                    .token_out
                    .as_ref()
                    .unwrap()
                    .amount
                    .parse::<u64>()
                    .unwrap_or_default(),
            ),
        )
        .set("amountOutUSD", BigDecimal::zero())
        .set("pool", format::format_address_string(&event.pool_address));
}
