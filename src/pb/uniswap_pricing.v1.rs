// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FactoryEvents {
    #[prost(message, repeated, tag="1")]
    pub pair_createds: ::prost::alloc::vec::Vec<PairCreated>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PairCreated {
    #[prost(string, tag="1")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub block_index: u32,
    #[prost(message, optional, tag="3")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub block_number: u64,
    #[prost(uint64, tag="5")]
    pub ordinal: u64,
    #[prost(message, optional, tag="6")]
    pub token0: ::core::option::Option<Erc20Token>,
    #[prost(message, optional, tag="7")]
    pub token1: ::core::option::Option<Erc20Token>,
    #[prost(string, tag="8")]
    pub pair_address: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub factory: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Tokens {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Erc20Token>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Token {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub decimals: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Price {
    #[prost(message, optional, tag="1")]
    pub token: ::core::option::Option<Erc20Token>,
    #[prost(string, tag="2")]
    pub price_usd: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub block_number: u64,
    #[prost(uint64, tag="4")]
    pub ordinal: u64,
    #[prost(enumeration="erc20_price::Source", tag="5")]
    pub source: i32,
}
/// Nested message and enum types in `Erc20Price`.
pub mod erc20_price {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Source {
        Oracles = 0,
        Chainlink = 1,
        Uniswap = 2,
    }
    impl Source {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Source::Oracles => "ORACLES",
                Source::Chainlink => "CHAINLINK",
                Source::Uniswap => "UNISWAP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ORACLES" => Some(Self::Oracles),
                "CHAINLINK" => Some(Self::Chainlink),
                "UNISWAP" => Some(Self::Uniswap),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Prices {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Erc20Price>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Warmup {
    #[prost(bool, tag="1")]
    pub is_warm: bool,
}
// @@protoc_insertion_point(module)
