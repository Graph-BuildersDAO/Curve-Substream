use std::ops::Div;

use substreams::{
    pb::substreams::Clock,
    store::{
        DeltaInt64, Deltas, StoreAdd, StoreAddBigDecimal, StoreGet, StoreGetBigDecimal,
        StoreGetProto, StoreNew,
    },
};

use crate::{
    common::prices::get_token_usd_price,
    key_management::store_key_manager::StoreKey,
    pb::{
        curve::types::v1::{events::pool_event::Type, Events, Pool},
        uniswap_pricing::v1::Erc20Price,
    },
    timeframe_management::{
        pruning::{
            pruners::{
                pool_volume_usd_pruner::PoolVolumeUsdPruner,
                token_volume_usd_pruner::TokenVolumeUsdPruner,
            },
            setup_timeframe_pruning,
        },
        utils::calculate_day_hour_id,
    },
};

#[substreams::handlers::store]
pub fn store_pool_volume_usd(
    clock: Clock,
    events: Events,
    pools_store: StoreGetProto<Pool>,
    current_time_deltas: Deltas<DeltaInt64>,
    chainlink_prices: StoreGetBigDecimal,
    uniswap_prices: StoreGetProto<Erc20Price>,
    output_store: StoreAddBigDecimal,
) {
    // Initialise pruning for pool/token volume usd data using `PoolVolumeUsdPruner`/`TokenVolumeUsdPruner`.
    // This setup registers the pruners to execute when new timeframes (day/hour) are detected,
    // ensuring outdated data is removed to maintain store efficiency.
    let pool_volume_usd_pruner = PoolVolumeUsdPruner {
        store: &output_store,
    };
    let token_volume_usd_pruner = TokenVolumeUsdPruner {
        store: &output_store,
    };
    setup_timeframe_pruning(
        &current_time_deltas,
        &[&pool_volume_usd_pruner, &token_volume_usd_pruner],
    );

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

                    output_store.add_many(
                        event.log_ordinal,
                        &vec![
                            StoreKey::pool_volume_usd_key(&event.pool_address),
                            StoreKey::pool_volume_usd_daily_key(&day_id, &event.pool_address),
                            StoreKey::pool_volume_usd_hourly_key(&hour_id, &event.pool_address),
                        ],
                        &volume_usd,
                    );
                    output_store.add_many(
                        event.log_ordinal,
                        &vec![
                            StoreKey::pool_token_volume_usd_daily_key(
                                &day_id,
                                &event.pool_address,
                                &swap.token_in_ref().token_address,
                            ),
                            StoreKey::pool_token_volume_usd_hourly_key(
                                &hour_id,
                                &event.pool_address,
                                &swap.token_in_ref().token_address,
                            ),
                        ],
                        token_in_amount_usd,
                    );
                    output_store.add_many(
                        event.log_ordinal,
                        &vec![
                            StoreKey::pool_token_volume_usd_daily_key(
                                &day_id,
                                &event.pool_address,
                                &swap.token_out_ref().token_address,
                            ),
                            StoreKey::pool_token_volume_usd_hourly_key(
                                &hour_id,
                                &event.pool_address,
                                &swap.token_out_ref().token_address,
                            ),
                        ],
                        token_out_amount_usd,
                    );
                }
                Type::SwapUnderlyingEvent(swap_underlying) => {
                    let base_pool_option = pools_store
                        .get_last(StoreKey::pool_key(&swap_underlying.base_pool_address));

                    let token_in_option = pool
                        .input_tokens
                        .iter()
                        .find(|t| t.address == swap_underlying.token_in_ref().token_address);

                    if let Some(token_in) = token_in_option {
                        let token_in_price =
                            get_token_usd_price(&token_in, &uniswap_prices, &chainlink_prices);

                        let token_in_amount_usd = token_in_price
                            * swap_underlying
                                .token_in_amount_big()
                                .to_decimal(token_in.decimals);

                        // If we have a Curve base pool tracked, we can incorporate
                        // the known token price into our TVL calculations.
                        if let Some(base_pool) = base_pool_option {
                            let token_out_option = base_pool.input_tokens.iter().find(|t| {
                                t.address == swap_underlying.token_out_ref().token_address
                            });

                            if let Some(token_out) = token_out_option {
                                let token_out_price = get_token_usd_price(
                                    &token_out,
                                    &uniswap_prices,
                                    &chainlink_prices,
                                );
                                let token_out_amount_usd = token_out_price
                                    * swap_underlying
                                        .token_out_amount_big()
                                        .to_decimal(token_out.decimals);

                                let volume_usd = (token_in_amount_usd.clone()
                                    + token_out_amount_usd.clone())
                                .div(2);

                                output_store.add_many(
                                    event.log_ordinal,
                                    &vec![
                                        StoreKey::pool_volume_usd_key(&event.pool_address),
                                        StoreKey::pool_volume_usd_daily_key(
                                            &day_id,
                                            &event.pool_address,
                                        ),
                                        StoreKey::pool_volume_usd_hourly_key(
                                            &hour_id,
                                            &event.pool_address,
                                        ),
                                    ],
                                    &volume_usd,
                                );
                            }
                        } else {
                            // If there is no Curve base pool, just use the known pools token for the TVL calculations
                            output_store.add_many(
                                event.log_ordinal,
                                &vec![
                                    StoreKey::pool_volume_usd_key(&event.pool_address),
                                    StoreKey::pool_volume_usd_daily_key(
                                        &day_id,
                                        &event.pool_address,
                                    ),
                                    StoreKey::pool_volume_usd_hourly_key(
                                        &hour_id,
                                        &event.pool_address,
                                    ),
                                ],
                                &token_in_amount_usd,
                            );
                        }

                        // For SwapUnderlying events, we only need to track the volume for the tokens related to this pool.
                        // The base pool handles the volume for the underlying token swapped out.
                        output_store.add_many(
                            event.log_ordinal,
                            &vec![
                                StoreKey::pool_token_volume_usd_daily_key(
                                    &day_id,
                                    &event.pool_address,
                                    &swap_underlying.token_in_ref().token_address,
                                ),
                                StoreKey::pool_token_volume_usd_hourly_key(
                                    &hour_id,
                                    &event.pool_address,
                                    &swap_underlying.token_in_ref().token_address,
                                ),
                            ],
                            token_in_amount_usd,
                        );
                    }
                }
                _ => {}
            }
        }
    }
}
