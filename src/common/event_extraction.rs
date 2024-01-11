// Generic Event Extraction that is not specific to any module.

use anyhow::anyhow;
use substreams::errors::Error;
use substreams_ethereum::{block_view, pb::eth::v2::TransactionTrace, Event};

use crate::abi::erc20::events::Transfer;

pub fn extract_specific_transfer_event(
    trx: &TransactionTrace,
    log_address: &Vec<u8>,
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
                if log.address == *log_address
                    && transfer.sender == *from
                    && transfer.receiver == *to
                {
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
