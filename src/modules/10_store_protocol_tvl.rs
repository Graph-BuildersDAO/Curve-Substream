use std::ops::Sub;

use substreams::store::{DeltaBigDecimal, Deltas, StoreAdd, StoreAddBigDecimal, StoreNew};

use crate::key_management::store_key_manager::StoreKey;

#[substreams::handlers::store]
pub fn store_protocol_tvl(
    pool_tvl_deltas: Deltas<DeltaBigDecimal>,
    output_store: StoreAddBigDecimal,
) {
    for delta in pool_tvl_deltas.iter() {
        let tvl_diff = delta.new_value.clone().sub(delta.old_value.clone());
        output_store.add(delta.ordinal, StoreKey::protocol_tvl_key(), tvl_diff)
    }
}
