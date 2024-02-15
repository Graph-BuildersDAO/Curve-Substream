use substreams::pb::substreams::store_delta::Operation;
use substreams::store::{DeltaInt64, Deltas};

// TimeframeChangeHandler is designed to manage changes in timeframes, specifically daily and hourly changes.
// It holds references to deltas for both timeframes and closures that define the actions to be taken when a new day or hour is detected.
// This allows us to pass in closures that can create snapshots, or prune old data when a new timeframe change occurs.
pub struct TimeframeChangeHandler<'a> {
    pub daily_deltas: &'a Deltas<DeltaInt64>,
    pub hourly_deltas: &'a Deltas<DeltaInt64>,
    pub on_new_day: Box<dyn FnMut(i64) + 'a>,
    pub on_new_hour: Option<Box<dyn FnMut(i64) + 'a>>,
}

impl<'a> TimeframeChangeHandler<'a> {
    pub fn handle_timeframe_changes(&mut self) {
        for delta in &self.daily_deltas.deltas {
            if delta.operation == Operation::Update && delta.old_value != delta.new_value {
                // Trigger the on_new_day closure, passing in the old value (previous day ID) as the argument.
                (self.on_new_day)(delta.old_value);
            }
        }

        if let Some(ref mut on_new_hour) = self.on_new_hour {
            for delta in &self.hourly_deltas.deltas {
                if delta.operation == Operation::Update && delta.old_value != delta.new_value {
                    // Trigger the on_new_hour closure, passing in the old value (previous hour ID) as the argument.
                    on_new_hour(delta.old_value);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;
    use substreams::pb::substreams::store_delta::Operation;
    use substreams::store::{DeltaInt64, Deltas};

    fn create_test_deltas(
        operation: Operation,
        old_value: i64,
        new_value: i64,
    ) -> Deltas<DeltaInt64> {
        Deltas {
            deltas: vec![DeltaInt64 {
                operation: operation,
                ordinal: 0,
                key: String::new(), // The key isn't important for this test
                old_value,
                new_value,
            }],
        }
    }

    #[test]
    fn test_handle_timeframe_changes_new_day() {
        let day_triggered = Rc::new(RefCell::new(false));
        let day_triggered_clone = day_triggered.clone();
        let daily_deltas = create_test_deltas(Operation::Update, 1, 2); // Simulating a change from day 1 to day 2

        let mut handler = TimeframeChangeHandler {
            daily_deltas: &daily_deltas,
            hourly_deltas: &Deltas { deltas: vec![] }, // No changes in hourly deltas for this test
            on_new_day: Box::new(move |_| *day_triggered_clone.borrow_mut() = true),
            on_new_hour: None,
        };

        handler.handle_timeframe_changes();

        assert!(
            *day_triggered.borrow(),
            "Expected on_new_day to be triggered"
        );
    }

    #[test]
    fn test_handle_timeframe_changes_new_hour() {
        let hour_triggered = Rc::new(RefCell::new(false));
        let hour_triggered_clone = hour_triggered.clone();
        let hourly_deltas = create_test_deltas(Operation::Update, 10, 11); // Simulating a change from hour 10 to hour 11

        let mut handler = TimeframeChangeHandler {
            daily_deltas: &Deltas { deltas: vec![] }, // No changes in daily deltas for this test
            hourly_deltas: &hourly_deltas,
            on_new_day: Box::new(|_| {}),
            on_new_hour: Some(Box::new(move |_| *hour_triggered_clone.borrow_mut() = true)),
        };

        handler.handle_timeframe_changes();

        assert!(
            *hour_triggered.borrow(),
            "Expected on_new_hour to be triggered"
        );
    }

    #[test]
    fn test_no_change_no_trigger() {
        let day_triggered = Rc::new(RefCell::new(false));
        let hour_triggered = Rc::new(RefCell::new(false));
        let day_triggered_clone = day_triggered.clone();
        let hour_triggered_clone = hour_triggered.clone();
        let daily_deltas = create_test_deltas(Operation::Update, 2, 2); // No actual change in day
        let hourly_deltas = create_test_deltas(Operation::Update, 11, 11); // No actual change in hour

        let mut handler = TimeframeChangeHandler {
            daily_deltas: &daily_deltas,
            hourly_deltas: &hourly_deltas,
            on_new_day: Box::new(|_| *day_triggered_clone.borrow_mut() = true),
            on_new_hour: Some(Box::new(|_| *hour_triggered_clone.borrow_mut() = true)),
        };

        handler.handle_timeframe_changes();

        assert!(
            !*day_triggered.borrow(),
            "on_new_day should not be triggered without a day change"
        );
        assert!(
            !*hour_triggered.borrow(),
            "on_new_hour should not be triggered without an hour change"
        );
    }

    #[test]
    fn test_operation_create_with_change_no_trigger() {
        let day_triggered = Rc::new(RefCell::new(false));
        let hour_triggered = Rc::new(RefCell::new(false));
        let day_triggered_clone = day_triggered.clone();
        let hour_triggered_clone = hour_triggered.clone();
        let daily_deltas = create_test_deltas(Operation::Create, 0, 1); // Simulating a creation of the first daily timeframe
        let hourly_deltas = create_test_deltas(Operation::Create, 0, 1); // Simulating a creation of the first hourly timeframe

        let mut handler = TimeframeChangeHandler {
            daily_deltas: &daily_deltas,
            hourly_deltas: &hourly_deltas,
            on_new_day: Box::new(|_| *day_triggered_clone.borrow_mut() = true),
            on_new_hour: Some(Box::new(|_| *hour_triggered_clone.borrow_mut() = true)),
        };

        handler.handle_timeframe_changes();

        assert!(
            !*day_triggered.borrow(),
            "on_new_day should not be triggered when delta is create operation"
        );
        assert!(
            !*hour_triggered.borrow(),
            "on_new_hour should not be triggered when delta is create operation"
        );
    }

    #[test]
    fn test_handle_timeframe_changes_no_deltas() {
        let day_triggered = Rc::new(RefCell::new(false));
        let hour_triggered = Rc::new(RefCell::new(false));
        let day_triggered_clone = day_triggered.clone();
        let hour_triggered_clone = hour_triggered.clone();

        let mut handler = TimeframeChangeHandler {
            daily_deltas: &Deltas { deltas: vec![] }, // No changes in daily deltas for this test
            hourly_deltas: &Deltas { deltas: vec![] }, // No changes in hourly deltas for this test
            on_new_day: Box::new(|_| *day_triggered_clone.borrow_mut() = true),
            on_new_hour: Some(Box::new(|_| *hour_triggered_clone.borrow_mut() = true)),
        };

        handler.handle_timeframe_changes();

        assert!(
            !*day_triggered.borrow(),
            "on_new_day should not be triggered when there are no daily deltas"
        );
        assert!(
            !*hour_triggered.borrow(),
            "on_new_hour should not be triggered when there are no hourly deltas"
        );
    }

    // Add more tests as necessary...
}
