use anyhow::anyhow;
use substreams::{
    errors::Error,
    pb::substreams::Clock,
    scalar::{BigDecimal, BigInt},
    store::{
        StoreAdd, StoreAddInt64, StoreGet, StoreGetInt64, StoreGetProto, StoreNew, StoreSet,
        StoreSetProto,
    },
    Hex,
};
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::{
    pb::eth::v2::{self as eth},
    Event, NULL_ADDRESS,
};

use crate::{
    abi::registry::events::{
        BasePoolAdded, CryptoPoolDeployed, MetaPoolDeployed, PlainPoolDeployed, PoolAdded1,
        PoolAdded2,
    },
    network_config::{
        PoolDetails, DEFAULT_NETWORK, MISSING_OLD_POOLS_DATA, POOL_REGISTRIES, PROTOCOL_ADDRESS,
    },
    pb::curve::types::v1::{events::PoolEvent, Events, Pool, Pools, Token},
    rpc::{pool, registry, token},
    utils::{
        create_missing_pool, create_pool, extract_swap_event, extract_transfer_event,
        format_address_string, format_address_vec, get_and_sort_input_tokens,
    },
};

mod abi;
mod constants;
mod network_config;
mod pb;
mod rpc;
mod utils;

substreams_ethereum::init!();

fn map_pool_added_1_events(
    blk: &eth::Block,
    pools: &mut Pools,
    address: [u8; 20],
) -> Result<(), Error> {
    pools.pools.append(
        &mut blk
            .events::<PoolAdded1>(&[&address])
            .filter_map(|(event, log)| {
                let lp_token_address = match registry::get_lp_token_address_from_registry(
                    &event.pool,
                    &address.to_vec(),
                ) {
                    Ok(addr) => addr,
                    Err(e) => {
                        substreams::log::debug!("Error in `map_pool_added_1_events`: {:?}", e);
                        return None;
                    }
                };
                let lp_token = match token::create_token(&lp_token_address, &event.pool) {
                    Ok(token) => token,
                    Err(e) => {
                        substreams::log::debug!("Error in `map_pool_added_1_events`: {:?}", e);
                        return None;
                    }
                };
                let (input_tokens, input_tokens_ordered) =
                    match get_and_sort_input_tokens(&event.pool) {
                        Ok(result) => result,
                        Err(e) => {
                            substreams::log::debug!("Error in `map_pool_added_1_events`: {:?}", e);
                            return None;
                        }
                    };
                Some(create_pool(
                    Hex::encode(&event.pool),
                    Hex::encode(address),
                    lp_token,
                    input_tokens_ordered,
                    input_tokens,
                    utils::is_metapool(&event.pool),
                    &log,
                    blk,
                ))
            })
            .collect(),
    );
    Ok(())
}

fn map_pool_added_2_events(
    blk: &eth::Block,
    pools: &mut Pools,
    address: [u8; 20],
) -> Result<(), Error> {
    pools.pools.append(
        &mut blk
            .events::<PoolAdded2>(&[&address])
            .filter_map(|(event, log)| {
                let lp_token_address = match registry::get_lp_token_address_from_registry(
                    &event.pool,
                    &address.to_vec(),
                ) {
                    Ok(addr) => addr,
                    Err(e) => {
                        substreams::log::debug!("Error in `map_pool_added_2_events`: {:?}", e);
                        return None;
                    }
                };
                let lp_token = match token::create_token(&lp_token_address, &event.pool) {
                    Ok(token) => token,
                    Err(e) => {
                        substreams::log::debug!("Error in `map_pool_added_2_events`: {:?}", e);
                        return None;
                    }
                };
                let (input_tokens, input_tokens_ordered) =
                    match get_and_sort_input_tokens(&event.pool) {
                        Ok(result) => result,
                        Err(e) => {
                            substreams::log::debug!("Error in `map_pool_added_2_events`: {:?}", e);
                            return None;
                        }
                    };

                Some(create_pool(
                    Hex::encode(&event.pool),
                    Hex::encode(address),
                    lp_token,
                    input_tokens_ordered,
                    input_tokens,
                    utils::is_metapool(&event.pool),
                    &log,
                    blk,
                ))
            })
            .collect(),
    );
    Ok(())
}

