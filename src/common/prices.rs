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
    pb::{
        curve::types::v1::{events::pool_event::PriceSource, Token},
        uniswap_pricing::v1::Erc20Price,
    },
    rpc::oracle::{
        get_usd_price_from_curve_calc, get_usd_price_from_sushi, get_usd_price_from_yearn,
    },
};

pub fn get_token_usd_price(
    token: &Token,
    uniswap_prices: &StoreGetProto<Erc20Price>,
    chainlink_prices: &StoreGetBigDecimal,
) -> (BigDecimal, PriceSource) {
    if BLACKLISTED_TOKENS.contains(&token.address.as_str()) {
        return (BigDecimal::zero(), PriceSource::Unknown);
    }

    if HARDCODED_STABLES
        .iter()
        .any(|&addr| Hex::encode(addr) == token.address)
    {
        return (one_usd_value(), PriceSource::Stablecoin);
    }

    let price = get_usd_price_from_chainlink(token, chainlink_prices)
        .map(|price| (price, PriceSource::Chainlink))
        .or_else(|| {
            get_usd_price_from_uniswap(token, uniswap_prices)
                .map(|price| (price, PriceSource::UniswapV2))
        })
        .or_else(|| {
            get_usd_price_from_yearn(token.address_vec()).map(|price| (price, PriceSource::Yearn))
        })
        .or_else(|| {
            get_usd_price_from_sushi(token.address_vec()).map(|price| (price, PriceSource::Sushi))
        })
        .or_else(|| {
            get_usd_price_from_curve_calc(token.address_vec())
                .map(|price| (price, PriceSource::CurveCalc))
        })
        .unwrap_or_else(|| {
            substreams::log::debug!("Failed to get price for token: {}", token.address);
            (BigDecimal::zero(), PriceSource::Unknown)
        });
        
    price
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
        .and_then(|price| {
            BigDecimal::from_str(&price.price_usd).ok()
        })
}
