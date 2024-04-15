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
    /// This refers to the index of the coin relative to the pools `coins` function
    #[prost(string, tag="1")]
    pub index: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub decimals: u64,
    #[prost(string, tag="6")]
    pub total_supply: ::prost::alloc::string::String,
    #[prost(bool, tag="7")]
    pub is_base_pool_lp_token: bool,
    /// Optional field to track the gauge for reward tokens
    #[prost(string, optional, tag="8")]
    pub gauge: ::core::option::Option<::prost::alloc::string::String>,
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
    #[prost(oneof="pool::PoolType", tags="12, 13, 14, 15, 16, 17, 18")]
    pub pool_type: ::core::option::Option<pool::PoolType>,
}
/// Nested message and enum types in `Pool`.
pub mod pool {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PoolType {
        #[prost(message, tag="12")]
        PlainPool(super::PlainPool),
        #[prost(message, tag="13")]
        CryptoPool(super::CryptoPool),
        #[prost(message, tag="14")]
        TricryptoPool(super::TriCryptoPool),
        #[prost(message, tag="15")]
        TwocryptoPool(super::TwoCryptoPool),
        #[prost(message, tag="16")]
        MetaPool(super::MetaPool),
        #[prost(message, tag="17")]
        LendingPool(super::LendingPool),
        #[prost(message, tag="18")]
        WildcardPool(super::WildcardPool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolRewards {
    #[prost(string, tag="1")]
    pub staked_output_token_amount: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub reward_token_emissions_native: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub reward_token_emissions_usd: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlainPool {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoPool {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriCryptoPool {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TwoCryptoPool {
}
/// Base Metapool structure
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaPool {
    #[prost(string, tag="1")]
    pub base_pool_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub underlying_tokens: ::prost::alloc::vec::Vec<Token>,
    #[prost(uint64, tag="3")]
    pub max_coin: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendingPool {
    #[prost(message, repeated, tag="2")]
    pub underlying_tokens: ::prost::alloc::vec::Vec<Token>,
    #[prost(oneof="lending_pool::LendingPoolType", tags="3, 4, 5, 6, 7, 8")]
    pub lending_pool_type: ::core::option::Option<lending_pool::LendingPoolType>,
}
/// Nested message and enum types in `LendingPool`.
pub mod lending_pool {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompoundLending {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompoundTetherLending {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AaveLending {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct YiEarnLending {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IronBankLending {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PaxLending {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LendingPoolType {
        #[prost(message, tag="3")]
        CompoundLending(CompoundLending),
        #[prost(message, tag="4")]
        CompoundTetherLending(CompoundTetherLending),
        #[prost(message, tag="5")]
        AaveLending(AaveLending),
        #[prost(message, tag="6")]
        YIearnLending(YiEarnLending),
        #[prost(message, tag="7")]
        IronbankLending(IronBankLending),
        #[prost(message, tag="8")]
        PaxLending(PaxLending),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WildcardPool {
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
    pub liquidity_events: ::prost::alloc::vec::Vec<LiquidityEvent>,
    #[prost(message, repeated, tag="2")]
    pub add_reward_events: ::prost::alloc::vec::Vec<AddRewardEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityEvent {
    #[prost(string, tag="1")]
    pub gauge: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pool: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub value: ::prost::alloc::string::String,
    #[prost(enumeration="GaugeLiquidityEventType", tag="5")]
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
pub struct AddRewardEvent {
    #[prost(string, tag="1")]
    pub gauge: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pool: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub reward_token: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub distributor: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="6")]
    pub tx_index: u32,
    #[prost(uint64, tag="7")]
    pub timestamp: u64,
    #[prost(uint64, tag="8")]
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
        #[prost(oneof="pool_event::Type", tags="1, 2, 3, 4, 5")]
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
        pub struct SwapUnderlyingMetaEvent {
            #[prost(message, optional, tag="1")]
            pub token_in: ::core::option::Option<TokenAmount>,
            #[prost(message, optional, tag="2")]
            pub token_out: ::core::option::Option<TokenAmount>,
            #[prost(string, tag="3")]
            pub base_pool_address: ::prost::alloc::string::String,
            #[prost(message, optional, tag="4")]
            pub lp_token_change: ::core::option::Option<LpTokenChange>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SwapUnderlyingLendingEvent {
            /// The underlying token provided by the user.
            #[prost(message, optional, tag="1")]
            pub token_in: ::core::option::Option<TokenAmount>,
            /// The underlying token received by the user.
            #[prost(message, optional, tag="2")]
            pub token_out: ::core::option::Option<TokenAmount>,
            /// Details the action (mint/burn) on the interest-bearing token corresponding to `token_in`.
            /// This reflects the change in the lending pool's liquidity for the `token_in` asset.
            /// Example:
            ///    - In an exchange of USDC for USDT within a lending-based Curve pool:
            ///      1. USDC is deposited into the Curve lending pool.
            ///      2. The deposited USDC is used to mint the interest-bearing token (e.g., aUSDC in Aave).
            ///      3. The pool burns some of its interest-bearing token for USDT (e.g., aUSDT), unlocking USDT.
            ///      4. The redeemed USDT is sent to the user initiating the exchange.
            ///      5. As a result, the lending pool's balance of aUSDC increases, and its balance of aUSDT decreases.
            #[prost(message, optional, tag="3")]
            pub interest_bearing_token_in_action: ::core::option::Option<LpTokenChange>,
            /// Similar to `interest_bearing_token_in_action`, but for the `token_out` asset.
            #[prost(message, optional, tag="4")]
            pub interest_bearing_token_out_action: ::core::option::Option<LpTokenChange>,
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
            #[prost(string, tag="3")]
            pub amount_usd: ::prost::alloc::string::String,
            #[prost(enumeration="TokenSource", tag="4")]
            pub source: i32,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct LpTokenChange {
            #[prost(string, tag="1")]
            pub token_address: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub amount: ::prost::alloc::string::String,
            #[prost(enumeration="LpTokenChangeType", tag="3")]
            pub change_type: i32,
        }
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum TokenSource {
            /// Default value, used when not dealing with a metapool `TokenExchangeUnderlying` event
            Default = 0,
            MetaPool = 1,
            BasePool = 2,
            /// The Curve Lending pool
            LendingPool = 3,
            /// The Lending Protocol associated with a Curve Lending Pool
            /// For example, if the Compound Curve Lending Pool has a DAI -> USDC swap, the USDC's source is the cUSDC contract
            LendingProtcol = 4,
        }
        impl TokenSource {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    TokenSource::Default => "DEFAULT",
                    TokenSource::MetaPool => "META_POOL",
                    TokenSource::BasePool => "BASE_POOL",
                    TokenSource::LendingPool => "LENDING_POOL",
                    TokenSource::LendingProtcol => "LENDING_PROTCOL",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "DEFAULT" => Some(Self::Default),
                    "META_POOL" => Some(Self::MetaPool),
                    "BASE_POOL" => Some(Self::BasePool),
                    "LENDING_POOL" => Some(Self::LendingPool),
                    "LENDING_PROTCOL" => Some(Self::LendingProtcol),
                    _ => None,
                }
            }
        }
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum LpTokenChangeType {
            Mint = 0,
            Burn = 1,
        }
        impl LpTokenChangeType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    LpTokenChangeType::Mint => "MINT",
                    LpTokenChangeType::Burn => "BURN",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "MINT" => Some(Self::Mint),
                    "BURN" => Some(Self::Burn),
                    _ => None,
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            #[prost(message, tag="1")]
            SwapEvent(SwapEvent),
            #[prost(message, tag="2")]
            SwapUnderlyingMetaEvent(SwapUnderlyingMetaEvent),
            #[prost(message, tag="3")]
            SwapUnderlyingLendingEvent(SwapUnderlyingLendingEvent),
            #[prost(message, tag="4")]
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
pub enum GaugeLiquidityEventType {
    Deposit = 0,
    Withdraw = 1,
}
impl GaugeLiquidityEventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GaugeLiquidityEventType::Deposit => "DEPOSIT",
            GaugeLiquidityEventType::Withdraw => "WITHDRAW",
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