fn map_base_pool_added_events(
    blk: &eth::Block,
    pools: &mut Pools,
    address: [u8; 20],
) -> Result<(), Error> {
    pools.pools.append(
        &mut blk
            .events::<BasePoolAdded>(&[&address])
            .filter_map(|(event, log)| {
                let lp_token_address = match pool::get_lp_token_address_from_pool(&event.base_pool)
                {
                    Ok(addr) => addr,
                    Err(e) => {
                        substreams::log::debug!("Error in `map_base_pool_added_events`: {:?}", e);
                        return None;
                    }
                };
                let lp_token = match token::create_token(&lp_token_address, &event.base_pool) {
                    Ok(token) => token,
                    Err(e) => {
                        substreams::log::debug!("Error in `map_base_pool_added_events`: {:?}", e);
                        return None;
                    }
                };
                let (input_tokens, input_tokens_ordered) =
                    match get_and_sort_input_tokens(&event.base_pool) {
                        Ok(result) => result,
                        Err(e) => {
                            substreams::log::debug!(
                                "Error in `map_base_pool_added_events`: {:?}",
                                e
                            );
                            return None;
                        }
                    };
                Some(create_pool(
                    Hex::encode(&event.base_pool),
                    Hex::encode(address),
                    lp_token,
                    input_tokens_ordered,
                    input_tokens,
                    utils::is_metapool(&event.base_pool),
                    &log,
                    blk,
                ))
            })
            .collect(),
    );
    Ok(())
}

fn map_crypto_pool_deployed_events(
    blk: &eth::Block,
    pools: &mut Pools,
    address: [u8; 20],
) -> Result<(), Error> {
    pools.pools.append(
        &mut blk
            .events::<CryptoPoolDeployed>(&[&address])
            .filter_map(|(event, log)| {
                // The minter of the LP token is the liquidity pool contract.
                let pool_address = match token::get_token_minter(&event.token) {
                    Ok(minter) => minter,
                    Err(e) => {
                        substreams::log::debug!(
                            "Error in `map_crypto_pool_deployed_events`: {:?}",
                            e
                        );
                        return None;
                    }
                };
                let lp_token = match token::create_token(&event.token, &pool_address) {
                    Ok(token) => token,
                    Err(e) => {
                        substreams::log::debug!(
                            "Error in `map_crypto_pool_deployed_events`: {:?}",
                            e
                        );
                        return None;
                    }
                };
                let (input_tokens, input_tokens_ordered) =
                    match get_and_sort_input_tokens(&pool_address) {
                        Ok(result) => result,
                        Err(e) => {
                            substreams::log::debug!(
                                "Error in `map_crypto_pool_deployed_events`: {:?}",
                                e
                            );
                            return None;
                        }
                    };
                Some(create_pool(
                    Hex::encode(&pool_address),
                    Hex::encode(address),
                    lp_token,
                    input_tokens_ordered,
                    input_tokens,
                    utils::is_metapool(&pool_address),
                    &log,
                    blk,
                ))
            })
            .collect(),
    );
    Ok(())
}

fn map_plain_pool_deployed_events(
    blk: &eth::Block,
    pools: &mut Pools,
    address: [u8; 20],
) -> Result<(), Error> {
    pools.pools.append(
        &mut blk
            .events::<PlainPoolDeployed>(&[&address])
            .filter_map(|(_event, log)| {
                let transfer = match extract_transfer_event(&log) {
                    Ok(event) => event,
                    Err(e) => {
                        substreams::log::debug!(
                            "Error in `map_plain_pool_deployed_events`: {:?}",
                            e
                        );
                        return None;
                    }
                };
                let lp_token = match token::create_token(&transfer.receiver, &transfer.receiver) {
                    Ok(token) => token,
                    Err(e) => {
                        substreams::log::debug!(
                            "Error in `map_plain_pool_deployed_events`: {:?}",
                            e
                        );
                        return None;
                    }
                };
                let (input_tokens, input_tokens_ordered) =
                    match get_and_sort_input_tokens(&transfer.receiver) {
                        Ok(result) => result,
                        Err(e) => {
                            substreams::log::debug!(
                                "Error in `map_crypto_pool_deployed_events`: {:?}",
                                e
                            );
                            return None;
                        }
                    };
                Some(create_pool(
                    Hex::encode(&transfer.receiver),
                    Hex::encode(address),
                    lp_token,
                    input_tokens_ordered,
                    input_tokens,
                    utils::is_metapool(&transfer.receiver),
                    &log,
                    blk,
                ))
            })
            .collect(),
    );
    Ok(())
}

