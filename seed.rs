
fn seed_pool_for_testing(pools: &mut Vec<Pool>) {
    pools.push(get_seed_pool());
    pools.push(get_second_seed_pool());
    // pools.push(get_usdm_3crv_pool());
    // pools.push(get_rc_amp_pool());
    // pools.push(get_gusd_pool());
}

fn get_seed_pool() -> Pool {
    Pool {
        name: "Curve.fi renBTC/wBTC/sBTC".to_string(),
        symbol: "crvRenWSBTC".to_string(),
        address: "7fc77b5c7614e1533320ea6ddc2eb61fa00a9714".to_string(),
        created_at_timestamp: 1592308622,
        created_at_block_number: 10276641,
        log_ordinal: 0,
        transaction_id: "6d1ef1e56febb5fdacedd5470aeb7ecbfe767963934821b62de9ad28da81179a"
            .to_string(),
        registry_address: "0000000000000000000000000000000000000000".to_string(),
        output_token: Some(Token {
            index: "0".to_string(),
            address: "075b1bb99792c9e1041ba13afef80c91a1e70fb3".to_string(),
            name: "Curve.fi renBTC/wBTC/sBTC".to_string(),
            symbol: "crvRenWSBTC".to_string(),
            decimals: 18,
            total_supply: "0".to_string(),
            is_base_pool_lp_token: true,
            gauge: None,
        }),
        input_tokens_ordered: vec![
            "eb4c2781e4eba804ce9a9803c67d0893436bb27d".to_string(),
            "2260fac5e5542a773aa44fbcfedf7c193bc2c599".to_string(),
            "fe18be6b3bd88a2d2a7f928d00292e7a9963cfc6".to_string(),
        ],
        input_tokens: vec![
            Token {
                index: "1".to_string(),
                address: "2260fac5e5542a773aa44fbcfedf7c193bc2c599".to_string(),
                name: "Wrapped BTC".to_string(),
                symbol: "WBTC".to_string(),
                decimals: 8,
                total_supply: "403120133215".to_string(),
                is_base_pool_lp_token: false,
                gauge: None,
            },
            Token {
                index: "0".to_string(),
                address: "eb4c2781e4eba804ce9a9803c67d0893436bb27d".to_string(),
                name: "renBTC".to_string(),
                symbol: "renBTC".to_string(),
                decimals: 8,
                total_supply: "13639283295".to_string(),
                is_base_pool_lp_token: false,
                gauge: None,
            },
            Token {
                index: "2".to_string(),
                address: "fe18be6b3bd88a2d2a7f928d00292e7a9963cfc6".to_string(),
                name: "Synth sBTC".to_string(),
                symbol: "sBTC".to_string(),
                decimals: 18,
                total_supply: "55061795981058907619".to_string(),
                is_base_pool_lp_token: false,
                gauge: None,
            },
        ],
        pool_type: Some(PoolType::PlainPool(PlainPool {})),
    }
}

