use network_config::CONTRACTS;
use rpc::{pool, registry, token};
use substreams::{errors::Error, Hex};
use substreams_ethereum::pb::eth::v2::{self as eth};

mod abi;
mod constants;
mod network_config;
mod pb;
mod rpc;
mod utils;

use abi::registry::events::{
    BasePoolAdded, CryptoPoolDeployed, MetaPoolDeployed, PlainPoolDeployed, PoolAdded1, PoolAdded2,
};
use pb::curve::types::v1::Pools;
use utils::{create_pool, extract_transfer_event, get_and_sort_input_tokens};

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

#[substreams::handlers::map]
fn map_pools_created(blk: eth::Block) -> Result<Pools, Error> {
    let mut pools = Pools::default();

    // TODO - Review: Should we use the `?` operator. This will stop the entire function call and return an error.
    for contract in CONTRACTS {
        map_pool_added_1_events(&blk, &mut pools, contract)?;
        map_pool_added_2_events(&blk, &mut pools, contract)?;
        map_base_pool_added_events(&blk, &mut pools, contract)?;
        map_crypto_pool_deployed_events(&blk, &mut pools, contract)?;
        map_plain_pool_deployed_events(&blk, &mut pools, contract)?;
        map_meta_pool_deployed_events(&blk, &mut pools, contract)?;
    }

    Ok(pools)
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
