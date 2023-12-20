pub enum StoreKey {
    Token(String),
    Pool(String),
    ProtocolPoolCount,
}

impl StoreKey {
    pub fn token_key(token_address: &str) -> String {
        StoreKey::Token(token_address.to_string()).to_key_string()
    }

    pub fn pool_key(pool_address: &str) -> String {
        StoreKey::Pool(pool_address.to_string()).to_key_string()
    }

    pub fn protocol_pool_count_key() -> String {
        StoreKey::ProtocolPoolCount.to_key_string()
    }

    fn to_key_string(&self) -> String {
        match self {
            StoreKey::Token(addr) => format!("Token:{}", addr),
            StoreKey::Pool(addr) => format!("Pool:{}", addr),
            StoreKey::ProtocolPoolCount => "ProtocolPoolCount".to_string(),
        }
    }
}