fn get_second_seed_pool() -> Pool {
    Pool {
        name: "Curve.fi tBTC/sbtcCrv".to_string(),
        symbol: "tbtc/sbtcCrv".to_string(),
        address: "c25099792e9349c7dd09759744ea681c7de2cb66".to_string(),
        created_at_timestamp: 1603235444,
        created_at_block_number: 11095928,
        log_ordinal: 0,
        transaction_id: "0f5d1178c6eab65d649c6784093c755501933f9c503d644ec3d1d7694727441d"
            .to_string(),
        registry_address: "0000000000000000000000000000000000000000".to_string(),
        output_token: Some(Token {
            index: "0".to_string(),
            address: "64eda51d3ad40d56b9dfc5554e06f94e1dd786fd".to_string(),
            name: "Curve.fi tBTC/sbtcCrv".to_string(),
            symbol: "tbtc/sbtcCrv".to_string(),
            decimals: 18,
            total_supply: "0".to_string(),
            is_base_pool_lp_token: false,
            gauge: None,
        }),
        input_tokens_ordered: vec![
            "8daebade922df735c38c80c7ebd708af50815faa".to_string(),
            "075b1bb99792c9e1041ba13afef80c91a1e70fb3".to_string(),
        ],
        input_tokens: vec![
            Token {
                index: "1".to_string(),
                address: "075b1bb99792c9e1041ba13afef80c91a1e70fb3".to_string(),
                name: "Curve.fi renBTC/wBTC/sBTC".to_string(),
                symbol: "crvRenWSBTC".to_string(),
                decimals: 18,
                total_supply: "9492494977647683274960".to_string(),
                is_base_pool_lp_token: true,
                gauge: None,
            },
            Token {
                index: "0".to_string(),
                address: "8daebade922df735c38c80c7ebd708af50815faa".to_string(),
                name: "tBTC".to_string(),
                symbol: "TBTC".to_string(),
                decimals: 18,
                total_supply: "609810000000000000000".to_string(),
                is_base_pool_lp_token: false,
                gauge: None,
            },
        ],
        pool_type: Some(PoolType::MetaPool(MetaPool {
            base_pool_address: "7fc77b5c7614e1533320ea6ddc2eb61fa00a9714".to_string(),
            underlying_tokens: vec![
                Token {
                    index: "0".to_string(),
                    address: "eb4c2781e4eba804ce9a9803c67d0893436bb27d".to_string(),
                    name: "renBTC".to_string(),
                    symbol: "renBTC".to_string(),
                    decimals: 8,
                    total_supply: "2690660333535".to_string(),
                    is_base_pool_lp_token: false,
                    gauge: None,
                },
                Token {
                    index: "1".to_string(),
                    address: "2260fac5e5542a773aa44fbcfedf7c193bc2c599".to_string(),
                    name: "Wrapped BTC".to_string(),
                    symbol: "WBTC".to_string(),
                    decimals: 8,
                    total_supply: "10678594738589".to_string(),
                    is_base_pool_lp_token: false,
                    gauge: None,
                },
                Token {
                    index: "2".to_string(),
                    address: "fe18be6b3bd88a2d2a7f928d00292e7a9963cfc6".to_string(),
                    name: "Synth sBTC".to_string(),
                    symbol: "sBTC".to_string(),
                    decimals: 18,
                    total_supply: "2461329976915795842772".to_string(),
                    is_base_pool_lp_token: false,
                    gauge: None,
                },
            ],
            max_coin: 1,
        })),
    }
}

