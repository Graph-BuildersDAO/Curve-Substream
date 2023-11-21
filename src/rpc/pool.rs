use anyhow::{anyhow, Ok};
use substreams::{errors::Error, Hex};
use substreams_ethereum::NULL_ADDRESS;

use crate::{abi::pool::functions, constants::MISSING_OLD_POOLS, utils::format_address};

pub fn get_lp_token_address_from_pool(pool_address: Vec<u8>) -> Result<Vec<u8>, Error> {
    // If the pool is in the missing old pools list, return the lp token address from there.
    if let Some(pool_config) = MISSING_OLD_POOLS.get(format_address(&pool_address).as_str()) {
        return Ok(pool_config.lp_token.to_vec());
    }

    let mut address_option = functions::LpToken {}.call(pool_address.clone());

    if let None = address_option {
        address_option = functions::Token {}.call(pool_address.clone());
    }
    let address = address_option.ok_or_else(|| {
        anyhow!(
            "Unable to get lp token from pool contract {:?} ",
            Hex::encode(&pool_address)
        )
    })?;
    if address == NULL_ADDRESS {
        return Err(anyhow!(
            "Null address returned getting lp token from pool contract {}",
            Hex::encode(&pool_address)
        ));
    }
    Ok(address)
}
