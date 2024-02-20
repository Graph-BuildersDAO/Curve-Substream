use std::{cell::RefCell, collections::HashSet, rc::Rc};

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
        pool_utils::{get_input_token_balances, get_input_token_weights},
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
    timeframe_management::snapshot::snapshot_utils::manage_timeframe_snapshots,
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
    current_time_deltas: Deltas<DeltaInt64>,
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

    manage_timeframe_snapshots(
        &clock,
        &current_time_deltas,
        &mut tables,
        &pool_count_store,
        &pool_addresses_store,
        &pools_store,
        &pool_tvl_store,
        &pool_volume_usd_store,
        &pool_volume_native_store,
        &protocol_tvl_store,
        &protocol_volume_store,
        &input_token_balances_store,
        &output_token_supply_store,
        &uniswap_prices,
        &chainlink_prices,
    );

    Ok(tables.to_entity_changes())
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
