use substreams::store::{StoreNew, StoreSetIfNotExists, StoreSetIfNotExistsInt64};

use crate::{key_management::store_key_manager::StoreKey, pb::curve::types::v1::CurveEvents};

// This module, is responsible for marking liquidity gauges that have been added to the GaugeController
// contract. By setting a value in the store for each gauge added, it effectively acts as a boolean
// indicator of whether a gauge is eligible for CRV rewards.
#[substreams::handlers::store]
pub fn store_controller_gauges(events: CurveEvents, store: StoreSetIfNotExistsInt64) {
    for new_gauge in events.controller_gauges {
        store.set_if_not_exists(
            new_gauge.log_ordinal,
            StoreKey::controller_gauge_added_key(&new_gauge.gauge),
            &1,
        )
    }
}
