pub enum StoreKey {
    // Curve sustream specific store key variants
    Pool(String),
    PoolFees(String),
    PoolVolumeUsd(String),
    PoolDailyVolumeUsd(String, i64),
    PoolHourlyVolumeUsd(String, i64),
    PoolTokenDailyVolumeNative(String, String, i64),
    PoolTokenHourlyVolumeNative(String, String, i64),
    PoolTokenDailyVolumeUsd(String, String, i64),
    PoolTokenHourlyVolumeUsd(String, String, i64),
    PoolTvl(String),
    ProtocolPoolCount,
    ProtocolVolumeUsd,
    ProtocolDailyVolumeUsd(i64),
    ProtocolTvl,
    Token(String),
    OutputTokenSupply(String),
    InputTokenBalance(String, String),
    // External packages store key variants
    UniswapPriceByTokenAddress(String),
    UniswapPriceByTokenSymbol(String),
    ChainlinkPriceBySymbol(String),
}

impl StoreKey {
    pub fn pool_key(pool_address: &str) -> String {
        StoreKey::Pool(pool_address.to_string()).to_key_string()
    }

    pub fn pool_fees_key(pool_address: &str) -> String {
        StoreKey::PoolFees(pool_address.to_string()).to_key_string()
    }

    pub fn pool_volume_usd_key(pool_address: &str) -> String {
        StoreKey::PoolVolumeUsd(pool_address.to_string()).to_key_string()
    }

    pub fn pool_volume_usd_daily_key(pool_address: &str, day_id: &i64) -> String {
        StoreKey::PoolDailyVolumeUsd(pool_address.to_string(), *day_id).to_key_string()
    }

    pub fn pool_volume_usd_hourly_key(pool_address: &str, hour_id: &i64) -> String {
        StoreKey::PoolHourlyVolumeUsd(pool_address.to_string(), *hour_id).to_key_string()
    }

    pub fn pool_token_volume_native_daily_key(
        pool_address: &str,
        token_address: &str,
        day_id: &i64,
    ) -> String {
        StoreKey::PoolTokenDailyVolumeNative(
            pool_address.to_string(),
            token_address.to_string(),
            *day_id,
        )
        .to_key_string()
    }

    pub fn pool_token_volume_native_hourly_key(
        pool_address: &str,
        token_address: &str,
        hour_id: &i64,
    ) -> String {
        StoreKey::PoolTokenHourlyVolumeNative(
            pool_address.to_string(),
            token_address.to_string(),
            *hour_id,
        )
        .to_key_string()
    }

    pub fn pool_token_volume_usd_daily_key(
        pool_address: &str,
        token_address: &str,
        day_id: &i64,
    ) -> String {
        StoreKey::PoolTokenDailyVolumeUsd(
            pool_address.to_string(),
            token_address.to_string(),
            *day_id,
        )
        .to_key_string()
    }

    pub fn pool_token_volume_usd_hourly_key(
        pool_address: &str,
        token_address: &str,
        hour_id: &i64,
    ) -> String {
        StoreKey::PoolTokenHourlyVolumeUsd(
            pool_address.to_string(),
            token_address.to_string(),
            *hour_id,
        )
        .to_key_string()
    }

    pub fn protocol_pool_count_key() -> String {
        StoreKey::ProtocolPoolCount.to_key_string()
    }

    pub fn pool_tvl_key(pool_address: &str) -> String {
        StoreKey::PoolTvl(pool_address.to_string()).to_key_string()
    }

    pub fn protocol_volume_usd_key() -> String {
        StoreKey::ProtocolVolumeUsd.to_key_string()
    }

    pub fn protocol_daily_volume_usd_key(day_id: &i64) -> String {
        StoreKey::ProtocolDailyVolumeUsd(*day_id).to_key_string()
    }

    pub fn protocol_tvl_key() -> String {
        StoreKey::ProtocolTvl.to_key_string()
    }

    pub fn token_key(token_address: &str) -> String {
        StoreKey::Token(token_address.to_string()).to_key_string()
    }

    pub fn output_token_supply_key(pool_address: &str) -> String {
        StoreKey::OutputTokenSupply(pool_address.to_string()).to_key_string()
    }

    pub fn input_token_balance_key(pool_address: &str, token_address: &str) -> String {
        StoreKey::InputTokenBalance(pool_address.to_string(), token_address.to_string())
            .to_key_string()
    }

    pub fn uniswap_price_by_token_address_key(token_address: &str) -> String {
        StoreKey::UniswapPriceByTokenAddress(token_address.to_string()).to_key_string()
    }

    pub fn uniswap_price_by_token_symbol_key(token_symbol: &str) -> String {
        StoreKey::UniswapPriceByTokenSymbol(token_symbol.to_string()).to_key_string()
    }

    pub fn chainlink_price_by_symbol_key(symbol: &str) -> String {
        StoreKey::ChainlinkPriceBySymbol(symbol.to_string()).to_key_string()
    }

