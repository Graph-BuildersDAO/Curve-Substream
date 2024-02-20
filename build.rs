use anyhow::{Ok, Result};
use regex::Regex;
use serde_json::Value;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};
use substreams_ethereum::Abigen;

// TODO: This file could do with a refactor
//       Consider creating helper functions for all the config read/writes.
fn main() -> Result<(), anyhow::Error> {
    let file_names = [
        "abi/common/ERC20.abi.json",
        "abi/curve/child_registries/BasePoolRegistry.abi.json",
        "abi/curve/child_registries/crvUSDPoolFactory.abi.json",
        "abi/curve/child_registries/CryptoPoolFactoryV2.abi.json",
        "abi/curve/child_registries/CryptoSwapRegistryOld.abi.json",
        "abi/curve/child_registries/CryptoSwapRegistryV2.abi.json",
        "abi/curve/child_registries/PoolRegistryV1.abi.json",
        "abi/curve/child_registries/PoolRegistryV1Old.abi.json",
        "abi/curve/child_registries/PoolRegistryV2Old.abi.json",
        "abi/curve/child_registries/StableSwapFactoryNG.abi.json",
        "abi/curve/child_registries/TriCryptoFactoryNG.abi.json",
        "abi/curve/AddressProvider.abi.json",
        "abi/curve/Pool.abi.json",
        "abi/curve/Registry.abi.json",
        "abi/oracle/SushiSwap.abi.json",
        "abi/oracle/YearnLens.abi.json",
    ];
    let file_output_names = [
        "src/abi/common/erc20.rs",
        "src/abi/curve/child_registries/base_pool_registry.rs",
        "src/abi/curve/child_registries/crv_usd_pool_factory.rs",
        "src/abi/curve/child_registries/crypto_pool_factory_v2.rs",
        "src/abi/curve/child_registries/crypto_swap_registry_old.rs",
        "src/abi/curve/child_registries/crypto_swap_registry_v2.rs",
        "src/abi/curve/child_registries/pool_registry_v1.rs",
        "src/abi/curve/child_registries/pool_registry_v1_old.rs",
        "src/abi/curve/child_registries/pool_registry_v2_old.rs",
        "src/abi/curve/child_registries/stable_swap_factory_ng.rs",
        "src/abi/curve/child_registries/tricrypto_factory_ng.rs",
        "src/abi/curve/address_provider.rs",
        "src/abi/curve/pool.rs",
        "src/abi/curve/registry.rs",
        "src/abi/oracle/sushiswap.rs",
        "src/abi/oracle/yearn_lens.rs",
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

    let mut modules_by_dir: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for f in file_output_names {
        let path = Path::new(f);
        if let (Some(parent), Some(file_stem)) = (path.parent(), path.file_stem()) {
            let parent = parent.to_str().unwrap().to_string();
            let file_stem = file_stem.to_str().unwrap().to_string();

            modules_by_dir.entry(parent).or_default().push(file_stem);
        }
    }

    for (dir, modules) in modules_by_dir {
        let mod_file_path = Path::new(&dir).join("mod.rs");
        let mut mod_file = File::create(mod_file_path)?;

        // Get subdirectories in the current directory
        let subdirs = get_subdirectories(Path::new(&dir));

        // Write a mod line for each subdirectory
        for subdir in subdirs {
            if let Some(name) = subdir.file_name().and_then(|n| n.to_str()) {
                writeln!(mod_file, "pub mod {};", name)?;
            }
        }

        for module in modules {
            writeln!(mod_file, "pub mod {};", module)?;
        }
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
        let network_substring = default_network
            .find(".")
            .map(|index| &default_network[index + 1..])
            .unwrap();
        output.push_str(&format!(
            "pub const DEFAULT_NETWORK: &str = \"{}\";\n",
            network_substring
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
        output.push_str(
            format!(
                "pub const POOL_REGISTRIES: [[u8; 20]; {}] = [",
                pool_registry.len()
            )
            .as_str(),
        );
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

    // Generating an array for hardcodedMetapools
    if let Some(hardcoded_metapools) = json["hardcodedMetaPools"].as_array() {
        output.push_str(
            format!(
                "\npub static HARDCODED_METAPOOLS: [[u8; 20]; {}] = [\n",
                hardcoded_metapools.len()
            )
            .as_str(),
        );
        for pool in hardcoded_metapools {
            let name = pool["name"].as_str().unwrap_or_default();
            let address = pool["address"]
                .as_str()
                .unwrap_or_default()
                .trim_start_matches("0x");
            output.push_str(&format!("hex!(\"{}\"), // {}\n", address, name));
        }
        output.push_str("];\n");
    } else {
        output.push_str("\npub static HARDCODED_METAPOOLS: [[u8; 20]; 0] = [];\n");
    }

    // Generating an array for hardcodedStables
    if let Some(hardcoded_stables) = json["hardcodedStables"].as_array() {
        output.push_str(
            format!(
                "\npub static HARDCODED_STABLES: [[u8; 20]; {}] = [\n",
                hardcoded_stables.len()
            )
            .as_str(),
        );
        for coin in hardcoded_stables {
            let name = coin["name"].as_str().unwrap_or_default();
            let address = coin["address"]
                .as_str()
                .unwrap_or_default()
                .trim_start_matches("0x");
            output.push_str(&format!("hex!(\"{}\"), // {}\n", address, name));
        }
        output.push_str("];\n");
    }

    if let Some(yearn_lens) = json["yearnLens"].as_object() {
        let address = yearn_lens["address"]
            .as_str()
            .unwrap_or_default()
            .trim_start_matches("0x");
        output.push_str(&format!(
            "\npub static YEARN_LENS: [u8; 20] = hex!(\"{}\");\n",
            address
        ));
    }

    if let Some(sushiswap) = json["sushiswap"].as_object() {
        let address = sushiswap["address"]
            .as_str()
            .unwrap_or_default()
            .trim_start_matches("0x");
        output.push_str(&format!(
            "\npub static SUSHISWAP: [u8; 20] = hex!(\"{}\");\n",
            address
        ));
    }

    if let Some(yearn_blacklist) = json["yearnLensBlacklist"].as_array() {
        output.push_str(
            format!(
                "\npub static YEARN_LENS_BLACKLIST: [[u8; 20]; {}] = [\n",
                yearn_blacklist.len()
            )
            .as_str(),
        );
        for token in yearn_blacklist {
            let name = token["name"].as_str().unwrap_or_default();
            let address = token["address"]
                .as_str()
                .unwrap_or_default()
                .trim_start_matches("0x");
            output.push_str(&format!("hex!(\"{}\"), // {}\n", address, name));
        }
        output.push_str("];\n");
    } else {
        output.push_str("\npub static YEARN_LENS_BLACKLIST: [[u8; 20]; 0] = [];\n");
    }

    if let Some(sushi_blacklist) = json["sushiBlacklist"].as_array() {
        output.push_str(
            format!(
                "\npub static SUSHI_BLACKLIST: [[u8; 20]; {}] = [\n",
                sushi_blacklist.len()
            )
            .as_str(),
        );
        for token in sushi_blacklist {
            let name = token["name"].as_str().unwrap_or_default();
            let address = token["address"]
                .as_str()
                .unwrap_or_default()
                .trim_start_matches("0x");
            output.push_str(&format!("hex!(\"{}\"), // {}\n", address, name));
        }
        output.push_str("];\n");
    } else {
        output.push_str("\npub static SUSHI_BLACKLIST: [[u8; 20]; 0] = [];\n");
    }

    fs::write(output_path, output)?;
    Ok(())
}

fn get_subdirectories(path: &Path) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}