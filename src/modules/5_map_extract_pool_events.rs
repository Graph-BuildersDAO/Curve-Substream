use substreams::{
    errors::Error,
    store::{StoreGet, StoreGetProto},
    Hex,
};
use substreams_ethereum::{
    pb::eth::v2::{self as eth},
    Event,
};

use crate::{
    abi::pool::events::{
        AddLiquidity1, AddLiquidity2, AddLiquidity3, AddLiquidity4, AddLiquidity5,
        RemoveLiquidity1, RemoveLiquidity2, RemoveLiquidity3, RemoveLiquidity4, RemoveLiquidity5,
        RemoveLiquidityImbalance1, RemoveLiquidityImbalance2, RemoveLiquidityImbalance3,
        RemoveLiquidityOne1, RemoveLiquidityOne2, TokenExchange1, TokenExchange2,
        TokenExchangeUnderlying,
    },
    pb::curve::types::v1::{events::PoolEvent, Events, Pool},
    utils::{
        extract_deposit_event, extract_swap_event, extract_withdraw_event,
        extract_withdraw_one_event,
    },
};

#[substreams::handlers::map]
pub fn map_extract_pool_events(
    blk: eth::Block,
    pools: StoreGetProto<Pool>,
) -> Result<Events, Error> {
    // Initialise events and its fields
    let mut events = Events::default();
    let mut pool_events: Vec<PoolEvent> = Vec::new();

    // Check if event is coming from the pool contract
    for trx in blk.transactions() {
        for (log, _call) in trx.logs_with_calls() {
            let pool_opt = pools.get_last(format!("pool:{}", Hex::encode(&log.address)));

            if let Some(pool) = pool_opt {
                if let Some(swap) = TokenExchange1::match_and_decode(&log) {
                    extract_swap_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        &swap.sold_id,
                        &swap.bought_id,
                        &swap.tokens_sold,
                        &swap.tokens_bought,
                        &swap.buyer,
                        false,
                    );
                } else if let Some(swap) = TokenExchange2::match_and_decode(&log) {
                    extract_swap_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        &swap.sold_id,
                        &swap.bought_id,
                        &swap.tokens_sold,
                        &swap.tokens_bought,
                        &swap.buyer,
                        false,
                    );
                } else if let Some(swap) = TokenExchangeUnderlying::match_and_decode(&log) {
                    extract_swap_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        &swap.sold_id,
                        &swap.bought_id,
                        &swap.tokens_sold,
                        &swap.tokens_bought,
                        &swap.buyer,
                        true,
                    );
                } else if let Some(deposit) = AddLiquidity1::match_and_decode(&log) {
                    let fees = vec![deposit.fee.into()];
                    extract_deposit_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        deposit.token_amounts.to_vec(),
                        fees,
                        deposit.provider,
                    );
                } else if let Some(deposit) = AddLiquidity2::match_and_decode(&log) {
                    let fees = vec![deposit.fee.into()];
                    extract_deposit_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        deposit.token_amounts.to_vec(),
                        fees,
                        deposit.provider,
                    );
                } else if let Some(deposit) = AddLiquidity3::match_and_decode(&log) {
                    let fees = deposit.fees.iter().map(ToString::to_string).collect();
                    extract_deposit_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        deposit.token_amounts.to_vec(),
                        fees,
                        deposit.provider,
                    );
                } else if let Some(deposit) = AddLiquidity4::match_and_decode(&log) {
                    let fees = deposit.fees.iter().map(ToString::to_string).collect();
                    extract_deposit_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        deposit.token_amounts.to_vec(),
                        fees,
                        deposit.provider,
                    );
                } else if let Some(deposit) = AddLiquidity5::match_and_decode(&log) {
                    let fees = deposit.fees.iter().map(ToString::to_string).collect();
                    extract_deposit_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        deposit.token_amounts.to_vec(),
                        fees,
                        deposit.provider,
                    );
                } else if let Some(withdraw) = RemoveLiquidity1::match_and_decode(&log) {
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        Vec::new(), // No fees on RemoveLiquidty1 events
                    );
                } else if let Some(withdraw) = RemoveLiquidity2::match_and_decode(&log) {
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        Vec::new(), // No fees on RemoveLiquidty2 events
                    );
                } else if let Some(withdraw) = RemoveLiquidity3::match_and_decode(&log) {
                    let fees: Vec<String> = withdraw.fees.iter().map(ToString::to_string).collect();
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        fees,
                    );
                } else if let Some(withdraw) = RemoveLiquidity4::match_and_decode(&log) {
                    let fees: Vec<String> = withdraw.fees.iter().map(ToString::to_string).collect();
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        fees,
                    );
                } else if let Some(withdraw) = RemoveLiquidity5::match_and_decode(&log) {
                    let fees: Vec<String> = withdraw.fees.iter().map(ToString::to_string).collect();
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        fees,
                    );
                } else if let Some(withdraw) = RemoveLiquidityImbalance1::match_and_decode(&log) {
                    let fees: Vec<String> = withdraw.fees.iter().map(ToString::to_string).collect();
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        fees,
                    );
                } else if let Some(withdraw) = RemoveLiquidityImbalance2::match_and_decode(&log) {
                    let fees: Vec<String> = withdraw.fees.iter().map(ToString::to_string).collect();
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        fees,
                    );
                } else if let Some(withdraw) = RemoveLiquidityImbalance3::match_and_decode(&log) {
                    let fees: Vec<String> = withdraw.fees.iter().map(ToString::to_string).collect();
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        fees,
                    );
                } else if let Some(withdraw) = RemoveLiquidityOne1::match_and_decode(&log) {
                    extract_withdraw_one_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amount,
                        withdraw.coin_amount,
                    );
                } else if let Some(withdraw) = RemoveLiquidityOne2::match_and_decode(&log) {
                    extract_withdraw_one_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amount,
                        withdraw.coin_amount,
                    );
                }
            }
        }
    }
    events.pool_events = pool_events;
    Ok(events)
}
