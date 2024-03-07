use substreams::store::{StoreAdd, StoreAddInt64, StoreNew};

use crate::{
    key_management::store_key_manager::StoreKey, pb::curve::types::v1::LiquidityGaugeEvents,
};

#[substreams::handlers::store]
pub fn store_reward_token_count(events: LiquidityGaugeEvents, store: StoreAddInt64) {
    for event in events.add_reward_events {
        store.add(
            0,
            StoreKey::liquidity_gauge_reward_token_count_key(&event.gauge),
            1,
        )
    }
}
