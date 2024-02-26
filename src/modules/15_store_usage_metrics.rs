use num_traits::ToPrimitive;
use substreams::{
    key,
    pb::substreams::Clock,
    store::{DeltaInt64, Deltas, StoreAdd, StoreAddInt64, StoreNew},
};

use crate::{
    key_management::store_key_manager::StoreKey,
    pb::curve::types::v1::{events::pool_event::Type, Events},
    timeframe_management::{
        pruning::{
            pruners::protocol_usage_metrics_pruner::ProtocolUsageMetricsPruneAction,
            setup_timeframe_pruning,
        },
        utils::calculate_day_hour_id,
    },
};

#[substreams::handlers::store]
pub fn store_usage_metrics(
    clock: Clock,
    events: Events,
    active_users_deltas: Deltas<DeltaInt64>,
    current_time_deltas: Deltas<DeltaInt64>,
    output_store: StoreAddInt64,
) {
    let (day_id, hour_id) = calculate_day_hour_id(clock.timestamp.unwrap().seconds);

    let (general, daily, hourly) = separate_active_users_deltas(&active_users_deltas);

    if !general.deltas.is_empty() {
        output_store.add(
            0,
            StoreKey::active_user_count_key(),
            general.deltas.len().to_i64().unwrap_or_default(),
        );
        output_store.add(
            0,
            StoreKey::active_user_daily_count_key(&day_id),
            daily.deltas.len().to_i64().unwrap_or_default(),
        );
        output_store.add(
            0,
            StoreKey::active_user_hourly_count_key(&hour_id),
            hourly.deltas.len().to_i64().unwrap_or_default(),
        );
    }

    if !events.pool_events.is_empty() {
        // Initialise counters for event types
        let mut total_events_count = 0;
        let mut swap_events_count = 0;
        let mut deposit_events_count = 0;
        let mut withdraw_events_count = 0;

        // Increment each relevant event type counter
        for event in events.pool_events {
            total_events_count += 1;

            if let Some(event_type) = &event.r#type {
                match event_type {
                    Type::SwapEvent(_) => {
                        swap_events_count += 1;
                    }
                    Type::SwapUnderlyingEvent(_) => {
                        swap_events_count += 1;
                    }
                    Type::DepositEvent(_) => {
                        deposit_events_count += 1;
                    }
                    Type::WithdrawEvent(_) => {
                        withdraw_events_count += 1;
                    }
                }
            }
        }

        // Update store values for liquidity event counts
        output_store.add_many(
            0,
            &vec![
                StoreKey::transaction_daily_count_key(&day_id),
                StoreKey::transaction_hourly_count_key(&hour_id),
            ],
            total_events_count,
        );
        output_store.add_many(
            0,
            &vec![
                StoreKey::swap_daily_count_key(&day_id),
                StoreKey::swap_hourly_count_key(&hour_id),
            ],
            swap_events_count,
        );
        output_store.add_many(
            0,
            &vec![
                StoreKey::deposit_daily_count_key(&day_id),
                StoreKey::deposit_hourly_count_key(&hour_id),
            ],
            deposit_events_count,
        );
        output_store.add_many(
            0,
            &vec![
                StoreKey::withdraw_daily_count_key(&day_id),
                StoreKey::withdraw_hourly_count_key(&hour_id),
            ],
            withdraw_events_count,
        );
    }

    let protocol_usage_metrics_pruner = ProtocolUsageMetricsPruneAction {
        store: &output_store,
    };

    setup_timeframe_pruning(&current_time_deltas, &[&protocol_usage_metrics_pruner]);
}

fn separate_active_users_deltas(
    deltas: &Deltas<DeltaInt64>,
) -> (Deltas<DeltaInt64>, Deltas<DeltaInt64>, Deltas<DeltaInt64>) {
    let (mut general, mut daily, mut hourly) = (Vec::new(), Vec::new(), Vec::new());

    for delta in deltas.iter() {
        match key::first_segment(&delta.key) {
            // TODO: Can we remove the reliance on these magic strings?
            //       Add these to the store key manager?
            "ActiveUser" => general.push(delta.clone()),
            "ActiveUserDaily" => daily.push(delta.clone()),
            "ActiveUserHourly" => hourly.push(delta.clone()),
            _ => {}
        }
    }

    (
        Deltas { deltas: general },
        Deltas { deltas: daily },
        Deltas { deltas: hourly },
    )
}
