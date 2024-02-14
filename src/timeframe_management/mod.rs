// The `timeframe_management` module encapsulates functionalities related to managing timeframes,
// creating snapshots of pool and protocol data, and pruning outdated data to maintain efficiency.
// It includes the following key components:
//
// - `SnapshotCreator`: Responsible for creating detailed snapshots of liquidity pools and protocol
//   financials based on predefined timeframes (daily, hourly, etc.).
//
// - `Pruning`: A set of functionalities and traits designed to prune outdated data from stores,
//   ensuring that only relevant and current data is maintained. This is crucial for optimizing
//   storage and query performance over time.
//
// - `Pruners`: Specific implementations of the pruning actions defined by traits in the `pruning/traits`
//   module. These are concrete actions that remove old data based on the timeframes that have
//   passed.
//
// - `TimeframeChangeHandler`: A central component responsible for detecting transitions
//   between timeframes (daily and hourly) based on deltas. It employs a reactive pattern,
//   executing specified closures when a new day or hour is detected. This allows for
//   dynamic actions such as snapshot creation or data pruning to be automatically triggered
//   in response to the passage of time.
//
// - `Utils`: Utility functions supporting both snapshot creation and pruning operations.
//
pub mod pruning;
pub mod snapshot;
pub mod timeframe_change_handler;
pub mod utils;
