pub enum StoreKey {
    // Curve sustream specific store key variants
    Pool(String),
    PoolFees(String),
    PoolTvl(String),
    ProtocolTvl,
    Token(String),
    OutputTokenSupply(String),
    InputTokenBalance(String, String),
    ProtocolPoolCount,
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

    pub fn pool_tvl_key(pool_address: &str) -> String {
        StoreKey::PoolTvl(pool_address.to_string()).to_key_string()
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

    pub fn protocol_pool_count_key() -> String {
        StoreKey::ProtocolPoolCount.to_key_string()
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

    pub fn extract_parts_from_key(key: &str) -> Option<(String, Option<String>)> {
        let parts: Vec<&str> = key.split(':').collect();
        match parts.as_slice() {
            ["Pool", addr] | ["PoolTvl", addr] | ["OutputTokenSupply", addr] => {
                Some((addr.to_string(), None))
            }
            ["InputTokenBalance", pool_addr, token_addr] => {
                Some((pool_addr.to_string(), Some(token_addr.to_string())))
            }
            // Handling new key patterns for Uniswap and Chainlink pricing
            ["UsdPriceByTokenAddress", addr] => {
                Some((addr.to_string(), None)) // Uniswap price by token address
            }
            ["UsdPriceByTokenSymbol", symbol] => {
                Some((symbol.to_string(), None)) // Uniswap price by token symbol
            }
            ["price_by_symbol", symbol, "USD"] => {
                Some((symbol.to_string(), None)) // Chainlink price by symbol
            }
            _ => None,
        }
    }

    fn to_key_string(&self) -> String {
        match self {
            StoreKey::Pool(addr) => format!("Pool:{}", addr),
            StoreKey::PoolFees(addr) => format!("PoolFees:{}", addr),
            StoreKey::PoolTvl(addr) => format!("PoolTvl:{}", addr),
            StoreKey::ProtocolTvl => "ProtocolTvl".to_string(),
            StoreKey::Token(addr) => format!("Token:{}", addr),
            StoreKey::OutputTokenSupply(addr) => format!("OutputTokenSupply:{}", addr),
            StoreKey::InputTokenBalance(pool_addr, token_addr) => {
                format!("InputTokenBalance:{}:{}", pool_addr, token_addr)
            }
            StoreKey::ProtocolPoolCount => "ProtocolPoolCount".to_string(),
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
