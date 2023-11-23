use network_config::CONTRACTS;
use rpc::{pool, registry, token};
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};

mod abi;

mod constants;
mod network_config;
mod pb;
mod rpc;
mod utils;

use abi::registry::events::{
    BasePoolAdded, CryptoPoolDeployed, MetaPoolDeployed, PlainPoolDeployed,
    PoolAdded1 as RegistryPoolAdded, PoolAdded2 as CryptoSwapPoolAdded,
};
use pb::curve::types::v1::{Pool, Pools};

substreams_ethereum::init!();

fn map_pool_added_1_events(blk: &eth::Block, pools: &mut Pools, address: [u8; 20]) {
    pools.pools.append(
        &mut blk
            .events::<RegistryPoolAdded>(&[&address])
            .filter_map(|(event, log)| {
                match registry::get_lp_token_address_from_registry(&event.pool, &address.to_vec()) {
                    Ok(lp_token_address) => {
                        if let Ok(lp_token) = token::create_token(&lp_token_address, &event.pool) {
                            if let Ok(input_tokens) = pool::get_pool_coins(&event.pool) {
                                let mut sorted_input_tokens = input_tokens.clone();
                                sorted_input_tokens.sort_by(|a, b| a.address.cmp(&b.address));
                                return Some(Pool {
                                    address: Hex::encode(&event.pool),
                                    name: lp_token.name.clone(),
                                    symbol: lp_token.symbol.clone(),
                                    created_at_timestamp: blk.timestamp_seconds(),
                                    created_at_block_number: blk.number,
                                    log_ordinal: log.ordinal(),
                                    transaction_id: Hex(&log.receipt.transaction.hash).to_string(),
                                    registry_address: Hex::encode(address),
                                    output_token: Some(lp_token),
                                    input_tokens_ordered: input_tokens
                                        .clone()
                                        .into_iter()
                                        .map(|token| token.address)
                                        .collect(),
                                    input_tokens: sorted_input_tokens,
                                    // TODO: We may be able to remove this if it is also false.
                                    //       This can just be set when creating the relevant subgraph entity.
                                    //       Will keep here for now as a reminder.
                                    is_single_sided: false,
                                    //       Could also extract this into `graph-out` module eventually.
                                    is_metapool: utils::is_metapool(&event.pool),
                                });
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Err(e) => {
                        substreams::log::debug!("Error in `map_pool_registry_events`: {:?}", e);
                        None
                    }
                }
            })
            .collect(),
    );
}

fn map_pool_added_2_events(blk: &eth::Block, pools: &mut Pools, address: [u8; 20]) {
    pools.pools.append(
        &mut blk
            .events::<CryptoSwapPoolAdded>(&[&address])
            .filter_map(|(event, log)| {
                match registry::get_lp_token_address_from_registry(&event.pool, &address.to_vec()) {
                    Ok(lp_token_address) => {
                        if let Ok(lp_token) = token::create_token(&lp_token_address, &event.pool) {
                            if let Ok(input_tokens) = pool::get_pool_coins(&event.pool) {
                                let mut sorted_input_tokens = input_tokens.clone();
                                sorted_input_tokens.sort_by(|a, b| a.address.cmp(&b.address));
                                return Some(Pool {
                                    address: Hex::encode(&event.pool),
                                    name: lp_token.name.clone(),
                                    symbol: lp_token.symbol.clone(),
                                    created_at_timestamp: blk.timestamp_seconds(),
                                    created_at_block_number: blk.number,
                                    log_ordinal: log.ordinal(),
                                    transaction_id: Hex(&log.receipt.transaction.hash).to_string(),
                                    registry_address: Hex::encode(address),
                                    output_token: Some(lp_token),
                                    input_tokens_ordered: input_tokens
                                        .clone()
                                        .into_iter()
                                        .map(|token| token.address)
                                        .collect(),
                                    input_tokens: sorted_input_tokens,
                                    is_single_sided: false,
                                    is_metapool: utils::is_metapool(&event.pool),
                                });
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Err(e) => {
                        substreams::log::debug!("Error in `map_pool_registry_events`: {:?}", e);
                        None
                    }
                }
            })
            .collect(),
    );
}

fn map_base_pool_added_events(blk: &eth::Block, pools: &mut Pools, address: [u8; 20]) {
    pools.pools.append(
        &mut blk
            .events::<BasePoolAdded>(&[&address])
            .filter_map(|(event, log)| {
                match pool::get_lp_token_address_from_pool(&event.base_pool) {
                    Ok(lp_token_address) => {
                        if let Ok(lp_token) =
                            token::create_token(&lp_token_address, &event.base_pool)
                        {
                            if let Ok(input_tokens) = pool::get_pool_coins(&event.base_pool) {
                                let mut sorted_input_tokens = input_tokens.clone();
                                sorted_input_tokens.sort_by(|a, b| a.address.cmp(&b.address));
                                return Some(Pool {
                                    address: Hex::encode(&event.base_pool),
                                    name: lp_token.name.clone(),
                                    symbol: lp_token.symbol.clone(),
                                    created_at_timestamp: blk.timestamp_seconds(),
                                    created_at_block_number: blk.number,
                                    log_ordinal: log.ordinal(),
                                    transaction_id: Hex(&log.receipt.transaction.hash).to_string(),
                                    registry_address: Hex::encode(address),
                                    output_token: Some(lp_token),
                                    input_tokens_ordered: input_tokens
                                        .clone()
                                        .into_iter()
                                        .map(|token| token.address)
                                        .collect(),
                                    input_tokens: sorted_input_tokens,
                                    is_single_sided: false,
                                    is_metapool: utils::is_metapool(&event.base_pool),
                                });
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Err(e) => {
                        substreams::log::debug!("Error in `map_pool_registry_events`: {:?}", e);
                        None
                    }
                }
            })
            .collect(),
    );
}

fn map_crypto_pool_deployed_events(blk: &eth::Block, pools: &mut Pools, address: [u8; 20]) {
    pools.pools.append(
        &mut blk
            .events::<CryptoPoolDeployed>(&[&address])
            .filter_map(|(event, log)| match token::get_token_minter(&event.token) {
                Ok(minter) => {
                    if let Ok(lp_token) = token::create_token(&event.token, &minter) {
                        if let Ok(input_tokens) = pool::get_pool_coins(&minter) {
                            let mut sorted_input_tokens = input_tokens.clone();
                            sorted_input_tokens.sort_by(|a, b| a.address.cmp(&b.address));
                            Some(Pool {
                                address: Hex::encode(&minter),
                                name: lp_token.name.clone(),
                                symbol: lp_token.symbol.clone(),
                                created_at_timestamp: blk.timestamp_seconds(),
                                created_at_block_number: blk.number,
                                log_ordinal: log.ordinal(),
                                transaction_id: Hex(&log.receipt.transaction.hash).to_string(),
                                registry_address: Hex::encode(address),
                                output_token: Some(lp_token),
                                input_tokens_ordered: input_tokens
                                    .clone()
                                    .into_iter()
                                    .map(|token| token.address)
                                    .collect(),
                                input_tokens: sorted_input_tokens,
                                is_single_sided: false,
                                is_metapool: utils::is_metapool(&minter),
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Err(e) => {
                    substreams::log::debug!("Error in `map_crypto_pool_deployed_events`: {:?}", e);
                    None
                }
            })
            .collect(),
    );
}

fn map_plain_pool_deployed_events(blk: &eth::Block, pools: &mut Pools, address: [u8; 20]) {
    pools.pools.append(
        &mut blk
            .events::<PlainPoolDeployed>(&[&address])
            .filter_map(|(_event, log)| {
                let trx = log.receipt.transaction;
                let transfer = trx
                    .calls
                    .iter()
                    .filter(|call| !call.state_reverted)
                    .flat_map(|call| call.logs.iter())
                    .find(|log| abi::erc20::events::Transfer::match_log(log));

                if let Some(transfer_log) = transfer {
                    if let Ok(transfer_event) = abi::erc20::events::Transfer::decode(transfer_log) {
                        if let Ok(lp_token) =
                            token::create_token(&transfer_event.receiver, &transfer_event.receiver)
                        {
                            if let Ok(input_tokens) = pool::get_pool_coins(&transfer_event.receiver)
                            {
                                let mut sorted_input_tokens = input_tokens.clone();
                                sorted_input_tokens.sort_by(|a, b| a.address.cmp(&b.address));

                                return Some(Pool {
                                    address: Hex::encode(&transfer_event.receiver),
                                    name: lp_token.name.clone(),
                                    symbol: lp_token.symbol.clone(),
                                    created_at_timestamp: blk.timestamp_seconds(),
                                    created_at_block_number: blk.number,
                                    log_ordinal: log.ordinal(),
                                    transaction_id: Hex(&log.receipt.transaction.hash).to_string(),
                                    registry_address: Hex::encode(address),
                                    output_token: Some(lp_token),
                                    input_tokens_ordered: input_tokens
                                        .clone()
                                        .into_iter()
                                        .map(|token| token.address)
                                        .collect(),
                                    input_tokens: sorted_input_tokens,
                                    is_single_sided: false,
                                    is_metapool: utils::is_metapool(&transfer_event.receiver),
                                });
                            }
                        }
                    }
                }
                None
            })
            .collect(),
    );
}

fn map_meta_pool_deployed_events(blk: &eth::Block, pools: &mut Pools, address: [u8; 20]) {
    pools.pools.append(
        &mut blk
            .events::<MetaPoolDeployed>(&[&address])
            .filter_map(|(_event, log)| {
                let trx = log.receipt.transaction;
                let transfer = trx
                    .calls
                    .iter()
                    .filter(|call| !call.state_reverted)
                    .flat_map(|call| call.logs.iter())
                    .find(|log| abi::erc20::events::Transfer::match_log(log));

                if let Some(transfer_log) = transfer {
                    if let Ok(transfer_event) = abi::erc20::events::Transfer::decode(transfer_log) {
                        if let Ok(lp_token) =
                            token::create_token(&transfer_event.receiver, &transfer_event.receiver)
                        {
                            if let Ok(input_tokens) = pool::get_pool_coins(&transfer_event.receiver)
                            {
                                let mut sorted_input_tokens = input_tokens.clone();
                                sorted_input_tokens.sort_by(|a, b| a.address.cmp(&b.address));

                                return Some(Pool {
                                    address: Hex::encode(&transfer_event.receiver),
                                    name: lp_token.name.clone(),
                                    symbol: lp_token.symbol.clone(),
                                    created_at_timestamp: blk.timestamp_seconds(),
                                    created_at_block_number: blk.number,
                                    log_ordinal: log.ordinal(),
                                    transaction_id: Hex(&log.receipt.transaction.hash).to_string(),
                                    registry_address: Hex::encode(address),
                                    output_token: Some(lp_token),
                                    input_tokens_ordered: input_tokens
                                        .clone()
                                        .into_iter()
                                        .map(|token| token.address)
                                        .collect(),
                                    input_tokens: sorted_input_tokens,
                                    is_single_sided: false,
                                    is_metapool: utils::is_metapool(&transfer_event.receiver),
                                });
                            }
                        }
                    }
                }
                None
            })
            .collect(),
    );
}

#[substreams::handlers::map]
fn map_pools_created(blk: eth::Block) -> Result<Pools, Error> {
    let mut pools = Pools::default();

    for contract in CONTRACTS {
        map_pool_added_1_events(&blk, &mut pools, contract);
        map_pool_added_2_events(&blk, &mut pools, contract);
        map_base_pool_added_events(&blk, &mut pools, contract);
        map_crypto_pool_deployed_events(&blk, &mut pools, contract);
        map_plain_pool_deployed_events(&blk, &mut pools, contract);
        map_meta_pool_deployed_events(&blk, &mut pools, contract);
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
