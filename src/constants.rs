use hex_literal::hex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use substreams::scalar::BigInt;

use crate::network_config::{PoolDetails, MISSING_OLD_POOLS_DATA};

// Chain Specific Contracts:
// ________________________
// Most of the network specific addresses/config are stored in the configuration file.
// (See `src/network-config.rs` for more details)
// Old pools needs to be stored as a hash map, so we lazily initialise this at runtime.

lazy_static! {
    pub static ref MISSING_OLD_POOLS: HashMap<&'static str, PoolDetails> =
        MISSING_OLD_POOLS_DATA.iter().cloned().collect();
}

// Global Constants:
// These will not be dynamic like the chain specific contracts above.
// ________________________

// The network names corresponding to the Network enum in the schema.
pub mod network {
    pub const ARBITRUM_ONE: &'static str = "ARBITRUM_ONE";
    pub const ARWEAVE_MAINNET: &'static str = "ARWEAVE_MAINNET";
    pub const AVALANCHE: &'static str = "AVALANCHE";
    pub const BOBA: &'static str = "BOBA";
    pub const AURORA: &'static str = "AURORA";
    pub const BSC: &'static str = "BSC"; // aka BNB Chain
    pub const CELO: &'static str = "CELO";
    pub const COSMOS: &'static str = "COSMOS";
    pub const CRONOS: &'static str = "CRONOS";
    pub const MAINNET: &'static str = "MAINNET"; // Ethereum mainnet
    pub const FANTOM: &'static str = "FANTOM";
    pub const FUSE: &'static str = "FUSE";
    pub const HARMONY: &'static str = "HARMONY";
    pub const JUNO: &'static str = "JUNO";
    pub const MOONBEAM: &'static str = "MOONBEAM";
    pub const MOONRIVER: &'static str = "MOONRIVER";
    pub const NEAR_MAINNET: &'static str = "NEAR_MAINNET";
    pub const OPTIMISM: &'static str = "OPTIMISM";
    pub const OSMOSIS: &'static str = "OSMOSIS";
    pub const MATIC: &'static str = "MATIC"; // aka Polygon
    pub const XDAI: &'static str = "XDAI"; // aka Gnosis Chain
}

pub const CURVE_ADDRESS_PROVIDER: [u8; 20] = hex!("0000000022d53366457f9d5e68ec105046fc4383");
pub const ETH_ADDRESS: [u8; 20] = hex!("EeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE");

pub fn default_decimals() -> BigInt {
    BigInt::from(18)
}
