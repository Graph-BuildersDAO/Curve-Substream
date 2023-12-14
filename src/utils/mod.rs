use anyhow::anyhow;
use num_traits::ToPrimitive;
use substreams::{
    errors::Error,
    scalar::{BigDecimal, BigInt},
    Hex,
};
use substreams_ethereum::{
    block_view,
    pb::eth::v2::{self as eth, Log, TransactionTrace},
    Event, NULL_ADDRESS,
};

use crate::{
    abi::erc20::events::Transfer,
    constants::network,
    network_config::{self, PROTOCOL_ADDRESS},
    pb::curve::types::v1::{
        events::{
            pool_event::{DepositEvent, SwapEvent, TokenAmount, Type, WithdrawEvent},
            PoolEvent,
        },
        Pool, Token,
    },
    rpc::{
        self, pool::get_pool_underlying_coins, registry::get_pool_underlying_coins_from_registry,
    },
};

pub fn format_address_vec(address: &Vec<u8>) -> String {
    format!("0x{}", Hex::encode(address))
}

pub fn format_address_string(address: &str) -> String {
    format!("0x{}", address)
}

pub fn get_protocol_id() -> String {
    format_address_vec(&PROTOCOL_ADDRESS.to_vec())
}

pub fn convert_enum_to_snake_case_prefix(snake_me: &str) -> String {
    snake_me.to_lowercase().replace("_", "-") + "-"
}

pub fn convert_bigint_to_decimal(value: &BigInt, denominator: u64) -> Result<BigDecimal, Error> {
    if *value == BigInt::from(0) {
        Ok(BigDecimal::from(0))
    } else {
        Ok(BigDecimal::from(value.clone()) / BigDecimal::from(denominator))
    }
}

pub fn is_base_pool_lp_token(lp_token_address: &Vec<u8>) -> bool {
    network_config::BASE_POOLS_LP_TOKEN
        .iter()
        .any(|&token_address| token_address.as_ref() == lp_token_address.as_slice())
}

pub fn is_metapool(pool_address: &Vec<u8>) -> bool {
    network_config::HARDCODED_METAPOOLS
        .iter()
        .any(|&token_address| token_address.as_ref() == pool_address.as_slice())
}

// This follows the logic from the original subgraph.
// An array of token addresses, and a sorted array of token structs is required.
pub fn get_and_sort_input_tokens(
    pool_address: &Vec<u8>,
) -> Result<(Vec<Token>, Vec<String>), Error> {
    let mut input_tokens = rpc::pool::get_pool_coins(&pool_address)?;
    let input_tokens_ordered = input_tokens
        .clone()
        .into_iter()
        .map(|token| token.address)
        .collect();
    input_tokens.sort_by(|a, b| a.address.cmp(&b.address));

    Ok((input_tokens, input_tokens_ordered))
}

pub fn create_pool(
    address: String,
    registry_address: String,
    lp_token: Token,
    input_tokens_ordered: Vec<String>,
    input_tokens: Vec<Token>,
    is_metapool: bool,
    log: &block_view::LogView,
    blk: &eth::Block,
) -> Pool {
    Pool {
        address,
        name: lp_token.name.clone(),
        symbol: lp_token.symbol.clone(),
        created_at_timestamp: blk.timestamp_seconds(),
        created_at_block_number: blk.number,
        log_ordinal: log.ordinal(),
        transaction_id: Hex(&log.receipt.transaction.hash).to_string(),
        registry_address,
        output_token: Some(lp_token),
        input_tokens_ordered,
        input_tokens,
        //       Could also extract this into `graph-out` module eventually.
        is_metapool,
    }
}

pub fn create_missing_pool(
    address: String,
    registry_address: String,
    lp_token: Token,
    input_tokens_ordered: Vec<String>,
    input_tokens: Vec<Token>,
    is_metapool: bool,
    blk: &eth::Block,
    hash: Vec<u8>,
) -> Pool {
    Pool {
        address,
        name: lp_token.name.clone(),
        symbol: lp_token.symbol.clone(),
        created_at_timestamp: blk.timestamp_seconds(),
        created_at_block_number: blk.number,
        log_ordinal: 0,
        transaction_id: Hex::encode(hash),
        registry_address,
        output_token: Some(lp_token),
        input_tokens_ordered,
        input_tokens,
        is_metapool,
    }
}

