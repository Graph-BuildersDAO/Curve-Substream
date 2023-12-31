#[path = "1_map_pools_created.rs"]
mod map_pools_created;

#[path = "2_store_pools_created.rs"]
mod store_pools_created;

#[path = "3_store_pool_count.rs"]
mod store_pool_count;

#[path = "4_store_tokens.rs"]
mod store_tokens;

#[path = "5_map_extract_pool_events.rs"]
mod map_extract_pool_events;

// TODO: Will decrement once we have added and finalised all the other modules.
#[path = "420_graph_out.rs"]
mod graph_out;

pub use graph_out::graph_out;
pub use map_extract_pool_events::map_extract_pool_events;
pub use map_pools_created::map_pools_created;
pub use store_pool_count::store_pool_count;
pub use store_pools_created::store_pools_created;
pub use store_tokens::store_tokens;
