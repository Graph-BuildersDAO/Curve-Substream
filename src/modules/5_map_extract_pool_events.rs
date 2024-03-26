use num_traits::ToPrimitive;
use substreams::{
    errors::Error,
    scalar::BigInt,
    store::{StoreGet, StoreGetBigDecimal, StoreGetProto},
    Hex,
};
use substreams_ethereum::{
    pb::eth::v2::{self as eth, Log, TransactionTrace},
    Event, NULL_ADDRESS,
};

use crate::{
    abi::{
        common::erc20::events::Transfer,
        curve::{
            pool::events::{
                AddLiquidity1, AddLiquidity2, AddLiquidity3, AddLiquidity4, AddLiquidity5,
                ApplyNewFee1, ApplyNewFee2, NewFee1, NewFee2, NewParameters1, NewParameters2,
                NewParameters3, RemoveLiquidity1, RemoveLiquidity2, RemoveLiquidity3,
                RemoveLiquidity4, RemoveLiquidity5, RemoveLiquidityImbalance1,
                RemoveLiquidityImbalance2, RemoveLiquidityImbalance3, RemoveLiquidityOne1,
                RemoveLiquidityOne2, TokenExchange1, TokenExchange2, TokenExchangeUnderlying,
            },
            pools::lending_pool,
        },
    },
    common::{
        event_extraction,
        pool_utils::{is_lending_pool, is_metapool},
    },
    key_management::store_key_manager::StoreKey,
    pb::{
        curve::types::v1::{
            events::{
                pool_event::{
                    DepositEvent, LpTokenChange, LpTokenChangeType, SwapEvent,
                    SwapUnderlyingLendingEvent, SwapUnderlyingMetaEvent, TokenAmount, TokenSource,
                    Type, WithdrawEvent,
                },
                FeeChangeEvent, PoolEvent,
            },
            pool::PoolType,
            Events, Pool,
        },
        uniswap_pricing::v1::Erc20Price,
    },
    rpc::pool::get_pool_fee_and_admin_fee,
};

