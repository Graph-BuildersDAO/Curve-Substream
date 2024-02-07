use std::str::FromStr;

use substreams::{
    scalar::BigDecimal,
    store::{StoreGet, StoreGetBigDecimal, StoreGetProto},
    Hex,
};

use crate::{
    constants::{one_usd_value, BLACKLISTED_TOKENS},
    key_management::store_key_manager::StoreKey,
    network_config::HARDCODED_STABLES,
    pb::{curve::types::v1::Token, uniswap_pricing::v1::Erc20Price},
    rpc::oracle::{get_usd_price_from_sushi, get_usd_price_from_yearn},
};

pub fn get_token_usd_price(
    token: &Token,
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
        return one_usd_value();
    }

    get_usd_price_from_chainlink(token, chainlink_prices)
        .or_else(|| get_usd_price_from_uniswap(token, uniswap_prices))
        .or_else(|| get_usd_price_from_yearn(token.address_vec()))
        .or_else(|| get_usd_price_from_sushi(token.address_vec()))
        // TODO: Currently we have only implemented two of the RPC price gets from the original subgraph.
        //       We need to implement all the remaining pricing RPC calls eventually.
        //       These act as a fallback in case we cannot get the price from the imported substream packages.
        .unwrap_or_else(|| {
            substreams::log::debug!("Failed to get price for token: {}", token.address);
            BigDecimal::zero()
        })
}

fn get_usd_price_from_chainlink(
    token: &Token,
    chainlink_prices: &StoreGetBigDecimal,
) -> Option<BigDecimal> {
    chainlink_prices.get_last(StoreKey::chainlink_price_by_symbol_key(&token.symbol))
}

fn get_usd_price_from_uniswap(
    token: &Token,
    uniswap_prices: &StoreGetProto<Erc20Price>,
) -> Option<BigDecimal> {
    uniswap_prices
        .get_last(StoreKey::uniswap_price_by_token_address_key(&token.address))
        .or_else(|| {
            uniswap_prices.get_last(StoreKey::uniswap_price_by_token_symbol_key(&token.symbol))
        })
        .and_then(|price| BigDecimal::from_str(&price.price_usd).ok())
}
