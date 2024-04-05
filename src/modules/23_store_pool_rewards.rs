use std::str::FromStr;

use substreams::{
    pb::substreams::Clock,
    scalar::{BigDecimal, BigInt},
    store::{
        StoreGet, StoreGetBigDecimal, StoreGetInt64, StoreGetProto, StoreGetString, StoreNew,
        StoreSet, StoreSetProto,
    },
};

use crate::{
    common::prices,
    constants::{curve_token, default_decimals, SECONDS_PER_DAY},
    key_management::store_key_manager::StoreKey,
    pb::{
        curve::types::v1::{LiquidityGauge, LiquidityGaugeEvents, PoolRewards, Token},
        uniswap_pricing::v1::Erc20Price,
    },
    rpc,
};

#[substreams::handlers::store]
pub fn store_pool_rewards(
    gauge_events: LiquidityGaugeEvents,
    gauge_store: StoreGetProto<LiquidityGauge>,
    gauge_controller_store: StoreGetInt64,
    crv_inflation_store: StoreGetString,
    reward_token_count_store: StoreGetInt64,
    reward_tokens_store: StoreGetProto<Token>,
    uniswap_prices: StoreGetProto<Erc20Price>,
    chainlink_prices: StoreGetBigDecimal,
    clock: Clock,
    output_store: StoreSetProto<PoolRewards>,
) {
    for event in gauge_events.liquidity_events {
        // String representations of the BigInt values for native emissions
        let mut reward_token_emissions_native: Vec<String> = Vec::new();
        // String representations of the BigDecimal values for USD emissions
        let mut reward_token_emissions_usd: Vec<String> = Vec::new();

        if let Some(gauge) = gauge_store.get_last(StoreKey::liquidity_gauge_key(&event.gauge)) {
            // Handle CRV Rewards - if it has been added to the `GaugeController`, it is eligible for CRV rewards
            if let Some(_) =
                gauge_controller_store.get_last(StoreKey::controller_gauge_added_key(&gauge.gauge))
            {
                if let Some(crv_inflation) =
                    crv_inflation_store.get_last(StoreKey::crv_inflation_rate_key())
                {
                    let gauge_rel_weight =
                        rpc::gauge::get_gauge_relative_weight(&gauge.address_vec())
                            .to_decimal(default_decimals());

                    let crv_emissions_native = (BigDecimal::from_str(&crv_inflation)
                        .unwrap_or_else(|_| BigDecimal::zero())
                        * gauge_rel_weight
                        * SECONDS_PER_DAY)
                        .to_bigint();
                    reward_token_emissions_native.push(crv_emissions_native.to_string());

                    let price_usd = prices::get_token_usd_price(
                        &curve_token(),
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                    let crv_emissions_usd =
                        crv_emissions_native.to_decimal(default_decimals()) * price_usd;
                    reward_token_emissions_usd.push(crv_emissions_usd.to_string());
                }
            }

            // Handle Permissionless Rewards
            if let Some(count) = reward_token_count_store.get_last(
                StoreKey::liquidity_gauge_reward_token_count_key(&gauge.gauge),
            ) {
                for index in 0..count {
                    if let Some(reward_token) = reward_tokens_store.get_last(
                        StoreKey::liquidity_gauge_reward_token_key(&gauge.gauge, &(index + 1)),
                    ) {
                        match rpc::gauge::get_reward_token_data(
                            &gauge.address_vec(),
                            &reward_token.address_vec(),
                        ) {
                            Some(reward_data) => {
                                if reward_data.period_finish.to_u64() as i64
                                    > clock.clone().timestamp.unwrap().seconds
                                {
                                    // Calculate native token emissions
                                    let token_emissions_native = reward_data.rate * SECONDS_PER_DAY;
                                    reward_token_emissions_native
                                        .push(token_emissions_native.to_string());

                                    let price_usd = prices::get_token_usd_price(
                                        &reward_token,
                                        &uniswap_prices,
                                        &chainlink_prices,
                                    );

                                    let token_emissions_usd = token_emissions_native
                                        .to_decimal(default_decimals())
                                        * price_usd;

                                    // Calculate USD emissions
                                    reward_token_emissions_usd
                                        .push(token_emissions_usd.to_string());
                                } else {
                                    reward_token_emissions_native.push(BigInt::zero().to_string());
                                    reward_token_emissions_usd.push(BigDecimal::zero().to_string());
                                }
                            }
                            None => {
                                reward_token_emissions_native.push(BigInt::zero().to_string());
                                reward_token_emissions_usd.push(BigDecimal::zero().to_string());
                            }
                        }
                    }
                }
            }
        }

        if reward_token_emissions_native.len() > 0 || reward_token_emissions_native.len() > 0 {
            output_store.set(
                event.log_ordinal,
                StoreKey::pool_rewards_key(&event.pool),
                &PoolRewards {
                    staked_output_token_amount: event.working_supply,
                    reward_token_emissions_native,
                    reward_token_emissions_usd,
                },
            )
        }
    }
}
