use substreams::store::{
    DeltaInt64, Deltas, StoreDelete, StoreGet, StoreGetInt64, StoreGetProto, StoreGetString,
    StoreNew, StoreSetString,
};

use crate::{
    key_management::store_key_manager::StoreKey,
    pb::curve::types::v1::Pool,
    snapshot::{
        timeframe_change_handler::TimeframeChangeHandler, utils::separate_timeframe_deltas,
    },
    types::timeframe::Timeframe,
};

// The `prune` module is responsible for setting up the pruning process based on time deltas.
// It utilises the TimeframeChangeHandler to trigger pruning operations at the appropriate times.
// The strategy we use is to identify when a new day/hour occurs, and prune data 2 timeframes prior.
// This stategy ensures that no premature data deletion occurs before a snapshot can be created.
#[substreams::handlers::store]
pub fn prune(
    pools_store: StoreGetProto<Pool>,
    pool_count_store: StoreGetInt64,
    pool_addresses_store: StoreGetString,
    current_time_deltas: Deltas<DeltaInt64>,
    prune_store: StoreSetString,
) {
    setup_timeframe_pruning(
        &pools_store,
        &pool_count_store,
        &pool_addresses_store,
        &prune_store,
        &current_time_deltas,
    );
}

// Sets up the TimeframeChangeHandler with specific pruning closures for daily and hourly timeframes.
fn setup_timeframe_pruning(
    pools_store: &StoreGetProto<Pool>,
    pool_count_store: &StoreGetInt64,
    pool_addresses_store: &StoreGetString,
    prune_store: &StoreSetString,
    current_time_deltas: &Deltas<DeltaInt64>,
) {
    let (daily_deltas, hourly_deltas) = separate_timeframe_deltas(current_time_deltas);

    let on_new_day = setup_pruning_closure(
        pools_store,
        pool_count_store,
        pool_addresses_store,
        prune_store,
        &Timeframe::Daily,
    );
    let on_new_hour = setup_pruning_closure(
        pools_store,
        pool_count_store,
        pool_addresses_store,
        prune_store,
        &Timeframe::Hourly,
    );

    let mut timeframe_change_handler = TimeframeChangeHandler {
        daily_deltas: &daily_deltas,
        hourly_deltas: &hourly_deltas,
        on_new_day: Box::new(on_new_day),
        on_new_hour: Some(Box::new(on_new_hour)),
    };

    timeframe_change_handler.handle_timeframe_changes();
}

// Creates a closure for pruning based on the provided timeframe.
fn setup_pruning_closure<'a>(
    pools_store: &'a StoreGetProto<Pool>,
    pool_count_store: &'a StoreGetInt64,
    pool_addresses_store: &'a StoreGetString,
    prune_store: &'a StoreSetString,
    timeframe: &'a Timeframe,
) -> impl FnMut(i64) + 'a {
    move |time_frame_id: i64| {
        // Previous day/hour is used. Minus 1 to ensure we are pruning 2 timeframes ago to ensure no premature data deletion.
        let prune_time_frame_id = time_frame_id - 1;

        // Prune all daily/hourly protocol data.
        if let Timeframe::Daily = timeframe {
            prune_protocol_volume_data(prune_time_frame_id, prune_store);
        }

        let pool_count = match pool_count_store.get_last(StoreKey::protocol_pool_count_key()) {
            Some(count) => count,
            None => return,
        };

        // Loop through each existing pool, and prune all relevant data related to the pool.
        for i in 1..pool_count {
            let pool_address = match pool_addresses_store.get_last(StoreKey::pool_address_key(&i)) {
                Some(address) => address,
                None => continue,
            };

            let pool = match pools_store.get_last(StoreKey::pool_key(&pool_address)) {
                Some(pool) => pool,
                None => continue,
            };

            prune_pool_volume_data(
                &pool,
                &pool_address,
                prune_time_frame_id,
                timeframe,
                prune_store,
            );
        }
    }
}

// Prunes protocol-wide volume data for a specific day.
fn prune_protocol_volume_data(prune_day_id: i64, prune_store: &StoreSetString) {
    prune_store.delete_prefix(0, &StoreKey::protocol_daily_volume_usd_key(&prune_day_id));
}

// Prunes pool level volume data for a specific day/hour.
fn prune_pool_volume_data(
    pool: &Pool,
    pool_address: &str,
    prune_time_frame_id: i64,
    timeframe: &Timeframe,
    prune_store: &StoreSetString,
) {
    // Prune daily/hourly pool volume data
    let pool_volume_usd_key = match timeframe {
        Timeframe::Daily => {
            StoreKey::pool_volume_usd_daily_key(&pool_address, &prune_time_frame_id)
        }
        Timeframe::Hourly => {
            StoreKey::pool_volume_usd_hourly_key(&pool_address, &prune_time_frame_id)
        }
    };
    prune_store.delete_prefix(0, &pool_volume_usd_key);

    // Prune all daily/hourly input token volume data
    for token in &pool.input_tokens {
        prune_token_volume_data(
            pool_address,
            &token.address,
            prune_time_frame_id,
            timeframe,
            prune_store,
        );
    }

    // Prune all daily/hourly output token volume data
    prune_token_volume_data(
        pool_address,
        &pool.output_token_ref().address,
        prune_time_frame_id,
        timeframe,
        prune_store,
    );
}

// Prunes volume data for a specific token within a pool.
fn prune_token_volume_data(
    pool_address: &str,
    token_address: &str,
    prune_time_frame_id: i64,
    timeframe: &Timeframe,
    prune_store: &StoreSetString,
) {
    let volume_native_key = match timeframe {
        Timeframe::Daily => StoreKey::pool_token_volume_native_daily_key(
            pool_address,
            token_address,
            &prune_time_frame_id,
        ),
        Timeframe::Hourly => StoreKey::pool_token_volume_native_hourly_key(
            pool_address,
            token_address,
            &prune_time_frame_id,
        ),
    };
    prune_store.delete_prefix(0, &volume_native_key);

    let volume_usd_key = match timeframe {
        Timeframe::Daily => StoreKey::pool_token_volume_usd_daily_key(
            pool_address,
            token_address,
            &prune_time_frame_id,
        ),
        Timeframe::Hourly => StoreKey::pool_token_volume_usd_hourly_key(
            pool_address,
            token_address,
            &prune_time_frame_id,
        ),
    };
    prune_store.delete_prefix(0, &volume_usd_key);
}
