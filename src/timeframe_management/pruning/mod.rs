use substreams::store::{DeltaInt64, Deltas};

use crate::{
    timeframe_management::{
        timeframe_change_handler::TimeframeChangeHandler, utils::separate_timeframe_deltas,
    },
    types::timeframe::Timeframe,
};

pub mod pruners;

// Pruner trait allows for implementing custom store pruning logic based on timeframes.
pub trait Pruner {
    fn prune(&self, prune_time_frame_id: i64, timeframe: Timeframe);
}

// Configures and initiates the pruning process based on detected changes in timeframes.
// It accepts a list of pruners that define specific pruning actions.
// Prune actions must implement the above `Pruner` trait.
pub fn setup_timeframe_pruning<'a>(
    current_time_deltas: &'a Deltas<DeltaInt64>,
    pruners: &[&'a dyn Pruner],
) {
    let (daily_deltas, hourly_deltas) = separate_timeframe_deltas(current_time_deltas);

    // Defines a closure to execute daily pruning actions.
    let on_new_day = Box::new(move |time_frame_id: i64| {
        // Calculate the timeframe ID for pruning to ensure we're pruning data
        // from two timeframes ago, avoiding premature data deletion.
        let time_frame_id = time_frame_id - 1;
        for pruner in pruners {
            pruner.prune(time_frame_id, Timeframe::Daily)
        }
    });

    // Defines a closure to execute hourly pruning actions.
    let on_new_hour = Box::new(move |time_frame_id: i64| {
        // Calculate the timeframe ID for pruning to ensure we're pruning data
        // from two timeframes ago, avoiding premature data deletion.
        let time_frame_id = time_frame_id - 1;
        for pruner in pruners {
            pruner.prune(time_frame_id, Timeframe::Hourly)
        }
    });

    // Initialise the TimeframeChangeHandler with the daily and hourly deltas and closures.
    // This handler will listen for changes in the timeframe and trigger the appropriate pruning actions.
    let mut timeframe_change_handler = TimeframeChangeHandler {
        daily_deltas: &daily_deltas,
        hourly_deltas: &hourly_deltas,
        on_new_day,
        on_new_hour: Some(on_new_hour),
    };

    // Executes the timeframe change handler to process any detected changes and perform pruning.
    timeframe_change_handler.handle_timeframe_changes();
}