fn get_usdm_3crv_pool() -> Pool {
    Pool {
        name: "USDM-3crv".to_string(),
        symbol: "USDM3crv".to_string(),
        address: "c83b79c07ece44b8b99ffa0e235c00add9124f9e".to_string(),
        created_at_timestamp: 1700773403,
        created_at_block_number: 18636990,
        log_ordinal: 4313,
        transaction_id: "3c60ce9a61a97b8b8a646fe2604a8cf7f5fa1bbfdee68a80e556984da30bc990"
            .to_string(),
        registry_address: "6a8cbed756804b16e05e741edabd5cb544ae21bf".to_string(),
        output_token: Some(Token {
            index: "0".to_string(),
            address: "c83b79c07ece44b8b99ffa0e235c00add9124f9e".to_string(),
            name: "USDM-3crv".to_string(),
            symbol: "USDM3crv".to_string(),
            decimals: 18,
            total_supply: "0".to_string(),
            is_base_pool_lp_token: false,
            gauge: None,
        }),
        input_tokens_ordered: vec![
            "59d9356e565ab3a36dd77763fc0d87feaf85508c".to_string(),
            "6c3f90f043a72fa612cbac8115ee7e52bde6e490".to_string(),
        ],
        input_tokens: vec![
            Token {
                index: "0".to_string(),
                address: "59d9356e565ab3a36dd77763fc0d87feaf85508c".to_string(),
                name: "Mountain Protocol USD".to_string(),
                symbol: "USDM".to_string(),
                decimals: 18,
                total_supply: "13597662958977963344877103".to_string(),
                is_base_pool_lp_token: false,
                gauge: None,
            },
            Token {
                index: "1".to_string(),
                address: "6c3f90f043a72fa612cbac8115ee7e52bde6e490".to_string(),
                name: "Curve.fi DAI/USDC/USDT".to_string(),
                symbol: "3Crv".to_string(),
                decimals: 18,
                total_supply: "190130077712616079198607983".to_string(),
                is_base_pool_lp_token: true,
                gauge: None,
            },
        ],
        pool_type: Some(PoolType::MetaPool(MetaPool {
            base_pool_address: "bebc44782c7db0a1a60cb6fe97d0b483032ff1c7".to_string(),
            underlying_tokens: vec![
                Token {
                    index: "0".to_string(),
                    address: "6b175474e89094c44da98b954eedeac495271d0f".to_string(),
                    name: "Dai Stablecoin".to_string(),
                    symbol: "DAI".to_string(),
                    decimals: 18,
                    total_supply: "3637675937363044499063709332".to_string(),
                    is_base_pool_lp_token: true,
                    gauge: None,
                },
                Token {
                    index: "1".to_string(),
                    address: "a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".to_string(),
                    name: "USD Coin".to_string(),
                    symbol: "USDC".to_string(),
                    decimals: 6,
                    total_supply: "22462828414096513".to_string(),
                    is_base_pool_lp_token: true,
                    gauge: None,
                },
                Token {
                    index: "2".to_string(),
                    address: "dac17f958d2ee523a2206206994597c13d831ec7".to_string(),
                    name: "Tether USD".to_string(),
                    symbol: "USDT".to_string(),
                    decimals: 6,
                    total_supply: "40013387300953492".to_string(),
                    is_base_pool_lp_token: true,
                    gauge: None,
                },
            ],
            max_coin: 1,
        })),
    }
}

fn get_rc_amp_pool() -> Pool {
    Pool {
        name: "Curve.fi Factory USD Metapool: RC_AMP_0.02_DAI_2021_7_31".to_string(),
        symbol: "RC_AMP3CRV-f".to_string(),
        address: "2a1e73bf81941630869c125194fbf8f5ec060ff0".to_string(),
        created_at_timestamp: 1624989562,
        created_at_block_number: 12730413,
        log_ordinal: 5904,
        transaction_id: "2511c657627650333678f35d907d4dd4cdfb2b9858c27dc586321a7888951f2a"
            .to_string(),
        registry_address: "0959158b6040d32d04c301a72cbfd6b39e21c9ae".to_string(),
        output_token: Some(Token {
            index: "0".to_string(),
            address: "2a1e73bf81941630869c125194fbf8f5ec060ff0".to_string(),
            name: "Curve.fi Factory USD Metapool: RC_AMP_0.02_DAI_2021_7_31".to_string(),
            symbol: "RC_AMP3CRV-f".to_string(),
            decimals: 18,
            total_supply: "0".to_string(),
            is_base_pool_lp_token: false,
            gauge: None,
        }),
        input_tokens_ordered: vec![
            "07028f0eb368195e5bd7c621f25a08e2e4e63d54".to_string(),
            "6c3f90f043a72fa612cbac8115ee7e52bde6e490".to_string(),
        ],
        input_tokens: vec![
            Token {
                index: "0".to_string(),
                address: "07028f0eb368195e5bd7c621f25a08e2e4e63d54".to_string(),
                name: "Ruler Protocol rToken".to_string(),
                symbol: "RC_AMP_0.02_DAI_2021_7_31".to_string(),
                decimals: 18,
                total_supply: "0".to_string(),
                is_base_pool_lp_token: false,
                gauge: None,
            },
            Token {
                index: "1".to_string(),
                address: "6c3f90f043a72fa612cbac8115ee7e52bde6e490".to_string(),
                name: "Curve.fi DAI/USDC/USDT".to_string(),
                symbol: "3Crv".to_string(),
                decimals: 18,
                total_supply: "1729402165357517945662570546".to_string(),
                is_base_pool_lp_token: true,
                gauge: None,
            },
        ],
        pool_type: Some(PoolType::MetaPool(MetaPool {
            base_pool_address: "bebc44782c7db0a1a60cb6fe97d0b483032ff1c7".to_string(),
            underlying_tokens: vec![
                Token {
                    index: "0".to_string(),
                    address: "6b175474e89094c44da98b954eedeac495271d0f".to_string(),
                    name: "Dai Stablecoin".to_string(),
                    symbol: "DAI".to_string(),
                    decimals: 18,
                    total_supply: "4957856861188195629573476891".to_string(),
                    is_base_pool_lp_token: true,
                    gauge: None,
                },
                Token {
                    index: "1".to_string(),
                    address: "a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".to_string(),
                    name: "USD Coin".to_string(),
                    symbol: "USDC".to_string(),
                    decimals: 6,
                    total_supply: "24700189847779550".to_string(),
                    is_base_pool_lp_token: true,
                    gauge: None,
                },
                Token {
                    index: "2".to_string(),
                    address: "dac17f958d2ee523a2206206994597c13d831ec7".to_string(),
                    name: "Tether USD".to_string(),
                    symbol: "USDT".to_string(),
                    decimals: 6,
                    total_supply: "30912401959975130".to_string(),
                    is_base_pool_lp_token: true,
                    gauge: None,
                },
            ],
            max_coin: 1,
        })),
    }
}

