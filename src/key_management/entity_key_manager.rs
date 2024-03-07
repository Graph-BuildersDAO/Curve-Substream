use crate::{
    common::{conversion::convert_enum_to_snake_case_prefix, format::format_address_string, utils},
    pb::curve::types::v1::LiquidityPoolFeeType,
};

pub enum EntityKey {
    Protocol,
    LiquidityPool(String),
    LiquidityPoolFee(String, String),
    LiquidityPoolDailySnapshot(String, i64),
    LiquidityPoolHourlySnapshot(String, i64),
    ProtocolDailyFinancialsSnapshot(i64),
    Token(String),
    RewardToken(String),
    PoolRewardToken(String, String),
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

    pub fn pool_fee_id(fee_type: &LiquidityPoolFeeType, pool_address: &str) -> String {
        EntityKey::LiquidityPoolFee(
            convert_enum_to_snake_case_prefix(fee_type.as_str_name()),
            pool_address.to_string(),
        )
        .to_key_string()
    }

    pub fn pool_daily_snapshot_key(pool_address: &str, day_id: &i64) -> String {
        EntityKey::LiquidityPoolDailySnapshot(pool_address.to_string(), *day_id).to_key_string()
    }

    pub fn pool_hourly_snapshot_key(pool_address: &str, hour_id: &i64) -> String {
        EntityKey::LiquidityPoolHourlySnapshot(pool_address.to_string(), *hour_id).to_key_string()
    }

    pub fn protocol_daily_financials_key(day_id: &i64) -> String {
        EntityKey::ProtocolDailyFinancialsSnapshot(*day_id).to_key_string()
    }

    pub fn token_key(token_address: &str) -> String {
        EntityKey::Token(token_address.to_string()).to_key_string()
    }

    pub fn reward_token_key(reward_token_address: &str) -> String {
        EntityKey::RewardToken(reward_token_address.to_string()).to_key_string()
    }

    pub fn pool_reward_token_key(pool_address: &str, reward_token_address: &str) -> String {
        EntityKey::PoolRewardToken(pool_address.to_string(), reward_token_address.to_string())
            .to_key_string()
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
            EntityKey::LiquidityPoolFee(fee_type, pool_address) => {
                format!("{}{}", fee_type, pool_address)
            }
            EntityKey::LiquidityPoolDailySnapshot(pool_address, day_id) => {
                format!(
                    "{}-{}",
                    format_address_string(pool_address),
                    day_id.to_string()
                )
            }
            EntityKey::LiquidityPoolHourlySnapshot(pool_address, hour_id) => {
                format!(
                    "{}-{}",
                    format_address_string(pool_address),
                    hour_id.to_string()
                )
            }
            EntityKey::ProtocolDailyFinancialsSnapshot(day_id) => day_id.to_string(),
            EntityKey::Token(token_address) => format_address_string(token_address),
            EntityKey::RewardToken(reward_token_address) => {
                format_address_string(reward_token_address)
            }
            EntityKey::PoolRewardToken(pool_address, reward_token_address) => {
                format!(
                    "{}-{}",
                    format_address_string(pool_address),
                    format_address_string(reward_token_address)
                )
            }
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
