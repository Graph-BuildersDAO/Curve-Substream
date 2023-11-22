use substreams::Hex;

use crate::network_config;

pub fn format_address(address: &Vec<u8>) -> String {
    format!("0x{}", Hex::encode(address))
}

pub fn is_base_pool_lp_token(lp_token_address: &Vec<u8>) -> bool {
    network_config::BASE_POOLS_LP_TOKEN
        .iter()
        .any(|&token_address| token_address.as_ref() == lp_token_address.as_slice())
}
