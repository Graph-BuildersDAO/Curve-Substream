use anyhow::anyhow;
use substreams::{errors::Error, log, scalar::BigInt, Hex};
use substreams_ethereum::rpc::RpcBatch;

use crate::{
    abi::common::erc20::functions,
    common::utils,
    constants,
    pb::curve::types::v1::Token,
    rpc::{common::decode_rpc_response, registry::is_main_registry_pool},
};

pub fn create_token(token_address: &Vec<u8>, pool_address: &Vec<u8>) -> Result<Token, Error> {
    if token_address == constants::ETH_ADDRESS.as_ref() {
        let total_supply = match get_token_supply(&token_address) {
            Ok(total_supply) => total_supply,
            Err(e) => {
                log::debug!("Error in `create_token`: {:?}", e);
                BigInt::from(0)
            }
        };
        return Ok(Token {
            address: Hex::encode(&token_address),
            name: String::from("ETH"),
            symbol: String::from("ETH"),
            decimals: constants::default_decimals().to_u64(),
            total_supply: total_supply.to_string(),
            is_base_pool_lp_token: false,
        });
    }

    let batch = RpcBatch::new();
    let responses = batch
        .add(functions::Decimals {}, token_address.clone())
        .add(functions::Name {}, token_address.clone())
        .add(functions::Symbol {}, token_address.clone())
        .add(functions::TotalSupply {}, token_address.clone())
        .execute()
        .unwrap()
        .responses;

    let decimals = decode_rpc_response::<_, functions::Decimals>(
        &responses[0],
        &format!(
            "{} is not an ERC20 token contract decimal `eth_call` failed",
            Hex::encode(&token_address)
        ),
    )
    .unwrap_or_else(|| constants::default_decimals());

    let name = decode_rpc_response::<_, functions::Name>(
        &responses[1],
        &format!(
            "{} is not an ERC20 token contract name `eth_call` failed",
            Hex::encode(&token_address)
        ),
    )
    .unwrap_or_else(|| read_string_from_bytes(responses[1].raw.as_ref()));

    let symbol = decode_rpc_response::<_, functions::Symbol>(
        &responses[2],
        &format!(
            "{} is not an ERC20 token contract symbol `eth_call` failed",
            Hex::encode(&token_address)
        ),
    )
    .unwrap_or_else(|| read_string_from_bytes(responses[2].raw.as_ref()));

    let total_supply = decode_rpc_response::<_, functions::TotalSupply>(
        &responses[3],
        &format!(
            "{} is not an ERC20 token contract total supply `eth_call` failed",
            Hex::encode(&token_address)
        ),
    )
    .unwrap_or_else(|| BigInt::from(0));

    return Ok(Token {
        address: Hex::encode(token_address),
        name,
        symbol,
        decimals: decimals.to_u64(),
        total_supply: total_supply.to_string(),
        is_base_pool_lp_token: utils::is_base_pool_lp_token(&token_address)
            || is_main_registry_pool(&pool_address),
    });
}

pub fn get_token_minter(token_address: &Vec<u8>) -> Result<Vec<u8>, Error> {
    let minter_option = functions::Minter {}.call(token_address.clone());

    let minter = minter_option.ok_or_else(|| {
        anyhow!(
            "Unable to get minter for token {:?}",
            Hex::encode(&token_address)
        )
    })?;
    Ok(minter)
}

pub fn get_token_supply(token_address: &Vec<u8>) -> Result<BigInt, Error> {
    functions::TotalSupply {}
        .call(token_address.to_owned())
        .ok_or_else(|| {
            anyhow!(
                "Unable to get total supply for token {:?}",
                Hex::encode(&token_address)
            )
        })
}

fn read_string_from_bytes(input: &[u8]) -> String {
    // we have to check if we have a valid utf8 representation and if we do
    // we return the value if not we return a DecodeError
    if let Some(last) = input.to_vec().iter().rev().position(|&pos| pos != 0) {
        return String::from_utf8_lossy(&input[0..input.len() - last]).to_string();
    }

    // use case when all the bytes are set to 0
    "".to_string()
}
