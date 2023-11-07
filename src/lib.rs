use hex_literal::hex;
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{self as eth};

mod abi;
mod pb;

use abi::registry::events::{
    BasePoolAdded, CryptoPoolDeployed, MetaPoolDeployed, PlainPoolDeployed,
    PoolAdded1 as RegistryPoolAdded, PoolAdded2 as CryptoSwapPoolAdded,
};
use pb::curve::types::v1::{Pool, Pools};

substreams_ethereum::init!();

const POOLREGISTRY_CONTRACT: [u8; 20] = hex!("90e00ace148ca3b23ac1bc8c240c2a7dd9c2d7f5");
const POOLREGISTRY_V2_CONTRACT: [u8; 20] = hex!("7d86446ddb609ed0f5f8684acf30380a356b2b4c");
const CRYPTOSWAP_REGISTRY_CONTRACT: [u8; 20] = hex!("8F942C20D02bEfc377D41445793068908E2250D0");
const METAPOOL_FACTORY_CONTRACT: [u8; 20] = hex!("B9fC157394Af804a3578134A6585C0dc9cc990d4");
const CRYPTOPOOL_FACTORY: [u8; 20] = hex!("F18056Bbd320E96A48e3Fbf8bC061322531aac99");

// Registry and Factory contracts
const CONTRACTS: [[u8; 20]; 5] = [
    POOLREGISTRY_CONTRACT,
    POOLREGISTRY_V2_CONTRACT,
    CRYPTOSWAP_REGISTRY_CONTRACT,
    METAPOOL_FACTORY_CONTRACT,
    CRYPTOPOOL_FACTORY,
];

fn map_pool_registry_events(blk: &eth::Block, pools: &mut Pools, address: [u8; 20]) {
    pools.pools.append(
        &mut blk
            .events::<RegistryPoolAdded>(&[&address])
            .map(|(event, log)| Pool {
                address: Hex::encode(event.pool),
                created_at_timestamp: blk.timestamp_seconds(),
                created_at_block_number: blk.number,
                log_ordinal: log.ordinal(),
                transaction_id: Hex(&log.receipt.transaction.hash).to_string(),
                registry_address: Hex::encode(address),
            })
            .collect(),
    );
}

fn map_cryptoswap_registry_events(blk: &eth::Block, pools: &mut Pools, address: [u8; 20]) {
    pools.pools.append(
        &mut blk
            .events::<CryptoSwapPoolAdded>(&[&address])
            .map(|(event, log)| Pool {
                address: Hex::encode(event.pool),
                created_at_timestamp: blk.timestamp_seconds(),
                created_at_block_number: blk.number,
                log_ordinal: log.ordinal(),
                transaction_id: Hex(&log.receipt.transaction.hash).to_string(),
                registry_address: Hex::encode(address),
            })
            .collect(),
    );
}

fn map_base_pool_added_events(blk: &eth::Block, pools: &mut Pools, address: [u8; 20]) {
    pools.pools.append(
        &mut blk
            .events::<BasePoolAdded>(&[&address])
            .map(|(event, log)| Pool {
                address: Hex::encode(event.base_pool),
                created_at_timestamp: blk.timestamp_seconds(),
                created_at_block_number: blk.number,
                log_ordinal: log.ordinal(),
                transaction_id: Hex(&log.receipt.transaction.hash).to_string(),
                registry_address: Hex::encode(address),
            })
            .collect(),
    );
}

fn map_crypto_pool_deployed_events(blk: &eth::Block, pools: &mut Pools, address: [u8; 20]) {
    pools.pools.append(
        &mut blk
            .events::<CryptoPoolDeployed>(&[&address])
            .map(|(event, log)| Pool {
                address: Hex::encode(event.token),
                created_at_timestamp: blk.timestamp_seconds(),
                created_at_block_number: blk.number,
                log_ordinal: log.ordinal(),
                transaction_id: Hex(&log.receipt.transaction.hash).to_string(),
                registry_address: Hex::encode(address),
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
                        return Some(Pool {
                            address: Hex::encode(transfer_event.receiver),
                            created_at_timestamp: blk.timestamp_seconds(),
                            created_at_block_number: blk.number,
                            log_ordinal: log.ordinal(),
                            transaction_id: Hex(&log.receipt.transaction.hash).to_string(),
                            registry_address: Hex::encode(address),
                        });
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
                        return Some(Pool {
                            address: Hex::encode(transfer_event.receiver),
                            created_at_timestamp: blk.timestamp_seconds(),
                            created_at_block_number: blk.number,
                            log_ordinal: log.ordinal(),
                            transaction_id: Hex(&log.receipt.transaction.hash).to_string(),
                            registry_address: Hex::encode(address),
                        });
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
        map_pool_registry_events(&blk, &mut pools, contract);
        map_cryptoswap_registry_events(&blk, &mut pools, contract);
        map_base_pool_added_events(&blk, &mut pools, contract);
        map_crypto_pool_deployed_events(&blk, &mut pools, contract);
        map_plain_pool_deployed_events(&blk, &mut pools, contract);
        map_meta_pool_deployed_events(&blk, &mut pools, contract);
    }

    Ok(pools)
}
