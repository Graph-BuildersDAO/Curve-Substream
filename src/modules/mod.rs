#[path = "1_map_pools_created.rs"]
mod map_pools_created;

#[path = "2_store_pools_created.rs"]
mod store_pools_created;

#[path = "3_store_pool_count.rs"]
mod store_pool_count;

#[path = "4_store_pool_addresses.rs"]
mod store_pool_addresses;

#[path = "4_store_tokens.rs"]
mod store_tokens;

#[path = "5_map_extract_pool_events.rs"]
mod map_extract_pool_events;

#[path = "6_store_pool_fees.rs"]
mod store_pool_fees;

#[path = "7_store_output_token_supply.rs"]
mod store_output_token_supply;

#[path = "8_store_input_token_balances.rs"]
mod store_input_token_balances;

#[path = "9_store_pool_volume_native.rs"]
mod store_pool_volume_native;

#[path = "10_store_pool_volume_usd.rs"]
mod store_pool_volume_usd;

#[path = "11_store_protocol_volume_usd.rs"]
mod store_protocol_volume_usd;

#[path = "12_store_pool_tvl.rs"]
mod store_pool_tvl;

#[path = "13_store_protocol_tvl.rs"]
mod store_protocol_tvl;

#[path = "14_store_active_users.rs"]
mod store_active_users;

#[path = "15_store_usage_metrics.rs"]
mod store_usage_metrics;

#[path = "16_store_current_time_snapshots.rs"]
mod store_current_time_snapshots;

// TODO: Will decrement once we have added and finalised all the other modules.
#[path = "420_graph_out.rs"]
mod graph_out;

pub use graph_out::graph_out;
pub use map_extract_pool_events::map_extract_pool_events;
pub use map_pools_created::map_pools_created;
pub use store_active_users::store_active_users;
pub use store_current_time_snapshots::store_current_time_snapshots;
pub use store_input_token_balances::store_input_token_balances;
pub use store_output_token_supply::store_output_token_supply;
pub use store_pool_addresses::store_pool_addresses;
pub use store_pool_count::store_pool_count;
pub use store_pool_fees::store_pool_fees;
pub use store_pool_tvl::store_pool_tvl;
pub use store_pool_volume_native::store_pool_volume_native;
pub use store_pool_volume_usd::store_pool_volume_usd;
pub use store_pools_created::store_pools_created;
pub use store_protocol_tvl::store_protocol_tvl;
pub use store_protocol_volume_usd::store_protocol_volume_usd;
pub use store_tokens::store_tokens;
pub use store_usage_metrics::store_usage_metrics;
