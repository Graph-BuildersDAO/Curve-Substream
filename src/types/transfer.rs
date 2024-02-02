use substreams::scalar::BigInt;

use crate::abi::common::erc20::events::Transfer;

/// Extends the ERC20 `Transfer` event with the token address, useful for identifying the token involved in a transfer.
pub struct TokenTransfer {
    pub transfer: Transfer,
    pub token_address: Vec<u8>,
}

impl TokenTransfer {
    pub fn new(transfer: Transfer, token_address: Vec<u8>) -> Self {
        TokenTransfer {
            transfer,
            token_address,
        }
    }
}

pub struct RemoveLiquidityTransfer {
    pub pool_address: Vec<u8>,
    pub provider: Vec<u8>,
    pub token_amount: BigInt,
    pub coin_amount: BigInt,
}

impl RemoveLiquidityTransfer {
    pub fn new(
        pool_address: Vec<u8>,
        provider: Vec<u8>,
        token_amount: BigInt,
        coin_amount: BigInt,
    ) -> Self {
        RemoveLiquidityTransfer {
            pool_address,
            provider,
            token_amount,
            coin_amount,
        }
    }
}
