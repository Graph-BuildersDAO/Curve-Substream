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
    abi::pool::events::{TokenExchange1, TokenExchange2, TokenExchangeUnderlying},
    pb::curve::types::v1::{events::PoolEvent, Events, Pool},
    utils::extract_swap_event,
};

#[substreams::handlers::map]
pub fn map_extract_pool_events(
    blk: eth::Block,
    pools: StoreGetProto<Pool>,
) -> Result<Events, Error> {
    // Initialise events
    let mut events = Events::default();

    // Init the events fields
    let mut pool_events: Vec<PoolEvent> = Vec::new();

    // Check if event is coming from the pool contract
    for trx in blk.transactions() {
        for (log, _call) in trx.logs_with_calls() {
            let pool_address = Hex::encode(&log.address);
            let pool_opt = pools.get_last(format!("pool:{}", pool_address));

            if let Some(pool) = pool_opt {
                // TODO: Consider using a match expression similar to the message enum match here:
                //       (https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/reference/expressions/match-expr.html)
                if let Some(swap) = TokenExchange1::match_and_decode(&log) {
                    extract_swap_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        &pool_address,
                        &swap.sold_id,
                        &swap.bought_id,
                        &swap.tokens_sold,
                        &swap.tokens_bought,
                        &swap.buyer,
                        false,
                    );
                }
                // TokenExchange2
                else if let Some(swap) = TokenExchange2::match_and_decode(&log) {
                    extract_swap_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        &pool_address,
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
                        &pool_address,
                        &swap.sold_id,
                        &swap.bought_id,
                        &swap.tokens_sold,
                        &swap.tokens_bought,
                        &swap.buyer,
                        true,
                    );
                }
            }
        }
    }
    events.pool_events = pool_events;
    Ok(events)
}
