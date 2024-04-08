pub enum RegistryType {
    BasePoolRegistry,
    CrvUSDPoolFactory,
    CryptoPoolFactoryV2,
    CryptoSwapRegistryV2,
    CryptoSwapRegistryOld,
    PoolRegistryV1,
    PoolRegistryV1Old,
    PoolRegistryV2Old,
    MetaPoolFactoryOld,
    StableSwapFactoryNG,
    TriCryptoFactoryNG,
    Unknown,
}

pub struct RegistryDetails {
    pub address: [u8; 20],
    pub registry_type: RegistryType,
}
