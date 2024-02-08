use std::ops::Add;

use substreams::{
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
        if let Some((pool_address, Some(token_address), _)) =
            StoreKey::extract_parts_from_key(&delta.key)
        {
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
}
