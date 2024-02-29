use crate::{
    common::event_extraction::extract_update_liquidity_limit_event,
    key_management::store_key_manager::StoreKey,
    pb::curve::types::v1::{
        LiquidityGauge, LiquidityGaugeEvent, LiquidityGaugeEventType, LiquidityGaugeEvents,
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
    Event,
};

use crate::abi::curve::gauges;

#[substreams::handlers::map]
pub fn map_gauge_events(
    blk: eth::Block,
    gauge_store: StoreGetProto<LiquidityGauge>,
) -> Result<LiquidityGaugeEvents, Error> {
    let mut gauge_events = LiquidityGaugeEvents::default();
    let mut events: Vec<LiquidityGaugeEvent> = Vec::new();

    for trx in blk.transactions() {
        for (log, _call) in trx.logs_with_calls() {
            let gauge_address = Hex::encode(&log.address);
            let gauge_opt = gauge_store.get_last(StoreKey::liquidity_gauge_key(&gauge_address));

            if let Some(gauge) = gauge_opt {
                if let Some(deposit) =
                    gauges::liquidity_gauge_v1::events::Deposit::match_and_decode(&log)
                {
                    if let Some(event) = handle_event(
                        &deposit.provider,
                        &deposit.value,
                        LiquidityGaugeEventType::Deposit,
                        &trx,
                        &gauge,
                        &blk,
                        &log,
                    ) {
                        events.push(event);
                    }
                }
                if let Some(withdraw) =
                    gauges::liquidity_gauge_v1::events::Withdraw::match_and_decode(&log)
                {
                    if let Some(event) = handle_event(
                        &withdraw.provider,
                        &withdraw.value,
                        LiquidityGaugeEventType::Withdraw,
                        &trx,
                        &gauge,
                        &blk,
                        &log,
                    ) {
                        events.push(event);
                    }
                }
            }
        }
    }
    gauge_events.events = events;
    Ok(gauge_events)
}

fn handle_event(
    event_provider: &Vec<u8>,
    event_value: &BigInt,
    event_type: LiquidityGaugeEventType,
    trx: &TransactionTrace,
    gauge: &LiquidityGauge,
    blk: &eth::Block,
    log: &Log,
) -> Option<LiquidityGaugeEvent> {
    if let Ok(update_event) = extract_update_liquidity_limit_event(trx, &gauge.address_vec()) {
        return Some(LiquidityGaugeEvent {
            gauge: gauge.gauge.clone(),
            pool: gauge.pool.clone(),
            provider: Hex::encode(event_provider),
            value: event_value.to_string(),
            r#type: event_type as i32,
            working_supply: update_event.working_supply.to_string(),
            transaction_hash: Hex::encode(&trx.hash),
            tx_index: trx.index,
            log_index: log.index,
            log_ordinal: log.ordinal,
            timestamp: blk.timestamp_seconds(),
            block_number: blk.number,
        });
    }
    None
}
