use anyhow::anyhow;
use num_traits::ToPrimitive;
use substreams::{
    errors::Error,
    scalar::BigInt,
    store::{StoreGet, StoreGetProto},
    Hex,
};
use substreams_ethereum::{
    pb::eth::v2::{self as eth, Log, TransactionTrace},
    Event, NULL_ADDRESS,
};

use crate::{
    abi::{
        erc20::events::Transfer,
        pool::events::{
            AddLiquidity1, AddLiquidity2, AddLiquidity3, AddLiquidity4, AddLiquidity5,
            RemoveLiquidity1, RemoveLiquidity2, RemoveLiquidity3, RemoveLiquidity4,
            RemoveLiquidity5, RemoveLiquidityImbalance1, RemoveLiquidityImbalance2,
            RemoveLiquidityImbalance3, RemoveLiquidityOne1, RemoveLiquidityOne2, TokenExchange1,
            TokenExchange2, TokenExchangeUnderlying,
        },
    },
    common::event_extraction,
    constants::network,
    network_config,
    pb::curve::types::v1::{
        events::{
            pool_event::{DepositEvent, SwapEvent, TokenAmount, Type, WithdrawEvent},
            PoolEvent,
        },
        Events, Pool,
    },
    rpc::{pool::get_pool_underlying_coins, registry::get_pool_underlying_coins_from_registry},
    store_key_manager::StoreKey,
};

#[substreams::handlers::map]
pub fn map_extract_pool_events(
    blk: eth::Block,
    pools_store: StoreGetProto<Pool>,
) -> Result<Events, Error> {
    // Initialise events and its fields
    let mut events = Events::default();
    let mut pool_events: Vec<PoolEvent> = Vec::new();

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
    with_underlying: bool,
) {
    let pool_address = &pool.address;
    substreams::log::info!(format!(
        "Extracting Swap from transaction {} and pool {}",
        Hex::encode(&trx.hash),
        &pool_address
    ));
    let in_address_index = sold_id.to_i32().to_usize().unwrap();
    let out_address_index = bought_id.to_i32().to_usize().unwrap();

    let (token_in_address, token_out_address) = if with_underlying {
        match get_underlying_coin_addresses(
            pool,
            &pool_address,
            in_address_index,
            out_address_index,
            bought_id,
        ) {
            Ok((in_addr, out_addr)) => (in_addr, out_addr),
            Err(e) => {
                substreams::log::debug!("Error in `extract_swap_event`: {:?}", e);
                return;
            }
        }
    } else {
        (
            pool.input_tokens_ordered[in_address_index].clone(),
            pool.input_tokens_ordered[out_address_index].clone(),
        )
    };
    let token_in = TokenAmount {
        token_address: token_in_address,
        amount: tokens_sold.into(),
    };

    let token_out = TokenAmount {
        token_address: token_out_address,
        amount: tokens_bought.into(),
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
        })
        .collect();
    let output_token_transfer = event_extraction::extract_specific_transfer_event(
        &trx,
        &pool_address,
        &NULL_ADDRESS.to_vec(),
        &provider,
    );
    // This is the amount of output token (LP token) transferred to the liquidity provider
    let output_token_amount = match output_token_transfer {
        Ok(transfer) => transfer.value,
        Err(e) => {
            substreams::log::debug!("Error in `map_extract_pool_events`: {:?}", e);
            BigInt::zero()
        }
    };
    let deposit_event = DepositEvent {
        input_tokens,
        output_token: Some(TokenAmount {
            token_address: pool.output_token_ref().address.clone(),
            amount: output_token_amount.into(),
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
        })
        .collect();
    let output_token_amount = match event_extraction::extract_specific_transfer_event(
        &trx,
        &pool_address,
        &provider,
        &NULL_ADDRESS.to_vec(),
    ) {
        Ok(burn_transfer) => burn_transfer.value,
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
                }
            } else {
                TokenAmount {
                    token_address: address.clone(),
                    amount: BigInt::zero().into(),
                }
            }
        })
        .collect();

    let withdraw_event = WithdrawEvent {
        input_tokens,
        output_token: Some(TokenAmount {
            token_address: pool.output_token_ref().address.clone(),
            amount: token_amount.into(),
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

fn get_underlying_coin_addresses(
    pool: &Pool,
    pool_address: &str,
    in_index: usize,
    out_index: usize,
    bought_id: &BigInt,
) -> Result<(String, String), Error> {
    let registry_address = Hex::decode(&pool.registry_address).unwrap();
    let pool_address = Hex::decode(pool_address).unwrap();
    let underlying_coins = if registry_address == NULL_ADDRESS.to_vec() {
        get_pool_underlying_coins(&pool_address)
    } else {
        get_pool_underlying_coins_from_registry(&pool_address, &registry_address)
    };
    match underlying_coins {
        Ok(coins) => {
            if !coins.is_empty() {
                // Shadowing as we need to mutate the value if it meets below conditions
                let mut in_index = in_index;
                // Same logic as the original subgraph
                if pool.is_metapool
                    && bought_id.clone() == BigInt::zero()
                    && (network_config::NETWORK.to_lowercase() == network::MAINNET.to_lowercase()
                        || network_config::NETWORK.to_lowercase() == network::FANTOM.to_lowercase()
                        || network_config::NETWORK.to_lowercase() == network::MATIC.to_lowercase()
                        || network_config::NETWORK.to_lowercase()
                            == network::ARBITRUM_ONE.to_lowercase())
                {
                    in_index = coins.len() - 1;
                }
                Ok((
                    Hex::encode(&coins[in_index]),
                    Hex::encode(&coins[out_index]),
                ))
            } else {
                Err(anyhow!("Error in `get_underlying_coin_addresses`: No underlying coins found for pool {}.", Hex::encode(&pool_address)))
            }
        }
        Err(e) => Err(anyhow!("Error in `get_underlying_coin_addresses`: {:?}", e)),
    }
}
