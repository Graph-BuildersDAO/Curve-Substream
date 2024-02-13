use substreams::pb::substreams::store_delta::Operation;
use substreams::store::{DeltaInt64, Deltas};

// TimeframeChangeHandler is designed to manage changes in timeframes, specifically daily and hourly changes.
// It holds references to deltas for both timeframes and closures that define the actions to be taken when a new day or hour is detected.
// This allows us to pass in closures that can create snapshots, or prune old data when a new timeframe change occurs.
pub struct TimeframeChangeHandler<'a> {
    pub daily_deltas: &'a Deltas<DeltaInt64>,
    pub hourly_deltas: &'a Deltas<DeltaInt64>,
    pub on_new_day: Box<dyn FnMut(i64) + 'a>,
    pub on_new_hour: Box<dyn FnMut(i64) + 'a>,
}

impl<'a> TimeframeChangeHandler<'a> {
    pub fn handle_timeframe_changes(&mut self) {
        for delta in &self.daily_deltas.deltas {
            if delta.operation == Operation::Update && delta.old_value != delta.new_value {
                // Trigger the on_new_day closure, passing in the old value (previous day ID) as the argument.
                (self.on_new_day)(delta.old_value);
            }
        }

        for delta in &self.hourly_deltas.deltas {
            if delta.operation == Operation::Update && delta.old_value != delta.new_value {
                // Trigger the on_new_hour closure, passing in the old value (previous hour ID) as the argument.
                (self.on_new_hour)(delta.old_value);
            }
        }
    }
}
