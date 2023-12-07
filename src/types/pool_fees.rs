use substreams::scalar::BigDecimal;

use crate::constants::LiquidityPoolFeeType;

pub struct PoolFee {
    id: String,
    fee_type: LiquidityPoolFeeType,
    fee_percentage: BigDecimal,
}

impl PoolFee {
    pub fn new(id: String, fee_type: LiquidityPoolFeeType, fee_percentage: BigDecimal) -> Self {
        PoolFee {
            id,
            fee_type,
            fee_percentage,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_fee_type(&self) -> &LiquidityPoolFeeType {
        &self.fee_type
    }

    pub fn get_fee_percentage(&self) -> &BigDecimal {
        &self.fee_percentage
    }
}

pub struct PoolFees {
    trading_fee: PoolFee,
    protocol_fee: PoolFee,
    lp_fee: PoolFee,
}

impl PoolFees {
    pub fn new(trading_fee: PoolFee, protocol_fee: PoolFee, lp_fee: PoolFee) -> Self {
        PoolFees {
            trading_fee,
            protocol_fee,
            lp_fee,
        }
    }

    pub fn trading_fee(&self) -> &PoolFee {
        &self.trading_fee
    }

    pub fn protocol_fee(&self) -> &PoolFee {
        &self.protocol_fee
    }

    pub fn lp_fee(&self) -> &PoolFee {
        &self.lp_fee
    }

    pub fn string_ids(&self) -> Vec<String> {
        vec![
            self.trading_fee.get_id().to_string(),
            self.protocol_fee.get_id().to_string(),
            self.lp_fee.get_id().to_string(),
        ]
    }
}
