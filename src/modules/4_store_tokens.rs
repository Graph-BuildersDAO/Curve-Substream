use substreams::store::{StoreAdd, StoreAddInt64, StoreNew};

use crate::pb::curve::types::v1::Pools;

#[substreams::handlers::store]
pub fn store_tokens(pools: Pools, store: StoreAddInt64) {
    for pool in pools.pools {
        let addr_output_token = pool.output_token.unwrap().address;
        let addr_input_tokens: Vec<String> = pool
            .input_tokens
            .iter()
            .map(|t| t.address.clone())
            .collect();

        let mut keys: Vec<String> = Vec::new();
        keys.push(format!("token:{addr_output_token}"));
        for addr in addr_input_tokens {
            keys.push(format!("token:{addr}"));
        }

        store.add_many(pool.log_ordinal, &keys, 1);
    }
}
