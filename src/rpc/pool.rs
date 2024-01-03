use anyhow::anyhow;
use substreams::{
    errors::Error,
    scalar::{BigDecimal, BigInt},
    Hex,
};
use substreams_ethereum::{rpc::RpcBatch, NULL_ADDRESS};

use crate::{
    abi::pool::functions,
    common::{
        conversion::{convert_bigint_to_decimal, convert_enum_to_snake_case_prefix},
        format::format_address_vec,
    },
    constants::{self, LiquidityPoolFeeType, FEE_DENOMINATOR, MISSING_OLD_POOLS},
    network_config::POOL_REGISTRIES,
    pb::curve::types::v1::Token,
    types::{PoolFee, PoolFees},
};

use super::{
    common::decode_rpc_response, registry::get_pool_underlying_coins_from_registry,
    token::create_token,
};

pub fn get_lp_token_address_from_pool(pool_address: &Vec<u8>) -> Result<Vec<u8>, Error> {
    // If the pool is in the missing old pools list, return the lp token address from there.
    if let Some(pool_config) = MISSING_OLD_POOLS.get(format_address_vec(&pool_address).as_str()) {
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

pub fn get_pool_underlying_coins(pool_address: &Vec<u8>) -> Result<[Vec<u8>; 8], Error> {
    let mut errors: Vec<Error> = Vec::new();
    for registry_address in POOL_REGISTRIES.iter().map(|&a| a.to_vec()) {
        match get_pool_underlying_coins_from_registry(&pool_address, &registry_address) {
            Ok(coins) => {
                if coins.len() != 0 && coins[0] != NULL_ADDRESS.to_vec() {
                    return Ok(coins);
                }
            }
            Err(e) => {
                errors.push(e);
            }
        }
    }
    // If we cannot get the coins from the registry, attempt to get them from the actual pool contract.
    let mut coins_array: [Vec<u8>; 8] = Default::default(); // Initialize with default values
    let mut index = 0;
    loop {
        let index_big_int = BigInt::from(index);
        let coins_option = functions::UnderlyingCoins {
            arg0: index_big_int,
        }
        .call(pool_address.clone());

        match coins_option {
            Some(coin) => coins_array[index] = coin.clone(),
            None => break,
        }
        index += 1;
    }
    if coins_array.iter().any(|coin| !coin.is_empty()) {
        return Ok(coins_array);
    }

    if errors.is_empty() {
        Err(anyhow!(
            "Unable to get underlying coins for pool {:?} from registry contracts",
            Hex::encode(&pool_address)
        ))
    } else {
        Err(anyhow!(
            "Unable to get underlying coins for pool {:?} from registry contracts. Errors: {:?}",
            Hex::encode(&pool_address),
            errors
        ))
    }
}

pub fn get_pool_fees(pool_address: &Vec<u8>) -> Result<PoolFees, Error> {
    let batch: RpcBatch = RpcBatch::new();
    let responses = batch
        .add(functions::Fee {}, pool_address.clone())
        .add(functions::AdminFee {}, pool_address.clone())
        .execute()
        .map_err(|e| anyhow!("Error in `get_pool_fees`: {:?}", e))?
        .responses;

    let total_fee = decode_rpc_response::<_, functions::Fee>(
        &responses[0],
        &format!(
            "{} is not a pool contract fee `eth_call` failed",
            Hex::encode(&pool_address)
        ),
    )
    .unwrap_or_else(|| constants::default_pool_fee());
    let total_fee = convert_bigint_to_decimal(&total_fee, FEE_DENOMINATOR)?;

    let admin_fee = decode_rpc_response::<_, functions::AdminFee>(
        &responses[1],
        &format!(
            "{} is not a pool contract admin fee `eth_call` failed",
            Hex::encode(&pool_address)
        ),
    )
    .unwrap_or_else(|| constants::default_admin_fee());

    let admin_fee = convert_bigint_to_decimal(&admin_fee, FEE_DENOMINATOR)?;

    let trading_fee_id =
        convert_enum_to_snake_case_prefix(LiquidityPoolFeeType::FixedTradingFee.as_str())
            + format_address_vec(&pool_address).as_str();
    // Calculate the trading fee. This is the total fee charged on a trade, expressed as a percentage.
    // The fee is multiplied by 100 to convert it from a decimal to a percentage format.
    let trading_fee = PoolFee::new(
        trading_fee_id,
        LiquidityPoolFeeType::FixedTradingFee,
        total_fee.clone() * BigDecimal::from(100),
    );

    let protocol_fee_id =
        convert_enum_to_snake_case_prefix(LiquidityPoolFeeType::FixedProtocolFee.as_str())
            + format_address_vec(&pool_address).as_str();
    // Calculate the protocol fee. This is a portion of the trading fees allocated to the protocol.
    // It is calculated as the product of the total fee and the admin fee, then converted to a percentage.
    let protocol_fee = PoolFee::new(
        protocol_fee_id,
        LiquidityPoolFeeType::FixedProtocolFee,
        total_fee.clone() * admin_fee.clone() * BigDecimal::from(100),
    );

    let lp_fee_id = convert_enum_to_snake_case_prefix(LiquidityPoolFeeType::FixedLpFee.as_str())
        + format_address_vec(&pool_address).as_str();
    // Calculate the LP fee. This is the fee allocated to liquidity providers.
    // It is the remaining fee after deducting the protocol's admin fee from the total fee,
    // then converted to a percentage.
    let lp_fee = PoolFee::new(
        lp_fee_id,
        LiquidityPoolFeeType::FixedLpFee,
        (total_fee.clone() - (total_fee * admin_fee)) * BigDecimal::from(100),
    );

    Ok(PoolFees::new(trading_fee, protocol_fee, lp_fee))
}
