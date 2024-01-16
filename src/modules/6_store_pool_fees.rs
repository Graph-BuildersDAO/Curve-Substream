use std::str::FromStr;

use anyhow::anyhow;
use substreams::{
    errors::Error,
    scalar::{BigDecimal, BigInt},
    store::{StoreNew, StoreSet, StoreSetProto},
    Hex,
};

use crate::{
    common::conversion::convert_bigint_to_decimal,
    constants::FEE_DENOMINATOR,
    pb::curve::types::v1::{events::NewParamsEvent, Events, Pool, PoolFees, Pools},
    rpc::pool::{calculate_pool_fees, get_pool_fees},
    store_key_manager::StoreKey,
};

#[substreams::handlers::store]
pub fn store_pool_fees(pools: Pools, events: Events, store: StoreSetProto<PoolFees>) {
    // If is a newly created pool, get the pools fees from the pool contract.
    for pool in pools.pools {
        process_pool_fee(&pool, &store)
    }
    // If there is an updated pool fee, update the store.
    for event in events.new_params_events {
        update_pool_fee_from_event(&event, &store)
    }
}

fn process_pool_fee(pool: &Pool, store: &StoreSetProto<PoolFees>) {
    let pool_address = match Hex::decode(&pool.address) {
        Ok(addr) => addr,
        Err(e) => {
            substreams::log::info!("Error decoding pool address in `process_pool_fee`: {}", e);
            return;
        }
    };
    match get_pool_fees(&pool_address) {
        Ok(pool_fees) => store.set(
            pool.log_ordinal,
            StoreKey::pool_fees_key(&pool.address),
            &pool_fees,
        ),
        Err(e) => substreams::log::info!("Error getting pool fees in `process_pool_fee`: {}", e),
    }
}

fn update_pool_fee_from_event(event: &NewParamsEvent, store: &StoreSetProto<PoolFees>) {
    let total_fee = match convert_fee_string_to_decimal(&event.fee) {
        Ok(fee) => fee,
        Err(e) => {
            substreams::log::info!(
                "Error converting total fee string to decimal in `process_pool_fee`: {}",
                e
            );
            return;
        }
    };
    let admin_fee = match convert_fee_string_to_decimal(&event.admin_fee) {
        Ok(fee) => fee,
        Err(e) => {
            substreams::log::info!(
                "Error converting admin fee string to decimal in `process_pool_fee`: {}",
                e
            );
            return;
        }
    };
    let pool_address = match Hex::decode(&event.pool_address) {
        Ok(addr) => addr,
        Err(e) => {
            substreams::log::info!("Error decoding pool address: {}", e);
            return;
        }
    };
    let pool_fees = calculate_pool_fees(total_fee, admin_fee, &pool_address);
    store.set(
        event.log_ordinal,
        StoreKey::pool_fees_key(&event.pool_address),
        &pool_fees,
    );
}

fn convert_fee_string_to_decimal(fee_str: &str) -> Result<BigDecimal, Error> {
    BigInt::from_str(fee_str)
        .map_err(|e| anyhow!("Error converting fee string to BigInt: {}", e))
        .and_then(|big_int| convert_bigint_to_decimal(&big_int, FEE_DENOMINATOR))
}
