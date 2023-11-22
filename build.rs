use anyhow::{Ok, Result};
use regex::Regex;
use serde_json::Value;
use std::fs;
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    let file_names = [
        "abi/AddressProvider.abi.json",
        "abi/ERC20.abi.json",
        "abi/Pool.abi.json",
        "abi/Registry.abi.json",
    ];
    let file_output_names = [
        "src/abi/address_provider.rs",
        "src/abi/erc20.rs",
        "src/abi/pool.rs",
        "src/abi/registry.rs",
    ];

    let mut i = 0;
    for f in file_names {
        let contents = fs::read_to_string(f).expect("Should have been able to read the file");

        // sanitize fields and attributes starting with an underscore
        let regex = Regex::new(r#"("\w+"\s?:\s?")_(\w+")"#).unwrap();
        let sanitized_abi_file = regex.replace_all(contents.as_str(), "${1}u_${2}");

        Abigen::from_bytes("Contract", sanitized_abi_file.as_bytes())?
            .generate()?
            .write_to_file(file_output_names[i])?;

        i = i + 1;
    }

    generate_network_config_from_json(
        // TODO: This will eventually need to be dynamic when we support multiple networks.
        "config/curve-finance-ethereum/configuration.json",
        "./src/network_config.rs",
    )
    .expect("Should have been able to generate the network configuration file");

    Ok(())
}

fn generate_network_config_from_json(path: &str, output_path: &str) -> Result<()> {
    let json_contents = fs::read_to_string(path)
        .expect("Should have been able to read the network configuration file");

    let json: Value = serde_json::from_str(&json_contents)?;

    let mut output = String::new();

    // Generated file imports
    output.push_str("use hex_literal::hex;\n\n");

    if let Some(network) = json["network"].as_str() {
        output.push_str(&format!("pub const NETWORK: &str = \"{}\";\n", network));
    }
    if let Some(default_network) = json["defaultNetwork"].as_str() {
        output.push_str(&format!(
            "pub const DEFAULT_NETWORK: &str = \"{}\";\n",
            default_network
        ));
    }
    if let Some(price_caching) = json["priceCaching"].as_u64() {
        output.push_str(&format!(
            "pub const PRICE_CACHING: u64 = {};\n",
            price_caching
        ));
    }
    if let Some(pool_info_contract) = json["poolInfoContract"].as_str() {
        output.push_str(&format!(
            "pub const POOL_INFO_CONTRACT: [u8; 20] = hex!(\"{}\");\n",
            pool_info_contract.trim_start_matches("0x")
        ));
    }
    if let Some(protocol_address) = json["protocolAddress"].as_str() {
        output.push_str(&format!(
            "pub const PROTOCOL_ADDRESS: [u8; 20] = hex!(\"{}\");\n",
            protocol_address.trim_start_matches("0x")
        ));
    }
    if let Some(crv_token_address) = json["crvTokenAddress"].as_str() {
        output.push_str(&format!(
            "pub const CRV_TOKEN_ADDRESS: [u8; 20] = hex!(\"{}\");\n",
            crv_token_address.trim_start_matches("0x")
        ));
    }
    if let Some(gauge_controller_address) = json["gaugeControllerContract"].as_str() {
        output.push_str(&format!(
            "pub const GAUGE_CONTROLLER_ADDRESS: [u8; 20] = hex!(\"{}\");\n",
            gauge_controller_address.trim_start_matches("0x")
        ));
    }

    // Generating constants for poolRegistry
    if let Some(pool_registry) = json["poolRegistry"].as_array() {
        output.push_str("pub const CONTRACTS: [[u8; 20]; 5] = [");
        for pool in pool_registry {
            let address = pool["address"]
                .as_str()
                .unwrap_or_default()
                .trim_start_matches("0x");
            output.push_str(&format!("hex!(\"{}\"), ", address));
        }
        output.push_str("];\n");
    }

    // Generating structs for missingOldPools
    output.push_str("\n#[derive(Debug, Clone)]\npub struct PoolDetails {\n    pub name: &'static str,\n    pub address: [u8; 20],\n    pub lp_token: [u8; 20],\n    pub start_block: u64,\n}\n");

    if let Some(missing_old_pools) = json["missingOldPools"].as_array() {
        output.push_str("\npub static MISSING_OLD_POOLS_DATA: &[(&str, PoolDetails)] = &[\n");
        for pool in missing_old_pools {
            let key = pool["address"].as_str().unwrap_or_default();
            let name = pool["name"].as_str().unwrap_or_default();
            let address = pool["address"]
                .as_str()
                .unwrap_or_default()
                .trim_start_matches("0x");
            let lp_token = pool["lpToken"]
                .as_str()
                .unwrap_or_default()
                .trim_start_matches("0x");
            let start_block = pool["startBlock"].as_str().unwrap_or_default();
            output.push_str(&format!("(\"{}\", PoolDetails {{ name: \"{}\", address: hex!(\"{}\"), lp_token: hex!(\"{}\"), start_block: {} }}),\n", key, name, address, lp_token, start_block));
        }
        output.push_str("];\n");
    }

    // Generating an array for basePoolsLpToken
    if let Some(base_pools_lp_token) = json["basePoolsLpToken"].as_array() {
        output.push_str(
            format!(
                "\npub static BASE_POOLS_LP_TOKEN: [[u8; 20]; {}] = [\n",
                base_pools_lp_token.len()
            )
            .as_str(),
        );
        for pool in base_pools_lp_token {
            let name = pool["name"].as_str().unwrap_or_default();
            let address = pool["address"]
                .as_str()
                .unwrap_or_default()
                .trim_start_matches("0x");
            output.push_str(&format!("hex!(\"{}\"), // {}\n", address, name));
        }
        output.push_str("];\n");
    }

    fs::write(output_path, output)?;

    Ok(())
}
