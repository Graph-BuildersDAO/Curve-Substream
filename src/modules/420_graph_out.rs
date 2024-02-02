use anyhow::anyhow;
use substreams::{
    errors::Error,
    pb::substreams::Clock,
    scalar::{BigDecimal, BigInt},
    store::{
        DeltaInt64, Deltas, StoreGet, StoreGetBigDecimal, StoreGetBigInt, StoreGetInt64,
        StoreGetProto,
    },
    Hex,
};
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

use crate::{
    common::{
        format::{self, format_address_string},
        utils,
    },
    constants,
    network_config::DEFAULT_NETWORK,
    pb::curve::types::v1::{
        events::{
            pool_event::{DepositEvent, SwapEvent, Type, WithdrawEvent},
            PoolEvent,
        },
        Events, Pool, Pools, Token,
    },
    rpc::pool::get_pool_fees,
    store_key_manager::StoreKey,
    types::{PoolFee, PoolFees},
};

// TODO: If this module gets too bulky, consider following an approach similar to Uniswap V2 SPS:
//       (https://github.com/messari/substreams/tree/1c148752f7eb6b75804542428630f7fa74bf6414/uniswap-v2/src/modules)
#[substreams::handlers::map]
pub fn graph_out(
    clock: Clock,
    pools: Pools,
    events: Events,
    pools_store: StoreGetProto<Pool>,
    tokens_store: StoreGetInt64,
    output_token_supply_store: StoreGetBigInt,
    input_token_balances_store: StoreGetBigInt,
    pool_tvl_store: StoreGetBigDecimal,
    protocol_tvl_store: StoreGetBigDecimal,
) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    create_protocol_entity(&mut tables, &clock);

    // Create entities related to Pool contract deployments
    for pool in pools.pools {
        let pool_address = Hex::decode(&pool.address)?;
        let pool_fees = get_pool_fees(&pool_address)?;

        create_pool_entity(&mut tables, &pool, &pool_fees);
        create_pool_fee_entities(&mut tables, &pool_fees);
        create_pool_token_entities(&mut tables, &pool, &tokens_store)?;
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
        .create_row("LiquidityPoolFee", fee.get_id())
        .set("feePercentage", fee.get_fee_percentage())
        .set("feeType", fee.get_fee_type().as_str());
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
    for event in pool_events {
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
                            &pool.input_tokens_ordered,
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
                            &pool.input_tokens_ordered,
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
                            &pool.input_tokens_ordered,
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
                .update_row("DexAmmProtocol", utils::get_protocol_id())
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
            format::format_address_string(&event.pool_address),
        )
        .set("outputTokenSupply", output_token_supply);
}

fn update_input_token_balances(
    tables: &mut Tables,
    event: &PoolEvent,
    input_tokens: &Vec<String>,
    input_token_balances_store: &StoreGetBigInt,
    pool_tvl_store: &StoreGetBigDecimal,
) {
    let input_token_balances: Vec<BigInt> = input_tokens
        .iter()
        .map(|input_token| {
            let input_token_balance_key =
                StoreKey::input_token_balance_key(&event.pool_address, &input_token);
            let input_token_balance = match input_token_balances_store
                .get_at(event.log_ordinal, &input_token_balance_key)
            {
                Some(balance) => balance,
                None => input_token_balances_store
                    .get_last(&input_token_balance_key)
                    .unwrap_or_else(|| {
                        substreams::log::debug!(
                            "No input token balance found for pool {} and token {}",
                            &event.pool_address,
                            &input_token
                        );
                        BigInt::zero()
                    }),
            };
            input_token_balance
        })
        .collect();

    let tvl = pool_tvl_store
        .get_last(StoreKey::pool_tvl_key(&event.pool_address))
        .unwrap_or(BigDecimal::zero());

    tables
        .update_row(
            "LiquidityPool",
            format::format_address_string(&event.pool_address),
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
    let key = format!("deposit-0x{}-{}", event.transaction_hash, event.log_index);
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
        .create_row("Deposit", key)
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
    let key = format!("withdraw-0x{}-{}", event.transaction_hash, event.log_index);
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
        .create_row("Withdraw", key)
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
    let key = format!("swap-0x{}-{}", event.transaction_hash, event.log_index);
    tables
        .create_row("Swap", key)
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
