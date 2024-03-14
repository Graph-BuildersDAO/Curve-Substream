use substreams::{
    pb::substreams::Clock,
    store::{DeltaInt64, Deltas, StoreNew, StoreSetIfNotExists, StoreSetIfNotExistsInt64},
};

use crate::{
    key_management::store_key_manager::StoreKey,
    pb::curve::types::v1::Events,
    timeframe_management::{
        pruning::{
            pruners::protocol_active_user_pruner::ProtocolActiveUserPruneAction,
            setup_timeframe_pruning,
        },
        utils::calculate_day_hour_id,
    },
};

#[substreams::handlers::store]
pub fn store_active_users(
    clock: Clock,
    events: Events,
    current_time_deltas: Deltas<DeltaInt64>,
    output_store: StoreSetIfNotExistsInt64,
) {
    // Initialise pruning for active users data using `ProtocolActiveUserPruneAction`.
    // This setup registers the pruner to execute when new timeframes (day/hour) are detected,
    // ensuring outdated data is removed to maintain store efficiency.
    let protocol_active_user_pruner = ProtocolActiveUserPruneAction {
        store: &output_store,
    };

    setup_timeframe_pruning(&current_time_deltas, &[&protocol_active_user_pruner]);

    let (day_id, hour_id) = calculate_day_hour_id(clock.timestamp.unwrap().seconds);

    for event in events.pool_events {
        output_store.set_if_not_exists_many(
            0,
            &vec![
                StoreKey::active_user_key(&event.from_address),
                StoreKey::active_user_daily_key(&day_id, &event.from_address),
                StoreKey::active_user_hourly_key(&hour_id, &event.from_address),
            ],
            &1,
        );
    }
}
