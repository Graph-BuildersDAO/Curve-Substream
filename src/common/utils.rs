use crate::network_config::{self, PROTOCOL_ADDRESS};

use super::format::format_address_vec;

pub fn get_protocol_id() -> String {
    format_address_vec(&PROTOCOL_ADDRESS.to_vec())
}

pub fn is_base_pool_lp_token(lp_token_address: &Vec<u8>) -> bool {
    network_config::BASE_POOLS_LP_TOKEN
        .iter()
        .any(|&token_address| token_address.as_ref() == lp_token_address.as_slice())
}
