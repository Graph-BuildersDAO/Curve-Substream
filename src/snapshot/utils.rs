use std::{cell::RefCell, rc::Rc};

use substreams::store::{DeltaInt64, Deltas};

use crate::{key_management::store_key_manager::StoreKey, types::timeframe::Timeframe};

use super::snapshot_creator::SnapshotCreator;

pub fn separate_timeframe_deltas(
    deltas: &Deltas<DeltaInt64>,
) -> (Deltas<DeltaInt64>, Deltas<DeltaInt64>) {
    let daily_deltas: Vec<DeltaInt64> = deltas
        .iter()
        .filter(|delta| delta.key == StoreKey::current_day_id_key())
        .cloned()
        .collect();

    let hourly_deltas: Vec<DeltaInt64> = deltas
        .iter()
        .filter(|delta| delta.key == StoreKey::current_hour_id_key())
        .cloned()
        .collect();

    (
        Deltas {
            deltas: daily_deltas,
        },
        Deltas {
            deltas: hourly_deltas,
        },
    )
}

pub fn prepare_snapshot_closure(
    snapshot_creator: Rc<RefCell<SnapshotCreator>>,
    snapshot_type: Timeframe,
) -> impl FnMut(i64) + '_ {
    move |time_frame_id| {
        let mut creator = snapshot_creator.borrow_mut();
        match snapshot_type {
            Timeframe::Daily => {
                creator.create_protocol_financials_daily_snapshot(&time_frame_id);
                creator.create_liquidity_pool_snapshots(&snapshot_type, &time_frame_id);
            }
            Timeframe::Hourly => {
                creator.create_liquidity_pool_snapshots(&snapshot_type, &time_frame_id);
            }
        }
    }
}

pub fn calculate_day_hour_id(timestamp_seconds: i64) -> (i64, i64) {
    let day_id = timestamp_seconds / 86400; // Number of seconds in a day
    let hour_id = timestamp_seconds / 3600; // Number of seconds in an hour
    (day_id, hour_id)
}