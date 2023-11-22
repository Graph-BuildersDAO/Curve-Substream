use hex_literal::hex;

pub const NETWORK: &str = "mainnet";
pub const DEFAULT_NETWORK: &str = "Network.MAINNET";
pub const PRICE_CACHING: u64 = 6000;
pub const POOL_INFO_CONTRACT: [u8; 20] = hex!("e64608E223433E8a03a1DaaeFD8Cb638C14B552C");
pub const PROTOCOL_ADDRESS: [u8; 20] = hex!("0000000022D53366457F9d5E68Ec105046FC4383");
pub const CRV_TOKEN_ADDRESS: [u8; 20] = hex!("d533a949740bb3306d119cc777fa900ba034cd52");
pub const GAUGE_CONTROLLER_ADDRESS: [u8; 20] = hex!("2f50d538606fa9edd2b11e2446beb18c9d5846bb");
pub const CONTRACTS: [[u8; 20]; 5] = [hex!("90e00ace148ca3b23ac1bc8c240c2a7dd9c2d7f5"), hex!("7d86446ddb609ed0f5f8684acf30380a356b2b4c"), hex!("B9fC157394Af804a3578134A6585C0dc9cc990d4"), hex!("8F942C20D02bEfc377D41445793068908E2250D0"), hex!("F18056Bbd320E96A48e3Fbf8bC061322531aac99"), ];

#[derive(Debug, Clone)]
pub struct PoolDetails {
    pub name: &'static str,
    pub address: [u8; 20],
    pub lp_token: [u8; 20],
    pub start_block: u64,
}

