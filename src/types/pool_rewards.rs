use std::str::FromStr;

use substreams::scalar::{BigDecimal, BigInt};

use crate::pb::curve::types::v1::PoolRewards;

impl PoolRewards {
    pub fn parse_reward_token_emissions_native(&self) -> Vec<BigInt> {
        self.reward_token_emissions_native
            .iter()
            .map(|rt| BigInt::from_str(rt).unwrap_or_else(|_| BigInt::zero()))
            .collect()
    }

    pub fn parse_reward_token_emissions_usd(&self) -> Vec<BigDecimal> {
        self.reward_token_emissions_usd
            .iter()
            .map(|rt| BigDecimal::from_str(rt).unwrap_or_else(|_| BigDecimal::zero()))
            .collect()
    }

    pub fn parse_staked_output_token_amount(&self) -> BigInt {
        BigInt::from_str(&self.staked_output_token_amount).unwrap_or_else(|_| BigInt::zero())
    }
}
