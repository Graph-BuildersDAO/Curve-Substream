use substreams::{scalar::BigInt, Hex};

use crate::pb::curve::types::v1::LiquidityGauge;

impl LiquidityGauge {
    pub fn address_vec(&self) -> Vec<u8> {
        Hex::decode(&self.gauge).unwrap()
    }
}

pub struct RewardData {
    pub token: Vec<u8>,
    pub distributor: Vec<u8>,
    pub period_finish: BigInt,
    pub rate: BigInt,
    pub last_update: BigInt,
    pub integral: BigInt,
}