#[substreams::handlers::map]
pub fn map_extract_pool_events(
    blk: eth::Block,
    pools_store: StoreGetProto<Pool>,
) -> Result<Events, Error> {
    // Initialise events and its fields
    let mut events = Events::default();
    let mut pool_events: Vec<PoolEvent> = Vec::new();
    let mut fee_change_events: Vec<FeeChangeEvent> = Vec::new();

    // Check if event is coming from the pool contract
    for trx in blk.transactions() {
        for (log, _call) in trx.logs_with_calls() {
            let pool_address = Hex::encode(&log.address);
            let pool_opt = pools_store.get_last(StoreKey::pool_key(&pool_address));

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
                    );
                } else if let Some(swap_underlying) =
                    TokenExchangeUnderlying::match_and_decode(&log)
                {
                    extract_swap_underlying_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        &swap_underlying.sold_id,
                        &swap_underlying.bought_id,
                        &swap_underlying.tokens_sold,
                        &swap_underlying.tokens_bought,
                        &swap_underlying.buyer,
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
                } else if let Some(fee_change) = ApplyNewFee1::match_and_decode(&log) {
                    fee_change_events.push(FeeChangeEvent {
                        transaction_hash: Hex::encode(&trx.hash),
                        tx_index: trx.index,
                        log_index: log.index,
                        log_ordinal: log.ordinal,
                        timestamp: blk.timestamp_seconds(),
                        block_number: blk.number,
                        fee: fee_change.fee.to_string(),
                        admin_fee: None,
                        pool_address: pool.address.clone(),
                    });
                } else if let Some(fee_change) = ApplyNewFee2::match_and_decode(&log) {
                    fee_change_events.push(FeeChangeEvent {
                        transaction_hash: Hex::encode(&trx.hash),
                        tx_index: trx.index,
                        log_index: log.index,
                        log_ordinal: log.ordinal,
                        timestamp: blk.timestamp_seconds(),
                        block_number: blk.number,
                        fee: fee_change.fee.to_string(),
                        admin_fee: None,
                        pool_address: pool.address.clone(),
                    });
                } else if let Some(fee_change) = NewFee1::match_and_decode(&log) {
                    fee_change_events.push(FeeChangeEvent {
                        transaction_hash: Hex::encode(&trx.hash),
                        tx_index: trx.index,
                        log_index: log.index,
                        log_ordinal: log.ordinal,
                        timestamp: blk.timestamp_seconds(),
                        block_number: blk.number,
                        fee: fee_change.fee.to_string(),
                        admin_fee: Some(fee_change.admin_fee.to_string()),
                        pool_address: pool.address.clone(),
                    });
                } else if let Some(fee_change) = NewFee2::match_and_decode(&log) {
                    fee_change_events.push(FeeChangeEvent {
                        transaction_hash: Hex::encode(&trx.hash),
                        tx_index: trx.index,
                        log_index: log.index,
                        log_ordinal: log.ordinal,
                        timestamp: blk.timestamp_seconds(),
                        block_number: blk.number,
                        fee: fee_change.fee.to_string(),
                        admin_fee: Some(fee_change.admin_fee.to_string()),
                        pool_address: pool.address.clone(),
                    });
                } else if let Some(fee_change) = NewParameters1::match_and_decode(&log) {
                    fee_change_events.push(FeeChangeEvent {
                        transaction_hash: Hex::encode(&trx.hash),
                        tx_index: trx.index,
                        log_index: log.index,
                        log_ordinal: log.ordinal,
                        timestamp: blk.timestamp_seconds(),
                        block_number: blk.number,
                        fee: fee_change.fee.to_string(),
                        admin_fee: Some(fee_change.admin_fee.to_string()),
                        pool_address: pool.address.clone(),
                    });
                } else if let Some(_fee_change) = NewParameters2::match_and_decode(&log) {
                    let (total_fee, admin_fee) = get_pool_fee_and_admin_fee(&pool.address_vec())?;

                    fee_change_events.push(FeeChangeEvent {
                        transaction_hash: Hex::encode(&trx.hash),
                        tx_index: trx.index,
                        log_index: log.index,
                        log_ordinal: log.ordinal,
                        timestamp: blk.timestamp_seconds(),
                        block_number: blk.number,
                        fee: total_fee.to_string(),
                        admin_fee: Some(admin_fee.to_string()),
                        pool_address: pool.address.clone(),
                    });
                } else if let Some(_fee_change) = NewParameters3::match_and_decode(&log) {
                    let (total_fee, admin_fee) = get_pool_fee_and_admin_fee(&pool.address_vec())?;

                    fee_change_events.push(FeeChangeEvent {
                        transaction_hash: Hex::encode(&trx.hash),
                        tx_index: trx.index,
                        log_index: log.index,
                        log_ordinal: log.ordinal,
                        timestamp: blk.timestamp_seconds(),
                        block_number: blk.number,
                        fee: total_fee.to_string(),
                        admin_fee: Some(admin_fee.to_string()),
                        pool_address: pool.address.clone(),
                    });
                }
            }
        }
    }
    events.pool_events = pool_events;
    events.fee_changes_events = fee_change_events;
    Ok(events)
}

fn extract_swap_event(
    pool_events: &mut Vec<PoolEvent>,
    blk: &eth::Block,
    trx: &TransactionTrace,
    log: &Log,
    pool: &Pool,
    sold_id: &BigInt,
    bought_id: &BigInt,
    tokens_sold: &BigInt,
    tokens_bought: &BigInt,
    buyer: &Vec<u8>,
) {
    let pool_address = &pool.address;
    substreams::log::info!(format!(
        "Extracting Swap from transaction {} and pool {}",
        Hex::encode(&trx.hash),
        &pool_address
    ));
    let in_address_index = sold_id.to_i32().to_usize().unwrap();
    let out_address_index = bought_id.to_i32().to_usize().unwrap();

    let token_in = TokenAmount {
        token_address: pool.input_tokens_ordered[in_address_index].clone(),
        amount: tokens_sold.into(),
        source: TokenSource::Default as i32,
    };

    let token_out = TokenAmount {
        token_address: pool.input_tokens_ordered[out_address_index].clone(),
        amount: tokens_bought.into(),
        source: TokenSource::Default as i32,
    };

    let swap_event = SwapEvent {
        token_in: Some(token_in),
        token_out: Some(token_out),
    };

    pool_events.push(PoolEvent {
        transaction_hash: Hex::encode(&trx.hash),
        tx_index: trx.index,
        log_index: log.index,
        log_ordinal: log.ordinal,
        to_address: pool_address.to_string(),
        from_address: Hex::encode(buyer),
        timestamp: blk.timestamp_seconds(),
        block_number: blk.number,
        pool_address: pool_address.to_string(),
        r#type: Some(Type::SwapEvent(swap_event)),
    })
}

