use std::ops::{Div, Mul};

use substreams::{
    scalar::{BigDecimal, BigInt},
    store::{StoreGet, StoreGetBigDecimal, StoreGetBigInt},
};

use crate::{
    key_management::store_key_manager::StoreKey,
    pb::curve::types::v1::{Pool, Token},
};

pub fn get_input_token_balances(
    pool_address: &str,
    input_tokens: &Vec<Token>,
    input_token_balances_store: &StoreGetBigInt,
) -> Vec<BigInt> {
    input_tokens
        .iter()
        .map(|token| {
            let input_token_balance_key =
                StoreKey::input_token_balance_key(&pool_address, &token.address);
            input_token_balances_store
                .get_last(input_token_balance_key)
                .unwrap_or_else(|| {
                    substreams::log::debug!(
                        "No input token balance found for pool {} and token {}",
                        pool_address,
                        token.address
                    );
                    BigInt::zero()
                })
        })
        .collect()
}

pub fn get_input_token_weights(
    pool: &Pool,
    pool_tvl_store: &StoreGetBigDecimal,
) -> Vec<BigDecimal> {
    if let Some(pool_tvl) = pool_tvl_store.get_last(StoreKey::pool_tvl_key(&pool.address)) {
        if pool_tvl == BigDecimal::zero() {
            vec![BigDecimal::zero(); pool.input_tokens.len()]
        } else {
            pool.input_tokens
                .iter()
                .map(|token| {
                    pool_tvl_store
                        .get_last(StoreKey::pool_token_tvl_key(&pool.address, &token.address))
                        .map_or(BigDecimal::zero(), |token_tvl| {
                            token_tvl.div(&pool_tvl).mul(BigDecimal::from(100))
                        })
                })
                .collect()
        }
    } else {
        vec![BigDecimal::zero(); pool.input_tokens.len()]
    }
}
