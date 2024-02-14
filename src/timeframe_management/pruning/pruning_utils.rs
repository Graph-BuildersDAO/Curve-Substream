use substreams::store::{
    DeltaInt64, Deltas, StoreGet, StoreGetInt64, StoreGetProto, StoreGetString,
};

use crate::{
    key_management::store_key_manager::StoreKey,
    pb::curve::types::v1::Pool,
    timeframe_management::{
        timeframe_change_handler::TimeframeChangeHandler, utils::separate_timeframe_deltas,
    },
    types::timeframe::Timeframe,
};

use super::traits::{PoolPruneAction, ProtocolPruneAction, TokenPruneAction};

// This function sets up the pruning process by determining when to prune based on timeframe changes
// and executing the appropriate pruning actions for protocols, pools, and tokens.
// Actions must implement the above traits. See `/pruners` for specific implementations.
pub fn setup_timeframe_pruning<'a>(
    pools_store: &'a StoreGetProto<Pool>,
    pool_count_store: &'a StoreGetInt64,
    pool_addresses_store: &'a StoreGetString,
    current_time_deltas: &'a Deltas<DeltaInt64>,
    protocol_prune_action: Option<&'a dyn ProtocolPruneAction>,
    pool_prune_action: Option<&'a dyn PoolPruneAction>,
    token_prune_action: Option<&'a dyn TokenPruneAction>,
) {
    let (daily_deltas, hourly_deltas) = separate_timeframe_deltas(current_time_deltas);

    // Create closures for daily and hourly pruning actions using the provided pruning actions.
    let on_new_day = create_pruning_closure(
        pools_store,
        pool_count_store,
        pool_addresses_store,
        protocol_prune_action,
        pool_prune_action,
        token_prune_action,
        Timeframe::Daily,
    );
    let on_new_hour = create_pruning_closure(
        pools_store,
        pool_count_store,
        pool_addresses_store,
        protocol_prune_action,
        pool_prune_action,
        token_prune_action,
        Timeframe::Hourly,
    );

    // Initialise the TimeframeChangeHandler with the daily and hourly deltas and closures.
    // This handler will listen for changes in the timeframe and trigger the appropriate pruning actions.
    let mut timeframe_change_handler = TimeframeChangeHandler {
        daily_deltas: &daily_deltas,
        hourly_deltas: &hourly_deltas,
        on_new_day,
        on_new_hour: Some(on_new_hour),
    };

    timeframe_change_handler.handle_timeframe_changes();
}

// This function creates a closure for pruning actions based on the specified timeframe.
// It dynamically generates a closure that will be called by the TimeframeChangeHandler
// when a new day or hour is detected, triggering the appropriate pruning actions.
fn create_pruning_closure<'a>(
    pools_store: &'a StoreGetProto<Pool>,
    pool_count_store: &'a StoreGetInt64,
    pool_addresses_store: &'a StoreGetString,
    protocol_prune_action: Option<&'a dyn ProtocolPruneAction>,
    pool_prune_action: Option<&'a dyn PoolPruneAction>,
    token_prune_action: Option<&'a dyn TokenPruneAction>,
    timeframe: Timeframe,
) -> Box<dyn FnMut(i64) + 'a> {
    Box::new(move |time_frame_id: i64| {
        // Calculate the timeframe ID for pruning to ensure we're pruning data
        // from two timeframes ago, avoiding premature data deletion.
        let prune_time_frame_id = time_frame_id - 1;

        // If a protocol-wide prune action is specified, execute it and return early,
        // as protocol prunes are currently independent of pool-level or token-level prunes.
        if let Some(action) = protocol_prune_action {
            action.prune_protocol(&prune_time_frame_id, &timeframe);
            return;
        }

        let pool_count = match pool_count_store.get_last(StoreKey::protocol_pool_count_key()) {
            Some(count) => count,
            None => return, // Safely handle the absence of pool_count
        };

        for i in 1..=pool_count {
            let pool_address = match pool_addresses_store.get_last(StoreKey::pool_address_key(&i)) {
                Some(address) => address,
                None => continue,
            };

            let pool = match pools_store.get_last(StoreKey::pool_key(&pool_address)) {
                Some(pool) => pool,
                None => continue,
            };

            // Execute the pool prune action if specified.
            if let Some(action) = pool_prune_action {
                action.prune_pool(&pool_address, &prune_time_frame_id, &timeframe)
            }

            // Execute the token prune action for each input and output token of the pool if specified.
            if let Some(action) = token_prune_action {
                for token in &pool.input_tokens {
                    action.prune_token(
                        &pool_address,
                        &token.address,
                        &prune_time_frame_id,
                        &timeframe,
                    );
                }

                action.prune_token(
                    &pool_address,
                    &pool.output_token_ref().address,
                    &prune_time_frame_id,
                    &timeframe,
                );
            }
        }
    })
}
