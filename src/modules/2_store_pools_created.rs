use substreams::store::{StoreNew, StoreSet, StoreSetProto};

use crate::pb::curve::types::v1::{Pool, Pools};

#[substreams::handlers::store]
fn store_pools_created(pools: Pools, store: StoreSetProto<Pool>) {
    for pool in pools.pools {
        let address = pool.address.clone();
        store.set(pool.log_ordinal, format!("pool:{}", address), &pool)
    }
}
