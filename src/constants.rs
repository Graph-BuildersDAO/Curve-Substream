use hex_literal::hex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use substreams::scalar::BigInt;

use crate::config::config::{PoolConfig, MISSING_OLD_POOLS_DATA};

// Chain Specific Contracts:
// ________________________
// Most of the network specific addresses/config are stored in the configuration file.
// (See `src/config/configuration.rs` for more details)
// Old pools needs to be stored as a hash map, so we lazily initialise this at runtime.

lazy_static! {
    pub static ref MISSING_OLD_POOLS: HashMap<&'static str, PoolConfig> =
        MISSING_OLD_POOLS_DATA.iter().cloned().collect();
}

// Global Constants:
// These will not be dynamic like the chain specific contracts above.
// ________________________

pub const CURVE_ADDRESS_PROVIDER: [u8; 20] = hex!("0000000022d53366457f9d5e68ec105046fc4383");
pub const ETH_ADDRESS: [u8; 20] = hex!("EeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE");

pub fn default_decimals() -> BigInt {
    BigInt::from(18)
}
