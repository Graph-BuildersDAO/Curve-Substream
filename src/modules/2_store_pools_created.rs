use substreams::store::{StoreNew, StoreSet, StoreSetProto};

use crate::{
    pb::curve::types::v1::{Pool, Pools},
    store_key_manager::StoreKey,
};

#[substreams::handlers::store]
fn store_pools_created(pools: Pools, store: StoreSetProto<Pool>) {
    for pool in pools.pools {
        let address = pool.address.clone();
        store.set(pool.log_ordinal, StoreKey::pool_key(&address), &pool)
    }
}
