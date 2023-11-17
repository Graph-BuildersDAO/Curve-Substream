use anyhow::{anyhow, Ok};
use substreams::{errors::Error, Hex};
use substreams_ethereum::NULL_ADDRESS;

use crate::abi::pool::functions;

pub fn get_lp_token_address_from_pool(pool_address: Vec<u8>) -> Result<Vec<u8>, Error> {
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
