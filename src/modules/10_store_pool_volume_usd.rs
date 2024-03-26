use std::ops::Div;

use substreams::{
    pb::substreams::Clock,
    store::{
        DeltaInt64, Deltas, StoreAdd, StoreAddBigDecimal, StoreGet, StoreGetBigDecimal,
        StoreGetProto, StoreNew,
    },
};

use crate::{
    common::{
        pool_utils::{is_base_to_meta_exchange, is_meta_to_base_exchange},
        prices::get_token_usd_price,
    },
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
                Type::SwapUnderlyingMetaEvent(swap_underlying) => {
                    let is_meta_to_base = is_meta_to_base_exchange(&swap_underlying);
                    let is_base_to_meta = is_base_to_meta_exchange(&swap_underlying);

                    if is_meta_to_base || is_base_to_meta {
                        let meta_token_opt = if is_meta_to_base {
                            pool.input_tokens
                                .iter()
                                .find(|t| t.address == swap_underlying.token_in_ref().token_address)
                        } else {
                            pool.input_tokens.iter().find(|t| {
                                t.address == swap_underlying.token_out_ref().token_address
                            })
                        };

                        if let Some(meta_token) = meta_token_opt {
                            let meta_token_price = get_token_usd_price(
                                &meta_token,
                                &uniswap_prices,
                                &chainlink_prices,
                            );
                            let meta_token_amount = if is_meta_to_base {
                                swap_underlying.token_in_ref().amount_big()
                            } else {
                                swap_underlying.token_out_ref().amount_big()
                            };
                            let meta_token_amount_usd = meta_token_price
                                * meta_token_amount.to_decimal(meta_token.decimals);

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
                                    StoreKey::pool_token_volume_usd_daily_key(
                                        &day_id,
                                        &event.pool_address,
                                        &meta_token.address,
                                    ),
                                    StoreKey::pool_token_volume_usd_hourly_key(
                                        &hour_id,
                                        &event.pool_address,
                                        &meta_token.address,
                                    ),
                                ],
                                &meta_token_amount_usd,
                            );
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
