use std::{cell::RefCell, rc::Rc};

use substreams::{
    pb::substreams::Clock,
    store::{
        DeltaInt64, Deltas, StoreGetBigDecimal, StoreGetBigInt, StoreGetInt64, StoreGetProto,
        StoreGetString,
    },
};
use substreams_entity_change::tables::Tables;

use crate::{
    pb::{curve::types::v1::{Pool, PoolRewards}, uniswap_pricing::v1::Erc20Price},
    timeframe_management::{
        timeframe_change_handler::TimeframeChangeHandler, utils::separate_timeframe_deltas,
    },
    types::timeframe::Timeframe,
};

use super::snapshot_creator::SnapshotCreator;

/// Manages the creation of snapshots by monitoring changes in timeframes (daily and hourly)
/// and triggering snapshot creation through a `SnapshotCreator`. It sets up a `TimeframeChangeHandler`
/// with closures that are called when a new day or hour is detected based on the deltas provided.
pub fn manage_timeframe_snapshots(
    clock: &Clock,
    deltas: &Deltas<DeltaInt64>,
    tables: &mut Tables,
    usage_metrics_store: &StoreGetInt64,
    pool_count_store: &StoreGetInt64,
    pool_addresses_store: &StoreGetString,
    pools_store: &StoreGetProto<Pool>,
    pool_tvl_store: &StoreGetBigDecimal,
    pool_volume_usd_store: &StoreGetBigDecimal,
    pool_volume_native_store: &StoreGetBigInt,
    protocol_tvl_store: &StoreGetBigDecimal,
    protocol_volume_store: &StoreGetBigDecimal,
    input_token_balances_store: &StoreGetBigInt,
    output_token_supply_store: &StoreGetBigInt,
    pool_rewards_store: &StoreGetProto<PoolRewards>,
    uniswap_prices: &StoreGetProto<Erc20Price>,
    chainlink_prices: &StoreGetBigDecimal,
) {
    let snapshot_creator = Rc::new(RefCell::new(SnapshotCreator::new(
        tables,
        clock,
        usage_metrics_store,
        pool_count_store,
        pool_addresses_store,
        pools_store,
        pool_tvl_store,
        pool_volume_usd_store,
        pool_volume_native_store,
        protocol_tvl_store,
        protocol_volume_store,
        input_token_balances_store,
        output_token_supply_store,
        pool_rewards_store,
        uniswap_prices,
        chainlink_prices,
    )));

    let (daily_deltas, hourly_deltas) = separate_timeframe_deltas(deltas);

    // Prepare closures for handling new day and new hour events.
    // These closures will utilize the SnapshotCreator to generate snapshots.
    // We use Rc::clone to ensure the SnapshotCreator can be shared among closures without taking ownership.
    let on_new_day = prepare_snapshot_closure(Rc::clone(&snapshot_creator), Timeframe::Daily);
    let on_new_hour = prepare_snapshot_closure(Rc::clone(&snapshot_creator), Timeframe::Hourly);

    // Initialise the TimeframeChangeHandler with the separated deltas and the prepared closures.
    // This handler will check for updates in daily and hourly deltas and trigger the appropriate closures.
    let mut timeframe_change_handler = TimeframeChangeHandler {
        daily_deltas: &daily_deltas,
        hourly_deltas: &hourly_deltas,
        on_new_day: Box::new(on_new_day),
        on_new_hour: Some(Box::new(on_new_hour)),
    };

    // Process the timeframe changes by iterating over deltas and triggering closures if conditions are met.
    timeframe_change_handler.handle_timeframe_changes();
}

/// Prepares a closure to create snapshots based on the specified timeframe.
/// This closure captures a `SnapshotCreator` and, depending on the timeframe,
/// calls methods on the `SnapshotCreator` to generate daily or hourly snapshots.
pub fn prepare_snapshot_closure(
    snapshot_creator: Rc<RefCell<SnapshotCreator>>,
    snapshot_type: Timeframe,
) -> impl FnMut(i64) + '_ {
    move |time_frame_id| {
        let mut creator = snapshot_creator.borrow_mut();
        match snapshot_type {
            Timeframe::Daily => {
                creator.create_usage_metrics_snapshots(&snapshot_type, &time_frame_id);
                creator.create_protocol_financials_daily_snapshot(&time_frame_id);
                creator.create_liquidity_pool_snapshots(&snapshot_type, &time_frame_id);
            }
            Timeframe::Hourly => {
                creator.create_usage_metrics_snapshots(&snapshot_type, &time_frame_id);
                creator.create_liquidity_pool_snapshots(&snapshot_type, &time_frame_id);
            }
        }
    }
}
