use std::str::FromStr;

use substreams::scalar::BigDecimal;

use crate::pb::curve::types::v1::{LiquidityPoolFeeType, PoolFee, PoolFees};

impl PoolFee {
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_fee_type(&self) -> &'static str {
        fee_type_to_string(self.fee_type)
    }

    pub fn get_fee_percentage(&self) -> BigDecimal {
        BigDecimal::from_str(&self.fee_percentage).unwrap_or(BigDecimal::zero())
    }
}

impl PoolFees {
    pub fn trading_fee(&self) -> &PoolFee {
        &self.trading_fee.as_ref().unwrap()
    }

    pub fn protocol_fee(&self) -> &PoolFee {
        &self.protocol_fee.as_ref().unwrap()
    }

    pub fn lp_fee(&self) -> &PoolFee {
        &self.lp_fee.as_ref().unwrap()
    }

    pub fn string_ids(&self) -> Vec<String> {
        vec![
            self.trading_fee().get_id().to_string(),
            self.protocol_fee().get_id().to_string(),
            self.lp_fee().get_id().to_string(),
        ]
    }
}

fn fee_type_to_string(fee_type: i32) -> &'static str {
    match LiquidityPoolFeeType::from_i32(fee_type) {
        Some(fee_enum) => fee_enum.as_str_name(),
        None => "UNKNOWN",
    }
}
