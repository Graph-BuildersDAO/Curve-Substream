// Generic Event Extraction that is not specific to any module.

use anyhow::anyhow;
use substreams::errors::Error;
use substreams_ethereum::{block_view, pb::eth::v2::TransactionTrace, Event, NULL_ADDRESS};

use crate::{
    abi::{
        common::erc20::events::Transfer,
        curve::pool::events::{RemoveLiquidityOne1, RemoveLiquidityOne2},
    },
    types::transfer::{RemoveLiquidityTransfer, TokenTransfer},
};

pub fn extract_specific_transfer_event(
    trx: &TransactionTrace,
    log_address: Option<&Vec<u8>>,
    from: Option<&Vec<u8>>,
    to: Option<&Vec<u8>>,
) -> Result<TokenTransfer, Error> {
    // Count the number of Some values among the parameters
    let provided_params_count = [log_address, from, to]
        .iter()
        .filter(|opt| opt.is_some())
        .count();

    // Ensure at least two params are provided. Allowing only one param is too permissive.
    if provided_params_count < 2 {
        return Err(anyhow!(
            "At least two of 'log_address', 'from', or 'to' must be specified"
        ));
    }

    trx.calls
        .iter()
        .filter(|call| !call.state_reverted)
        .flat_map(|call| call.logs.iter())
        .find_map(|log| {
            // Check if `log_address` is provided and matches, or if it's None
            let address_match = log_address.map_or(true, |addr| &log.address == addr);

            Transfer::match_and_decode(log).and_then(|transfer| {
                // If 'from' is None, ignore receiver check; otherwise, check if it matches
                let from_match = from.map_or(true, |from_addr| transfer.sender == *from_addr);

                // If 'to' is None, ignore receiver check; otherwise, check if it matches
                let to_match = to.map_or(true, |to_addr| transfer.receiver == *to_addr);

                // If all the params match a Transfer event in the transactions logs, return it.
                if address_match && from_match && to_match {
                    Some(TokenTransfer::new(transfer, log.address.clone()))
                } else {
                    None
                }
            })
        })
        .ok_or_else(|| anyhow!("No transfer event found"))
}

// TODO refactor/optimise
// Only use for MetaPool and PlainPool deployments where we can ensure there is only one Transfer event.
pub fn extract_pool_creation_transfer_event(
    log: &block_view::LogView,
) -> Result<TokenTransfer, Error> {
    log.receipt
        .transaction
        .calls
        .iter()
        .filter(|call| !call.state_reverted)
        .flat_map(|call| call.logs.iter())
        .find_map(|log| {
            if Transfer::match_log(log) {
                // Attempt to decode the log and pair it with the log reference if successful
                match Transfer::decode(log) {
                    Ok(transfer) => {
                        if &transfer.sender == &NULL_ADDRESS.to_vec()
                            && &transfer.receiver == &log.address
                        {
                            // Pair the decoded transfer with the log
                            Some((transfer, log))
                        } else {
                            if &transfer.sender == &NULL_ADDRESS.to_vec() {
                                Some((transfer, log))
                            } else {
                                None
                            }
                        }
                    }
                    Err(_) => None, // Ignore this log if decoding fails
                }
            } else {
                None // Not a transfer log, so we skip it
            }
        })
        .ok_or_else(|| anyhow!("No transfer event found in the transaction"))
        // Here the log is the one associated with the found transfer event
        .and_then(|(transfer, transfer_log)| {
            Ok(TokenTransfer::new(transfer, transfer_log.address.to_vec()))
        })
}

pub fn extract_remove_liquidity_one_event(
    trx: &TransactionTrace,
) -> Result<RemoveLiquidityTransfer, Error> {
    let transfer_event = trx
        .calls
        .iter()
        .filter(|call| !call.state_reverted)
        .flat_map(|call| call.logs.iter())
        .find_map(|log| {
            if RemoveLiquidityOne1::match_log(log) {
                RemoveLiquidityOne1::decode(log).ok().and_then(|ev| {
                    Some(RemoveLiquidityTransfer::new(
                        log.address.clone(),
                        ev.provider,
                        ev.token_amount,
                        ev.coin_amount,
                    ))
                })
            } else if RemoveLiquidityOne2::match_log(log) {
                RemoveLiquidityOne2::decode(log).ok().and_then(|ev| {
                    Some(RemoveLiquidityTransfer::new(
                        log.address.clone(),
                        ev.provider,
                        ev.token_amount,
                        ev.coin_amount,
                    ))
                })
            } else {
                None
            }
        });
    transfer_event.ok_or_else(|| {
        anyhow!("No RemoveLiquidityOne1 or RemoveLiquidityOne2 event found in the transaction")
    })
}
