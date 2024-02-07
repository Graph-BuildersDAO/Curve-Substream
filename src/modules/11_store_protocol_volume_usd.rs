use std::ops::Sub;

use substreams::{
    pb::substreams::Clock,
    store::{DeltaBigDecimal, Deltas, StoreAdd, StoreAddBigDecimal, StoreNew},
};

use crate::key_management::store_key_manager::StoreKey;

#[substreams::handlers::store]
pub fn store_protocol_volume_usd(
    clock: Clock,
    pool_volume_deltas: Deltas<DeltaBigDecimal>,
    output_store: StoreAddBigDecimal,
) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id = timestamp_seconds / 86400;
    let prev_day_id = day_id - 1;

    for delta in pool_volume_deltas.iter() {
        let tvl_diff = delta.new_value.clone().sub(delta.old_value.clone());
        output_store.add_many(
            delta.ordinal,
            &vec![
                StoreKey::protocol_volume_usd_key(),
                StoreKey::protocol_daily_volume_usd_key(&day_id),
            ],
            tvl_diff,
        );
    }
}
