use anyhow::anyhow;
use substreams::{errors::Error, Hex};
use substreams_ethereum::{
    block_view,
    pb::eth::v2::{self as eth},
};

use crate::{
    abi::erc20::events::Transfer,
    network_config,
    pb::curve::types::v1::{Pool, Token},
    rpc,
};

pub fn format_address(address: &Vec<u8>) -> String {
    format!("0x{}", Hex::encode(address))
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
        is_single_sided: false,
        //       Could also extract this into `graph-out` module eventually.
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
