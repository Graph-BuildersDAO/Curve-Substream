use substreams::store::{StoreAdd, StoreAddBigInt, StoreNew};

use crate::{
    pb::curve::types::v1::{events::pool_event::Type, Events},
    store_key_manager::StoreKey,
};

#[substreams::handlers::store]
pub fn store_output_token_supply(events: Events, store: StoreAddBigInt) {
    for event in events.pool_events {
        // on deposits or withdrawals, output token supply is updated
        if let Some(event_type) = &event.r#type {
            match event_type {
                Type::DepositEvent(deposit) => store.add(
                    event.log_ordinal,
                    StoreKey::output_token_supply_key(&event.pool_address),
                    deposit.output_token_amount_big(),
                ),
                Type::WithdrawEvent(withdraw) => store.add(
                    event.log_ordinal,
                    StoreKey::output_token_supply_key(&event.pool_address),
                    withdraw.output_token_amount_big().neg(),
                ),
                _ => {}
            }
        }
    }
}
