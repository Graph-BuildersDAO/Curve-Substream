use hex_literal::hex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use substreams::scalar::{BigDecimal, BigInt};

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

pub mod protocol_type {
    pub const EXCHANGE: &'static str = "EXCHANGE";
    pub const LENDING: &'static str = "LENDING";
    pub const YIELD: &'static str = "YIELD";
    pub const BRIDGE: &'static str = "BRIDGE";
    pub const GENERIC: &'static str = "GENERIC";
}

pub enum LiquidityPoolFeeType {
    FixedTradingFee,
    TieredTradingFee,
    DynamicTradingFee,
    FixedLpFee,
    DynamicLpFee,
    FixedProtocolFee,
    DynamicProtocolFee,
}

impl LiquidityPoolFeeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LiquidityPoolFeeType::FixedTradingFee => "FIXED_TRADING_FEE",
            LiquidityPoolFeeType::TieredTradingFee => "TIERED_TRADING_FEE",
            LiquidityPoolFeeType::DynamicTradingFee => "DYNAMIC_TRADING_FEE",
            LiquidityPoolFeeType::FixedLpFee => "FIXED_LP_FEE",
            LiquidityPoolFeeType::DynamicLpFee => "DYNAMIC_LP_FEE",
            LiquidityPoolFeeType::FixedProtocolFee => "FIXED_PROTOCOL_FEE",
            LiquidityPoolFeeType::DynamicProtocolFee => "DYNAMIC_PROTOCOL_FEE",
        }
    }
}

pub mod protocol {
    pub const NAME: &'static str = "Curve Finance";
    pub const SLUG: &'static str = "curve-finance";
    pub const SCHEMA_VERSION: &'static str = "1.3.0";
    pub const SUBGRAPH_VERSION: &'static str = "1.0.0";
    pub const METHODOLOGY_VERSION: &'static str = "1.0.0";
}

pub const CURVE_ADDRESS_PROVIDER: [u8; 20] = hex!("0000000022d53366457f9d5e68ec105046fc4383");
pub const ETH_ADDRESS: [u8; 20] = hex!("EeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE");

pub static BLACKLISTED_TOKENS: &[&str] = &[
    "0000000000000000000000000000000000000000", // Null Address
    "b755b949c126c04e0348dd881a5cf55d424742b2", // Curve USD-BTC-ETH
    "d79138c49c49200a1afc935171d1bdad084fdc95", // Curve.fi Factory Plain Pool: 3pool
    "37c9be6c81990398e9b87494484afc6a4608c25d", // Curve.fi Factory Plain Pool: blizz
    "f72beacc6fd334e14a7ddac25c3ce1eb8a827e10", // Curve.fi Factory USD Metapool: Defrost H2O
    "ae6aab43c4f3e0cea4ab83752c278f8debaba689", // dForce
    "aa9dfbf31d2f807ca4d9f7be281d75ca7bdce64d", // Curve.fi Factory Plain Pool: Curve DD2Pool
    "83f798e925bcd4017eb265844fddabb448f1707d", // iearn USDT
    "e6354ed5bc4b393a5aad09f21c46e101e692d447", // iearn USDT
    "1be5d71f2da660bfdee8012ddc58d024448a0a59", // iearn USDT
    "7f86bf177dd4f3494b841a37e810a34dd56c829b", // TricryptoUSDC
    "f5f5b97624542d72a9e06f04804bf81baa15e2b4", // TricryptoUSDT
    "aa91cdd7abb47f821cf07a2d38cc8668deaf1bdc", // 2jpy-2-f
    "8343091f2499fd4b6174a46d067a920a3b851ff9", // jJPY
];

pub fn default_decimals() -> BigInt {
    BigInt::from(18)
}

pub fn default_usdc_decimals() -> BigInt {
    BigInt::from(6)
}

pub fn default_usd_price() -> BigDecimal {
    BigDecimal::from(1000000)
}

pub const FEE_DENOMINATOR: u64 = 10000000000;

pub fn default_pool_fee() -> BigInt {
    BigInt::from(4000000)
}

pub fn default_admin_fee() -> BigInt {
    BigInt::from(5000000000 as i64)
}
