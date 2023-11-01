mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const POOLREGISTRY_TRACKED_CONTRACT: [u8; 20] = hex!("90e00ace148ca3b23ac1bc8c240c2a7dd9c2d7f5");
const POOLREGISTRY2_TRACKED_CONTRACT: [u8; 20] = hex!("7d86446ddb609ed0f5f8684acf30380a356b2b4c");

fn map_poolregistry_events(blk: &eth::Block, events: &mut contract::Events) {
    events.poolregistry_pool_addeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == POOLREGISTRY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::poolregistry_contract::events::PoolAdded::match_and_decode(log) {
                        return Some(contract::PoolregistryPoolAdded {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            pool: event.pool,
                            rate_method_id: event.rate_method_id,
                        });
                    }

                    None
                })
        })
        .collect());
    events.poolregistry_pool_removeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == POOLREGISTRY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::poolregistry_contract::events::PoolRemoved::match_and_decode(log) {
                        return Some(contract::PoolregistryPoolRemoved {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            pool: event.pool,
                        });
                    }

                    None
                })
        })
        .collect());
}

fn map_poolregistry2_events(blk: &eth::Block, events: &mut contract::Events) {
    events.poolregistry2_pool_addeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == POOLREGISTRY2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::poolregistry2_contract::events::PoolAdded::match_and_decode(log) {
                        return Some(contract::Poolregistry2PoolAdded {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            pool: event.pool,
                            rate_method_id: event.rate_method_id,
                        });
                    }

                    None
                })
        })
        .collect());
    events.poolregistry2_pool_removeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == POOLREGISTRY2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::poolregistry2_contract::events::PoolRemoved::match_and_decode(log) {
                        return Some(contract::Poolregistry2PoolRemoved {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            pool: event.pool,
                        });
                    }

                    None
                })
        })
        .collect());
}


fn graph_poolregistry_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.poolregistry_pool_addeds.iter().for_each(|evt| {
        tables
            .create_row("poolregistry_pool_added", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("pool", Hex(&evt.pool).to_string())
            .set("rate_method_id", Hex(&evt.rate_method_id).to_string());
    });
    events.poolregistry_pool_removeds.iter().for_each(|evt| {
        tables
            .create_row("poolregistry_pool_removed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("pool", Hex(&evt.pool).to_string());
    });
}
fn graph_poolregistry2_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.poolregistry2_pool_addeds.iter().for_each(|evt| {
        tables
            .create_row("poolregistry2_pool_added", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("pool", Hex(&evt.pool).to_string())
            .set("rate_method_id", Hex(&evt.rate_method_id).to_string());
    });
    events.poolregistry2_pool_removeds.iter().for_each(|evt| {
        tables
            .create_row("poolregistry2_pool_removed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("pool", Hex(&evt.pool).to_string());
    });
}

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_poolregistry_events(&blk, &mut events);
    map_poolregistry2_events(&blk, &mut events);
    Ok(events)
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_poolregistry_out(&events, &mut tables);
    graph_poolregistry2_out(&events, &mut tables);
    Ok(tables.to_entity_changes())
}
