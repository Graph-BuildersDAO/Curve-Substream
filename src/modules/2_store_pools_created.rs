use substreams::store::{StoreNew, StoreSet, StoreSetProto};

use crate::{
    key_management::store_key_manager::StoreKey,
    pb::curve::types::v1::{CurveEvents, Pool},
};

#[substreams::handlers::store]
fn store_pools_created(events: CurveEvents, store: StoreSetProto<Pool>) {
    for pool in events.pools {
        let address = pool.address.clone();
        store.set(pool.log_ordinal, StoreKey::pool_key(&address), &pool)
    }
}
