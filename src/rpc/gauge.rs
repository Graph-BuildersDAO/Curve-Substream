use substreams::{scalar::BigInt, Hex};

use crate::{
    abi::curve::{gauge_controller, gauges},
    network_config::GAUGE_CONTROLLER_ADDRESS,
    types::gauge::RewardData,
};

pub fn get_gauge_relative_weight(gauge_address: &Vec<u8>) -> BigInt {
    match (gauge_controller::functions::GaugeRelativeWeight1 {
        addr: gauge_address.clone(),
    }
    .call(GAUGE_CONTROLLER_ADDRESS.to_vec()))
    {
        Some(weight) => weight,
        None => {
            substreams::log::debug!(
                "Failed to get gauge {} relative weight from gauge controller",
                Hex::encode(&gauge_address)
            );
            BigInt::zero()
        }
    }
}

pub fn get_reward_token_data(
    gauge_address: &Vec<u8>,
    token_address: &Vec<u8>,
) -> Option<RewardData> {
    match (gauges::liquidity_gauge_v4::functions::RewardData {
        arg0: token_address.clone(),
    }
    .call(gauge_address.clone()))
    {
        Some((token, distributor, period_finish, rate, last_update, integral)) => {
            return Some(RewardData {
                token,
                distributor,
                period_finish,
                rate,
                last_update,
                integral,
            })
        }
        None => {
            // V5 and V6 gauges have the same function ABI, so we just need to match against one
            match (gauges::liquidity_gauge_v6::functions::RewardData {
                arg0: token_address.clone(),
            }
            .call(gauge_address.clone()))
            {
                Some((token, distributor, period_finish, rate, last_update, integral)) => {
                    return Some(RewardData {
                        token,
                        distributor,
                        period_finish,
                        rate,
                        last_update,
                        integral,
                    })
                }
                None => {
                    substreams::log::debug!(
                        "Cannot find reward data for token {} on gauge {}",
                        Hex::encode(token_address),
                        Hex::encode(gauge_address)
                    );
                    None
                }
            }
        }
    }
}
