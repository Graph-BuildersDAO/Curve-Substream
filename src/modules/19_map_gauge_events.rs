use std::collections::HashSet;

use crate::{
    abi::curve::{gauges, ownership_proxies},
    key_management::store_key_manager::StoreKey,
    pb::curve::types::v1::{
        AddRewardEvent, GaugeLiquidityEventType, LiquidityEvent, LiquidityGauge,
        LiquidityGaugeEvents,
    },
};
use substreams::{
    errors::Error,
    scalar::BigInt,
    store::{StoreGet, StoreGetProto},
    Hex,
};
use substreams_ethereum::{
    pb::eth::v2::{self as eth, Log, TransactionTrace},
    Event, Function,
};

#[substreams::handlers::map]
pub fn map_gauge_events(
    blk: eth::Block,
    gauge_store: StoreGetProto<LiquidityGauge>,
) -> Result<LiquidityGaugeEvents, Error> {
    let mut gauge_events = LiquidityGaugeEvents::default();
    let mut liquidity_events: Vec<LiquidityEvent> = Vec::new();
    let mut unique_add_reward_events: Vec<AddRewardEvent> = Vec::new();
    let mut seen_tx_hashes = HashSet::new();

    for trx in blk.transactions() {
        // Handle liquidity events (`Deposit`, `Withdraw`) by digging into the logs.
        for (log, _) in trx.logs_with_calls() {
            let gauge_address = Hex::encode(&log.address);
            let gauge_opt = gauge_store.get_last(StoreKey::liquidity_gauge_key(&gauge_address));

            if let Some(gauge) = gauge_opt {
                if let Some(deposit) =
                    gauges::liquidity_gauge_v1::events::Deposit::match_and_decode(&log)
                {
                    handle_liquidity_event(
                        &deposit.provider,
                        &deposit.value,
                        GaugeLiquidityEventType::Deposit,
                        &trx,
                        &gauge,
                        &blk,
                        &log,
                        &mut liquidity_events,
                    );
                }
                if let Some(withdraw) =
                    gauges::liquidity_gauge_v1::events::Withdraw::match_and_decode(&log)
                {
                    handle_liquidity_event(
                        &withdraw.provider,
                        &withdraw.value,
                        GaugeLiquidityEventType::Withdraw,
                        &trx,
                        &gauge,
                        &blk,
                        &log,
                        &mut liquidity_events,
                    );
                }
            }
        }
        // Handle AddReward function calls as these do not emit events and need to be captured by examining function calls.
        for call_view in trx.calls().filter(|call| !call.call.state_reverted) {
            if let Some(add_reward_call) =
                // Although there are multiple ABIs for proxies, the `add_reward` function remains the same.
                // Therefore we only need to match for one of these and it will catch all with the same function ABI.
                ownership_proxies::factory_owner::functions::AddReward::match_and_decode(
                        &call_view.call,
                    )
            {
                handle_add_reward_event(
                    trx,
                    &blk,
                    &add_reward_call.u_gauge,
                    &add_reward_call.u_reward_token,
                    &add_reward_call.u_distributor,
                    &mut seen_tx_hashes,
                    &mut unique_add_reward_events,
                    &gauge_store,
                );
            }
            if let Some(add_reward_call) =
                // Although there are multiple ABIs for different version of `LiquidityGauge`, the `add_reward` function remains the same.
                // Therefore we only need to match for one of these and it will catch all with the same function ABI.
                gauges::liquidity_gauge_v6::functions::AddReward::match_and_decode(
                        &call_view.call,
                    )
            {
                handle_add_reward_event(
                    trx,
                    &blk,
                    &call_view.call.address,
                    &add_reward_call.u_reward_token,
                    &add_reward_call.u_distributor,
                    &mut seen_tx_hashes,
                    &mut unique_add_reward_events,
                    &gauge_store,
                );
            }
        }
    }

    gauge_events.liquidity_events = liquidity_events;
    gauge_events.add_reward_events = unique_add_reward_events;

    Ok(gauge_events)
}

fn handle_liquidity_event(
    event_provider: &Vec<u8>,
    event_value: &BigInt,
    event_type: GaugeLiquidityEventType,
    trx: &TransactionTrace,
    gauge: &LiquidityGauge,
    blk: &eth::Block,
    log: &Log,
    liquidity_events: &mut Vec<LiquidityEvent>,
) {
    if let Some(working_supply) = extract_gauge_working_supply(trx, &gauge.address_vec()) {
        substreams::log::debug!("pushing to evnets");
        liquidity_events.push(LiquidityEvent {
            gauge: gauge.gauge.clone(),
            pool: gauge.pool.clone(),
            provider: Hex::encode(event_provider),
            value: event_value.to_string(),
            r#type: event_type as i32,
            working_supply: working_supply.to_string(),
            transaction_hash: Hex::encode(&trx.hash),
            tx_index: trx.index,
            log_index: log.index,
            log_ordinal: log.ordinal,
            timestamp: blk.timestamp_seconds(),
            block_number: blk.number,
        });
    }
}

fn handle_add_reward_event(
    trx: &TransactionTrace,
    blk: &eth::Block,
    gauge_address: &Vec<u8>,
    reward_token: &Vec<u8>,
    distributor: &Vec<u8>,
    seen_tx_hashes: &mut HashSet<String>,
    unique_add_reward_events: &mut Vec<AddRewardEvent>,
    gauge_store: &StoreGetProto<LiquidityGauge>,
) {
    // Shadowing as don't need the raw address after this call
    let gauge_address = StoreKey::liquidity_gauge_key(&Hex::encode(gauge_address));

    // Check if the gauge related to the add_reward call exists in the gauge store
    if let Some(gauge) = gauge_store.get_last(&gauge_address) {
        let tx_hash_str = Hex::encode(&trx.hash);

        if seen_tx_hashes.insert(tx_hash_str.clone()) {
            // If the transaction hash was successfully inserted (i.e., it's a new unique hash), add the event.
            unique_add_reward_events.push(AddRewardEvent {
                gauge: Hex::encode(&gauge_address),
                pool: gauge.pool,
                reward_token: Hex::encode(&reward_token),
                distributor: Hex::encode(&distributor),
                transaction_hash: tx_hash_str,
                tx_index: trx.index,
                timestamp: blk.timestamp_seconds(),
                block_number: blk.number,
            })
        }
    }
}

pub fn extract_gauge_working_supply(trx: &TransactionTrace, gauge: &Vec<u8>) -> Option<BigInt> {
    trx.calls
        .iter()
        .filter(|call| !call.state_reverted)
        .flat_map(|call| call.logs.iter())
        .find_map(|log| {
            if log.address == *gauge {
                if let Some(update_v1) =
                    gauges::liquidity_gauge_v1::events::UpdateLiquidityLimit::match_and_decode(&log)
                {
                    return Some(update_v1.working_supply);
                } else if let Some(update_v6) =
                    gauges::liquidity_gauge_v6::events::UpdateLiquidityLimit::match_and_decode(&log)
                {
                    return Some(update_v6.working_supply);
                }
            }
            substreams::log::debug!(
                "Failed to extract UpdateLiquidityLimit for Gauge {} and Tx {}",
                Hex::encode(gauge),
                Hex::encode(&trx.hash)
            );
            None
        })
}