    pub fn extract_parts_from_key(key: &str) -> Option<(String, Option<String>, Option<i64>)> {
        let parts: Vec<&str> = key.split(':').collect();
        match parts.get(0).map(|s| *s) {
            Some("Pool")
            | Some("PoolFees")
            | Some("PoolVolumeUsd")
            | Some("PoolTvl")
            | Some("Token")
            | Some("OutputTokenSupply")
            | Some("ProtocolPoolCount")
            | Some("ProtocolVolumeUsd")
            | Some("ProtocolTvl") => parts.get(1).map(|&addr| (addr.to_string(), None, None)),

            Some("PoolDailyVolumeUsd")
            | Some("PoolHourlyVolumeUsd")
            | Some("ProtocolDailyVolumeUsd") => match (parts.get(1), parts.get(2)) {
                (Some(&addr), Some(&time_id)) => {
                    Some((addr.to_string(), None, time_id.parse::<i64>().ok()))
                }
                _ => None,
            },

            Some("PoolTokenDailyVolumeNative")
            | Some("PoolTokenHourlyVolumeNative")
            | Some("PoolTokenDailyVolumeUsd")
            | Some("PoolTokenHourlyVolumeUsd") => {
                match (parts.get(1), parts.get(2), parts.get(3)) {
                    (Some(&pool_addr), Some(&token_addr), Some(&day_id)) => Some((
                        pool_addr.to_string(),
                        Some(token_addr.to_string()),
                        day_id.parse::<i64>().ok(),
                    )),
                    _ => None,
                }
            },

            Some("InputTokenBalance") => match (parts.get(1), parts.get(2)) {
                (Some(&pool_addr), Some(&token_addr)) => {
                    Some((pool_addr.to_string(), Some(token_addr.to_string()), None))
                }
                _ => None,
            },

            Some("UsdPriceByTokenAddress")
            | Some("UsdPriceByTokenSymbol")
            | Some("price_by_symbol") => {
                parts.get(1).map(|&symbol| (symbol.to_string(), None, None))
            }
            _ => None,
        }
    }

    fn to_key_string(&self) -> String {
        match self {
            StoreKey::Pool(addr) => format!("Pool:{}", addr),
            StoreKey::PoolFees(addr) => format!("PoolFees:{}", addr),
            StoreKey::PoolVolumeUsd(addr) => format!("PoolVolumeUsd:{}", addr),
            StoreKey::PoolDailyVolumeUsd(addr, day_id) => {
                format!("PoolDailyVolumeUsd:{}:{}", addr, day_id)
            }
            StoreKey::PoolHourlyVolumeUsd(addr, hour_id) => {
                format!("PoolHourlyVolumeUsd:{}:{}", addr, hour_id)
            }
            StoreKey::PoolTokenDailyVolumeNative(pool_addr, token_addr, day_id) => {
                format!(
                    "PoolTokenDailyVolumeNative:{}:{}:{}",
                    pool_addr, token_addr, day_id
                )
            }
            StoreKey::PoolTokenHourlyVolumeNative(pool_addr, token_addr, hour_id) => {
                format!(
                    "PoolTokenHourlyVolumeNative:{}:{}:{}",
                    pool_addr, token_addr, hour_id
                )
            }
            StoreKey::PoolTokenDailyVolumeUsd(pool_addr, token_addr, day_id) => {
                format!(
                    "PoolTokenDailyVolumeUsd:{}:{}:{}",
                    pool_addr, token_addr, day_id
                )
            }
            StoreKey::PoolTokenHourlyVolumeUsd(pool_addr, token_addr, hour_id) => {
                format!(
                    "PoolTokenHourlyVolumeUsd:{}:{}:{}",
                    pool_addr, token_addr, hour_id
                )
            }
            StoreKey::PoolTvl(addr) => format!("PoolTvl:{}", addr),
            StoreKey::ProtocolPoolCount => "ProtocolPoolCount".to_string(),
            StoreKey::ProtocolVolumeUsd => "ProtocolVolumeUsd".to_string(),
            StoreKey::ProtocolDailyVolumeUsd(day_id) => {
                format!("ProtocolDailyVolumeUsd:{}", day_id)
            }
            StoreKey::ProtocolTvl => "ProtocolTvl".to_string(),
            StoreKey::Token(addr) => format!("Token:{}", addr),
            StoreKey::OutputTokenSupply(addr) => format!("OutputTokenSupply:{}", addr),
            StoreKey::InputTokenBalance(pool_addr, token_addr) => {
                format!("InputTokenBalance:{}:{}", pool_addr, token_addr)
            }
            StoreKey::UniswapPriceByTokenAddress(addr) => {
                format!("UsdPriceByTokenAddress:{}", addr)
            }
            StoreKey::UniswapPriceByTokenSymbol(symbol) => {
                format!("UsdPriceByTokenSymbol:{}", symbol)
            }
            StoreKey::ChainlinkPriceBySymbol(symbol) => format!("price_by_symbol:{}:USD", symbol),
        }
    }
}
