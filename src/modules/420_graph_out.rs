use anyhow::anyhow;
use substreams::{
    errors::Error,
    pb::substreams::Clock,
    scalar::{BigDecimal, BigInt},
    store::{StoreGet, StoreGetInt64},
    Hex,
};
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

use crate::{
    constants,
    network_config::{DEFAULT_NETWORK, PROTOCOL_ADDRESS},
    pb::curve::types::v1::{Pool, Pools, Token},
    rpc::pool::get_pool_fees,
    types::{PoolFee, PoolFees},
    utils::{format_address_string, format_address_vec},
};

// TODO: If this module gets too bulky, consider following an approach similar to Uniswap V2 SPS:
//       (https://github.com/messari/substreams/tree/1c148752f7eb6b75804542428630f7fa74bf6414/uniswap-v2/src/modules)
#[substreams::handlers::map]
pub fn graph_out(
    clock: Clock,
    pools: Pools,
    tokens_store: StoreGetInt64,
) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    create_protocol_entity(&mut tables, &clock);

    // Create entities related to Pool contract deployments
    for pool in pools.pools {
        let pool_address = Hex::decode(&pool.address)?;
        let pool_fees = get_pool_fees(&pool_address)?;

        create_pool_entities(&mut tables, &pool, &pool_fees);
        create_pool_fee_entities(&mut tables, &pool_fees);
        create_pool_token_entities(&mut tables, &pool, &tokens_store)?;
    }
    Ok(tables.to_entity_changes())
}

fn create_protocol_entity(tables: &mut Tables, clock: &Clock) {
    // TODO: We should pass in the start block once we provide multi-network support.
    if clock.number.eq(&9456293) {
        tables
            .create_row(
                "DexAmmProtocol",
                format_address_vec(&PROTOCOL_ADDRESS.to_vec()),
            )
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

fn create_pool_entities(tables: &mut Tables, pool: &Pool, pool_fees: &PoolFees) {
    let input_token_addresses: Vec<String> = pool
        .input_tokens
        .iter()
        .map(|t| format_address_string(&t.address))
        .collect();

    // There is no liquidity when a Pool is first deployed, so we set the balances and weights to zero.
    let input_token_balances = vec![BigInt::zero().to_string(); input_token_addresses.len()];
    let input_token_weights = vec![BigDecimal::zero().to_string(); input_token_addresses.len()];

    tables
        .create_row("LiquidityPool", format_address_string(&pool.address))
        .set("protocol", format_address_vec(&PROTOCOL_ADDRESS.to_vec()))
        .set("name", &pool.name)
        .set("symbol", &pool.symbol)
        .set("inputTokens", input_token_addresses)
        .set("_inputTokensOrdered", &pool.input_tokens_ordered)
        .set(
            "outputToken",
            format_address_string(&pool.output_token_ref().address),
        )
        .set("fees", pool_fees.string_ids())
        .set("isSingleSided", &pool.is_single_sided)
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
            format_address_string(&pool.registry_address),
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

        // TODO: We will be using store keys a lot. Could we make a module which handles everything related to the keys?
        //       https://github.com/messari/substreams/blob/master/uniswap-v2/src/store_key.rs
        match tokens_store.get_at(pool.log_ordinal, format!("token:{}", token_address)) {
            Some(count) => {
                // If count is one, this is the first time we have seen this token,
                // and we only need to create a token entity once.
                if count.eq(&1) {
                    tables
                        .create_row("Token", format_address_string(&token_address))
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
