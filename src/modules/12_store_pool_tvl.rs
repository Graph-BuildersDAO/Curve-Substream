use std::ops::Add;

use substreams::{
    key,
    scalar::BigDecimal,
    store::{
        DeltaBigInt, Deltas, StoreGet, StoreGetBigDecimal, StoreGetBigInt, StoreGetProto, StoreNew,
        StoreSet, StoreSetBigDecimal,
    },
};

use crate::{
    common::prices::get_token_usd_price,
    key_management::store_key_manager::StoreKey,
    pb::{curve::types::v1::Pool, uniswap_pricing::v1::Erc20Price},
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
    for delta in balances_deltas.deltas {
        let pool_address = key::segment_at(&delta.key, 1);
        let token_address = key::segment_at(&delta.key, 2);

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
                // TODO: We may be able to optimise here by getting the token price upstream when the balances change.
                //       Check if we get the price in any other modules that use the balances store, and if so, we can
                //       minimise the amount of store calls by getting it once and setting it alongside the balance changes.
                let price_usd = get_token_usd_price(&token, &uniswap_prices, &chainlink_prices);
                let token_tvl = balance.to_decimal(token.decimals) * price_usd;

                // Store Input Token TVL for a specific Pool
                output_store.set(
                    delta.ordinal,
                    StoreKey::pool_token_tvl_key(&pool_address, &token.address),
                    &token_tvl,
                );
                tvl = tvl.add(token_tvl);
            }
        }
        // Store Pool total TVL
        output_store.set(delta.ordinal, StoreKey::pool_tvl_key(&pool_address), &tvl);
    }
}
