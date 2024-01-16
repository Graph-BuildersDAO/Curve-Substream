use std::{
    ops::{Div, Mul, Sub},
    str::FromStr,
};

use substreams::{
    scalar::{BigDecimal, BigInt},
    store::{StoreAdd, StoreAddBigInt, StoreGet, StoreGetProto, StoreNew},
};

use crate::{
    pb::curve::types::v1::{events::pool_event::Type, Events, PoolFees},
    store_key_manager::StoreKey,
};

#[substreams::handlers::store]
pub fn store_input_token_balances(
    events: Events,
    pool_fees_store: StoreGetProto<PoolFees>,
    store: StoreAddBigInt,
) {
    for event in events.pool_events {
        if let Some(event_type) = &event.r#type {
            match event_type {
                Type::DepositEvent(deposit) => {
                    // Deposit events will always increase the balances of input tokens
                    for (index, input_token) in deposit.input_tokens.iter().enumerate() {
                        let default_fee_str = String::from("0");
                        let fee_str = deposit.fees.get(index).unwrap_or(&default_fee_str);
                        let fee = BigInt::from_str(fee_str).unwrap_or(BigInt::zero());

                        store.add(
                            event.log_ordinal,
                            StoreKey::input_token_balance_key(
                                &event.pool_address,
                                &input_token.token_address,
                            ),
                            input_token.amount_big().sub(fee),
                        )
                    }
                }
                Type::WithdrawEvent(withdraw) => {
                    // Withdraw events will always decrease the balances of input tokens
                    for (index, input_token) in withdraw.input_tokens.iter().enumerate() {
                        let default_fee_str = String::from("0");
                        let fee_str = withdraw.fees.get(index).unwrap_or(&default_fee_str);
                        let fee = BigInt::from_str(fee_str).unwrap_or(BigInt::zero());

                        store.add(
                            event.log_ordinal,
                            StoreKey::input_token_balance_key(
                                &event.pool_address,
                                &input_token.token_address,
                            ),
                            input_token.amount_big().sub(fee).neg(),
                        )
                    }
                }
                Type::SwapEvent(swap) => {
                    // Update input token balance
                    let pool_fees = pool_fees_store
                        .get_last(StoreKey::pool_fees_key(&event.pool_address))
                        .unwrap();

                    let input_token_amount = swap.token_in_amount_big();
                    let output_token_amount = swap.token_out_amount_big();

                    let fee_percentage = BigDecimal::from_str(&pool_fees.lp_fee().fee_percentage)
                        .expect("Invalid fee percentage string");
                    let fee_fraction = fee_percentage.div(BigDecimal::from(100));
                    let fee_amount_bigdecimal =
                        BigDecimal::from(output_token_amount.clone()) * fee_fraction;
                    let fee_amount_bigint = fee_amount_bigdecimal.to_bigint();

                    store.add(
                        event.log_ordinal,
                        StoreKey::input_token_balance_key(
                            &event.pool_address,
                            &swap.token_in_ref().token_address,
                        ),
                        input_token_amount,
                    );
                    // Update output token balance
                    store.add(
                        event.log_ordinal,
                        StoreKey::input_token_balance_key(
                            &event.pool_address,
                            &swap.token_out_ref().token_address,
                        ),
                        output_token_amount.neg().sub(fee_amount_bigint),
                    );
                }
            }
        }
    }
}
