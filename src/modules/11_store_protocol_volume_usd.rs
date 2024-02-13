use std::ops::Sub;

use substreams::{
    pb::substreams::Clock,
    store::{DeltaBigDecimal, Deltas, StoreAdd, StoreAddBigDecimal, StoreNew},
};

use crate::{key_management::store_key_manager::StoreKey, snapshot::utils::calculate_day_hour_id};

#[substreams::handlers::store]
pub fn store_protocol_volume_usd(
    clock: Clock,
    pool_volume_deltas: Deltas<DeltaBigDecimal>,
    output_store: StoreAddBigDecimal,
) {
    let (day_id, _) = calculate_day_hour_id(clock.timestamp.unwrap().seconds);

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
