use substreams::store::{StoreAdd, StoreAddBigInt, StoreNew};

use crate::{
    pb::curve::types::v1::{events::pool_event::Type, Events},
    store_key_manager::StoreKey,
};

#[substreams::handlers::store]
pub fn store_input_token_balances(events: Events, store: StoreAddBigInt) {
    for event in events.pool_events {
        if let Some(event_type) = &event.r#type {
            match event_type {
                Type::DepositEvent(deposit) => {
                    // Deposit events will always increase the balances of input tokens
                    for input_token in &deposit.input_tokens {
                        store.add(
                            event.log_ordinal,
                            StoreKey::input_token_balance_key(
                                &event.pool_address,
                                &input_token.token_address,
                            ),
                            input_token.amount_big(),
                        )
                    }
                }
                Type::WithdrawEvent(withdraw) => {
                    // Withdraw events will always decrease the balances of input tokens
                    for input_token in &withdraw.input_tokens {
                        store.add(
                            event.log_ordinal,
                            StoreKey::input_token_balance_key(
                                &event.pool_address,
                                &input_token.token_address,
                            ),
                            input_token.amount_big().neg(),
                        )
                    }
                }
                Type::SwapEvent(swap) => {
                    // Update input token balance
                    store.add(
                        event.log_ordinal,
                        StoreKey::input_token_balance_key(
                            &event.pool_address,
                            &swap.token_in_ref().token_address,
                        ),
                        swap.token_in_amount_big(),
                    );
                    // Update output token balance
                    store.add(
                        event.log_ordinal,
                        StoreKey::input_token_balance_key(
                            &event.pool_address,
                            &swap.token_out_ref().token_address,
                        ),
                        swap.token_out_amount_big().neg(),
                    );
                }
            }
        }
    }
}
