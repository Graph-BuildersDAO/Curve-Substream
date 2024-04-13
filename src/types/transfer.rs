use crate::abi::common::erc20::events::Transfer;

/// Extends the ERC20 `Transfer` event with the token address, useful for identifying the token involved in a transfer.
#[derive(Clone)]
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
