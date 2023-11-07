// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub created_at_timestamp: u64,
    #[prost(uint64, tag="3")]
    pub created_at_block_number: u64,
    #[prost(uint64, tag="4")]
    pub log_ordinal: u64,
    #[prost(string, tag="5")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub registry_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pools {
    #[prost(message, repeated, tag="1")]
    pub pools: ::prost::alloc::vec::Vec<Pool>,
}
// @@protoc_insertion_point(module)
