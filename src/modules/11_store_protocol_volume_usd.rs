use std::ops::Sub;

use substreams::{
    key,
    pb::substreams::Clock,
    store::{DeltaBigDecimal, DeltaInt64, Deltas, StoreAdd, StoreAddBigDecimal, StoreNew},
};

use crate::{
    key_management::store_key_manager::StoreKey,
    timeframe_management::{
        pruning::{
            pruners::protocol_volume_usd_pruner::ProtocolVolumeUsdPruneAction,
            setup_timeframe_pruning,
        },
        utils::calculate_day_hour_id,
    },
};

#[substreams::handlers::store]
pub fn store_protocol_volume_usd(
    clock: Clock,
    pool_volume_deltas: Deltas<DeltaBigDecimal>,
    current_time_deltas: Deltas<DeltaInt64>,
    output_store: StoreAddBigDecimal,
) {
    // Initialise pruning for protocol volume usd data using `ProtocolVolumeUsdPruneAction`.
    // This setup registers the pruner to execute when new timeframes (day/hour) are detected,
    // ensuring outdated data is removed to maintain store efficiency.
    let protocol_volume_usd_pruner = ProtocolVolumeUsdPruneAction {
        store: &output_store,
    };
    setup_timeframe_pruning(&current_time_deltas, &[&protocol_volume_usd_pruner]);

    let (day_id, _) = calculate_day_hour_id(clock.timestamp.unwrap().seconds);

    for delta in pool_volume_deltas.iter() {
        if key::first_segment(&delta.key) == "PoolVolumeUsd" {
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
}
