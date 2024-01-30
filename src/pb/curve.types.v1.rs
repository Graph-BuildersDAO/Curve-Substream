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
    #[prost(string, repeated, tag="10")]
    pub input_tokens_ordered: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="11")]
    pub input_tokens: ::prost::alloc::vec::Vec<Token>,
    #[prost(bool, tag="12")]
    pub is_metapool: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pools {
    #[prost(message, repeated, tag="1")]
    pub pools: ::prost::alloc::vec::Vec<Pool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub pool_events: ::prost::alloc::vec::Vec<events::PoolEvent>,
}
/// Nested message and enum types in `Events`.
pub mod events {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PoolEvent {
        /// Common fields
        #[prost(string, tag="4")]
        pub transaction_hash: ::prost::alloc::string::String,
        #[prost(uint32, tag="5")]
        pub tx_index: u32,
        #[prost(uint32, tag="6")]
        pub log_index: u32,
        #[prost(uint64, tag="7")]
        pub log_ordinal: u64,
        #[prost(string, tag="8")]
        pub to_address: ::prost::alloc::string::String,
        #[prost(string, tag="9")]
        pub from_address: ::prost::alloc::string::String,
        #[prost(uint64, tag="10")]
        pub timestamp: u64,
        #[prost(uint64, tag="11")]
        pub block_number: u64,
        /// TODO is there benefit in storing the total supply here or in the event types?
        #[prost(string, tag="12")]
        pub pool_address: ::prost::alloc::string::String,
        #[prost(oneof="pool_event::Type", tags="1, 2, 3")]
        pub r#type: ::core::option::Option<pool_event::Type>,
    }
    /// Nested message and enum types in `PoolEvent`.
    pub mod pool_event {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SwapEvent {
            #[prost(message, optional, tag="1")]
            pub token_in: ::core::option::Option<TokenAmount>,
            #[prost(message, optional, tag="2")]
            pub token_out: ::core::option::Option<TokenAmount>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DepositEvent {
            #[prost(message, repeated, tag="1")]
            pub input_tokens: ::prost::alloc::vec::Vec<TokenAmount>,
            #[prost(message, optional, tag="2")]
            pub output_token: ::core::option::Option<TokenAmount>,
            #[prost(string, repeated, tag="3")]
            pub fees: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct WithdrawEvent {
            #[prost(message, repeated, tag="1")]
            pub input_tokens: ::prost::alloc::vec::Vec<TokenAmount>,
            #[prost(message, optional, tag="2")]
            pub output_token: ::core::option::Option<TokenAmount>,
            #[prost(string, repeated, tag="3")]
            pub fees: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TokenAmount {
            #[prost(string, tag="1")]
            pub token_address: ::prost::alloc::string::String,
            /// BigInt in token's native amount
            #[prost(string, tag="2")]
            pub amount: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            #[prost(message, tag="1")]
            SwapEvent(SwapEvent),
            #[prost(message, tag="2")]
            DepositEvent(DepositEvent),
            #[prost(message, tag="3")]
            WithdrawEvent(WithdrawEvent),
        }
    }
}
// @@protoc_insertion_point(module)
