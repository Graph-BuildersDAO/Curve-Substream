pub enum StoreKey {
    Pool(String),
    Token(String),
    OutputTokenSupply(String),
    InputTokenBalance(String, String),
    ProtocolPoolCount,
}

impl StoreKey {
    pub fn pool_key(pool_address: &str) -> String {
        StoreKey::Pool(pool_address.to_string()).to_key_string()
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

    fn to_key_string(&self) -> String {
        match self {
            StoreKey::Pool(addr) => format!("Pool:{}", addr),
            StoreKey::Token(addr) => format!("Token:{}", addr),
            StoreKey::OutputTokenSupply(addr) => format!("OutputTokenSupply:{}", addr),
            StoreKey::InputTokenBalance(pool_addr, token_addr) => {
                format!("InputTokenBalance:{}:{}", pool_addr, token_addr)
            }
            StoreKey::ProtocolPoolCount => "ProtocolPoolCount".to_string(),
        }
    }
}
