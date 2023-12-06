use anyhow::anyhow;
use num_traits::ToPrimitive;
use substreams::{errors::Error, scalar::BigInt, Hex};
use substreams_ethereum::{
    block_view,
    pb::eth::v2::{self as eth, Log, TransactionTrace},
    NULL_ADDRESS,
};

use crate::{
    abi::erc20::events::Transfer,
    constants::network,
    network_config,
    pb::curve::types::v1::{
        events::{
            pool_event::{SwapEvent, TokenAmount, Type},
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

pub fn format_address_string(address: &String) -> String {
    format!("0x{}", address)
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
        // TODO: We may be able to remove this if it is also false.
        //       This can just be set when creating the relevant subgraph entity.
        //       Will keep here for now as a reminder.
        //       Make sure to do the same in missing pool version of function.
        is_single_sided: false,
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
        is_single_sided: false,
        is_metapool,
    }
}

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
    pool_address: &String,
    sold_id: &BigInt,
    bought_id: &BigInt,
    tokens_sold: &BigInt,
    tokens_bought: &BigInt,
    buyer: &Vec<u8>,
    with_underlying: bool,
) {
    substreams::log::info!(format!(
        "Extracting Swap from transaction {} and pool {}",
        Hex::encode(&trx.hash),
        &pool_address
    ));
    let in_address_index = sold_id.to_i32().to_usize().unwrap();
    let out_address_index = bought_id.to_i32().to_usize().unwrap();

    let (token_in_address, token_out_address) = if with_underlying {
        match get_underlying_coin_addresses(
            &pool,
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
        substreams::log::debug!(format!("Length is {}.", &pool.input_tokens_ordered.len()));
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
        log_index: log.index as u64,
        log_ordinal: log.ordinal,
        to_address: pool_address.clone(),
        from_address: Hex::encode(buyer),
        timestamp: blk.timestamp_seconds(),
        block_number: blk.number,
        pool_address: pool_address.clone(),
        r#type: Some(Type::SwapEvent(swap_event)),
    })
}

pub fn get_underlying_coin_addresses(
    pool: &Pool,
    pool_address: &String,
    in_index: usize,
    out_index: usize,
    bought_id: &BigInt,
) -> Result<(String, String), Error> {
    let registry_address = pool.registry_address.clone().into_bytes();
    let underlying_coins = if registry_address == NULL_ADDRESS.to_vec() {
        get_pool_underlying_coins(&registry_address)
    } else {
        get_pool_underlying_coins_from_registry(
            &pool_address.clone().into_bytes(),
            &registry_address,
        )
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
                Err(anyhow!("Error in `get_underlying_coin_addresses`: No underlying coins found for pool {}.", &pool_address))
            }
        }
        Err(e) => Err(anyhow!("Error in `get_underlying_coin_addresses`: {:?}", e)),
    }
}
