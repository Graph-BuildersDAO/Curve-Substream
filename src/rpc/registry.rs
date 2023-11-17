use anyhow::anyhow;
use substreams::{errors::Error, Hex};
use substreams_ethereum::NULL_ADDRESS;

use crate::abi::registry::functions;

pub fn get_lp_token_address_from_registry(
    pool_address: Vec<u8>,
    registry_address: Vec<u8>,
) -> Result<Vec<u8>, Error> {
    let address_option = functions::GetLpToken {
        arg0: pool_address.clone(),
    }
    .call(registry_address.clone());

    let address = address_option.ok_or_else(|| {
        anyhow!(
            "Unable to get lp token for pool {:?} from contract {:?}",
            Hex::encode(&pool_address),
            Hex::encode(&registry_address)
        )
    })?;
    if address == NULL_ADDRESS {
        return Err(anyhow!(
            "Null address returned getting lp token for pool {} from contract {}",
            Hex::encode(&pool_address),
            Hex::encode(&registry_address)
        ));
    }
    Ok(address)
}
