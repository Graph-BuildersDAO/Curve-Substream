use substreams::store::{StoreAdd, StoreAddInt64, StoreNew};

use crate::{key_management::store_key_manager::StoreKey, pb::curve::types::v1::CurveEvents};

#[substreams::handlers::store]
pub fn store_tokens(events: CurveEvents, store: StoreAddInt64) {
    for pool in events.pools {
        let addr_output_token = pool.output_token.unwrap().address;
        let addr_input_tokens: Vec<String> = pool
            .input_tokens
            .iter()
            .map(|t| t.address.clone())
            .collect();

        let mut keys: Vec<String> = Vec::new();
        keys.push(StoreKey::token_key(&addr_output_token));
        for addr in addr_input_tokens {
            keys.push(StoreKey::token_key(&addr));
        }

        store.add_many(pool.log_ordinal, &keys, 1);
    }
}
