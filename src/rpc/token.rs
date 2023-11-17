use substreams::{log, scalar::BigInt, Hex};
use substreams_ethereum::rpc::RpcBatch;

use crate::{
    abi::erc20::functions, constants, pb::curve::types::v1::Token,
    rpc::common::decode_rpc_response,
};

pub fn create_token(token_address: Vec<u8>) -> Option<Token> {
    if token_address == constants::ETH_ADDRESS {
        return Some(Token {
            address: Hex::encode(&token_address),
            name: String::from("ETH"),
            symbol: String::from("ETH"),
            decimals: constants::default_decimals().to_u64(),
            total_supply: get_token_supply(token_address).unwrap().to_string(),
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

    let decimals = match decode_rpc_response::<_, functions::Decimals>(
        &responses[0],
        "{} is not a an ERC20 token contract decimal `eth_call` failed",
    ) {
        Some(decoded_decimals) => decoded_decimals.to_u64(),
        None => {
            log::debug!(
                "Failed to decode decimals for token {}",
                Hex::encode(&token_address)
            );
            return None;
        }
    };
    log::debug!("decoded_decimals ok");

    let name = decode_rpc_response::<_, functions::Name>(
        &responses[1],
        &format!(
            "{} is not an ERC20 token contract name `eth_call` failed",
            Hex::encode(&token_address)
        ),
    )
    .unwrap_or_else(|| read_string_from_bytes(responses[1].raw.as_ref()));
    log::debug!("decoded name ok");

    let symbol = decode_rpc_response::<_, functions::Symbol>(
        &responses[2],
        &format!(
            "{} is not an ERC20 token contract symbol `eth_call` failed",
            Hex::encode(&token_address)
        ),
    )
    .unwrap_or_else(|| read_string_from_bytes(responses[2].raw.as_ref()));
    log::debug!("decoded symbol ok");

    let total_supply = decode_rpc_response::<_, functions::TotalSupply>(
        &responses[3],
        &format!(
            "{} is not an ERC20 token contract total supply `eth_call` failed",
            Hex::encode(&token_address)
        ),
    )
    .unwrap_or_else(|| BigInt::from(0));
    log::debug!("decoded supply ok");

    return Some(Token {
        address: Hex::encode(token_address),
        name,
        symbol,
        decimals,
        total_supply: total_supply.to_string(),
    });
}

fn get_token_supply(token_address: Vec<u8>) -> Option<BigInt> {
    let supply = functions::TotalSupply {}
        .call(token_address)
        .unwrap_or(BigInt::from(0));
    log::debug!("token supply: {}", supply);
    Some(supply)
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
