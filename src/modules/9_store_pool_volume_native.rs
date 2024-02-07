use substreams::{
    pb::substreams::Clock,
    store::{StoreAdd, StoreAddBigInt, StoreGet, StoreGetProto, StoreNew},
};

use crate::{
    key_management::store_key_manager::StoreKey,
    pb::curve::types::v1::{events::pool_event::Type, Events, Pool},
};

#[substreams::handlers::store]
pub fn store_pool_volume_native(
    clock: Clock,
    events: Events,
    pools_store: StoreGetProto<Pool>,
    store: StoreAddBigInt,
) {
    // TODO. Could we move this to some utils?
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id = timestamp_seconds / 86400;
    let hour_id = timestamp_seconds / 3600;
    let prev_day_id = day_id - 1;
    let prev_hour_id = hour_id - 1;

    for event in events.pool_events {
        // Ensure there is a pool for this event
        let _pool = pools_store.must_get_last(StoreKey::pool_key(&event.pool_address));
        if let Some(event_type) = &event.r#type {
            match event_type {
                Type::SwapEvent(swap) => {
                    store.add_many(
                        event.log_ordinal,
                        &vec![
                            StoreKey::pool_token_volume_native_daily_key(
                                &event.pool_address,
                                &swap.token_in_ref().token_address,
                                &day_id,
                            ),
                            StoreKey::pool_token_volume_native_hourly_key(
                                &event.pool_address,
                                &swap.token_in_ref().token_address,
                                &hour_id,
                            ),
                        ],
                        swap.token_in_amount_big(),
                    );
                    store.add_many(
                        event.log_ordinal,
                        &vec![
                            StoreKey::pool_token_volume_native_daily_key(
                                &event.pool_address,
                                &swap.token_out_ref().token_address,
                                &day_id,
                            ),
                            StoreKey::pool_token_volume_native_hourly_key(
                                &event.pool_address,
                                &swap.token_out_ref().token_address,
                                &hour_id,
                            ),
                        ],
                        swap.token_out_amount_big(),
                    );
                }
                Type::SwapUnderlyingEvent(swap_underlying) => {
                    store.add_many(
                        event.log_ordinal,
                        &vec![
                            StoreKey::pool_token_volume_native_daily_key(
                                &event.pool_address,
                                &swap_underlying.token_in_ref().token_address,
                                &day_id,
                            ),
                            StoreKey::pool_token_volume_native_hourly_key(
                                &event.pool_address,
                                &swap_underlying.token_in_ref().token_address,
                                &hour_id,
                            ),
                        ],
                        swap_underlying.token_in_amount_big(),
                    );
                }
                _ => {}
            }
        }
    }
}