fn map_meta_pool_deployed_events(
    blk: &eth::Block,
    pools: &mut Pools,
    address: [u8; 20],
) -> Result<(), Error> {
    pools.pools.append(
        &mut blk
            .events::<MetaPoolDeployed>(&[&address])
            .filter_map(|(_event, log)| {
                let transfer = match extract_transfer_event(&log) {
                    Ok(event) => event,
                    Err(e) => {
                        substreams::log::debug!(
                            "Error in `map_plain_pool_deployed_events`: {:?}",
                            e
                        );
                        return None;
                    }
                };
                let lp_token = match token::create_token(&transfer.receiver, &transfer.receiver) {
                    Ok(token) => token,
                    Err(e) => {
                        substreams::log::debug!(
                            "Error in `map_plain_pool_deployed_events`: {:?}",
                            e
                        );
                        return None;
                    }
                };
                let (input_tokens, input_tokens_ordered) =
                    match get_and_sort_input_tokens(&transfer.receiver) {
                        Ok(result) => result,
                        Err(e) => {
                            substreams::log::debug!(
                                "Error in `map_crypto_pool_deployed_events`: {:?}",
                                e
                            );
                            return None;
                        }
                    };
                Some(create_pool(
                    Hex::encode(&transfer.receiver),
                    Hex::encode(address),
                    lp_token,
                    input_tokens_ordered,
                    input_tokens,
                    utils::is_metapool(&transfer.receiver),
                    &log,
                    blk,
                ))
            })
            .collect(),
    );
    Ok(())
}

fn add_missing_pool(blk: &eth::Block, pools: &mut Pools, pool: &PoolDetails) -> Result<(), Error> {
    let pool_address = pool.address.to_vec();
    let lp_token = match token::create_token(&pool.lp_token.to_vec(), &pool_address) {
        Ok(token) => token,
        Err(e) => {
            return Err(anyhow!("Error in `add_missing_pools`: {:?}", e));
        }
    };
    let (input_tokens, input_tokens_ordered) = match get_and_sort_input_tokens(&pool_address) {
        Ok(result) => result,
        Err(e) => {
            return Err(anyhow!("Error in `add_missing_pools`: {:?}", e));
        }
    };
    let hash = blk
        .transactions()
        .find(|trx| trx.to == pool_address)
        .map(|tx| tx.hash.clone())
        .unwrap_or_else(|| NULL_ADDRESS.to_vec());

    pools.pools.push(create_missing_pool(
        Hex::encode(pool_address),
        Hex::encode(NULL_ADDRESS.to_vec()),
        lp_token,
        input_tokens_ordered,
        input_tokens,
        false,
        blk,
        hash,
    ));

    substreams::log::info!("Added missing pool: {:?}", pool);
    Ok(())
}

#[substreams::handlers::map]
fn map_pools_created(blk: eth::Block) -> Result<Pools, Vec<Error>> {
    let mut pools = Pools::default();

    // Need to add pools that were deployed before any registry/factory contracts handled pool deployment
    for &(_pool_address, ref pool_details) in MISSING_OLD_POOLS_DATA.iter() {
        if pool_details.start_block == blk.number {
            match add_missing_pool(&blk, &mut pools, pool_details) {
                Ok(_) => {}
                Err(e) => {
                    return Err(vec![e]);
                }
            }
            // No old pools were deployed on the same block, so we can skip the rest of the loop
            break;
        }
    }

    // This calls each event mapping func for each contract address.
    // As we nothing is returned with the `Ok` variant, we can just ignore it,
    // and use the `Err` variant to collect any errors that occur.
    let errors: Vec<Error> = POOL_REGISTRIES
        .iter()
        .flat_map(|&contract| {
            [
                map_pool_added_1_events(&blk, &mut pools, contract),
                map_pool_added_2_events(&blk, &mut pools, contract),
                map_base_pool_added_events(&blk, &mut pools, contract),
                map_crypto_pool_deployed_events(&blk, &mut pools, contract),
                map_plain_pool_deployed_events(&blk, &mut pools, contract),
                map_meta_pool_deployed_events(&blk, &mut pools, contract),
            ]
        })
        .filter_map(Result::err)
        .collect();

    if errors.is_empty() {
        return Ok(pools);
    }
    Err(errors)
}