pub static MISSING_OLD_POOLS_DATA: &[(&str, PoolDetails)] = &[
("0xbebc44782c7db0a1a60cb6fe97d0b483032ff1c7", PoolDetails { name: "3pool", address: hex!("bebc44782c7db0a1a60cb6fe97d0b483032ff1c7"), lp_token: hex!("6c3f90f043a72fa612cbac8115ee7e52bde6e490"), start_block: 10809473 }),
("0x79a8c46dea5ada233abaffd40f3a0a2b1e5a4f27", PoolDetails { name: "busd", address: hex!("79a8c46dea5ada233abaffd40f3a0a2b1e5a4f27"), lp_token: hex!("3b3ac5386837dc563660fb6a0937dfaa5924333b"), start_block: 9567295 }),
("0xdebf20617708857ebe4f679508e7b7863a8a8eee", PoolDetails { name: "aave", address: hex!("debf20617708857ebe4f679508e7b7863a8a8eee"), lp_token: hex!("fd2a8fa60abd58efe3eee34dd494cd491dc14900"), start_block: 11497106 }),
("0xa96a65c051bf88b4095ee1f2451c2a9d43f53ae2", PoolDetails { name: "aeth", address: hex!("a96a65c051bf88b4095ee1f2451c2a9d43f53ae2"), lp_token: hex!("aa17a236f2badc98ddc0cf999abb47d47fc0a6cf"), start_block: 11774139 }),
("0xa2b47e3d5c44877cca798226b7b8118f9bfb7a56", PoolDetails { name: "compound", address: hex!("a2b47e3d5c44877cca798226b7b8118f9bfb7a56"), lp_token: hex!("845838df265dcd2c412a1dc9e959c7d08537f8a2"), start_block: 9554040 }),
("0x0ce6a5ff5217e38315f87032cf90686c96627caa", PoolDetails { name: "eurs", address: hex!("0ce6a5ff5217e38315f87032cf90686c96627caa"), lp_token: hex!("194ebd173f6cdace046c53eacce9b953f28411d1"), start_block: 11466871 }),
("0x4ca9b3063ec5866a4b82e437059d2c43d1be596f", PoolDetails { name: "hbtc", address: hex!("4ca9b3063ec5866a4b82e437059d2c43d1be596f"), lp_token: hex!("b19059ebb43466c323583928285a49f558e572fd"), start_block: 10732328 }),
("0x2dded6da1bf5dbdf597c45fcfaa3194e53ecfeaf", PoolDetails { name: "ib", address: hex!("2dded6da1bf5dbdf597c45fcfaa3194e53ecfeaf"), lp_token: hex!("5282a4ef67d9c33135340fb3289cc1711c13638c"), start_block: 11831119 }),
("0xf178c0b5bb7e7abf4e12a4838c7b7c5ba2c623c0", PoolDetails { name: "link", address: hex!("f178c0b5bb7e7abf4e12a4838c7b7c5ba2c623c0"), lp_token: hex!("cee60cfa923170e4f8204ae08b4fa6a3f5656f3a"), start_block: 11875215 }),
("0x06364f10b501e868329afbc005b3492902d6c763", PoolDetails { name: "pax", address: hex!("06364f10b501e868329afbc005b3492902d6c763"), lp_token: hex!("d905e2eaebe188fc92179b6350807d8bd91db0d8"), start_block: 10041041 }),
("0x93054188d876f558f4a66b2ef1d97d16edf0895b", PoolDetails { name: "ren", address: hex!("93054188d876f558f4a66b2ef1d97d16edf0895b"), lp_token: hex!("49849c98ae39fff122806c06791fa73784fb3675"), start_block: 10151385 }),
("0xeb16ae0052ed37f479f7fe63849198df1765a733", PoolDetails { name: "saave", address: hex!("eb16ae0052ed37f479f7fe63849198df1765a733"), lp_token: hex!("02d341ccb60faaf662bc0554d13778015d1b285c"), start_block: 11772500 }),
("0x7fc77b5c7614e1533320ea6ddc2eb61fa00a9714", PoolDetails { name: "sbtc", address: hex!("7fc77b5c7614e1533320ea6ddc2eb61fa00a9714"), lp_token: hex!("075b1bb99792c9e1041ba13afef80c91a1e70fb3"), start_block: 10276641 }),
("0xc5424b857f758e906013f3555dad202e4bdb4567", PoolDetails { name: "seth", address: hex!("c5424b857f758e906013f3555dad202e4bdb4567"), lp_token: hex!("a3d87fffce63b53e0d54faa1cc983b7eb0b74a9c"), start_block: 11491884 }),
("0xdc24316b9ae028f1497c275eb9192a3ea0f67022", PoolDetails { name: "steth", address: hex!("dc24316b9ae028f1497c275eb9192a3ea0f67022"), lp_token: hex!("06325440d014e39736583c165c2963ba99faf14e"), start_block: 11592551 }),
("0xa5407eae9ba41422680e2e00537571bcc53efbfd", PoolDetails { name: "susd", address: hex!("a5407eae9ba41422680e2e00537571bcc53efbfd"), lp_token: hex!("c25a3a3b969415c80451098fa907ec722572917f"), start_block: 9906598 }),
("0x52ea46506b9cc5ef470c5bf89f17dc28bb35d85c", PoolDetails { name: "usdt", address: hex!("52ea46506b9cc5ef470c5bf89f17dc28bb35d85c"), lp_token: hex!("9fc689ccada600b6df723d9e47d84d76664a1f23"), start_block: 9456293 }),
("0x45f783cce6b7ff23b2ab2d70e416cdb7d6055f51", PoolDetails { name: "y", address: hex!("45f783cce6b7ff23b2ab2d70e416cdb7d6055f51"), lp_token: hex!("df5e0e81dff6faf3a7e52ba697820c5e32d806a8"), start_block: 9476468 }),
("0x8038c01a0390a8c547446a0b2c18fc9aefecc10c", PoolDetails { name: "dusd", address: hex!("8038c01a0390a8c547446a0b2c18fc9aefecc10c"), lp_token: hex!("3a664ab939fd8482048609f652f9a0b0677337b9"), start_block: 11187276 }),
("0x4f062658eaaf2c1ccf8c8e36d6824cdf41167956", PoolDetails { name: "gusd", address: hex!("4f062658eaaf2c1ccf8c8e36d6824cdf41167956"), lp_token: hex!("d2967f45c4f384deea880f807be904762a3dea07"), start_block: 11005604 }),
("0x3ef6a01a0f81d6046290f3e2a8c5b843e738e604", PoolDetails { name: "husd", address: hex!("3ef6a01a0f81d6046290f3e2a8c5b843e738e604"), lp_token: hex!("5b5cfe992adac0c9d48e05854b2d91c73a003858"), start_block: 11010070 }),
("0xe7a24ef0c5e95ffb0f6684b813a78f2a3ad7d171", PoolDetails { name: "linkusd", address: hex!("e7a24ef0c5e95ffb0f6684b813a78f2a3ad7d171"), lp_token: hex!("6d65b498cb23deaba52db31c93da9bffb340fb8f"), start_block: 11011556 }),
("0x8474ddbe98f5aa3179b3b3f5942d724afcdec9f6", PoolDetails { name: "musd", address: hex!("8474ddbe98f5aa3179b3b3f5942d724afcdec9f6"), lp_token: hex!("1aef73d49dedc4b1778d0706583995958dc862e6"), start_block: 11011940 }),
("0xc18cc39da8b11da8c3541c598ee022258f9744da", PoolDetails { name: "rsv", address: hex!("c18cc39da8b11da8c3541c598ee022258f9744da"), lp_token: hex!("c2ee6b0334c261ed60c72f6054450b61b8f18e35"), start_block: 11037531 }),
("0x3e01dd8a5e1fb3481f0f589056b428fc308af0fb", PoolDetails { name: "usdk", address: hex!("3e01dd8a5e1fb3481f0f589056b428fc308af0fb"), lp_token: hex!("97e2768e8e73511ca874545dc5ff8067eb19b787"), start_block: 11010305 }),
("0x0f9cb53ebe405d49a0bbdbd291a65ff571bc83e1", PoolDetails { name: "usdn", address: hex!("0f9cb53ebe405d49a0bbdbd291a65ff571bc83e1"), lp_token: hex!("4f3e8f405cf5afc05d68142f3783bdfe13811522"), start_block: 11010514 }),
("0x42d7025938bec20b69cbae5a77421082407f053a", PoolDetails { name: "usdp", address: hex!("42d7025938bec20b69cbae5a77421082407f053a"), lp_token: hex!("7eb40e450b9655f4b3cc4259bcc731c63ff55ae6"), start_block: 11922057 }),
("0x890f4e345b1daed0367a877a1612f86a1f86985f", PoolDetails { name: "ust", address: hex!("890f4e345b1daed0367a877a1612f86a1f86985f"), lp_token: hex!("94e131324b6054c0d789b190b2dac504e4361b53"), start_block: 11466568 }),
("0x071c661b4deefb59e2a3ddb20db036821eee8f4b", PoolDetails { name: "bbtc", address: hex!("071c661b4deefb59e2a3ddb20db036821eee8f4b"), lp_token: hex!("410e3e86ef427e30b9235497143881f717d93c2a"), start_block: 11455022 }),
("0xd81da8d904b52208541bade1bd6595d8a251f8dd", PoolDetails { name: "obtc", address: hex!("d81da8d904b52208541bade1bd6595d8a251f8dd"), lp_token: hex!("2fe94ea3d5d4a175184081439753de15aef9d614"), start_block: 11459238 }),
("0x7f55dde206dbad629c080068923b36fe9d6bdbef", PoolDetails { name: "pbtc", address: hex!("7f55dde206dbad629c080068923b36fe9d6bdbef"), lp_token: hex!("de5331ac4b3630f94853ff322b66407e0d6331e8"), start_block: 11421596 }),
("0xc25099792e9349c7dd09759744ea681c7de2cb66", PoolDetails { name: "tbtc", address: hex!("c25099792e9349c7dd09759744ea681c7de2cb66"), lp_token: hex!("64eda51d3ad40d56b9dfc5554e06f94e1dd786fd"), start_block: 11095928 }),
("0xecd5e75afb02efa118af914515d6521aabd189f1", PoolDetails { name: "tusd", address: hex!("ecd5e75afb02efa118af914515d6521aabd189f1"), lp_token: hex!("ecd5e75afb02efa118af914515d6521aabd189f1"), start_block: 12010370 }),
("0x4807862aa8b2bf68830e4c8dc86d0e9a998e085a", PoolDetails { name: "busdv2", address: hex!("4807862aa8b2bf68830e4c8dc86d0e9a998e085a"), lp_token: hex!("4807862aa8b2bf68830e4c8dc86d0e9a998e085a"), start_block: 12240440 }),
("0xf9440930043eb3997fc70e1339dbb11f341de7a8", PoolDetails { name: "reth", address: hex!("f9440930043eb3997fc70e1339dbb11f341de7a8"), lp_token: hex!("53a901d48795c58f485cbb38df08fa96a24669d5"), start_block: 12463576 }),
("0x43b4fdfd4ff969587185cdb6f0bd875c5fc83f8c", PoolDetails { name: "alusd", address: hex!("43b4fdfd4ff969587185cdb6f0bd875c5fc83f8c"), lp_token: hex!("43b4fdfd4ff969587185cdb6f0bd875c5fc83f8c"), start_block: 11956693 }),
("0x80466c64868e1ab14a1ddf27a676c3fcbe638fe5", PoolDetails { name: "tricrypto", address: hex!("80466c64868e1ab14a1ddf27a676c3fcbe638fe5"), lp_token: hex!("ca3d75ac011bf5ad07a98d02f18225f9bd9a6bdf"), start_block: 12521538 }),
("0x618788357d0ebd8a37e763adab3bc575d54c2c7d", PoolDetails { name: "rai", address: hex!("618788357d0ebd8a37e763adab3bc575d54c2c7d"), lp_token: hex!("6ba5b4e438fa0aaf7c1bd179285af65d13bd3d90"), start_block: 13634171 }),
("0x5a6a4d54456819380173272a5e8e9b9904bdf41b", PoolDetails { name: "mim", address: hex!("5a6a4d54456819380173272a5e8e9b9904bdf41b"), lp_token: hex!("5a6a4d54456819380173272a5e8e9b9904bdf41b"), start_block: 12567592 }),
("0xfd5db7463a3ab53fd211b4af195c5bccc1a03890", PoolDetails { name: "eurt", address: hex!("fd5db7463a3ab53fd211b4af195c5bccc1a03890"), lp_token: hex!("fd5db7463a3ab53fd211b4af195c5bccc1a03890"), start_block: 12921922 }),
("0x4e0915c88bc70750d68c481540f081fefaf22273", PoolDetails { name: "4pool", address: hex!("4e0915c88bc70750d68c481540f081fefaf22273"), lp_token: hex!("4e0915c88bc70750d68c481540f081fefaf22273"), start_block: 14631356 }),
("0x1005f7406f32a61bd760cfa14accd2737913d546", PoolDetails { name: "2pool", address: hex!("1005f7406f32a61bd760cfa14accd2737913d546"), lp_token: hex!("1005f7406f32a61bd760cfa14accd2737913d546"), start_block: 14631073 }),
("0xdcef968d416a41cdac0ed8702fac8128a64241a2", PoolDetails { name: "Curve.fi FRAX/USDC", address: hex!("dcef968d416a41cdac0ed8702fac8128a64241a2"), lp_token: hex!("3175df0976dfa876431c2e9ee6bc45b65d3473cc"), start_block: 14939588 }),
("0xd632f22692fac7611d2aa1c0d552930d43caed3b", PoolDetails { name: "frax", address: hex!("d632f22692fac7611d2aa1c0d552930d43caed3b"), lp_token: hex!("d632f22692fac7611d2aa1c0d552930d43caed3b"), start_block: 11972002 }),
("0xed279fdd11ca84beef15af5d39bb4d4bee23f0ca", PoolDetails { name: "lusd", address: hex!("ed279fdd11ca84beef15af5d39bb4d4bee23f0ca"), lp_token: hex!("ed279fdd11ca84beef15af5d39bb4d4bee23f0ca"), start_block: 12242627 }),
];