fn extract_swap_underlying_event(
    pool_events: &mut Vec<PoolEvent>,
    blk: &eth::Block,
    trx: &TransactionTrace,
    log: &Log,
    pool: &Pool,
    sold_id: &BigInt,
    bought_id: &BigInt,
    tokens_sold: &BigInt,
    tokens_bought: &BigInt,
    buyer: &Vec<u8>,
) {
    if is_metapool(pool) {
        // Check if the pool is a metapool and retrieve the base pool's address.
        let base_pool_address = match &pool.pool_type {
            Some(PoolType::MetaPool(meta)) => meta.base_pool_address.clone(),
            _ => {
                substreams::log::debug!(
                    "Pool is not a metapool, skipping `TokenExchangeUnderlying` event processing."
                );
                return;
            }
        };

        if base_pool_address.is_empty() {
            substreams::log::debug!("Base pool address not found for metapool.");
            return;
        }

        substreams::log::info!(
            "Extracting Swap Underlying from transaction {} and pool {}",
            Hex::encode(&trx.hash),
            &pool.address
        );

        // Determine the addresses and sources of the input and output tokens
        // based on their indices and the type of the pool.
        let (token_in, token_out) = determine_underlying_exchange_tokens(
            &pool,
            sold_id.to_i32(),
            bought_id.to_i32(),
            tokens_sold,
            tokens_bought,
        );

        // Determine if there's a change in the metapools base pool LP token balance. This could be a burn
        // or mint operation depending on the swap direction (metapool to base pool or vice versa).
        let base_pool_lp_token_address = pool.input_tokens.get(1).unwrap().address_vec();
        let lp_token_change = if token_in.source() == TokenSource::MetaPool
            && token_out.source() == TokenSource::BasePool
        {
            extract_lp_token_change(trx, pool, &base_pool_lp_token_address, true)
        } else if token_in.source() == TokenSource::BasePool
            && token_out.source() == TokenSource::MetaPool
        {
            extract_lp_token_change(trx, pool, &base_pool_lp_token_address, false)
        } else {
            None
        };

        let swap_underlying_event = SwapUnderlyingMetaEvent {
            token_in: Some(token_in),
            token_out: Some(token_out),
            lp_token_change: lp_token_change,
            base_pool_address,
        };

        pool_events.push(PoolEvent {
            transaction_hash: Hex::encode(&trx.hash),
            tx_index: trx.index,
            log_index: log.index,
            log_ordinal: log.ordinal,
            to_address: pool.address.to_string(),
            from_address: Hex::encode(buyer),
            timestamp: blk.timestamp_seconds(),
            block_number: blk.number,
            pool_address: pool.address.to_string(),
            r#type: Some(Type::SwapUnderlyingMetaEvent(swap_underlying_event)),
        })
    } else if let Some(PoolType::LendingPool(lending_pool)) = &pool.pool_type {
        let token_in_address = lending_pool
            .underlying_tokens
            .get(sold_id.to_i32() as usize)
            .map_or_else(
                || {
                    substreams::log::info!(
                        "Sold token index {} is out of bounds for LendingPool",
                        sold_id
                    );
                    None
                },
                |token| Some(token.address.clone()),
            );

        let token_out_address = lending_pool
            .underlying_tokens
            .get(bought_id.to_i32() as usize)
            .map_or_else(
                || {
                    substreams::log::info!(
                        "Bought token index {} is out of bounds for LendingPool",
                        bought_id
                    );
                    None
                },
                |token| Some(token.address.clone()),
            );

        if let (Some(in_address), Some(out_address)) = (token_in_address, token_out_address) {
            let token_in = TokenAmount {
                token_address: in_address,
                amount: tokens_sold.into(),
                source: TokenSource::LendingPool as i32,
            };

            let token_out = TokenAmount {
                token_address: out_address,
                amount: tokens_bought.into(),
                source: TokenSource::LendingPool as i32,
            };

            let interest_token_in_address =
                match Hex::decode(&pool.input_tokens_ordered[sold_id.to_i32() as usize]) {
                    Ok(address) => address,
                    Err(e) => {
                        substreams::log::info!(
                            "Failed to decode interest bearing token in address for sold_id {}: {}",
                            sold_id,
                            e
                        );
                        return; // Exit from the function, skipping this event
                    }
                };

            let interest_token_out_address =
                match Hex::decode(&pool.input_tokens_ordered[bought_id.to_i32() as usize]) {
                    Ok(address) => address,
                    Err(e) => {
                        substreams::log::info!(
                        "Failed to decode interest bearing token out address for bought_id {}: {}",
                        bought_id,
                        e
                    );
                        return; // Exit from the function, skipping this event
                    }
                };

            // Token in will have a corresponding mint event
            let interest_token_in_change =
                extract_lending_pool_token_change(trx, pool, &interest_token_in_address, false);

            // Token out will have a corresponding burn event
            let interest_token_out_change =
                extract_lending_pool_token_change(trx, pool, &interest_token_out_address, true);

            let swap_underlying_event = SwapUnderlyingLendingEvent {
                token_in: Some(token_in),
                token_out: Some(token_out),
                interest_bearing_token_in_action: interest_token_in_change,
                interest_bearing_token_out_action: interest_token_out_change,
            };

            pool_events.push(PoolEvent {
                transaction_hash: Hex::encode(&trx.hash),
                tx_index: trx.index,
                log_index: log.index,
                log_ordinal: log.ordinal,
                to_address: pool.address.to_string(),
                from_address: Hex::encode(buyer),
                timestamp: blk.timestamp_seconds(),
                block_number: blk.number,
                pool_address: pool.address.to_string(),
                r#type: Some(Type::SwapUnderlyingLendingEvent(swap_underlying_event)),
            })
        }
    }
}

