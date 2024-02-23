pub enum StoreKey {
    // Curve sustream specific store key variants
    Pool(String),
    PoolAddress(i64),
    PoolFees(String),
    PoolVolumeUsd(String),
    PoolDailyVolumeUsd(String, i64),
    PoolHourlyVolumeUsd(String, i64),
    PoolTokenDailyVolumeNative(String, String, i64),
    PoolTokenHourlyVolumeNative(String, String, i64),
    PoolTokenDailyVolumeUsd(String, String, i64),
    PoolTokenHourlyVolumeUsd(String, String, i64),
    PoolTvl(String),
    PoolTokenTvl(String, String),
    ProtocolPoolCount,
    ProtocolVolumeUsd,
    ProtocolDailyVolumeUsd(i64),
    ProtocolTvl,
    Token(String),
    OutputTokenSupply(String),
    InputTokenBalance(String, String),
    ActiveUser(String),
    ActiveUserDaily(i64, String),
    ActiveUserDailyPrune(i64),
    ActiveUserHourly(i64, String),
    ActiveUserHourlyPrune(i64),
    ActiveUserCount,
    ActiveUserDailyCount(i64),
    ActiveUserHourlyCount(i64),
    TransactionDailyCount(i64),
    TransactionHourlyCount(i64),
    SwapDailyCount(i64),
    SwapHourlyCount(i64),
    DepositDailyCount(i64),
    DepositHourlyCount(i64),
    WithdrawDailyCount(i64),
    WithdrawHourlyCount(i64),
    CurrentDayId,
    CurrentHourId,
    // External packages store key variants
    UniswapPriceByTokenAddress(String),
    UniswapPriceByTokenSymbol(String),
    ChainlinkPriceBySymbol(String),
}

impl StoreKey {
    pub fn pool_key(pool_address: &str) -> String {
        StoreKey::Pool(pool_address.to_string()).to_key_string()
    }

