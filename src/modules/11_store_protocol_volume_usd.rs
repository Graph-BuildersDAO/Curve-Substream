use std::ops::Sub;

use substreams::{
    pb::substreams::Clock,
    store::{
        DeltaBigDecimal, DeltaInt64, Deltas, StoreAdd, StoreAddBigDecimal, StoreGet, StoreGetInt64,
        StoreGetProto, StoreGetString, StoreNew,
    },
};

use crate::{
    key_management::store_key_manager::StoreKey,
    pb::curve::types::v1::Pool,
    timeframe_management::{
        pruning::{
            pruners::protocol_volume_usd_pruner::ProtocolVolumeUsdPruneAction,
            pruning_utils::setup_timeframe_pruning,
            traits::{PoolPruneAction, TokenPruneAction},
        },
        utils::calculate_day_hour_id,
    },
};

#[substreams::handlers::store]
pub fn store_protocol_volume_usd(
    clock: Clock,
    pool_volume_deltas: Deltas<DeltaBigDecimal>,
    pools_store: StoreGetProto<Pool>,
    pool_count_store: StoreGetInt64,
    pool_addresses_store: StoreGetString,
    current_time_deltas: Deltas<DeltaInt64>,
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

    // Initialise pruning for protocol volume usd data using `ProtocolVolumeUsdPruneAction`.
    // This setup registers the pruner to execute when new timeframes (day/hour) are detected,
    // ensuring outdated data is removed to maintain store efficiency. Pool and Token level pruning
    // are not required for this module, hence passed as `None`.
    let protocol_volume_usd_pruner = ProtocolVolumeUsdPruneAction {
        store: &output_store,
    };
    setup_timeframe_pruning(
        &pools_store,
        &pool_count_store,
        &pool_addresses_store,
        &current_time_deltas,
        Some(&protocol_volume_usd_pruner),
        None as Option<&dyn PoolPruneAction>,
        None as Option<&dyn TokenPruneAction>,
    );
}
