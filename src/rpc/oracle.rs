use substreams::scalar::BigDecimal;

use crate::{
    abi::oracle::{sushiswap, yearn_lens},
    constants::default_usdc_decimals,
    network_config::{SUSHISWAP, SUSHI_BLACKLIST, YEARN_LENS, YEARN_LENS_BLACKLIST},
};

// TODO: Implement oracle type logic
//       Need to figure out when this should be saved to a token.
//       In the subgraph its does it each time a getOrCreateToken is called.
//       If we list out all the times its called this would help decide when we
//       should update it in the substream.
pub fn get_usd_price_from_yearn(token_address: Vec<u8>) -> Option<BigDecimal> {
    if YEARN_LENS_BLACKLIST
        .iter()
        .any(|&addr| addr.as_ref() == token_address.as_slice())
    {
        return None;
    }

    let price_opt =
        yearn_lens::functions::GetPriceUsdcRecommended { token_address }.call(YEARN_LENS.to_vec());

    if let Some(price) = price_opt {
        return Some(price.to_decimal(default_usdc_decimals()));
    }
    None
}

pub fn get_usd_price_from_sushi(token_address: Vec<u8>) -> Option<BigDecimal> {
    if SUSHI_BLACKLIST
        .iter()
        .any(|&addr| addr.as_ref() == token_address.as_slice())
    {
        return None;
    }
    let price_opt = sushiswap::functions::GetPriceUsdc { token_address }.call(SUSHISWAP.to_vec());

    if let Some(price) = price_opt {
        return Some(price.to_decimal(default_usdc_decimals()));
    }
    None
}