fn get_gusd_pool() -> Pool {
    Pool {
        name: "Curve.fi GUSD/3Crv".to_string(),
        symbol: "gusd3CRV".to_string(),
        address: "4f062658eaaf2c1ccf8c8e36d6824cdf41167956".to_string(),
        created_at_timestamp: 1602032924,
        created_at_block_number: 11005604,
        log_ordinal: 0,
        transaction_id: "8d47a7c764056da0b00f2bd4bab041c1afeb9fb2ac808e2bc5c09d7e5df7d8c4"
            .to_string(),
        registry_address: "0000000000000000000000000000000000000000".to_string(),
        output_token: Some(Token {
            index: "0".to_string(),
            address: "d2967f45c4f384deea880f807be904762a3dea07".to_string(),
            name: "Curve.fi GUSD/3Crv".to_string(),
            symbol: "gusd3CRV".to_string(),
            decimals: 18,
            total_supply: "0".to_string(),
            is_base_pool_lp_token: true,
            gauge: None,
        }),
        input_tokens_ordered: vec![
            "056fd409e1d7a124bd7017459dfea2f387b6d5cd".to_string(),
            "6c3f90f043a72fa612cbac8115ee7e52bde6e490".to_string(),
        ],
        input_tokens: vec![
            Token {
                index: "0".to_string(),
                address: "056fd409e1d7a124bd7017459dfea2f387b6d5cd".to_string(),
                name: "Gemini dollar".to_string(),
                symbol: "GUSD".to_string(),
                decimals: 2,
                total_supply: "1169306953".to_string(),
                is_base_pool_lp_token: false,
                gauge: None,
            },
            Token {
                index: "1".to_string(),
                address: "6c3f90f043a72fa612cbac8115ee7e52bde6e490".to_string(),
                name: "Curve.fi DAI/USDC/USDT".to_string(),
                symbol: "3Crv".to_string(),
                decimals: 18,
                total_supply: "300325065928878887545079383".to_string(),
                is_base_pool_lp_token: true,
                gauge: None,
            },
        ],
        pool_type: Some(PoolType::MetaPool(MetaPool {
            base_pool_address: "bebc44782c7db0a1a60cb6fe97d0b483032ff1c7".to_string(),
            underlying_tokens: vec![
                Token {
                    index: "0".to_string(),
                    address: "6b175474e89094c44da98b954eedeac495271d0f".to_string(),
                    name: "Dai Stablecoin".to_string(),
                    symbol: "DAI".to_string(),
                    decimals: 18,
                    total_supply: "589548200742009952749096107".to_string(),
                    is_base_pool_lp_token: false,
                    gauge: None,
                },
                Token {
                    index: "1".to_string(),
                    address: "a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".to_string(),
                    name: "USD Coin".to_string(),
                    symbol: "USDC".to_string(),
                    decimals: 6,
                    total_supply: "2673285640224813".to_string(),
                    is_base_pool_lp_token: false,
                    gauge: None,
                },
                Token {
                    index: "2".to_string(),
                    address: "dac17f958d2ee523a2206206994597c13d831ec7".to_string(),
                    name: "Tether USD".to_string(),
                    symbol: "USDT".to_string(),
                    decimals: 6,
                    total_supply: "10334254640569842".to_string(),
                    is_base_pool_lp_token: false,
                    gauge: None,
                },
            ],
            max_coin: 1,
        })),
    }
}

