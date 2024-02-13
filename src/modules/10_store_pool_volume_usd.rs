use std::ops::Div;

use substreams::{
    pb::substreams::Clock,
    store::{StoreAdd, StoreAddBigDecimal, StoreGet, StoreGetBigDecimal, StoreGetProto, StoreNew},
};

use crate::{
    common::prices::get_token_usd_price,
    key_management::store_key_manager::StoreKey,
    pb::{
        curve::types::v1::{events::pool_event::Type, Events, Pool},
        uniswap_pricing::v1::Erc20Price,
    },
    snapshot::utils::calculate_day_hour_id,
};

#[substreams::handlers::store]
pub fn store_pool_volume_usd(
    clock: Clock,
    events: Events,
    pools_store: StoreGetProto<Pool>,
    chainlink_prices: StoreGetBigDecimal,
    uniswap_prices: StoreGetProto<Erc20Price>,
    store: StoreAddBigDecimal,
) {
    let (day_id, hour_id) = calculate_day_hour_id(clock.timestamp.unwrap().seconds);

    for event in events.pool_events {
        if let Some(event_type) = &event.r#type {
            // Get events related Pool
            let pool = pools_store.must_get_last(StoreKey::pool_key(&event.pool_address));

            match event_type {
                Type::SwapEvent(swap) => {
                    let token_in = pool
                        .input_tokens
                        .iter()
                        .find(|t| t.address == swap.token_in_ref().token_address);
                    let token_out = pool
                        .input_tokens
                        .iter()
                        .find(|t| t.address == swap.token_out_ref().token_address);

                    if token_in.is_none() || token_out.is_none() {
                        substreams::log::debug!("Token_in or Token_out not found in pool for swap event. Skipping event.");
                        continue;
                    }

                    let token_in = token_in.unwrap();
                    let token_out = token_out.unwrap();

                    let token_in_price =
                        get_token_usd_price(&token_in, &uniswap_prices, &chainlink_prices);
                    let token_out_price =
                        get_token_usd_price(&token_out, &uniswap_prices, &chainlink_prices);

                    let token_in_amount_usd =
                        token_in_price * swap.token_in_amount_big().to_decimal(token_in.decimals);
                    let token_out_amount_usd = token_out_price
                        * swap.token_out_amount_big().to_decimal(token_out.decimals);

                    let volume_usd =
                        (token_in_amount_usd.clone() + token_out_amount_usd.clone()).div(2);

                    store.add_many(
                        event.log_ordinal,
                        &vec![
                            StoreKey::pool_volume_usd_key(&event.pool_address),
                            StoreKey::pool_volume_usd_daily_key(&event.pool_address, &day_id),
                            StoreKey::pool_volume_usd_hourly_key(&event.pool_address, &hour_id),
                        ],
                        &volume_usd,
                    );
                    store.add_many(
                        event.log_ordinal,
                        &vec![
                            StoreKey::pool_token_volume_usd_daily_key(
                                &event.pool_address,
                                &swap.token_in_ref().token_address,
                                &day_id,
                            ),
                            StoreKey::pool_token_volume_usd_hourly_key(
                                &event.pool_address,
                                &swap.token_in_ref().token_address,
                                &hour_id,
                            ),
                        ],
                        token_in_amount_usd,
                    );
                    store.add_many(
                        event.log_ordinal,
                        &vec![
                            StoreKey::pool_token_volume_usd_daily_key(
                                &event.pool_address,
                                &swap.token_out_ref().token_address,
                                &day_id,
                            ),
                            StoreKey::pool_token_volume_usd_hourly_key(
                                &event.pool_address,
                                &swap.token_out_ref().token_address,
                                &hour_id,
                            ),
                        ],
                        token_out_amount_usd,
                    );
                }
                Type::SwapUnderlyingEvent(swap_underlying) => {
                    let base_pool = pools_store
                        .must_get_last(StoreKey::pool_key(&swap_underlying.base_pool_address));

                    let token_in = pool
                        .input_tokens
                        .iter()
                        .find(|t| t.address == swap_underlying.token_in_ref().token_address);
                    let token_out = base_pool
                        .input_tokens
                        .iter()
                        .find(|t| t.address == swap_underlying.token_out_ref().token_address);

                    if token_in.is_none() || token_out.is_none() {
                        substreams::log::debug!("Token_in or Token_out not found in pool for swap event. Skipping event.");
                        continue;
                    }

                    let token_in = token_in.unwrap();
                    let token_out = token_out.unwrap();

                    let token_in_price =
                        get_token_usd_price(&token_in, &uniswap_prices, &chainlink_prices);
                    let token_out_price =
                        get_token_usd_price(&token_out, &uniswap_prices, &chainlink_prices);

                    let token_in_amount_usd = token_in_price
                        * swap_underlying
                            .token_in_amount_big()
                            .to_decimal(token_in.decimals);
                    let token_out_amount_usd = token_out_price
                        * swap_underlying
                            .token_out_amount_big()
                            .to_decimal(token_out.decimals);

                    let volume_usd =
                        (token_in_amount_usd.clone() + token_out_amount_usd.clone()).div(2);

                    store.add_many(
                        event.log_ordinal,
                        &vec![
                            StoreKey::pool_volume_usd_key(&event.pool_address),
                            StoreKey::pool_volume_usd_daily_key(&event.pool_address, &day_id),
                            StoreKey::pool_volume_usd_hourly_key(&event.pool_address, &hour_id),
                        ],
                        &volume_usd,
                    );

                    // For SwapUnderlying events, we only need to track the volume for the tokens related to this pool.
                    // The base pool handles the volume for the underlying token swapped out.
                    store.add_many(
                        event.log_ordinal,
                        &vec![
                            StoreKey::pool_token_volume_usd_daily_key(
                                &event.pool_address,
                                &swap_underlying.token_in_ref().token_address,
                                &day_id,
                            ),
                            StoreKey::pool_token_volume_usd_hourly_key(
                                &event.pool_address,
                                &swap_underlying.token_in_ref().token_address,
                                &hour_id,
                            ),
                        ],
                        token_in_amount_usd,
                    );
                }
                _ => {}
            }
        }
    }
}
