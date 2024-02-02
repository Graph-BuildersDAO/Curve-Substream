use substreams::store::{StoreAdd, StoreAddInt64, StoreNew};

use crate::{key_management::store_key_manager::StoreKey, pb::curve::types::v1::Pools};

#[substreams::handlers::store]
pub fn store_pool_count(pools: Pools, store: StoreAddInt64) {
    for pool in pools.pools {
        store.add(pool.log_ordinal, StoreKey::protocol_pool_count_key(), 1)
    }
}
