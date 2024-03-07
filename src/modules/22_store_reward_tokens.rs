use substreams::store::{
    DeltaInt64, Deltas, StoreGet, StoreGetProto, StoreNew, StoreSet, StoreSetProto,
};

use crate::{
    key_management::store_key_manager::StoreKey,
    pb::curve::types::v1::{LiquidityGauge, LiquidityGaugeEvents, Pool, Token},
    rpc,
};

#[substreams::handlers::store]
pub fn store_reward_tokens(
    events: LiquidityGaugeEvents,
    gauges_store: StoreGetProto<LiquidityGauge>,
    reward_token_count_deltas: Deltas<DeltaInt64>,
    pools_store: StoreGetProto<Pool>,
    output_store: StoreSetProto<Token>,
) {
    for event in events.add_reward_events {
        if let Some(gauge) = gauges_store.get_last(StoreKey::liquidity_gauge_key(&event.gauge)) {
            if let Some(pool) = pools_store.get_last(StoreKey::pool_key(&gauge.pool)) {
                if let Some(delta) = reward_token_count_deltas.deltas.iter().find(|d| {
                    d.key == StoreKey::liquidity_gauge_reward_token_count_key(&gauge.gauge)
                }) {
                    let reward_token_count = delta.new_value;
                    let key = StoreKey::liquidity_gauge_reward_token_key(
                        &gauge.gauge,
                        &reward_token_count,
                    );
                    // Check if the reward token data exists in the stored pool to avoid redundant RPC calls
                    if let Some(reward_token) = pool
                        .input_tokens
                        .iter()
                        .find(|t| t.address == event.reward_token)
                    {
                        output_store.set(
                            0,
                            key,
                            &Token {
                                address: reward_token.address.to_string(),
                                name: reward_token.name.to_string(),
                                symbol: reward_token.symbol.to_string(),
                                decimals: reward_token.decimals,
                                total_supply: reward_token.total_supply.to_string(),
                                is_base_pool_lp_token: reward_token.is_base_pool_lp_token,
                                gauge: Some(gauge.gauge.to_string()),
                            },
                        )
                    } else {
                        match rpc::token::create_token(
                            &event.reward_token_vec(),
                            &pool.address_vec(),
                            Some(&gauge.gauge),
                        ) {
                            Ok(token) => output_store.set(0, key, &token),
                            Err(e) => {
                                substreams::log::debug!("Error in `store_reward_tokens`: {:?}", e);
                            }
                        };
                    }
                }
            }
        }
    }
}
