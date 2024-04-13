use substreams::store::{StoreAdd, StoreAddBigInt, StoreNew};

use crate::{
    key_management::store_key_manager::StoreKey,
    pb::curve::types::v1::{
        events::pool_event::{LpTokenChangeType, Type},
        Events,
    },
};

#[substreams::handlers::store]
pub fn store_input_token_balances(events: Events, store: StoreAddBigInt) {
    for event in events.pool_events {
        if let Some(event_type) = &event.r#type {
            match event_type {
                Type::DepositEvent(deposit) => {
                    // `Deposit` events will always increase the balances of input tokens
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
                    // `Withdraw` events will always decrease the balances of input tokens
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
                Type::SwapUnderlyingMetaEvent(swap_underlying) => {
                    if let Some(lp_token_change) = &swap_underlying.lp_token_change {
                        match LpTokenChangeType::from_i32(lp_token_change.change_type) {
                            // This represents a Metapool asset being exchanged for a Base pool underlying asset
                            // LP tokens from the base pool that the metapool holds are burnt in exchange for an underlying asset.
                            // A remove liquidity event will also be emitted as part of the transaction. The output token balance
                            // on the base pool will be updated by the remove liquidity event, so we don't need to handle that here.
                            Some(LpTokenChangeType::Burn) => {
                                store.add(
                                    event.log_ordinal,
                                    StoreKey::input_token_balance_key(
                                        &event.pool_address,
                                        &swap_underlying.token_in_ref().token_address,
                                    ),
                                    swap_underlying.token_in_amount_big(),
                                );
                                store.add(
                                    event.log_ordinal,
                                    StoreKey::input_token_balance_key(
                                        &event.pool_address,
                                        &lp_token_change.token_address,
                                    ),
                                    swap_underlying.lp_token_change_amount_big().neg(),
                                );
                            }
                            // This represents a Base pools underlying asset being exchanged for a Metapool asset.
                            // The Metapool provides liquidity to the base pool in exchange for LP tokens.
                            // This then enables the Metapool to provide its own asset to the buyer.
                            // An add liquidity event will also be emitted as part of the transaction. The output token balance
                            // on the base pool will be updated by the remove liquidity event, so we don't need to handle that here.
                            Some(LpTokenChangeType::Mint) => {
                                store.add(
                                    event.log_ordinal,
                                    StoreKey::input_token_balance_key(
                                        &event.pool_address,
                                        &swap_underlying.token_out_ref().token_address,
                                    ),
                                    swap_underlying.token_out_amount_big().neg(),
                                );
                                store.add(
                                    event.log_ordinal,
                                    StoreKey::input_token_balance_key(
                                        &event.pool_address,
                                        &lp_token_change.token_address,
                                    ),
                                    swap_underlying.lp_token_change_amount_big(),
                                );
                            }
                            // This represents when a base pools underlying asset is exchanged for another of its assets.
                            // There are no changes to the metapools balances as a result, as it merely facilitates the exchange.
                            None => {}
                        }
                    }
                }
                Type::SwapUnderlyingLendingEvent(swap_underlying) => {
                    // TODO we can potentially use the MINT/BURN enum to check whether we should be adding/subtracting
                    // A lending pool contains interest bearing tokens. These are the balances that
                    // change during a `TokenExchangeUnderlying` event on this pool.
                    if let Some(in_action) = &swap_underlying.interest_bearing_token_in_action {
                        store.add(
                            event.log_ordinal,
                            StoreKey::input_token_balance_key(
                                &event.pool_address,
                                &in_action.token_address,
                            ),
                            swap_underlying.interest_bearing_token_in_action_amount_big(),
                        );
                    }
                    if let Some(out_action) = &swap_underlying.interest_bearing_token_out_action {
                        store.add(
                            event.log_ordinal,
                            StoreKey::input_token_balance_key(
                                &event.pool_address,
                                &out_action.token_address,
                            ),
                            swap_underlying
                                .interest_bearing_token_out_action_amount_big()
                                .neg(),
                        )
                    }
                }
            }
        }
    }
}
