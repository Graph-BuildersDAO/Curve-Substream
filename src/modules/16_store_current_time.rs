use substreams::{
    pb::substreams::Clock,
    store::{StoreNew, StoreSet, StoreSetInt64},
};

use crate::{key_management::store_key_manager::StoreKey, timeframe_management::utils::calculate_day_hour_id};

// TODO: Move this to the first module
#[substreams::handlers::store]
pub fn store_current_time(clock: Clock, store: StoreSetInt64) {
    let (day_id, hour_id) = calculate_day_hour_id(clock.timestamp.unwrap().seconds);
    store.set(0, StoreKey::current_day_id_key(), &day_id);
    store.set(0, StoreKey::current_hour_id_key(), &hour_id);
}