pub fn extract_specific_transfer_event(
    trx: &TransactionTrace,
    from: &Vec<u8>,
    to: &Vec<u8>,
) -> Result<Transfer, Error> {
    trx.calls
        .iter()
        .filter(|call| !call.state_reverted)
        .flat_map(|call| call.logs.iter())
        .find_map(|log| {
            // Directly return the result of the match_and_decode if the conditions are met
            Transfer::match_and_decode(log).and_then(|transfer| {
                if transfer.sender == *from && transfer.receiver == *to {
                    Some(transfer)
                } else {
                    None
                }
            })
        })
        .ok_or_else(|| anyhow!("No transfer event found"))
}

// Only use for MetaPool and PlainPool deployments where we can ensure there is only one Transfer event.
pub fn extract_transfer_event(log: &block_view::LogView) -> Result<Transfer, Error> {
    log.receipt
        .transaction
        .calls
        .iter()
        .filter(|call| !call.state_reverted)
        .flat_map(|call| call.logs.iter())
        .find(|log| Transfer::match_log(log))
        .ok_or_else(|| anyhow!("No transfer event found in the transaction"))
        .and_then(|log| Transfer::decode(log).map_err(|e| anyhow!(e)))
}

pub fn extract_swap_event(
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
                substreams::log::debug!(format!("Error in `extract_swap_event`: {:?}", e));
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

pub fn extract_deposit_event(
    pool_events: &mut Vec<PoolEvent>,
    blk: &eth::Block,
    trx: &TransactionTrace,
    log: &Log,
    pool: &Pool,
    token_amounts: Vec<BigInt>,
    fees: Vec<String>,
    provider: Vec<u8>,
) {
    let pool_address = &pool.address;
    substreams::log::info!(format!(
        "Extracting Deposit from transaction {} and pool {}",
        Hex::encode(&trx.hash),
        pool_address
    ));

    let input_tokens = token_amounts
        .iter()
        .enumerate()
        .map(|(i, amount)| TokenAmount {
            token_address: pool.input_tokens_ordered[i].clone(),
            amount: amount.into(),
        })
        .collect();

    let output_token_transfer =
        extract_specific_transfer_event(&trx, &NULL_ADDRESS.to_vec(), &provider);

    // This is the amount of output token (LP token) transferred to the liqudiity provider
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
        to_address: pool_address.to_string(),
        from_address: Hex::encode(provider),
        timestamp: blk.timestamp_seconds(),
        block_number: blk.number,
        pool_address: pool_address.to_string(),
        r#type: Some(Type::DepositEvent(deposit_event)),
    })
}

// Multiple Token Withdrawl
pub fn extract_withdraw_event(
    pool_events: &mut Vec<PoolEvent>,
    blk: &eth::Block,
    trx: &TransactionTrace,
    log: &Log,
    pool: &Pool,
    provider: Vec<u8>,
    token_amounts: Vec<BigInt>,
    fees: Vec<String>,
) {
    let pool_address = &pool.address;
    substreams::log::info!(format!(
        "Extracting Withdrawal from transaction {} and pool {}",
        Hex::encode(&trx.hash),
        pool_address
    ));

    let input_tokens: Vec<TokenAmount> = token_amounts
        .iter()
        .enumerate()
        .map(|(i, amount)| TokenAmount {
            token_address: pool.input_tokens_ordered[i].clone(),
            amount: amount.into(),
        })
        .collect();
    let output_token_amount =
        match extract_specific_transfer_event(&trx, &provider, &NULL_ADDRESS.to_vec()) {
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
        to_address: pool_address.to_string(),
        from_address: Hex::encode(provider),
        timestamp: blk.timestamp_seconds(),
        block_number: blk.number,
        pool_address: pool_address.to_string(),
        r#type: Some(Type::WithdrawEvent(withdraw_event)),
    })
}

// Single Token Withdrawal
pub fn extract_withdraw_one_event(
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

pub fn get_underlying_coin_addresses(
    pool: &Pool,
    pool_address: &str,
    in_index: usize,
    out_index: usize,
    bought_id: &BigInt,
) -> Result<(String, String), Error> {
    let registry_address = Hex::decode(&pool.registry_address).unwrap();
    let pool_address = Hex::decode(pool_address).unwrap();

    let underlying_coins = if registry_address == NULL_ADDRESS.to_vec() {
        get_pool_underlying_coins(&registry_address)
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
                    String::from_utf8(coins[in_index].clone()).unwrap(),
                    String::from_utf8(coins[out_index].clone()).unwrap(),
                ))
            } else {
                Err(anyhow!("Error in `get_underlying_coin_addresses`: No underlying coins found for pool {}.", Hex::encode(&pool_address)))
            }
        }
        Err(e) => Err(anyhow!("Error in `get_underlying_coin_addresses`: {:?}", e)),
    }
}