#[substreams::handlers::store]
fn store_pools_created(pools: Pools, store: StoreSetProto<Pool>) {
    for pool in pools.pools {
        let address = pool.address.clone();
        store.set(pool.log_ordinal, format!("pool:{}", address), &pool)
    }
}

#[substreams::handlers::store]
pub fn store_pool_count(pools: Pools, store: StoreAddInt64) {
    for pool in pools.pools {
        store.add(pool.log_ordinal, format!("protocol:poolCount"), 1)
    }
}

// This will be used in the `graph_out` map to create Token entities.
#[substreams::handlers::store]
pub fn store_tokens(pools: Pools, store: StoreAddInt64) {
    for pool in pools.pools {
        let addr_output_token = pool.output_token.unwrap().address;
        let addr_input_tokens: Vec<String> = pool
            .input_tokens
            .iter()
            .map(|t| t.address.clone())
            .collect();

        let mut keys: Vec<String> = Vec::new();
        keys.push(format!("token:{addr_output_token}"));
        for addr in addr_input_tokens {
            keys.push(format!("token:{addr}"));
        }

        store.add_many(pool.log_ordinal, &keys, 1);
    }
}

#[substreams::handlers::map]
pub fn map_extract_pool_events(
    blk: eth::Block,
    pools: StoreGetProto<Pool>,
) -> Result<Events, Error> {
    // Initialise events
    let mut events = Events::default();

    // Init the events fields
    let mut pool_events: Vec<PoolEvent> = Vec::new();

    // Check if event is coming from the pool contract
    for trx in blk.transactions() {
        for (log, _call) in trx.logs_with_calls() {
            let pool_address = Hex::encode(&log.address);
            let pool_opt = pools.get_last(format!("pool:{}", pool_address));

            if let Some(pool) = pool_opt {
                // TODO: Test and resolve both TokenExchanges
                //       The only differene between TE1/2 is the type of uint in the actual ABI.
                //       We will need to check if they decode for the same events and handle accordingly.
                //       If they do, we may be able to remove the check for both events.
                //
                //       TokenExchange1

                // TODO: Consider using a match expression similar to the message enum match here:
                //       (https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/reference/expressions/match-expr.html)
                if let Some(swap) = abi::pool::events::TokenExchange1::match_and_decode(&log) {
                    extract_swap_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        &pool_address,
                        &swap.sold_id,
                        &swap.bought_id,
                        &swap.tokens_sold,
                        &swap.tokens_bought,
                        &swap.buyer,
                        false,
                    );
                }
                // TokenExchange2
                else if let Some(swap) = abi::pool::events::TokenExchange2::match_and_decode(&log)
                {
                    extract_swap_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        &pool_address,
                        &swap.sold_id,
                        &swap.bought_id,
                        &swap.tokens_sold,
                        &swap.tokens_bought,
                        &swap.buyer,
                        false,
                    );
                } else if let Some(swap) =
                    abi::pool::events::TokenExchangeUnderlying::match_and_decode(&log)
                {
                    extract_swap_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        &pool_address,
                        &swap.sold_id,
                        &swap.bought_id,
                        &swap.tokens_sold,
                        &swap.tokens_bought,
                        &swap.buyer,
                        true,
                    );
                }
            }
        }
    }
    events.pool_events = pool_events;
    Ok(events)
}

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

// TODO: There is a lot of code duplication here. This will be refactored in the future.
//       We can extract the common logic into a separate function called process_pool_event,
//       which could look something like this:
//
// fn process_event<F>(event_data: &[u8], address: [u8; 20], blk: &eth::Block, process_logic: F) -> Option<Pool>
// where
//     F: Fn(&[u8], [u8; 20], &eth::Block) -> Result<Pool, Error>,
// {
//     match process_logic(event_data, address, blk) {
//         Ok(pool) => Some(pool),
//         Err(e) => {
//             substreams::log::debug!("Error processing event: {:?}", e);
//             None
//         }
//     }
// }
//
//       Specific event handling logic can then be passed in as a closure.
