use crate::common::{format::format_address_string, utils};

pub enum EntityKey {
    Protocol,
    LiquidityPool(String),
    Deposit(String, String),
    Swap(String, String),
    Withdraw(String, String),
}

impl EntityKey {
    pub fn protocol_key() -> String {
        EntityKey::Protocol.to_key_string()
    }

    // Pass in pool address as is (without 0x).
    // This function will handle the formatting.
    pub fn liquidity_pool_key(pool_address: &str) -> String {
        EntityKey::LiquidityPool(pool_address.to_string()).to_key_string()
    }

    pub fn deposit_key(transaction_hash: &str, log_index: &u32) -> String {
        EntityKey::Deposit(transaction_hash.to_string(), log_index.to_string()).to_key_string()
    }

    pub fn swap_key(transaction_hash: &str, log_index: &u32) -> String {
        EntityKey::Swap(transaction_hash.to_string(), log_index.to_string()).to_key_string()
    }

    pub fn withdraw_key(transaction_hash: &str, log_index: &u32) -> String {
        EntityKey::Withdraw(transaction_hash.to_string(), log_index.to_string()).to_key_string()
    }

    fn to_key_string(&self) -> String {
        match self {
            EntityKey::Protocol => utils::get_protocol_id(),
            EntityKey::LiquidityPool(address) => format_address_string(address),
            EntityKey::Deposit(tx_hash, log_index) => {
                format!("deposit-0x{}-{}", tx_hash, log_index)
            }
            EntityKey::Swap(tx_hash, log_index) => {
                format!("swap-0x{}-{}", tx_hash, log_index)
            }
            EntityKey::Withdraw(tx_hash, log_index) => {
                format!("withdraw-0x{}-{}", tx_hash, log_index)
            }
        }
    }
}
