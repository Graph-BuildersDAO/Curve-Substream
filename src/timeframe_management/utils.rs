use substreams::store::{DeltaInt64, Deltas};

use crate::key_management::store_key_manager::StoreKey;

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

pub fn calculate_day_hour_id(timestamp_seconds: i64) -> (i64, i64) {
    let day_id = timestamp_seconds / 86400; // Number of seconds in a day
    let hour_id = timestamp_seconds / 3600; // Number of seconds in an hour
    (day_id, hour_id)
}
