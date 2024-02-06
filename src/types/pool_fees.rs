use std::str::FromStr;

use substreams::scalar::BigDecimal;

use crate::pb::curve::types::v1::{PoolFee, PoolFees};

impl PoolFee {
    pub fn fee_percentage_big_decimal(&self) -> BigDecimal {
        BigDecimal::from_str(self.fee_percentage.as_str()).unwrap()
    }
}

impl PoolFees {
    pub fn trading_fee(&self) -> &PoolFee {
        self.trading_fee.as_ref().unwrap()
    }

    pub fn protocol_fee(&self) -> &PoolFee {
        self.protocol_fee.as_ref().unwrap()
    }

    pub fn lp_fee(&self) -> &PoolFee {
        self.lp_fee.as_ref().unwrap()
    }

    pub fn string_ids(&self) -> Vec<String> {
        vec![
            self.trading_fee().id.clone(),
            self.protocol_fee().id.clone(),
            self.lp_fee().id.clone(),
        ]
    }
}
