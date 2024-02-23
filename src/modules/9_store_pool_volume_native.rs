use substreams::{
    pb::substreams::Clock,
    scalar::BigInt,
    store::{
        DeltaInt64, Deltas, StoreAdd, StoreAddBigInt, StoreGet, StoreGetInt64, StoreGetProto,
        StoreGetString, StoreNew,
    },
};

use crate::{
    key_management::store_key_manager::StoreKey,
    pb::curve::types::v1::{
        events::{pool_event::Type, PoolEvent},
        Events, Pool,
    },
    timeframe_management::{
        pruning::{
            pruners::token_volume_native_pruner::TokenVolumeNativePruner,
            pruning_utils::setup_timeframe_pruning,
            traits::{PoolPruneAction, ProtocolPruneAction},
        },
        utils::calculate_day_hour_id,
    },
};

#[substreams::handlers::store]
pub fn store_pool_volume_native(
    clock: Clock,
    events: Events,
    pools_store: StoreGetProto<Pool>,
    pool_count_store: StoreGetInt64,
    pool_addresses_store: StoreGetString,
    current_time_deltas: Deltas<DeltaInt64>,
    output_store: StoreAddBigInt,
) {
    let (day_id, hour_id) = calculate_day_hour_id(clock.timestamp.unwrap().seconds);

    for event in events.pool_events {
        // Ensure there is a pool for this event
        let _pool = pools_store.must_get_last(StoreKey::pool_key(&event.pool_address));
        if let Some(event_type) = &event.r#type {
            match event_type {
                Type::SwapEvent(swap) => {
                    update_pool_volume(
                        &output_store,
                        &event,
                        &swap.token_in_ref().token_address,
                        swap.token_in_amount_big(),
                        &day_id,
                        &hour_id,
                    );
                    update_pool_volume(
                        &output_store,
                        &event,
                        &swap.token_out_ref().token_address,
                        swap.token_out_amount_big(),
                        &day_id,
                        &hour_id,
                    );
                }
                Type::SwapUnderlyingEvent(swap_underlying) => {
                    update_pool_volume(
                        &output_store,
                        &event,
                        &swap_underlying.token_in_ref().token_address,
                        swap_underlying.token_in_amount_big(),
                        &day_id,
                        &hour_id,
                    );
                }
                _ => {}
            }
        }
    }

    // Initialise pruning for token volume native data using `TokenVolumeNativePruner`.
    // This setup registers the pruner to execute when new timeframes (day/hour) are detected,
    // ensuring outdated data is removed to maintain store efficiency. Protocol and pool level pruning
    // are not required for this module, hence passed as `None`.
    let token_volume_native_pruner = TokenVolumeNativePruner {
        store: &output_store,
    };
    setup_timeframe_pruning(
        Some(&pools_store),
        Some(&pool_count_store),
        Some(&pool_addresses_store),
        &current_time_deltas,
        None as Option<&dyn ProtocolPruneAction>,
        None as Option<&dyn PoolPruneAction>,
        Some(&token_volume_native_pruner),
    );
}

fn update_pool_volume(
    output_store: &StoreAddBigInt,
    event: &PoolEvent,
    token_address: &String,
    amount: BigInt,
    day_id: &i64,
    hour_id: &i64,
) {
    output_store.add_many(
        event.log_ordinal,
        &vec![
            StoreKey::pool_token_volume_native_daily_key(
                &event.pool_address,
                &token_address,
                &day_id,
            ),
            StoreKey::pool_token_volume_native_hourly_key(
                &event.pool_address,
                &token_address,
                &hour_id,
            ),
        ],
        amount,
    );
}