pub static BASE_POOLS_LP_TOKEN: [[u8; 20]; 19] = [
hex!("6c3F90f043a72FA612cbac8115EE7e52BDe6E490"), // 3crv
hex!("075b1bb99792c9E1041bA13afEf80C91a1e70fB3"), // renbtc
hex!("C25a3A3b969415c80451098fa907EC722572917F"), // susd
hex!("3B3Ac5386837Dc563660FB6a0937DFAa5924333B"), // busd
hex!("845838DF265Dcd2c412A1Dc9e959c7d08537f8a2"), // compound
hex!("b19059ebb43466C323583928285a49f558E572Fd"), // hbtc
hex!("D905e2eaeBe188fc92179b6350807D8bd91Db0D8"), // pax
hex!("49849C98ae39Fff122806C06791Fa73784FB3675"), // ren
hex!("A3D87FffcE63B53E0d54fAa1cc983B7eB0b74A9c"), // seth
hex!("9fC689CCaDa600B6DF723D9E47D84d76664a1F23"), // usdt
hex!("dF5e0e81Dff6FAF3A7e52BA697820c5e32D806A8"), // y
hex!("3a664Ab939FD8482048609f652f9a0B0677337B9"), // dusd
hex!("D2967f45c4f384DEEa880F807Be904762a3DeA07"), // gusd
hex!("5B5CFE992AdAC0C9D48E05854B2d91C73a003858"), // husd
hex!("6D65b498cb23deAba52db31c93Da9BFFb340FB8F"), // linkusd
hex!("1AEf73d49Dedc4b1778d0706583995958Dc862e6"), // musd
hex!("C2Ee6b0334C261ED60C72f6054450b61B8f18E35"), // rsv
hex!("97E2768e8E73511cA874545DC5Ff8067eB19B787"), // usdk
hex!("4f3E8F405CF5aFC05D68142F3783bDfE13811522"), // usdn
];