    pub fn pool_address_key(current_count: &i64) -> String {
        StoreKey::PoolAddress(current_count.to_owned()).to_key_string()
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

    pub fn pool_token_tvl_key(pool_address: &str, token_address: &str) -> String {
        StoreKey::PoolTokenTvl(pool_address.to_string(), token_address.to_string()).to_key_string()
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

    pub fn active_user_key(user_address: &str) -> String {
        StoreKey::ActiveUser(user_address.to_string()).to_key_string()
    }

    pub fn active_user_daily_key(day_id: &i64, user_address: &str) -> String {
        StoreKey::ActiveUserDaily(*day_id, user_address.to_string()).to_key_string()
    }

    pub fn active_user_daily_prune_key(day_id: &i64) -> String {
        StoreKey::ActiveUserDailyPrune(*day_id).to_key_string()
    }

    pub fn active_user_hourly_key(hour_id: &i64, user_address: &str) -> String {
        StoreKey::ActiveUserHourly(*hour_id, user_address.to_string()).to_key_string()
    }

    pub fn active_user_hourly_prune_key(hour_id: &i64) -> String {
        StoreKey::ActiveUserHourlyPrune(*hour_id).to_key_string()
    }

    pub fn active_user_count_key() -> String {
        StoreKey::ActiveUserCount.to_key_string()
    }

    pub fn active_user_daily_count_key(day_id: &i64) -> String {
        StoreKey::ActiveUserDailyCount(*day_id).to_key_string()
    }

    pub fn active_user_hourly_count_key(hour_id: &i64) -> String {
        StoreKey::ActiveUserHourlyCount(*hour_id).to_key_string()
    }

    pub fn transaction_daily_count_key(day_id: &i64) -> String {
        StoreKey::TransactionDailyCount(*day_id).to_key_string()
    }

    pub fn transaction_hourly_count_key(hour_id: &i64) -> String {
        StoreKey::TransactionHourlyCount(*hour_id).to_key_string()
    }

    pub fn swap_daily_count_key(day_id: &i64) -> String {
        StoreKey::SwapDailyCount(*day_id).to_key_string()
    }

    pub fn swap_hourly_count_key(hour_id: &i64) -> String {
        StoreKey::SwapHourlyCount(*hour_id).to_key_string()
    }

    pub fn deposit_daily_count_key(day_id: &i64) -> String {
        StoreKey::DepositDailyCount(*day_id).to_key_string()
    }

    pub fn deposit_hourly_count_key(hour_id: &i64) -> String {
        StoreKey::DepositHourlyCount(*hour_id).to_key_string()
    }

    pub fn withdraw_daily_count_key(day_id: &i64) -> String {
        StoreKey::WithdrawDailyCount(*day_id).to_key_string()
    }

    pub fn withdraw_hourly_count_key(hour_id: &i64) -> String {
        StoreKey::WithdrawHourlyCount(*hour_id).to_key_string()
    }

    pub fn current_day_id_key() -> String {
        StoreKey::CurrentDayId.to_key_string()
    }

    pub fn current_hour_id_key() -> String {
        StoreKey::CurrentHourId.to_key_string()
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

    // TODO: Consider removing this and just using `key::try_first_segment` etc...
    pub fn extract_parts_from_key(key: &str) -> Option<(String, Option<String>, Option<i64>)> {
        let parts: Vec<&str> = key.split(':').collect();
        match parts.get(0).map(|s| *s) {
            Some("PoolAddress") => parts.get(1).map(|&id| (id.to_string(), None, None)),

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
            }

            Some("InputTokenBalance") | Some("PoolTokenTvl") => {
                match (parts.get(1), parts.get(2)) {
                    (Some(&pool_addr), Some(&token_addr)) => {
                        Some((pool_addr.to_string(), Some(token_addr.to_string()), None))
                    }
                    _ => None,
                }
            }

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
            StoreKey::PoolAddress(count) => format!("PoolAddress:{}", count.to_string()),
            StoreKey::PoolFees(addr) => format!("PoolFees:{}", addr),
            StoreKey::PoolVolumeUsd(addr) => format!("PoolVolumeUsd:{}", addr),
            StoreKey::PoolDailyVolumeUsd(addr, day_id) => {
                format!("PoolDailyVolumeUsd:{}:{}", addr, day_id.to_string())
            }
            StoreKey::PoolHourlyVolumeUsd(addr, hour_id) => {
                format!("PoolHourlyVolumeUsd:{}:{}", addr, hour_id.to_string())
            }
            StoreKey::PoolTokenDailyVolumeNative(pool_addr, token_addr, day_id) => {
                format!(
                    "PoolTokenDailyVolumeNative:{}:{}:{}",
                    pool_addr,
                    token_addr,
                    day_id.to_string()
                )
            }
            StoreKey::PoolTokenHourlyVolumeNative(pool_addr, token_addr, hour_id) => {
                format!(
                    "PoolTokenHourlyVolumeNative:{}:{}:{}",
                    pool_addr,
                    token_addr,
                    hour_id.to_string()
                )
            }
            StoreKey::PoolTokenDailyVolumeUsd(pool_addr, token_addr, day_id) => {
                format!(
                    "PoolTokenDailyVolumeUsd:{}:{}:{}",
                    pool_addr,
                    token_addr,
                    day_id.to_string()
                )
            }
            StoreKey::PoolTokenHourlyVolumeUsd(pool_addr, token_addr, hour_id) => {
                format!(
                    "PoolTokenHourlyVolumeUsd:{}:{}:{}",
                    pool_addr,
                    token_addr,
                    hour_id.to_string()
                )
            }
            StoreKey::PoolTvl(addr) => format!("PoolTvl:{}", addr),
            StoreKey::PoolTokenTvl(pool, token) => format!("PoolTokenTvl:{}:{}", pool, token),
            StoreKey::ProtocolPoolCount => "ProtocolPoolCount".to_string(),
            StoreKey::ProtocolVolumeUsd => "ProtocolVolumeUsd".to_string(),
            StoreKey::ProtocolDailyVolumeUsd(day_id) => {
                format!("ProtocolDailyVolumeUsd:{}", day_id.to_string())
            }
            StoreKey::ProtocolTvl => "ProtocolTvl".to_string(),
            StoreKey::Token(addr) => format!("Token:{}", addr),
            StoreKey::OutputTokenSupply(addr) => format!("OutputTokenSupply:{}", addr),
            StoreKey::InputTokenBalance(pool_addr, token_addr) => {
                format!("InputTokenBalance:{}:{}", pool_addr, token_addr)
            }
            StoreKey::ActiveUser(user_addr) => format!("ActiveUser:{}", user_addr),
            StoreKey::ActiveUserDaily(day_id, user_addr) => {
                format!("ActiveUserDaily:{}:{}", day_id.to_string(), user_addr)
            }
            StoreKey::ActiveUserDailyPrune(day_id) => {
                format!("ActiveUserDaily:{}", day_id.to_string())
            }
            StoreKey::ActiveUserHourly(hour_id, user_addr) => {
                format!("ActiveUserHourly:{}:{}", hour_id.to_string(), user_addr)
            }
            StoreKey::ActiveUserHourlyPrune(hour_id) => {
                format!("ActiveUserHourly:{}", hour_id.to_string())
            }
            StoreKey::ActiveUserCount => "ActiveUserCount".to_string(),
            StoreKey::ActiveUserDailyCount(day_id) => {
                format!("ActiveUserDailyCount:{}", day_id.to_string())
            }
            StoreKey::ActiveUserHourlyCount(hour_id) => {
                format!("ActiveUserHourlyCount:{}", hour_id.to_string())
            }
            StoreKey::TransactionDailyCount(day_id) => {
                format!("TransactionDailyCount:{}", day_id.to_string())
            }
            StoreKey::TransactionHourlyCount(hour_id) => {
                format!("TransactionHourlyCount:{}", hour_id.to_string())
            }
            StoreKey::SwapDailyCount(day_id) => {
                format!("SwapDailyCount:{}", day_id.to_string())
            }
            StoreKey::SwapHourlyCount(hour_id) => {
                format!("SwapHourlyCount:{}", hour_id.to_string())
            }
            StoreKey::DepositDailyCount(day_id) => {
                format!("DepositDailyCount:{}", day_id.to_string())
            }
            StoreKey::DepositHourlyCount(hour_id) => {
                format!("DepositHourlyCount:{}", hour_id.to_string())
            }
            StoreKey::WithdrawDailyCount(day_id) => {
                format!("WithdrawDailyCount:{}", day_id.to_string())
            }
            StoreKey::WithdrawHourlyCount(hour_id) => {
                format!("WithdrawHourlyCount:{}", hour_id.to_string())
            }
            StoreKey::CurrentDayId => "CurrentDayId".to_string(),
            StoreKey::CurrentHourId => "CurrentHourId".to_string(),
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
