use substreams::store::{StoreNew, StoreSet, StoreSetString};

use crate::{key_management::store_key_manager::StoreKey, pb::curve::types::v1::CurveEvents};

// This module, is responsible for marking liquidity gauges that have been added to the GaugeController
// contract. By setting a value in the store for each gauge added, it effectively acts as a boolean
// indicator of whether a gauge is eligible for CRV rewards.
#[substreams::handlers::store]
pub fn store_crv_inflation(events: CurveEvents, store: StoreSetString) {
    if let Some(inflation_event) = events.update_mining_parameters_event {
        store.set(0, StoreKey::crv_inflation_rate_key(), &inflation_event.rate);
    }
}
