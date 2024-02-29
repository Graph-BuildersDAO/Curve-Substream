use substreams::Hex;

use crate::pb::curve::types::v1::LiquidityGauge;

impl LiquidityGauge {
    pub fn address_vec(&self) -> Vec<u8> {
        Hex::decode(&self.gauge).unwrap()
    }
}
