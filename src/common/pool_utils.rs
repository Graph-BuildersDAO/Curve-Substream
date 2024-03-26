use std::ops::{Div, Mul};

use substreams::{
    scalar::{BigDecimal, BigInt},
    store::{StoreGet, StoreGetBigDecimal, StoreGetBigInt},
};

use crate::{
    key_management::store_key_manager::StoreKey,
    pb::curve::types::v1::{
        events::pool_event::{SwapUnderlyingMetaEvent, TokenSource},
        pool::PoolType,
        Pool, Token,
    },
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

pub fn is_metapool(pool: &Pool) -> bool {
    matches!(pool.pool_type, Some(PoolType::MetaPool(_)))
}

pub fn is_lending_pool(pool: &Pool) -> bool {
    matches!(pool.pool_type, Some(PoolType::LendingPool(_)))
}

// Checks whether a TokenExchangeUnderlying event is a Metapool Asset -> Base Pool Asset exchange
pub fn is_meta_to_base_exchange(swap_underlying: &SwapUnderlyingMetaEvent) -> bool {
    swap_underlying.token_in_ref().source() == TokenSource::MetaPool
        && swap_underlying.token_out_ref().source() == TokenSource::BasePool
}

// Checks whether a TokenExchangeUnderlying event is a Base Pool Asset -> Metapool Asset exchange
pub fn is_base_to_meta_exchange(swap_underlying: &SwapUnderlyingMetaEvent) -> bool {
    swap_underlying.token_in_ref().source() == TokenSource::BasePool
        && swap_underlying.token_out_ref().source() == TokenSource::MetaPool
}