fn determine_underlying_exchange_tokens(
    pool: &Pool,
    sold_id: i32,
    bought_id: i32,
    tokens_sold: &BigInt,
    tokens_bought: &BigInt,
) -> (TokenAmount, TokenAmount) {
    let get_token_info = |id, pool_type: &PoolType| -> (String, TokenSource) {
        if id == 0 {
            (pool.input_tokens[0].address.clone(), TokenSource::MetaPool)
        } else {
            let metapool = match pool_type {
                PoolType::MetaPool(meta) => meta,
                _ => panic!("Unsupported pool type or metapool information not available."),
            };

            let address = metapool.underlying_tokens
                [(id - metapool.max_coin.to_i32().unwrap()) as usize]
                .address
                .clone();
            (address, TokenSource::BasePool)
        }
    };

    let pool_type = pool.pool_type.as_ref().expect("Pool type must be set.");
    let (token_in_address, token_in_source) = get_token_info(sold_id, pool_type);
    let (token_out_address, token_out_source) = get_token_info(bought_id, pool_type);

    let token_in = TokenAmount {
        token_address: token_in_address,
        amount: tokens_sold.to_string(),
        source: token_in_source as i32,
    };

    let token_out = TokenAmount {
        token_address: token_out_address,
        amount: tokens_bought.to_string(),
        source: token_out_source as i32,
    };

    (token_in, token_out)
}

fn extract_lp_token_change(
    trx: &TransactionTrace,
    pool: &Pool,
    lp_token_address: &Vec<u8>,
    is_burn: bool,
) -> Option<LpTokenChange> {
    let pool_address = pool.address_vec();
    let null_address = NULL_ADDRESS.to_vec();

    let from: &Vec<u8>;
    let to: &Vec<u8>;

    // Determine the 'from' and 'to' addresses based on whether it's a burn or mint event.
    if is_burn {
        from = &pool_address;
        to = &null_address;
    } else {
        from = &null_address;
        to = &pool_address;
    }

    // Attempt to extract the specific transfer event for the LP token.
    match event_extraction::extract_specific_transfer_event(
        trx,
        Some(lp_token_address),
        Some(from),
        Some(to),
    ) {
        Ok(transfer) => Some(LpTokenChange {
            token_address: Hex::encode(transfer.token_address),
            amount: transfer.transfer.value.into(),
            change_type: if is_burn {
                LpTokenChangeType::Burn
            } else {
                LpTokenChangeType::Mint
            } as i32,
        }),
        Err(e) => {
            substreams::log::debug!("Error extracting LP token change: {:?}", e);
            None
        }
    }
}

