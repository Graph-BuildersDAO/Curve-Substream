use substreams::store::{
    DeltaInt64, Deltas, StoreNew, StoreSetIfNotExists, StoreSetIfNotExistsString,
};

use crate::{key_management::store_key_manager::StoreKey, pb::curve::types::v1::CurveEvents};

#[substreams::handlers::store]
pub fn store_pool_addresses(
    events: CurveEvents,
    pools_count_deltas: Deltas<DeltaInt64>,
    store: StoreSetIfNotExistsString,
) {
    // Use `zip` to iterate over pools and their corresponding count deltas simultaneously
    for (pool, count_delta) in events
        .pools
        .iter()
        .zip(pools_count_deltas.deltas.iter())
    {
        let key = StoreKey::pool_address_key(&count_delta.new_value);
        let value = pool.address.clone();

        store.set_if_not_exists(0, &key, &value);
    }
}
