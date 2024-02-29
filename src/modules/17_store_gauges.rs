use substreams::store::{StoreNew, StoreSet, StoreSetProto};

use crate::{
    key_management::store_key_manager::StoreKey,
    pb::curve::types::v1::{LiquidityGauge, CurveEvents},
};

// TODO: These gauge/inflation related modules should be reordered based on the module data flow.
#[substreams::handlers::store]
pub fn store_gauges(events: CurveEvents, store: StoreSetProto<LiquidityGauge>) {
    for gauge in events.gauges {
        store.set(
            gauge.log_ordinal,
            StoreKey::liquidity_gauge_key(&gauge.gauge),
            &gauge,
        )
    }
}
