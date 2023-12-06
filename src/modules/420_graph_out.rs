use anyhow::anyhow;
use substreams::{
    errors::Error,
    pb::substreams::Clock,
    scalar::{BigDecimal, BigInt},
    store::{StoreGet, StoreGetInt64},
};
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

use crate::{
    constants,
    network_config::{DEFAULT_NETWORK, PROTOCOL_ADDRESS},
    pb::curve::types::v1::{Pools, Token},
    utils::{format_address_string, format_address_vec},
};

// TODO: Eventually we will want to extract the `graph-out` logic into seperate functions.
//       Consider following an approach like UniswapV2 or V3 SPS's.
#[substreams::handlers::map]
pub fn graph_out(
    clock: Clock,
    pools: Pools,
    tokens_store: StoreGetInt64,
) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

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

    // Create Pool entities
    for pool in pools.pools {
        let input_token_addresses: Vec<String> = pool
            .input_tokens
            .iter()
            .map(|t| format_address_string(&t.address))
            .collect();

        let output_token = pool.output_token.as_ref().unwrap();

        tables
            .create_row("LiquidityPool", format_address_string(&pool.address))
            .set("protocol", format_address_vec(&PROTOCOL_ADDRESS.to_vec()))
            .set("name", &pool.name)
            .set("symbol", &pool.symbol)
            .set("inputTokens", input_token_addresses)
            .set("_inputTokensOrdered", &pool.input_tokens_ordered)
            .set("outputToken", format_address_string(&output_token.address))
            .set("isSingleSided", &pool.is_single_sided)
            .set("createdTimestamp", BigInt::from(pool.created_at_timestamp))
            .set(
                "createdBlockNumber",
                BigInt::from(pool.created_at_block_number),
            )
            .set(
                "_registryAddress",
                format_address_string(&pool.registry_address),
            )
            .set("_isMetapool", &pool.is_metapool);

        let ord = pool.log_ordinal;

        // Create Token entities for pool
        let pool_tokens: Vec<Token> = std::iter::once(output_token.to_owned())
            .chain(pool.input_tokens.into_iter())
            .collect();
        for token in pool_tokens {
            let token_address = token.address;
            // TODO: We will be using store keys a lot. Could we make a module which handles everything related to the keys?
            //       https://github.com/messari/substreams/blob/master/uniswap-v2/src/store_key.rs
            match tokens_store.get_at(ord, format!("token:{}", token_address)) {
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
                            .set("_totalSupply", BigInt::zero());
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
    }
    Ok(tables.to_entity_changes())
}
