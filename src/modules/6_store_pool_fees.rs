use substreams::store::{StoreNew, StoreSet, StoreSetProto};

use crate::{
    key_management::store_key_manager::StoreKey,
    pb::curve::types::v1::{Events, Pool, PoolFees, Pools},
    rpc::pool::{calculate_pool_fees, get_pool_fee_and_admin_fee},
};

#[substreams::handlers::store]
pub fn store_pool_fees(pools: Pools, events: Events, store: StoreSetProto<PoolFees>) {
    // If is a newly created pool, get the pools fees from the pool contract.
    for pool in pools.pools {
        process_pool_fee(&pool, &store)
    }
    // If there is an updated pool fee, update the store.
    for event in events.fee_changes_events {
        let pool_fees = calculate_pool_fees(
            event.fee_big(),
            event.admin_fee_big(),
            &event.pool_address_vec(),
        );
        store.set(
            event.log_ordinal,
            StoreKey::pool_fees_key(&event.pool_address),
            &pool_fees,
        );
    }
}

fn process_pool_fee(pool: &Pool, store: &StoreSetProto<PoolFees>) {
    let pool_address = pool.address_vec();
    let fee_res = get_pool_fee_and_admin_fee(&pool_address);

    if let Err(e) = &fee_res {
        substreams::log::debug!("Failed to fetch pool fees for {}: {}", pool.address, e);
        return;
    }

    if let Ok((total_fee, admin_fee)) = fee_res {
        let pool_fees = calculate_pool_fees(total_fee, admin_fee, &pool_address);
        store.set(
            pool.log_ordinal,
            StoreKey::pool_fees_key(&pool.address),
            &pool_fees,
        );
    }
}
