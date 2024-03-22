use anyhow::anyhow;
use substreams::{errors::Error, Hex};
use substreams_ethereum::{block_view, pb::eth::v2 as eth, NULL_ADDRESS};

use crate::{
    abi::curve::{
        child_registries::{
            crv_usd_pool_factory, crypto_pool_factory_v2, pool_registry_v1, stable_swap_factory_ng,
            tricrypto_factory_ng,
        },
        crv_token, gauge_controller,
    },
    common::event_extraction,
    network_config::{
        PoolDetails, PoolType as PoolTypeConfig, CRV_TOKEN_ADDRESS, GAUGE_CONTROLLER_ADDRESS,
        MISSING_OLD_POOLS_DATA, POOL_REGISTRIES,
    },
    pb::curve::types::v1::{
        pool::PoolType, ControllerNewGauge, CryptoPool, CurveEvents, LiquidityGauge, MetaPool,
        PlainPool, Pool, Token, TriCryptoPool, UpdateMiningParametersEvent,
    },
    rpc::{self, pool, token},
    types::event_traits::PlainPoolDeployedEvent,
};

#[substreams::handlers::map]
pub fn map_curve_events(blk: eth::Block) -> Result<CurveEvents, Vec<Error>> {
    let mut curve_events = CurveEvents::default();
    let mut pools: Vec<Pool> = Vec::new();
    // Liquidity Gauges deployed via registry/factories
    let mut gauges: Vec<LiquidityGauge> = Vec::new();
    // Liquidity Gauges that have been previously deployed, and now added to the GaugeController contract
    let mut controller_gauges: Vec<ControllerNewGauge> = Vec::new();

    // Need to add pools that were deployed before any registry/factory contracts handled pool deployment
    for &(_pool_address, ref pool_details) in MISSING_OLD_POOLS_DATA.iter() {
        if pool_details.start_block == blk.number {
            match add_missing_pool(&blk, &mut pools, pool_details) {
                Ok(_) => {}
                Err(e) => {
                    return Err(vec![e]);
                }
            }
            // No old pools were deployed on the same block, so we can skip the rest of the loop
            break;
        }
    }

    // This calls each event mapping func for each contract address.
    // As nothing is returned with the `Ok` variant, we can just ignore it,
    // and use the `Err` variant to collect any errors that occur.
    let mut errors: Vec<Error> = POOL_REGISTRIES
        .iter()
        .flat_map(|&contract| {
            [
                // Track pools that have been deployed from registry/factory contracts
                map_crypto_pool_deployed_events(&blk, &mut pools, contract),
                map_plain_pool_deployed_events::<crv_usd_pool_factory::events::PlainPoolDeployed>(
                    &blk, &mut pools, contract,
                ),
                map_plain_pool_deployed_events::<pool_registry_v1::events::PlainPoolDeployed>(
                    &blk, &mut pools, contract,
                ),
                map_plain_pool_deployed_events::<stable_swap_factory_ng::events::PlainPoolDeployed>(
                    &blk, &mut pools, contract,
                ),
                map_meta_pool_deployed_events(&blk, &mut pools, contract),
                map_tricrypto_pool_deployed_events(&blk, &mut pools, contract),
                // Track liquidity gauges that have been deployed from registry/factory contracts
                map_liquidity_gauge_deployed_events(&blk, &mut gauges, contract),
                map_liquidity_gauge_deployed_with_token_events(&blk, &mut gauges, contract),
            ]
        })
        .filter_map(Result::err)
        .collect();

    // Extracts NewGauge events from the GaugeController contract
    match map_controller_new_gauge_events(&blk, &mut controller_gauges, GAUGE_CONTROLLER_ADDRESS) {
        // As nothing is returned with the `Ok` variant, we can just ignore it,
        // and use the `Err` variant to collect any error that occurs.
        Err(e) => errors.push(e),
        _ => {}
    };

    let crv_mining_update_event = map_crv_mining_update_events(&blk, CRV_TOKEN_ADDRESS);

    // Extract CRV mining params updates to track inflation rate
    match crv_mining_update_event {
        Ok(Some(event)) => {
            curve_events.update_mining_parameters_event = Some(event);
        }
        Err(e) => {
            errors.push(e);
        }
        _ => {}
    }

    // seed_pool_for_testing(&mut pools);

    curve_events.pools = pools;
    curve_events.gauges = gauges;
    curve_events.controller_gauges = controller_gauges;

    // Sort by log ordinal to maintain determinism when handling these messages downstream
    curve_events
        .pools
        .sort_by(|a, b| a.log_ordinal.cmp(&b.log_ordinal));
    curve_events
        .gauges
        .sort_by(|a, b| a.log_ordinal.cmp(&b.log_ordinal));
    curve_events
        .controller_gauges
        .sort_by(|a, b| a.log_ordinal.cmp(&b.log_ordinal));

    if errors.is_empty() {
        return Ok(curve_events);
    }
    Err(errors)
}

