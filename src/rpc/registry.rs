use anyhow::anyhow;
use substreams::{errors::Error, Hex};
use substreams_ethereum::NULL_ADDRESS;

use crate::{
    abi::curve::{address_provider as address_provider_abi, registry},
    constants,
};

// Logic from the original subgraph.
// Is not the main registry pool if we cannot get the registry,
// or get the lp token address from the registry.
pub fn is_main_registry_pool(pool_address: &Vec<u8>) -> bool {
    match (address_provider_abi::functions::GetRegistry {}
        .call(constants::CURVE_ADDRESS_PROVIDER.to_vec()))
    {
        Some(registry_address) => {
            match get_lp_token_address_from_registry(pool_address, &registry_address) {
                Ok(_lp_token_address) => true,
                Err(_e) => false,
            }
        }
        None => false,
    }
}

pub fn get_lp_token_address_from_registry(
    pool_address: &Vec<u8>,
    registry_address: &Vec<u8>,
) -> Result<Vec<u8>, Error> {
    let address_option = registry::functions::GetLpToken {
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
