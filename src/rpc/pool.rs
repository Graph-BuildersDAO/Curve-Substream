use anyhow::{anyhow, Error};
use substreams::{scalar::BigInt, Hex};
use substreams_ethereum::NULL_ADDRESS;

use crate::{
    abi::pool::functions, constants::MISSING_OLD_POOLS, pb::curve::types::v1::Token,
    utils::format_address,
};

use super::token::create_token;

pub fn get_lp_token_address_from_pool(pool_address: &Vec<u8>) -> Result<Vec<u8>, Error> {
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

pub fn get_pool_coins(pool_address: &Vec<u8>) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut idx = 0;

    while idx >= 0 {
        let input_token_option = functions::Coins1 {
            i: BigInt::from(idx),
        }
        .call(pool_address.clone());

        let input_token = match input_token_option {
            Some(token) => token,
            None => functions::Coins2 {
                arg0: BigInt::from(idx),
            }
            .call(pool_address.clone())
            .unwrap_or_else(|| NULL_ADDRESS.to_vec()),
        };

        if input_token == NULL_ADDRESS.to_vec() {
            break;
        }

        match create_token(&input_token, &pool_address) {
            Ok(token) => {
                tokens.push(token);
            }
            Err(e) => {
                return Err(anyhow!("Error in `get_pool_coins`: {:?}", e));
            }
        }
        idx += 1;
    }
    Ok(tokens)
}