// TODO this is the same as above, could refactor so we use one
fn extract_lending_pool_token_change(
    trx: &TransactionTrace,
    pool: &Pool,
    token_address: &Vec<u8>,
    is_burn: bool,
) -> Option<LpTokenChange> {
    let pool_address = pool.address_vec();
    let null_address = NULL_ADDRESS.to_vec();

    substreams::log::debug!("pool address is: {:?}", Hex::encode(&pool_address));
    substreams::log::debug!("token address is: {:?}", Hex::encode(&token_address));

    let from: &Vec<u8>;
    let to: &Vec<u8>;

    // Determine the 'from' and 'to' addresses based on whether it's a burn or mint event.
    if is_burn {
        from = &pool_address;
        to = &null_address;
    } else {
        from = &null_address;
        to = &pool_address;
    }

    // Attempt to extract the specific transfer event for the LP token.
    match event_extraction::extract_specific_transfer_event(
        trx,
        Some(token_address),
        Some(from),
        Some(to),
    ) {
        Ok(transfer) => Some(LpTokenChange {
            token_address: Hex::encode(transfer.token_address),
            amount: transfer.transfer.value.into(),
            change_type: if is_burn {
                LpTokenChangeType::Burn
            } else {
                LpTokenChangeType::Mint
            } as i32,
        }),
        Err(e) => {
            substreams::log::debug!("Error extracting LP token change: {:?}", e);
            None
        }
    }
}

fn extract_deposit_event(
    pool_events: &mut Vec<PoolEvent>,
    blk: &eth::Block,
    trx: &TransactionTrace,
    log: &Log,
    pool: &Pool,
    token_amounts: Vec<BigInt>,
    fees: Vec<String>,
    provider: Vec<u8>,
) {
    substreams::log::info!(format!(
        "Extracting Deposit from transaction {} and pool {}",
        Hex::encode(&trx.hash),
        &pool.address
    ));
    let pool_address = match Hex::decode(&pool.output_token_ref().address.clone()) {
        Ok(address) => address,
        Err(e) => {
            substreams::log::debug!("Error in `extract_deposit_event`: {:?}", e);
            return;
        }
    };
    let input_tokens = token_amounts
        .iter()
        .enumerate()
        .map(|(i, amount)| TokenAmount {
            token_address: pool.input_tokens_ordered[i].clone(),
            amount: amount.into(),
            source: TokenSource::Default as i32,
        })
        .collect();

    let output_token_amount = event_extraction::extract_specific_transfer_event(
        &trx,
        Some(&pool_address),
        Some(&NULL_ADDRESS.to_vec()),
        Some(&provider),
    )
    .or_else(|_| {
        // If finding a `Transfer` event with the provider as `to` fails, it may involve a Deposit Zap contract.
        // In such cases, LP tokens are sent to the user interacting with the Deposit Zap, not the contract itself.
        // Hence, we retry without specifying the `to` address, though the initial attempt aims for precision when possible.
        event_extraction::extract_specific_transfer_event(
            &trx,
            Some(&pool_address),
            Some(&NULL_ADDRESS.to_vec()),
            None,
        )
    })
    .map(|transfer| transfer.transfer.value)
    .unwrap_or_else(|e| {
        substreams::log::debug!("Error in `map_extract_pool_events`: {:?}", e);
        BigInt::zero()
    });

    let deposit_event = DepositEvent {
        input_tokens,
        output_token: Some(TokenAmount {
            token_address: pool.output_token_ref().address.clone(),
            amount: output_token_amount.into(),
            source: TokenSource::Default as i32,
        }),
        fees,
    };
    pool_events.push(PoolEvent {
        transaction_hash: Hex::encode(&trx.hash),
        tx_index: trx.index,
        log_index: log.index,
        log_ordinal: log.ordinal,
        to_address: pool.address.clone(),
        from_address: Hex::encode(provider),
        timestamp: blk.timestamp_seconds(),
        block_number: blk.number,
        pool_address: pool.address.clone(),
        r#type: Some(Type::DepositEvent(deposit_event)),
    })
}

