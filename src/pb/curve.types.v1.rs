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
pub struct LiquidityGauge {
    #[prost(string, tag="1")]
    pub pool: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub gauge: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, tag="4")]
    pub created_at_timestamp: u64,
    #[prost(uint64, tag="5")]
    pub created_at_block_number: u64,
    #[prost(uint64, tag="6")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityGauges {
    #[prost(message, repeated, tag="1")]
    pub liquidity_gauges: ::prost::alloc::vec::Vec<LiquidityGauge>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityGaugeEvents {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<LiquidityGaugeEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityGaugeEvent {
    #[prost(string, tag="1")]
    pub gauge: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pool: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub value: ::prost::alloc::string::String,
    #[prost(enumeration="LiquidityGaugeEventType", tag="5")]
    pub r#type: i32,
    #[prost(string, tag="6")]
    pub working_supply: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="8")]
    pub tx_index: u32,
    #[prost(uint32, tag="9")]
    pub log_index: u32,
    #[prost(uint64, tag="10")]
    pub log_ordinal: u64,
    #[prost(uint64, tag="11")]
    pub timestamp: u64,
    #[prost(uint64, tag="12")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControllerNewGauge {
    #[prost(string, tag="1")]
    pub gauge: ::prost::alloc::string::String,
    #[prost(enumeration="GaugeType", tag="2")]
    pub r#type: i32,
    /// String representation of BigInt
    #[prost(string, tag="3")]
    pub weight: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub created_at_timestamp: u64,
    #[prost(uint64, tag="5")]
    pub created_at_block_number: u64,
    #[prost(uint64, tag="6")]
    pub log_ordinal: u64,
}
/// This event is emitted from the CRV token, and allows us to keep track of in the inflation rate
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMiningParametersEvent {
    #[prost(string, tag="1")]
    pub time: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub rate: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub supply: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub created_at_timestamp: u64,
    #[prost(uint64, tag="5")]
    pub created_at_block_number: u64,
    #[prost(uint64, tag="6")]
    pub log_ordinal: u64,
}
/// This includes pool and gauge deployments, and GaugeController add events.
/// When already deployed gauges are added to the controller, they become eligible for CRV rewards.
/// We also track UpdateMiningParametersEvent from the CRV contract, to keep track of inflation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurveEvents {
    #[prost(message, repeated, tag="1")]
    pub pools: ::prost::alloc::vec::Vec<Pool>,
    #[prost(message, repeated, tag="2")]
    pub gauges: ::prost::alloc::vec::Vec<LiquidityGauge>,
    #[prost(message, repeated, tag="3")]
    pub controller_gauges: ::prost::alloc::vec::Vec<ControllerNewGauge>,
    #[prost(message, optional, tag="4")]
    pub update_mining_parameters_event: ::core::option::Option<UpdateMiningParametersEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolFee {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration="LiquidityPoolFeeType", tag="2")]
    pub fee_type: i32,
    /// BigDecimal string representation
    #[prost(string, tag="3")]
    pub fee_percentage: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolFees {
    #[prost(message, optional, tag="1")]
    pub trading_fee: ::core::option::Option<PoolFee>,
    #[prost(message, optional, tag="2")]
    pub protocol_fee: ::core::option::Option<PoolFee>,
    #[prost(message, optional, tag="3")]
    pub lp_fee: ::core::option::Option<PoolFee>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub pool_events: ::prost::alloc::vec::Vec<events::PoolEvent>,
    #[prost(message, repeated, tag="2")]
    pub fee_changes_events: ::prost::alloc::vec::Vec<events::FeeChangeEvent>,
}
/// Nested message and enum types in `Events`.
pub mod events {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PoolEvent {
        /// Common fields
        #[prost(string, tag="6")]
        pub transaction_hash: ::prost::alloc::string::String,
        #[prost(uint32, tag="7")]
        pub tx_index: u32,
        #[prost(uint32, tag="8")]
        pub log_index: u32,
        #[prost(uint64, tag="9")]
        pub log_ordinal: u64,
        #[prost(string, tag="10")]
        pub to_address: ::prost::alloc::string::String,
        #[prost(string, tag="11")]
        pub from_address: ::prost::alloc::string::String,
        #[prost(uint64, tag="12")]
        pub timestamp: u64,
        #[prost(uint64, tag="13")]
        pub block_number: u64,
        /// TODO is there benefit in storing the total supply here or in the event types?
        #[prost(string, tag="14")]
        pub pool_address: ::prost::alloc::string::String,
        #[prost(oneof="pool_event::Type", tags="1, 2, 3, 5")]
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
        pub struct SwapUnderlyingEvent {
            #[prost(message, optional, tag="1")]
            pub token_in: ::core::option::Option<TokenAmount>,
            #[prost(message, optional, tag="2")]
            pub token_out: ::core::option::Option<TokenAmount>,
            #[prost(string, tag="3")]
            pub base_pool_address: ::prost::alloc::string::String,
            #[prost(message, optional, tag="4")]
            pub lp_token_burnt: ::core::option::Option<TokenAmount>,
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
            SwapUnderlyingEvent(SwapUnderlyingEvent),
            #[prost(message, tag="3")]
            DepositEvent(DepositEvent),
            #[prost(message, tag="5")]
            WithdrawEvent(WithdrawEvent),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FeeChangeEvent {
        #[prost(string, tag="1")]
        pub transaction_hash: ::prost::alloc::string::String,
        #[prost(uint32, tag="2")]
        pub tx_index: u32,
        #[prost(uint32, tag="3")]
        pub log_index: u32,
        #[prost(uint64, tag="4")]
        pub log_ordinal: u64,
        #[prost(uint64, tag="5")]
        pub timestamp: u64,
        #[prost(uint64, tag="6")]
        pub block_number: u64,
        #[prost(string, tag="7")]
        pub fee: ::prost::alloc::string::String,
        #[prost(string, optional, tag="8")]
        pub admin_fee: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, tag="9")]
        pub pool_address: ::prost::alloc::string::String,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LiquidityGaugeEventType {
    Deposit = 0,
    Withdraw = 1,
}
impl LiquidityGaugeEventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LiquidityGaugeEventType::Deposit => "DEPOSIT",
            LiquidityGaugeEventType::Withdraw => "WITHDRAW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEPOSIT" => Some(Self::Deposit),
            "WITHDRAW" => Some(Self::Withdraw),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GaugeType {
    EthereumStable = 0,
    Fantom = 1,
    Polygon = 2,
    Deprecated1 = 3,
    Gnosis = 4,
    EthereumCrypto = 5,
    Deprecated2 = 6,
    Arbitrum = 7,
    Avalanche = 8,
    Harmony = 9,
    Fundraising = 10,
    Optimism = 11,
    BinanceSmartChain = 12,
}
impl GaugeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GaugeType::EthereumStable => "ETHEREUM_STABLE",
            GaugeType::Fantom => "FANTOM",
            GaugeType::Polygon => "POLYGON",
            GaugeType::Deprecated1 => "DEPRECATED_1",
            GaugeType::Gnosis => "GNOSIS",
            GaugeType::EthereumCrypto => "ETHEREUM_CRYPTO",
            GaugeType::Deprecated2 => "DEPRECATED_2",
            GaugeType::Arbitrum => "ARBITRUM",
            GaugeType::Avalanche => "AVALANCHE",
            GaugeType::Harmony => "HARMONY",
            GaugeType::Fundraising => "FUNDRAISING",
            GaugeType::Optimism => "OPTIMISM",
            GaugeType::BinanceSmartChain => "BINANCE_SMART_CHAIN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ETHEREUM_STABLE" => Some(Self::EthereumStable),
            "FANTOM" => Some(Self::Fantom),
            "POLYGON" => Some(Self::Polygon),
            "DEPRECATED_1" => Some(Self::Deprecated1),
            "GNOSIS" => Some(Self::Gnosis),
            "ETHEREUM_CRYPTO" => Some(Self::EthereumCrypto),
            "DEPRECATED_2" => Some(Self::Deprecated2),
            "ARBITRUM" => Some(Self::Arbitrum),
            "AVALANCHE" => Some(Self::Avalanche),
            "HARMONY" => Some(Self::Harmony),
            "FUNDRAISING" => Some(Self::Fundraising),
            "OPTIMISM" => Some(Self::Optimism),
            "BINANCE_SMART_CHAIN" => Some(Self::BinanceSmartChain),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LiquidityPoolFeeType {
    Unknown = 0,
    FixedTradingFee = 1,
    FixedProtocolFee = 2,
    FixedLpFee = 3,
}
impl LiquidityPoolFeeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LiquidityPoolFeeType::Unknown => "UNKNOWN",
            LiquidityPoolFeeType::FixedTradingFee => "FIXED_TRADING_FEE",
            LiquidityPoolFeeType::FixedProtocolFee => "FIXED_PROTOCOL_FEE",
            LiquidityPoolFeeType::FixedLpFee => "FIXED_LP_FEE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "FIXED_TRADING_FEE" => Some(Self::FixedTradingFee),
            "FIXED_PROTOCOL_FEE" => Some(Self::FixedProtocolFee),
            "FIXED_LP_FEE" => Some(Self::FixedLpFee),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