fn get_usdk3crv_pool() -> Pool {
    Pool {
        name: "Curve.fi USDK/3Crv".to_string(),
        symbol: "usdk3CRV".to_string(),
        address: "3e01dd8a5e1fb3481f0f589056b428fc308af0fb".to_string(),
        created_at_timestamp: 1602097014,
        created_at_block_number: 11010305,
        log_ordinal: 0,
        transaction_id: "0bc48bb72f61bc85c71f00bc535037962bd718bf4d49d92899ebe2b929e4f139".to_string(),
        registry_address: "0000000000000000000000000000000000000000".to_string(),
        output_token: Some(Token {
            index: "0".to_string(),
            address: "97e2768e8e73511ca874545dc5ff8067eb19b787".to_string(),
            name: "Curve.fi USDK/3Crv".to_string(),
            symbol: "usdk3CRV".to_string(),
            decimals: 18,
            total_supply: "0".to_string(),
            is_base_pool_lp_token: true,
            gauge: None,
        }),
        input_tokens_ordered: vec![
            "1c48f86ae57291f7686349f12601910bd8d470bb".to_string(),
            "6c3f90f043a72fa612cbac8115ee7e52bde6e490".to_string(),
        ],
        input_tokens: vec![
            Token {
                index: "0".to_string(),
                address: "1c48f86ae57291f7686349f12601910bd8d470bb".to_string(),
                name: "USDK".to_string(),
                symbol: "USDK".to_string(),
                decimals: 18,
                total_supply: "22478711000000000000000000".to_string(),
                is_base_pool_lp_token: false,
                gauge: None,
            },
            Token {
                index: "1".to_string(),
                address: "6c3f90f043a72fa612cbac8115ee7e52bde6e490".to_string(),
                name: "Curve.fi DAI/USDC/USDT".to_string(),
                symbol: "3Crv".to_string(),
                decimals: 18,
                total_supply: "300467516746588395129255474".to_string(),
                is_base_pool_lp_token: true,
                gauge: None,
            },
        ],
        pool_type: Some(PoolType::MetaPool(MetaPool {
            base_pool_address: "bebc44782c7db0a1a60cb6fe97d0b483032ff1c7".to_string(),
            underlying_tokens: vec![
                Token {
                    index: "0".to_string(),
                    address: "6b175474e89094c44da98b954eedeac495271d0f".to_string(),
                    name: "Dai Stablecoin".to_string(),
                    symbol: "DAI".to_string(),
                    decimals: 18,
                    total_supply: "590491829280669711175159712".to_string(),
                    is_base_pool_lp_token: false,
                    gauge: None,
                },
                Token {
                    index: "1".to_string(),
                    address: "a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".to_string(),
                    name: "USD Coin".to_string(),
                    symbol: "USDC".to_string(),
                    decimals: 6,
                    total_supply: "2708370009585077".to_string(),
                    is_base_pool_lp_token: false,
                    gauge: None,
                },
                Token {
                    index: "2".to_string(),
                    address: "dac17f958d2ee523a2206206994597c13d831ec7".to_string(),
                    name: "Tether USD".to_string(),
                    symbol: "USDT".to_string(),
                    decimals: 6,
                    total_supply: "10334246184469201".to_string(),
                    is_base_pool_lp_token: false,
                    gauge: None,
                },
            ],
            max_coin: 1,
        })),
    }
}
