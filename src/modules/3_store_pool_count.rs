use substreams::store::{StoreAdd, StoreAddInt64, StoreNew};

use crate::{pb::curve::types::v1::Pools, store_key_manager::StoreKey};

#[substreams::handlers::store]
pub fn store_pool_count(pools: Pools, store: StoreAddInt64) {
    for pool in pools.pools {
        store.add(pool.log_ordinal, StoreKey::protocol_pool_count_key(), 1)
    }
}
