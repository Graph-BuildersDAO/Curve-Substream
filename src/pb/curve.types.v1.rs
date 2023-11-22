// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tokens {
    #[prost(message, repeated, tag="1")]
    pub tokens: ::prost::alloc::vec::Vec<Token>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub decimals: u64,
    #[prost(string, tag="5")]
    pub total_supply: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub is_base_pool_lp_token: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub created_at_timestamp: u64,
    #[prost(uint64, tag="5")]
    pub created_at_block_number: u64,
    #[prost(uint64, tag="6")]
    pub log_ordinal: u64,
    #[prost(string, tag="7")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub registry_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="9")]
    pub output_token: ::core::option::Option<Token>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pools {
    #[prost(message, repeated, tag="1")]
    pub pools: ::prost::alloc::vec::Vec<Pool>,
}
// @@protoc_insertion_point(module)
