use substreams::{
    pb::substreams::Clock,
    store::{StoreNew, StoreSet, StoreSetInt64},
};

use crate::key_management::store_key_manager::StoreKey;

#[substreams::handlers::store]
pub fn store_current_time_snapshots(clock: Clock, store: StoreSetInt64) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id = timestamp_seconds / 86400; // 86,400 seconds in a day
    let hour_id = timestamp_seconds / 3600; // 3,600 seconds in an hour

    store.set(0, StoreKey::current_day_id_key(), &day_id);
    store.set(0, StoreKey::current_hour_id_key(), &hour_id);
}
