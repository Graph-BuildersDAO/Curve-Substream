use hex_literal::hex;
use substreams::scalar::BigInt;

// Chain Specific Contracts (Registries and Factories):
// ________________________
// TODO: Currently am hardcoding Ethereum Mainnet contracts.
//       This should be changed to a more dynamic approach, similar to the subgraph.
//       A config file could be used to specify the contracts for each chain.
//       This constants file can then be generated via a script or at build or compile time.
//       Need to look into the most appropriate solution.
const POOLREGISTRY_CONTRACT: [u8; 20] = hex!("90e00ace148ca3b23ac1bc8c240c2a7dd9c2d7f5");
const POOLREGISTRY_V2_CONTRACT: [u8; 20] = hex!("7d86446ddb609ed0f5f8684acf30380a356b2b4c");
const CRYPTOSWAP_REGISTRY_CONTRACT: [u8; 20] = hex!("8F942C20D02bEfc377D41445793068908E2250D0");
const METAPOOL_FACTORY_CONTRACT: [u8; 20] = hex!("B9fC157394Af804a3578134A6585C0dc9cc990d4");
const CRYPTOPOOL_FACTORY: [u8; 20] = hex!("F18056Bbd320E96A48e3Fbf8bC061322531aac99");

pub const CONTRACTS: [[u8; 20]; 5] = [
    POOLREGISTRY_CONTRACT,
    POOLREGISTRY_V2_CONTRACT,
    CRYPTOSWAP_REGISTRY_CONTRACT,
    METAPOOL_FACTORY_CONTRACT,
    CRYPTOPOOL_FACTORY,
];

// Global Constants:
// These will not be dynamic like the chain specific contracts above.
// ________________________

pub const ETH_ADDRESS: [u8; 20] = hex!("EeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE");

pub fn default_decimals() -> BigInt {
    BigInt::from(18)
}