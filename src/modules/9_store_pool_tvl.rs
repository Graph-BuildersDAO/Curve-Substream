use std::{ops::Add, str::FromStr};

use substreams::{
    scalar::{BigDecimal, BigInt},
    store::{
        DeltaBigInt, Deltas, StoreGet, StoreGetBigDecimal, StoreGetBigInt, StoreGetProto, StoreNew,
        StoreSet, StoreSetBigDecimal,
    },
    Hex,
};

use crate::{
    constants::{one_usd_value, BLACKLISTED_TOKENS},
    key_management::store_key_manager::StoreKey,
    network_config::HARDCODED_STABLES,
    pb::{
        curve::types::v1::{Pool, Token},
        uniswap_pricing::v1::Erc20Price,
    },
    rpc::oracle::{get_usdc_price_price_sushi, get_usdc_price_yearn},
};

#[substreams::handlers::store]
pub fn store_pool_tvl(
    pools_store: StoreGetProto<Pool>,
    balances_store: StoreGetBigInt,
    balances_deltas: Deltas<DeltaBigInt>,
    chainlink_prices: StoreGetBigDecimal,
    uniswap_prices: StoreGetProto<Erc20Price>,
    output_store: StoreSetBigDecimal,
) {
    // loop through balances deltas
    for delta in balances_deltas.deltas {
        if let Some((pool_address, Some(token_address))) =
            StoreKey::extract_parts_from_key(&delta.key)
        {
            // Get prices of tokens from price stores
            let pool = pools_store.must_get_last(StoreKey::pool_key(&pool_address));

            let mut tvl = BigDecimal::zero();

            for token in pool.input_tokens {
                let store_balance = if token.address == token_address {
                    // We know that a delta matches this token address,
                    // so get the specific store value using ordinal.
                    balances_store.get_at(
                        delta.ordinal,
                        StoreKey::input_token_balance_key(&pool_address, &token.address),
                    )
                } else {
                    // No delta matches, so just get the last balance from the store.
                    balances_store.get_last(StoreKey::input_token_balance_key(
                        &pool_address,
                        &token.address,
                    ))
                };

                if let Some(balance) = store_balance {
                    tvl = tvl.add(calculate_token_tvl(
                        &token,
                        balance,
                        &uniswap_prices,
                        &chainlink_prices,
                    ));
                }
            }
            output_store.set(delta.ordinal, StoreKey::pool_tvl_key(&pool_address), &tvl);
        }
    }
}

fn calculate_token_tvl(
    token: &Token,
    balance: BigInt,
    uniswap_prices: &StoreGetProto<Erc20Price>,
    chainlink_prices: &StoreGetBigDecimal,
) -> BigDecimal {
    if BLACKLISTED_TOKENS.contains(&token.address.as_str()) {
        return BigDecimal::zero();
    }

    if HARDCODED_STABLES
        .iter()
        .any(|&addr| Hex::encode(addr) == token.address)
    {
        return one_usd_value() * balance.to_decimal(token.decimals);
    }

    // Attempt to get a price from the Uniswap Prices store
    if let Some(price) = uniswap_prices
        .get_last(StoreKey::uniswap_price_by_token_address_key(&token.address))
        .or_else(|| {
            uniswap_prices.get_last(StoreKey::uniswap_price_by_token_symbol_key(&token.symbol))
        })
    {
        BigDecimal::from_str(&price.price_usd).unwrap_or_else(|_| BigDecimal::zero())
            * balance.to_decimal(token.decimals)
    }
    // Attempt to get a price from the Chainlink Prices store
    else if let Some(price) =
        chainlink_prices.get_last(StoreKey::chainlink_price_by_symbol_key(&token.symbol))
    {
        price * balance.to_decimal(token.decimals)
    }
    // Fallback to fetching price using RPC calls if we cannot get via a store
    else if let Some(price) = get_usdc_price_yearn(token.address_vec()) {
        price * balance.to_decimal(token.decimals)
    } else if let Some(price) = get_usdc_price_price_sushi(token.address_vec()) {
        price * balance.to_decimal(token.decimals)
    } else {
        BigDecimal::zero()
    }
}
