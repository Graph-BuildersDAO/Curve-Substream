// Generic Event Extraction that is not specific to any module.

use anyhow::anyhow;
use substreams::{errors::Error, scalar::BigInt};
use substreams_ethereum::{block_view, pb::eth::v2::TransactionTrace, Event, NULL_ADDRESS};

use crate::{
    abi::common::erc20::events::Transfer,
    types::{
        registry::{RegistryDetails, RegistryType},
        transfer::TokenTransfer,
    },
};

pub fn extract_specific_transfer_event(
    trx: &TransactionTrace,
    log_address: Option<&Vec<u8>>,
    from: Option<&Vec<u8>>,
    to: Option<&Vec<u8>>,
    value: Option<&BigInt>,
    reference_log_index: u32,
) -> Result<TokenTransfer, Error> {
    // Count the number of Some values among the parameters
    let mut provided_params_count = [log_address, from, to]
        .iter()
        .filter(|opt| opt.is_some())
        .count();

    // Type of `value` differs from the other params, so must be counted separately
    if value.is_some() {
        provided_params_count += 1;
    }

    // Ensure at least two params are provided. Allowing only one param is too permissive.
    if provided_params_count < 2 {
        return Err(anyhow!(
            "At least two of 'log_address', 'from', 'to', or 'value' must be specified"
        ));
    }

    // Collect potential transfers that match the provided parameters and are earlier than the reference log index.
    let mut matching_transfers: Vec<(u32, TokenTransfer)> = trx
        .calls
        .iter()
        .filter(|call| !call.state_reverted)
        .flat_map(|call| call.logs.iter())
        .filter_map(|log| {
            // Ignore logs that occur after the reference event to ensure causality in the transaction order.
            if log.index > reference_log_index {
                return None;
            };

            if let Some(transfer) = Transfer::match_and_decode(log) {
                // Check if `log_address` is provided and matches, or if it's None
                let address_match = log_address.map_or(true, |addr| &log.address == addr);

                // If 'from' is None, ignore receiver check; otherwise, check if it matches
                let from_match = from.map_or(true, |from_addr| transfer.sender == *from_addr);

                // If 'to' is None, ignore receiver check; otherwise, check if it matches
                let to_match = to.map_or(true, |to_addr| transfer.receiver == *to_addr);

                // If 'value' is None, ignore value check; otherwise, check if it matches
                let value_match = value.map_or(true, |val| transfer.value == *val);

                // If all the params match a Transfer event in the transactions logs, return it.
                if address_match && from_match && to_match && value_match {
                    return Some((log.index, TokenTransfer::new(transfer, log.address.clone())));
                }
            }
            None
        })
        .collect();

    // Sort the matching transfers by their proximity to the reference log index in ascending order.
    matching_transfers.sort_by_key(|&(index, _)| reference_log_index - index);

    // Return the closest matching transfer if any exist.
    // The closest matching transfer will relate to the specific liquidity event we want to extract for.
    matching_transfers
        .first()
        .map(|(_, transfer)| transfer.clone())
        .ok_or_else(|| anyhow!("No transfer event found"))
}

// Only use for MetaPool and PlainPool deployments where we can ensure there is only one Transfer event.
pub fn extract_pool_creation_transfer_event(
    log: &block_view::LogView,
    registry: &RegistryDetails,
) -> Result<TokenTransfer, Error> {
    log.receipt
        .transaction
        .calls
        .iter()
        .filter(|call| !call.state_reverted)
        .flat_map(|call| call.logs.iter())
        .find_map(|log| {
            if let Some(transfer) = Transfer::match_and_decode(log) {
                let is_valid_transfer = match registry.registry_type {
                    RegistryType::CrvUSDPoolFactory
                    | RegistryType::PoolRegistryV1
                    | RegistryType::MetaPoolFactoryOld => {
                        // Criteria that pertains to the relevant transfer event for Plain pools deployed via
                        // CrvUSDPoolFactory/PoolRegistryV1, and Metapools deployed via MetaPoolFactoryOld
                        &transfer.sender == &NULL_ADDRESS.to_vec()
                            && &transfer.receiver == &log.address
                    }
                    RegistryType::StableSwapFactoryNG => {
                        // Criteria that pertains to the relevant transfer event for Plain or Meta pools deployed via StableSwapFactoryNG
                        &transfer.sender == &NULL_ADDRESS.to_vec()
                            && transfer.receiver == registry.address.to_vec()
                    }
                    _ => false,
                };
                // If the transfer event matches the criteria, return the decoded transfer event and the log
                if is_valid_transfer {
                    return Some((transfer, log));
                }
            }
            None // Skip if it's not a matching transfer log
        })
        .ok_or_else(|| anyhow!("No transfer event found in the transaction"))
        // Here the log is the one associated with the found transfer event
        .and_then(|(transfer, transfer_log)| {
            Ok(TokenTransfer::new(transfer, transfer_log.address.to_vec()))
        })
}
