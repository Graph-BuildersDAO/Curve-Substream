use anyhow::anyhow;
use substreams::{errors::Error, Hex};
use substreams_ethereum::{block_view, pb::eth::v2 as eth, NULL_ADDRESS};

use crate::{
    abi::curve::child_registries::{
        crv_usd_pool_factory, crypto_pool_factory_v2, pool_registry_v1, stable_swap_factory_ng,
        tricrypto_factory_ng,
    },
    common::{event_extraction, utils},
    network_config::{PoolDetails, MISSING_OLD_POOLS_DATA, POOL_REGISTRIES},
    pb::curve::types::v1::{Pool, Pools, Token},
    rpc::{pool, token},
    types::event_traits::PlainPoolDeployedEvent,
};

#[substreams::handlers::map]
pub fn map_pools_created(blk: eth::Block) -> Result<Pools, Vec<Error>> {
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
                map_crypto_pool_deployed_events(&blk, &mut pools, contract),
                map_plain_pool_deployed_events::<crv_usd_pool_factory::events::PlainPoolDeployed>(
                    &blk, &mut pools, contract,
                ),
                map_plain_pool_deployed_events::<pool_registry_v1::events::PlainPoolDeployed>(
                    &blk, &mut pools, contract,
                ),
                map_plain_pool_deployed_events::<stable_swap_factory_ng::events::PlainPoolDeployed>(
                    &blk, &mut pools, contract,
                ),
                map_meta_pool_deployed_events(&blk, &mut pools, contract),
                map_tricrypto_pool_deployed_events(&blk, &mut pools, contract),
            ]
        })
        .filter_map(Result::err)
        .collect();

    if errors.is_empty() {
        return Ok(pools);
    }
    Err(errors)
}

fn map_crypto_pool_deployed_events(
    blk: &eth::Block,
    pools: &mut Pools,
    address: [u8; 20],
) -> Result<(), Error> {
    pools.pools.append(
        &mut blk
            .events::<crypto_pool_factory_v2::events::CryptoPoolDeployed>(&[&address])
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
                substreams::log::debug!("CryptoPoolDeployed Event");

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

fn map_plain_pool_deployed_events<E: PlainPoolDeployedEvent + substreams_ethereum::Event>(
    blk: &eth::Block,
    pools: &mut Pools,
    address: [u8; 20],
) -> Result<(), Error> {
    pools.pools.append(
        &mut blk
            .events::<E>(&[&address])
            .filter_map(|(_event, log)| {
                let transfer = match event_extraction::extract_transfer_event(&log) {
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
                substreams::log::debug!("PlainPoolDeployed Event");

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
            // In this block we process MetaPoolDeployed events specifically from `pool_registry_v1::events::MetaPoolDeployed`.
            // However, due to the ABI compatibility, this will also include MetaPoolDeployed events from other contracts such as
            // `crv_usd_pool_factory::events::MetaPoolDeployed` and `stable_swap_factory_ng::events::MetaPoolDeployed`,
            // since they share the same ABI structure for this event type. This ensures that all MetaPoolDeployed events,
            // regardless of the originating contract, are captured and processed here as long as they are emitted to the specified address.
            .events::<pool_registry_v1::events::MetaPoolDeployed>(&[&address])
            .filter_map(|(_event, log)| {
                let transfer = match event_extraction::extract_transfer_event(&log) {
                    Ok(event) => event,
                    Err(e) => {
                        substreams::log::debug!(
                            "Error in `map_meta_pool_deployed_events`: {:?}",
                            e
                        );
                        return None;
                    }
                };
                let lp_token = match token::create_token(&transfer.receiver, &transfer.receiver) {
                    Ok(token) => token,
                    Err(e) => {
                        substreams::log::debug!(
                            "Error in `map_meta_pool_deployed_events`: {:?}",
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
                                "Error in `map_meta_pool_deployed_events`: {:?}",
                                e
                            );
                            return None;
                        }
                    };
                substreams::log::debug!("MetaPoolDeployed Event");

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

fn map_tricrypto_pool_deployed_events(
    blk: &eth::Block,
    pools: &mut Pools,
    address: [u8; 20],
) -> Result<(), Error> {
    pools.pools.append(
        &mut blk
            .events::<tricrypto_factory_ng::events::TricryptoPoolDeployed>(&[&address])
            .filter_map(|(event, log)| {
                let lp_token = match token::create_token(&event.pool, &event.pool) {
                    Ok(token) => token,
                    Err(e) => {
                        substreams::log::debug!(
                            "Error in `map_meta_pool_deployed_events`: {:?}",
                            e
                        );
                        return None;
                    }
                };
                let (input_tokens, input_tokens_ordered) =
                    match get_and_sort_input_tokens(&event.pool) {
                        Ok(result) => result,
                        Err(e) => {
                            substreams::log::debug!(
                                "Error in `map_tricrypto_pool_deployed_events`: {:?}",
                                e
                            );
                            return None;
                        }
                    };
                substreams::log::debug!("TricryptoPoolDeployed Event");

                Some(create_pool(
                    Hex::encode(&event.pool),
                    Hex::encode(address),
                    lp_token,
                    input_tokens_ordered,
                    input_tokens,
                    false,
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

fn create_pool(
    address: String,
    registry_address: String,
    lp_token: Token,
    input_tokens_ordered: Vec<String>,
    input_tokens: Vec<Token>,
    is_metapool: bool,
    log: &block_view::LogView,
    blk: &eth::Block,
) -> Pool {
    Pool {
        address,
        name: lp_token.name.clone(),
        symbol: lp_token.symbol.clone(),
        created_at_timestamp: blk.timestamp_seconds(),
        created_at_block_number: blk.number,
        log_ordinal: log.ordinal(),
        transaction_id: Hex(&log.receipt.transaction.hash).to_string(),
        registry_address,
        output_token: Some(lp_token),
        input_tokens_ordered,
        input_tokens,
        is_metapool,
    }
}

fn create_missing_pool(
    address: String,
    registry_address: String,
    lp_token: Token,
    input_tokens_ordered: Vec<String>,
    input_tokens: Vec<Token>,
    is_metapool: bool,
    blk: &eth::Block,
    hash: Vec<u8>,
) -> Pool {
    Pool {
        address,
        name: lp_token.name.clone(),
        symbol: lp_token.symbol.clone(),
        created_at_timestamp: blk.timestamp_seconds(),
        created_at_block_number: blk.number,
        log_ordinal: 0,
        transaction_id: Hex::encode(hash),
        registry_address,
        output_token: Some(lp_token),
        input_tokens_ordered,
        input_tokens,
        is_metapool,
    }
}

// This follows the logic from the original subgraph.
// An array of token addresses, and a sorted array of token structs is required.
fn get_and_sort_input_tokens(pool_address: &Vec<u8>) -> Result<(Vec<Token>, Vec<String>), Error> {
    let mut input_tokens = pool::get_pool_coins(&pool_address)?;
    let input_tokens_ordered = input_tokens
        .clone()
        .into_iter()
        .map(|token| token.address)
        .collect();
    input_tokens.sort_by(|a, b| a.address.cmp(&b.address));

    Ok((input_tokens, input_tokens_ordered))
}
