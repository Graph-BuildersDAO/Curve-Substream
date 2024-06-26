use anyhow::anyhow;
use substreams::{
    errors::Error,
    scalar::{BigDecimal, BigInt},
    Hex,
};
use substreams_ethereum::{rpc::RpcBatch, NULL_ADDRESS};

use crate::{
    abi::curve::{
        pool::functions,
        pools::{lending_pool, metapool_old},
    },
    common::format::format_address_vec,
    constants::{self, FEE_DECIMALS},
    key_management::entity_key_manager::EntityKey,
    pb::curve::types::v1::{LiquidityPoolFeeType, PoolFee, PoolFees, Token},
};

use super::{common::decode_rpc_response, token::create_token};

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

        match create_token(idx.to_string(), &input_token, &pool_address, None) {
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

pub fn get_lending_pool_underlying_coins(pool_address: &Vec<u8>) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut idx = 0;

    while idx >= 0 {
        let underlying_token_option = lending_pool::functions::UnderlyingCoins1 {
            arg0: BigInt::from(idx),
        }
        .call(pool_address.clone());

        let underlying_token = match underlying_token_option {
            Some(token) => token,
            None => match (lending_pool::functions::UnderlyingCoins2 {
                arg0: BigInt::from(idx),
            }
            .call(pool_address.clone()))
            {
                Some(token) => token,
                None => break,
            },
        };

        match create_token(idx.to_string(), &underlying_token, &pool_address, None) {
            Ok(token) => {
                tokens.push(token);
            }
            Err(e) => {
                return Err(anyhow!("Error in `get_pool_underlying_coins`: {:?}", e));
            }
        }
        idx += 1;
    }
    Ok(tokens)
}

pub fn get_old_metapool_base_pool(pool_address: &Vec<u8>) -> Option<Vec<u8>> {
    metapool_old::functions::BasePool {}.call(pool_address.clone())
}

pub fn get_old_metapool_underlying_coins(pool_address: &Vec<u8>) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut idx = 0;

    while idx >= 0 {
        let input_token_option = metapool_old::functions::BaseCoins {
            arg0: BigInt::from(idx),
        }
        .call(pool_address.clone());

        let input_token = match input_token_option {
            Some(token) => token,
            None => {
                break;
            }
        };

        match create_token(idx.to_string(), &input_token, &pool_address, None) {
            Ok(token) => {
                tokens.push(token);
            }
            Err(e) => {
                return Err(anyhow!("Error in `get_old_metapool_base_pool`: {:?}", e));
            }
        }
        idx += 1;
    }
    Ok(tokens)
}

pub fn get_pool_fee_and_admin_fee(pool_address: &Vec<u8>) -> Result<(BigInt, BigInt), Error> {
    let batch = RpcBatch::new();
    let responses = batch
        .add(functions::Fee {}, pool_address.clone())
        .add(functions::AdminFee {}, pool_address.clone())
        .execute()
        .map_err(|e| {
            anyhow!(
                "RPC batch execution error in `get_pool_fee_and_admin_fee` for pool {}: {:?}",
                Hex::encode(pool_address),
                e
            )
        })?
        .responses;

    let total_fee = match decode_rpc_response::<_, functions::Fee>(
        &responses[0],
        &format!(
            "{} is not a pool contract fee `eth_call` failed",
            Hex::encode(&pool_address)
        ),
    ) {
        Some(fee) => fee,
        None => {
            substreams::log::debug!(
                "Failed to decode total fee fee for pool {}",
                Hex::encode(pool_address)
            );
            // If a None value is returned, use the default pool fee.
            constants::default_pool_fee()
        }
    };

    let admin_fee = match decode_rpc_response::<_, functions::AdminFee>(
        &responses[1],
        &format!(
            "{} is not a pool contract admin fee `eth_call` failed",
            Hex::encode(&pool_address)
        ),
    ) {
        Some(fee) => fee,
        None => {
            substreams::log::debug!(
                "Failed to decode admin fee for pool {}",
                Hex::encode(pool_address)
            );
            // If a None value is returned, use the default admin fee.
            constants::default_admin_fee()
        }
    };

    Ok((total_fee, admin_fee))
}

// Computes trading (total), protocol (admin), and LP fees for a given liquidity pool from total and admin fee values.
// - `total_fee`: The raw BigInt fee charged by the pool.
// - `admin_fee`: The portion of the total fee allocated to the protocol.
// - `pool_address`: The address of the liquidity pool.
// Returns a `PoolFees` struct containing detailed fee information.
pub fn calculate_pool_fees(
    total_fee: BigInt,
    admin_fee: BigInt,
    pool_address: &Vec<u8>,
) -> PoolFees {
    // Shadowing as do not need BigInt val anymore.
    // Perform zero checks to avoid div by zero errors.
    let total_fee = if total_fee == BigInt::zero() {
        BigDecimal::zero()
    } else {
        total_fee.to_decimal(FEE_DECIMALS)
    };

    let admin_fee = if admin_fee == BigInt::zero() {
        BigDecimal::zero()
    } else {
        admin_fee.to_decimal(FEE_DECIMALS)
    };

    let trading_fee_id = EntityKey::pool_fee_id(
        &LiquidityPoolFeeType::FixedTradingFee,
        &format_address_vec(&pool_address),
    );
    // Calculate the trading fee. This is the total fee charged on a trade, expressed as a percentage.
    // The fee is multiplied by 100 to convert it from a decimal to a percentage format.
    let trading_fee = PoolFee {
        id: trading_fee_id,
        fee_type: LiquidityPoolFeeType::FixedTradingFee as i32,
        fee_percentage: (total_fee.clone() * BigDecimal::from(100)).to_string(),
    };

    let protocol_fee_id = EntityKey::pool_fee_id(
        &LiquidityPoolFeeType::FixedProtocolFee,
        &format_address_vec(&pool_address),
    );
    // Calculate the protocol fee. This is a portion of the trading fees allocated to the protocol.
    // It is calculated as the product of the total fee and the admin fee, then converted to a percentage.
    let protocol_fee = PoolFee {
        id: protocol_fee_id,
        fee_type: LiquidityPoolFeeType::FixedProtocolFee as i32,
        fee_percentage: (total_fee.clone() * admin_fee.clone() * BigDecimal::from(100)).to_string(),
    };

    let lp_fee_id = EntityKey::pool_fee_id(
        &LiquidityPoolFeeType::FixedLpFee,
        &format_address_vec(&pool_address),
    );
    // Calculate the LP fee. This is the fee allocated to liquidity providers.
    // It is the remaining fee after deducting the protocol's admin fee from the total fee,
    // then converted to a percentage.
    let lp_fee = PoolFee {
        id: lp_fee_id,
        fee_type: LiquidityPoolFeeType::FixedLpFee as i32,
        fee_percentage: ((total_fee.clone() - (total_fee * admin_fee)) * BigDecimal::from(100))
            .to_string(),
    };

    PoolFees {
        trading_fee: Some(trading_fee),
        protocol_fee: Some(protocol_fee),
        lp_fee: Some(lp_fee),
    }
}