// Multiple Token Withdrawl
fn extract_withdraw_event(
    pool_events: &mut Vec<PoolEvent>,
    blk: &eth::Block,
    trx: &TransactionTrace,
    log: &Log,
    pool: &Pool,
    provider: Vec<u8>,
    token_amounts: Vec<BigInt>,
    fees: Vec<String>,
) {
    substreams::log::info!(format!(
        "Extracting Withdrawal from transaction {} and pool {}",
        Hex::encode(&trx.hash),
        &pool.address
    ));
    let pool_address = match Hex::decode(&pool.output_token_ref().address.clone()) {
        Ok(address) => address,
        Err(e) => {
            substreams::log::debug!("Error in `extract_withdraw_event`: {:?}", e);
            return;
        }
    };
    let input_tokens: Vec<TokenAmount> = token_amounts
        .iter()
        .enumerate()
        .map(|(i, amount)| TokenAmount {
            token_address: pool.input_tokens_ordered[i].clone(),
            amount: amount.into(),
            source: TokenSource::Default as i32,
        })
        .collect();
    let output_token_amount = match event_extraction::extract_specific_transfer_event(
        &trx,
        Some(&pool_address),
        Some(&provider),
        Some(&NULL_ADDRESS.to_vec()),
    ) {
        Ok(burn_transfer) => burn_transfer.transfer.value,
        Err(e) => {
            substreams::log::debug!("Error in `map_extract_pool_events`: {:?}", e);
            BigInt::zero()
        }
    };
    let withdraw_event = WithdrawEvent {
        input_tokens,
        output_token: Some(TokenAmount {
            token_address: pool.output_token_ref().address.clone(),
            amount: output_token_amount.into(),
            source: TokenSource::Default as i32,
        }),
        fees,
    };
    pool_events.push(PoolEvent {
        transaction_hash: Hex::encode(&trx.hash),
        tx_index: trx.index,
        log_index: log.index,
        log_ordinal: log.ordinal,
        to_address: pool.address.clone(),
        from_address: Hex::encode(provider),
        timestamp: blk.timestamp_seconds(),
        block_number: blk.number,
        pool_address: pool.address.clone(),
        r#type: Some(Type::WithdrawEvent(withdraw_event)),
    })
}

// Single Token Withdrawal
fn extract_withdraw_one_event(
    pool_events: &mut Vec<PoolEvent>,
    blk: &eth::Block,
    trx: &TransactionTrace,
    log: &Log,
    pool: &Pool,
    provider: Vec<u8>,
    token_amount: BigInt,
    coin_amount: BigInt,
) {
    let pool_address = &pool.address;
    substreams::log::info!(format!(
        "Extracting Withdraw from transaction {} and pool {}",
        Hex::encode(&trx.hash),
        pool_address
    ));

    let token_transfer_log = trx
        .calls
        .iter()
        .filter(|call| !call.state_reverted)
        .flat_map(|call| call.logs.iter())
        .find_map(|log| {
            // Directly return the result of the match_and_decode if the conditions are met
            if let Some(transfer) = Transfer::match_and_decode(log) {
                if transfer.sender == *log.address && transfer.receiver == provider {
                    return Some(log);
                }
            }
            None
        });

    let input_tokens = pool
        .input_tokens_ordered
        .iter()
        .map(|address| {
            if token_transfer_log.is_some()
                && &Hex::encode(&token_transfer_log.unwrap().address) == address
            {
                TokenAmount {
                    token_address: address.clone(),
                    amount: coin_amount.clone().into(),
                    source: TokenSource::Default as i32,
                }
            } else {
                TokenAmount {
                    token_address: address.clone(),
                    amount: BigInt::zero().into(),
                    source: TokenSource::Default as i32,
                }
            }
        })
        .collect();

    let withdraw_event = WithdrawEvent {
        input_tokens,
        output_token: Some(TokenAmount {
            token_address: pool.output_token_ref().address.clone(),
            amount: token_amount.into(),
            source: TokenSource::Default as i32,
        }),
        fees: Vec::new(),
    };

    pool_events.push(PoolEvent {
        transaction_hash: Hex::encode(&trx.hash),
        tx_index: trx.index,
        log_index: log.index,
        log_ordinal: log.ordinal,
        to_address: pool_address.to_string(),
        from_address: Hex::encode(provider),
        timestamp: blk.timestamp_seconds(),
        block_number: blk.number,
        pool_address: pool_address.to_string(),
        r#type: Some(Type::WithdrawEvent(withdraw_event)),
    })
}
