#[path = "1_map_curve_events.rs"]
mod map_curve_events;

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

#[path = "16_store_current_time.rs"]
mod store_current_time;

#[path = "17_store_gauges.rs"]
mod store_gauges;

#[path = "18_store_controller_gauges.rs"]
mod store_controller_gauges;

#[path = "19_map_gauge_events.rs"]
mod map_gauge_events;

#[path = "20_store_crv_inflation.rs"]
mod store_crv_inflation;

#[path = "21_store_reward_token_count.rs"]
mod store_reward_token_count;

#[path = "22_store_reward_tokens.rs"]
mod store_reward_tokens;

#[path = "23_store_pool_rewards.rs"]
mod store_pool_rewards;

// TODO: Will decrement once we have added and finalised all the other modules.
#[path = "420_graph_out.rs"]
mod graph_out;

pub use graph_out::graph_out;
pub use map_curve_events::map_curve_events;
pub use map_extract_pool_events::map_extract_pool_events;
pub use map_gauge_events::map_gauge_events;
pub use store_active_users::store_active_users;
pub use store_controller_gauges::store_controller_gauges;
pub use store_crv_inflation::store_crv_inflation;
pub use store_current_time::store_current_time;
pub use store_gauges::store_gauges;
pub use store_input_token_balances::store_input_token_balances;
pub use store_output_token_supply::store_output_token_supply;
pub use store_pool_addresses::store_pool_addresses;
pub use store_pool_count::store_pool_count;
pub use store_pool_fees::store_pool_fees;
pub use store_pool_rewards::store_pool_rewards;
pub use store_pool_tvl::store_pool_tvl;
pub use store_pool_volume_native::store_pool_volume_native;
pub use store_pool_volume_usd::store_pool_volume_usd;
pub use store_pools_created::store_pools_created;
pub use store_protocol_tvl::store_protocol_tvl;
pub use store_protocol_volume_usd::store_protocol_volume_usd;
pub use store_reward_token_count::store_reward_token_count;
pub use store_reward_tokens::store_reward_tokens;
pub use store_tokens::store_tokens;
pub use store_usage_metrics::store_usage_metrics;