fn add_missing_pool(
    blk: &eth::Block,
    pools: &mut Vec<Pool>,
    pool: &PoolDetails,
) -> Result<(), Error> {
    let pool_address = pool.address.to_vec();
    let lp_token = match token::create_token(&pool.lp_token.to_vec(), &pool_address, None) {
        Ok(token) => token,
        Err(e) => {
            return Err(anyhow!("Error in `add_missing_pools`: {:?}", e));
        }
    };
    let (input_tokens, input_tokens_ordered) = match get_and_sort_input_tokens(&pool_address) {
        Ok(result) => result,
        Err(e) => {
            return Err(anyhow!("Error in `add_missing_pools`: {:?}", e));
        }
    };
    let hash = blk
        .transactions()
        .find(|trx| trx.to == pool_address)
        .map(|tx| tx.hash.clone())
        .unwrap_or_else(|| NULL_ADDRESS.to_vec());

    match pool.pool_type {
        PoolTypeConfig::Plain => {
            pools.push(create_missing_pool(
                Hex::encode(pool_address),
                Hex::encode(NULL_ADDRESS.to_vec()),
                lp_token,
                input_tokens_ordered,
                input_tokens,
                blk,
                hash,
                PoolType::PlainPool(PlainPool {}),
            ));
        }
        PoolTypeConfig::Crypto => {
            pools.push(create_missing_pool(
                Hex::encode(pool_address),
                Hex::encode(NULL_ADDRESS.to_vec()),
                lp_token,
                input_tokens_ordered,
                input_tokens,
                blk,
                hash,
                PoolType::PlainPool(PlainPool {}),
            ));
        }
        PoolTypeConfig::TriCrypto => {
            pools.push(create_missing_pool(
                Hex::encode(pool_address),
                Hex::encode(NULL_ADDRESS.to_vec()),
                lp_token,
                input_tokens_ordered,
                input_tokens,
                blk,
                hash,
                PoolType::PlainPool(PlainPool {}),
            ));
        }
        PoolTypeConfig::Lending => {}
        PoolTypeConfig::Meta => {
            if let Some(base_pool) = pool::get_old_metapool_base_pool(&pool.address.to_vec()) {
                if let Ok(underlying_coins) =
                    pool::get_old_metapool_underlying_coins(&pool.address.to_vec())
                {
                    pools.push(create_missing_pool(
                        Hex::encode(pool_address),
                        Hex::encode(NULL_ADDRESS.to_vec()),
                        lp_token,
                        input_tokens_ordered,
                        input_tokens,
                        blk,
                        hash,
                        PoolType::MetaPool(MetaPool {
                            base_pool_address: Hex::encode(base_pool),
                            underlying_tokens: underlying_coins,
                            max_coin: 1,
                        }),
                    ));
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn map_crypto_pool_deployed_events(
    blk: &eth::Block,
    pools: &mut Vec<Pool>,
    address: [u8; 20],
) -> Result<(), Error> {
    pools.append(
        &mut blk
            .events::<crypto_pool_factory_v2::events::CryptoPoolDeployed>(&[&address])
            .filter_map(|(event, log)| {
                // The minter of the LP token is the liquidity pool contract.
                let pool_address = match token::get_token_minter(&event.token) {
                    Ok(minter) => minter,
                    Err(e) => {
                        substreams::log::debug!(
                            "Error in `map_crypto_pool_deployed_events`: {:?}",
                            e
                        );
                        return None;
                    }
                };
                let lp_token = match token::create_token(&event.token, &pool_address, None) {
                    Ok(token) => token,
                    Err(e) => {
                        substreams::log::debug!(
                            "Error in `map_crypto_pool_deployed_events`: {:?}",
                            e
                        );
                        return None;
                    }
                };
                let (input_tokens, input_tokens_ordered) =
                    match get_and_sort_input_tokens(&pool_address) {
                        Ok(result) => result,
                        Err(e) => {
                            substreams::log::debug!(
                                "Error in `map_crypto_pool_deployed_events`: {:?}",
                                e
                            );
                            return None;
                        }
                    };
                substreams::log::debug!("Adding a CryptoPool");

                Some(create_pool(
                    Hex::encode(&pool_address),
                    Hex::encode(address),
                    lp_token,
                    input_tokens_ordered,
                    input_tokens,
                    &log,
                    blk,
                    PoolType::CryptoPool(CryptoPool {}),
                ))
            })
            .collect(),
    );
    Ok(())
}

fn map_plain_pool_deployed_events<E: PlainPoolDeployedEvent + substreams_ethereum::Event>(
    blk: &eth::Block,
    pools: &mut Vec<Pool>,
    // todo this could be named more aptly as it is the registry/factory address
    address: [u8; 20],
) -> Result<(), Error> {
    pools.append(
        &mut blk
            .events::<E>(&[&address])
            .filter_map(|(_event, log)| {
                let transfer = match event_extraction::extract_transfer_event(&log) {
                    Ok(event) => event,
                    Err(e) => {
                        substreams::log::debug!(
                            "Error in `map_plain_pool_deployed_events`: {:?}",
                            e
                        );
                        return None;
                    }
                };

                let plain_pool_address = &transfer.token_address;

                let lp_token =
                    match token::create_token(plain_pool_address, plain_pool_address, None) {
                        Ok(token) => token,
                        Err(e) => {
                            substreams::log::debug!(
                                "Error in `map_plain_pool_deployed_events`: {:?}",
                                e
                            );
                            return None;
                        }
                    };
                let (input_tokens, input_tokens_ordered) =
                    match get_and_sort_input_tokens(plain_pool_address) {
                        Ok(result) => result,
                        Err(e) => {
                            substreams::log::debug!(
                                "Error in `map_plain_pool_deployed_events`: {:?}",
                                e
                            );
                            return None;
                        }
                    };
                substreams::log::debug!("Adding a PlainPool");

                Some(create_pool(
                    Hex::encode(plain_pool_address),
                    Hex::encode(address),
                    lp_token,
                    input_tokens_ordered,
                    input_tokens,
                    &log,
                    blk,
                    PoolType::PlainPool(PlainPool {}),
                ))
            })
            .collect(),
    );
    Ok(())
}

fn map_meta_pool_deployed_events(
    blk: &eth::Block,
    pools: &mut Vec<Pool>,
    address: [u8; 20],
) -> Result<(), Error> {
    pools.append(
        &mut blk
            // In this block we process MetaPoolDeployed events specifically from `pool_registry_v1::events::MetaPoolDeployed`.
            // However, due to the ABI compatibility, this will also include MetaPoolDeployed events from other contracts such as
            // `crv_usd_pool_factory::events::MetaPoolDeployed` and `stable_swap_factory_ng::events::MetaPoolDeployed`,
            // since they share the same ABI structure for this event type. This ensures that all MetaPoolDeployed events,
            // regardless of the originating contract, are captured and processed here as long as they are emitted to the specified address.
            .events::<pool_registry_v1::events::MetaPoolDeployed>(&[&address])
            .filter_map(|(event, log)| {
                let transfer = match event_extraction::extract_transfer_event(&log) {
                    Ok(event) => event,
                    Err(e) => {
                        substreams::log::debug!(
                            "Error in `map_meta_pool_deployed_events`: {:?}",
                            e
                        );
                        return None;
                    }
                };
                // The pool and LP token are the same for base pools
                let metapool_address = &transfer.token_address;

                substreams::log::debug!("Metapool address is: {}", Hex::encode(metapool_address));

                let lp_token = match token::create_token(metapool_address, metapool_address, None) {
                    Ok(token) => token,
                    Err(e) => {
                        substreams::log::debug!(
                            "Error in `map_meta_pool_deployed_events`: {:?}",
                            e
                        );
                        return None;
                    }
                };
                let (input_tokens, input_tokens_ordered) =
                    match get_and_sort_input_tokens(metapool_address) {
                        Ok(result) => result,
                        Err(e) => {
                            substreams::log::debug!(
                                "Error in `map_meta_pool_deployed_events`: {:?}",
                                e
                            );
                            return None;
                        }
                    };
                if let Ok(underlying_coins) = rpc::pool::get_pool_coins(&event.base_pool) {
                    let pool_type = PoolType::MetaPool(MetaPool {
                        base_pool_address: Hex::encode(event.base_pool),
                        underlying_tokens: underlying_coins,
                        max_coin: 1,
                    });

                    substreams::log::debug!("Adding MetaPool");

                    Some(create_pool(
                        Hex::encode(metapool_address),
                        Hex::encode(address),
                        lp_token,
                        input_tokens_ordered,
                        input_tokens,
                        &log,
                        blk,
                        pool_type,
                    ))
                } else {
                    None
                }
            })
            .collect(),
    );
    Ok(())
}

fn map_tricrypto_pool_deployed_events(
    blk: &eth::Block,
    pools: &mut Vec<Pool>,
    address: [u8; 20],
) -> Result<(), Error> {
    pools.append(
        &mut blk
            .events::<tricrypto_factory_ng::events::TricryptoPoolDeployed>(&[&address])
            .filter_map(|(event, log)| {
                let lp_token = match token::create_token(&event.pool, &event.pool, None) {
                    Ok(token) => token,
                    Err(e) => {
                        substreams::log::debug!(
                            "Error in `map_tricrypto_pool_deployed_events`: {:?}",
                            e
                        );
                        return None;
                    }
                };
                let (input_tokens, input_tokens_ordered) =
                    match get_and_sort_input_tokens(&event.pool) {
                        Ok(result) => result,
                        Err(e) => {
                            substreams::log::debug!(
                                "Error in `map_tricrypto_pool_deployed_events`: {:?}",
                                e
                            );
                            return None;
                        }
                    };
                substreams::log::debug!("Added TricryptoPool");

                Some(create_pool(
                    Hex::encode(&event.pool),
                    Hex::encode(address),
                    lp_token,
                    input_tokens_ordered,
                    input_tokens,
                    &log,
                    blk,
                    PoolType::TricryptoPool(TriCryptoPool {}),
                ))
            })
            .collect(),
    );
    Ok(())
}

fn map_liquidity_gauge_deployed_events(
    blk: &eth::Block,
    gauges: &mut Vec<LiquidityGauge>,
    address: [u8; 20],
) -> Result<(), Error> {
    gauges.append(
        &mut blk
            // Although there are multiple ABIs for `LiquidityGaugeDeployed` events across different registries/factories,
            // the ABI is the same. Therefore we only need to use one of them.
            .events::<crv_usd_pool_factory::events::LiquidityGaugeDeployed>(&[&address])
            .filter_map(|(event, log)| {
                Some(LiquidityGauge {
                    gauge: Hex::encode(event.gauge),
                    pool: Hex::encode(event.pool),
                    token: None,
                    created_at_timestamp: blk.timestamp_seconds(),
                    created_at_block_number: blk.number,
                    log_ordinal: log.ordinal(),
                })
            })
            .collect(),
    );
    Ok(())
}

fn map_liquidity_gauge_deployed_with_token_events(
    blk: &eth::Block,
    gauges: &mut Vec<LiquidityGauge>,
    address: [u8; 20],
) -> Result<(), Error> {
    gauges.append(
        &mut blk
            // Although there are multiple ABIs for `LiquidityGaugeDeployed` events across different registries/factories,
            // the ABI is the same. Therefore we only need to use one of them.
            .events::<crypto_pool_factory_v2::events::LiquidityGaugeDeployed>(&[&address])
            .filter_map(|(event, log)| {
                Some(LiquidityGauge {
                    gauge: Hex::encode(event.gauge),
                    pool: Hex::encode(event.pool),
                    token: Some(Hex::encode(event.token)),
                    created_at_timestamp: blk.timestamp_seconds(),
                    created_at_block_number: blk.number,
                    log_ordinal: log.ordinal(),
                })
            })
            .collect(),
    );
    Ok(())
}

fn map_controller_new_gauge_events(
    blk: &eth::Block,
    controller_gauges: &mut Vec<ControllerNewGauge>,
    address: [u8; 20],
) -> Result<(), Error> {
    controller_gauges.append(
        &mut blk
            .events::<gauge_controller::events::NewGauge>(&[&address])
            .filter_map(|(event, log)| {
                Some(ControllerNewGauge {
                    gauge: Hex::encode(event.addr),
                    r#type: event.gauge_type.to_i32(),
                    weight: event.weight.to_string(),
                    created_at_timestamp: blk.timestamp_seconds(),
                    created_at_block_number: blk.number,
                    log_ordinal: log.ordinal(),
                })
            })
            .collect(),
    );
    Ok(())
}

fn map_crv_mining_update_events(
    blk: &eth::Block,
    address: [u8; 20],
) -> Result<Option<UpdateMiningParametersEvent>, Error> {
    let event = blk
        .events::<crv_token::events::UpdateMiningParameters>(&[&address])
        .find_map(|(event, log)| {
            Some(UpdateMiningParametersEvent {
                time: event.time.to_string(),
                rate: event.rate.to_string(),
                supply: event.supply.to_string(),
                created_at_timestamp: blk.timestamp_seconds(),
                created_at_block_number: blk.number,
                log_ordinal: log.ordinal(),
            })
        });
    Ok(event)
}

fn create_pool(
    address: String,
    registry_address: String,
    lp_token: Token,
    input_tokens_ordered: Vec<String>,
    input_tokens: Vec<Token>,
    log: &block_view::LogView,
    blk: &eth::Block,
    pool_type: PoolType,
) -> Pool {
    Pool {
        address,
        name: lp_token.name.clone(),
        symbol: lp_token.symbol.clone(),
        created_at_timestamp: blk.timestamp_seconds(),
        created_at_block_number: blk.number,
        log_ordinal: log.ordinal(),
        transaction_id: Hex(&log.receipt.transaction.hash).to_string(),
        registry_address,
        output_token: Some(lp_token),
        input_tokens_ordered,
        input_tokens,
        pool_type: Some(pool_type),
    }
}

fn create_missing_pool(
    address: String,
    registry_address: String,
    lp_token: Token,
    input_tokens_ordered: Vec<String>,
    input_tokens: Vec<Token>,
    blk: &eth::Block,
    hash: Vec<u8>,
    pool_type: PoolType,
) -> Pool {
    Pool {
        address,
        name: lp_token.name.clone(),
        symbol: lp_token.symbol.clone(),
        created_at_timestamp: blk.timestamp_seconds(),
        created_at_block_number: blk.number,
        log_ordinal: 0,
        transaction_id: Hex::encode(hash),
        registry_address,
        output_token: Some(lp_token),
        input_tokens_ordered,
        input_tokens,
        pool_type: Some(pool_type),
    }
}

// This follows the logic from the original subgraph.
// An array of token addresses, and a sorted array of token structs is required.
fn get_and_sort_input_tokens(pool_address: &Vec<u8>) -> Result<(Vec<Token>, Vec<String>), Error> {
    let mut input_tokens = pool::get_pool_coins(&pool_address)?;
    let input_tokens_ordered = input_tokens
        .clone()
        .into_iter()
        .map(|token| token.address)
        .collect();
    input_tokens.sort_by(|a, b| a.address.cmp(&b.address));

    Ok((input_tokens, input_tokens_ordered))
}

// TODO delete after testing
fn seed_pool_for_testing(pools: &mut Vec<Pool>) {
    pools.push(get_metapool());
}
fn get_metapool() -> Pool {
    Pool {
        name: "Curve.fi Factory USD Metapool: L3USD3CRV".to_string(),
        symbol: "L3USD3CRV3CRV-f".to_string(),
        address: "79ce6be6ae0995b1c8ed3e8ae54de0e437dec8c3".to_string(),
        created_at_timestamp: 1701611519,
        created_at_block_number: 18706277,
        log_ordinal: 2844,
        transaction_id: "3df74962bc58833ee086d4474b3dd5286801cc5d5f7c12be92626097fe3fe74c"
            .to_string(),
        registry_address: "b9fc157394af804a3578134a6585c0dc9cc990d4".to_string(),
        output_token: Some(Token {
            address: "79ce6be6ae0995b1c8ed3e8ae54de0e437dec8c3".to_string(),
            name: "Curve.fi Factory USD Metapool: L3USD3CRV".to_string(),
            symbol: "L3USD3CRV3CRV-f".to_string(),
            decimals: 18,
            total_supply: "0".to_string(),
            is_base_pool_lp_token: false,
            gauge: None,
        }),
        input_tokens_ordered: vec![
            "2c2d8a078b33bf7782a16acce2c5ba6653a90d5f".to_string(),
            "6c3f90f043a72fa612cbac8115ee7e52bde6e490".to_string(),
        ],
        input_tokens: vec![
            Token {
                address: "2c2d8a078b33bf7782a16acce2c5ba6653a90d5f".to_string(),
                name: "L3USD".to_string(),
                symbol: "L3USD".to_string(),
                decimals: 18,
                total_supply: "88888888000000000000000000".to_string(),
                is_base_pool_lp_token: false,
                gauge: None,
            },
            Token {
                address: "6c3f90f043a72fa612cbac8115ee7e52bde6e490".to_string(),
                name: "Curve.fi DAI/USDC/USDT".to_string(),
                symbol: "3Crv".to_string(),
                decimals: 18,
                total_supply: "190535607806721468949805568".to_string(),
                is_base_pool_lp_token: true,
                gauge: None,
            },
        ],
        pool_type: Some(PoolType::MetaPool(MetaPool {
            base_pool_address: "bebc44782c7db0a1a60cb6fe97d0b483032ff1c7".to_string(),
            underlying_tokens: vec![
                Token {
                    address: "6b175474e89094c44da98b954eedeac495271d0f".to_string(),
                    name: "Dai Stablecoin".to_string(),
                    symbol: "DAI".to_string(),
                    decimals: 18,
                    total_supply: "3674325983605876519355232114".to_string(),
                    is_base_pool_lp_token: true,
                    gauge: None,
                },
                Token {
                    address: "a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".to_string(),
                    name: "USD Coin".to_string(),
                    symbol: "USDC".to_string(),
                    decimals: 6,
                    total_supply: "22508074118982608".to_string(),
                    is_base_pool_lp_token: true,
                    gauge: None,
                },
                Token {
                    address: "dac17f958d2ee523a2206206994597c13d831ec7".to_string(),
                    name: "Tether USD".to_string(),
                    symbol: "USDT".to_string(),
                    decimals: 6,
                    total_supply: "41013387300953492".to_string(),
                    is_base_pool_lp_token: true,
                    gauge: None,
                },
            ],
            max_coin: 1,
        })),
    }
}
