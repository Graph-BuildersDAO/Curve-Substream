use anyhow::anyhow;
use num_traits::ToPrimitive;
use substreams::{
    errors::Error,
    scalar::BigInt,
    store::{StoreGet, StoreGetBigDecimal, StoreGetProto},
    Hex,
};
use substreams_ethereum::{
    pb::eth::v2::{self as eth, Log, TransactionTrace},
    Event, NULL_ADDRESS,
};

use crate::{
    abi::curve::pool::events::{
        AddLiquidity1, AddLiquidity2, AddLiquidity3, AddLiquidity4, AddLiquidity5, AddLiquidity6,
        AddLiquidity7, AddLiquidity8, ApplyNewFee1, ApplyNewFee2, NewFee1, NewFee2, NewParameters1,
        NewParameters2, NewParameters3, NewParameters4, RemoveLiquidity1, RemoveLiquidity2,
        RemoveLiquidity3, RemoveLiquidity4, RemoveLiquidity5, RemoveLiquidity6,
        RemoveLiquidityImbalance1, RemoveLiquidityImbalance2, RemoveLiquidityImbalance3,
        RemoveLiquidityImbalance4, RemoveLiquidityOne1, RemoveLiquidityOne2, RemoveLiquidityOne3,
        RemoveLiquidityOne4, RemoveLiquidityOne5, TokenExchange1, TokenExchange2, TokenExchange3,
        TokenExchangeUnderlying,
    },
    common::{event_extraction, pool_utils::is_metapool, prices::get_token_usd_price},
    constants::ETH_ADDRESS,
    key_management::store_key_manager::StoreKey,
    pb::{
        curve::types::v1::{
            events::{
                pool_event::{
                    DepositEvent, LpTokenChange, LpTokenChangeType, PriceSource, SwapEvent,
                    SwapUnderlyingLendingEvent, SwapUnderlyingMetaEvent, TokenAmount, TokenSource,
                    Type, WithdrawEvent,
                },
                FeeChangeEvent, PoolEvent,
            },
            lending_pool::LendingPoolType,
            pool::PoolType,
            Events, Pool, Token,
        },
        uniswap_pricing::v1::Erc20Price,
    },
    rpc::pool::get_pool_fee_and_admin_fee,
};

#[substreams::handlers::map]
pub fn map_extract_pool_events(
    blk: eth::Block,
    pools_store: StoreGetProto<Pool>,
    uniswap_prices: StoreGetProto<Erc20Price>,
    chainlink_prices: StoreGetBigDecimal,
) -> Result<Events, Error> {
    // Initialise events and its fields
    let mut events = Events::default();
    let mut pool_events: Vec<PoolEvent> = Vec::new();
    let mut fee_change_events: Vec<FeeChangeEvent> = Vec::new();

    // Check if event is coming from the pool contract
    for trx in blk.transactions() {
        for (log, _call) in trx.logs_with_calls() {
            let pool_address = Hex::encode(&log.address);
            let pool_opt = pools_store.get_last(StoreKey::pool_key(&pool_address));

            if let Some(pool) = pool_opt {
                if let Some(swap) = TokenExchange1::match_and_decode(&log) {
                    extract_swap_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        &swap.sold_id,
                        &swap.bought_id,
                        &swap.tokens_sold,
                        &swap.tokens_bought,
                        &swap.buyer,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(swap) = TokenExchange2::match_and_decode(&log) {
                    extract_swap_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        &swap.sold_id,
                        &swap.bought_id,
                        &swap.tokens_sold,
                        &swap.tokens_bought,
                        &swap.buyer,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(swap) = TokenExchange3::match_and_decode(&log) {
                    extract_swap_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        &swap.sold_id,
                        &swap.bought_id,
                        &swap.tokens_sold,
                        &swap.tokens_bought,
                        &swap.buyer,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(swap_underlying) =
                    TokenExchangeUnderlying::match_and_decode(&log)
                {
                    extract_swap_underlying_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        &swap_underlying.sold_id,
                        &swap_underlying.bought_id,
                        &swap_underlying.tokens_sold,
                        &swap_underlying.tokens_bought,
                        &swap_underlying.buyer,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(deposit) = AddLiquidity1::match_and_decode(&log) {
                    let fees = vec![deposit.fee.into()];
                    extract_deposit_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        deposit.token_amounts.to_vec(),
                        fees,
                        deposit.provider,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(deposit) = AddLiquidity2::match_and_decode(&log) {
                    let fees = vec![deposit.fee.into()];
                    extract_deposit_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        deposit.token_amounts.to_vec(),
                        fees,
                        deposit.provider,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(deposit) = AddLiquidity3::match_and_decode(&log) {
                    let fees = deposit.fees.iter().map(ToString::to_string).collect();
                    extract_deposit_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        deposit.token_amounts.to_vec(),
                        fees,
                        deposit.provider,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(deposit) = AddLiquidity4::match_and_decode(&log) {
                    let fees = deposit.fees.iter().map(ToString::to_string).collect();
                    extract_deposit_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        deposit.token_amounts.to_vec(),
                        fees,
                        deposit.provider,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(deposit) = AddLiquidity5::match_and_decode(&log) {
                    let fees = deposit.fees.iter().map(ToString::to_string).collect();
                    extract_deposit_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        deposit.token_amounts.to_vec(),
                        fees,
                        deposit.provider,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(deposit) = AddLiquidity6::match_and_decode(&log) {
                    let fees = vec![deposit.fee.into()];
                    extract_deposit_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        deposit.token_amounts.to_vec(),
                        fees,
                        deposit.provider,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(deposit) = AddLiquidity7::match_and_decode(&log) {
                    let fees = deposit.fees.iter().map(ToString::to_string).collect();
                    extract_deposit_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        deposit.token_amounts.to_vec(),
                        fees,
                        deposit.provider,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(deposit) = AddLiquidity8::match_and_decode(&log) {
                    let fees = vec![deposit.fee.into()];
                    extract_deposit_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        deposit.token_amounts.to_vec(),
                        fees,
                        deposit.provider,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(withdraw) = RemoveLiquidity1::match_and_decode(&log) {
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        Vec::new(), // No fees on RemoveLiquidty1 events
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(withdraw) = RemoveLiquidity2::match_and_decode(&log) {
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        Vec::new(), // No fees on RemoveLiquidty2 events
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(withdraw) = RemoveLiquidity3::match_and_decode(&log) {
                    let fees: Vec<String> = withdraw.fees.iter().map(ToString::to_string).collect();
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        fees,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(withdraw) = RemoveLiquidity4::match_and_decode(&log) {
                    let fees: Vec<String> = withdraw.fees.iter().map(ToString::to_string).collect();
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        fees,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(withdraw) = RemoveLiquidity5::match_and_decode(&log) {
                    let fees: Vec<String> = withdraw.fees.iter().map(ToString::to_string).collect();
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        fees,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(withdraw) = RemoveLiquidity6::match_and_decode(&log) {
                    let fees: Vec<String> = withdraw.fees.iter().map(ToString::to_string).collect();
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        fees,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(withdraw) = RemoveLiquidityImbalance1::match_and_decode(&log) {
                    let fees: Vec<String> = withdraw.fees.iter().map(ToString::to_string).collect();
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        fees,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(withdraw) = RemoveLiquidityImbalance2::match_and_decode(&log) {
                    let fees: Vec<String> = withdraw.fees.iter().map(ToString::to_string).collect();
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        fees,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(withdraw) = RemoveLiquidityImbalance3::match_and_decode(&log) {
                    let fees: Vec<String> = withdraw.fees.iter().map(ToString::to_string).collect();
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        fees,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(withdraw) = RemoveLiquidityImbalance4::match_and_decode(&log) {
                    let fees: Vec<String> = withdraw.fees.iter().map(ToString::to_string).collect();
                    extract_withdraw_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amounts.to_vec(),
                        fees,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(withdraw) = RemoveLiquidityOne1::match_and_decode(&log) {
                    extract_withdraw_one_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amount,
                        withdraw.coin_amount,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(withdraw) = RemoveLiquidityOne2::match_and_decode(&log) {
                    // TODO: This is a hacky work around for the issue caused by the similarity between `RemoveLiquidityOne2` and `RemoveLiquidityOne5`.
                    //       If a `RemoveLiquidityOne5` event is emitted, it matches against `RemoveLiquidityOne2` due to them having the same event parameter types.
                    //       I have messaged SF regarding this, and will be looking into it. If the issue is not fixed we will have to look at removing the reliance on
                    //       a merged ABI for the Pools. This workaround should suffice for now. In essence, for `RemoveLiquidityOne5` events, the `token_supply` is being
                    //       used as the `coin_amount` for the `RemoveLiquidityOne5` event.

                    // If the `coin_amount` is less than 10, we can assume it is `RemoveLiquidityOne5` event, as this parameter slot is used for the `coin_index`.
                    // It is incredibly unlikely that a `coin_amount` less than 10 will part of a withdrawal as this would be an incredibly low token amount.
                    if &withdraw.coin_amount < &BigInt::from(10) {
                        extract_withdraw_one_event(
                            &mut pool_events,
                            &blk,
                            trx,
                            log,
                            &pool,
                            withdraw.provider,
                            withdraw.token_amount,
                            // Using `token_supply` as `coin_amount` for `RemoveLiquidityOne5`
                            withdraw.token_supply,
                            &uniswap_prices,
                            &chainlink_prices,
                        );
                    } else {
                        // Extracting using the normal `RemoveLiquidityOne2` event params
                        extract_withdraw_one_event(
                            &mut pool_events,
                            &blk,
                            trx,
                            log,
                            &pool,
                            withdraw.provider,
                            withdraw.token_amount,
                            withdraw.coin_amount,
                            &uniswap_prices,
                            &chainlink_prices,
                        );
                    }
                } else if let Some(withdraw) = RemoveLiquidityOne3::match_and_decode(&log) {
                    extract_withdraw_one_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amount,
                        withdraw.coin_amount,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(withdraw) = RemoveLiquidityOne4::match_and_decode(&log) {
                    extract_withdraw_one_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amount,
                        withdraw.coin_amount,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(withdraw) = RemoveLiquidityOne5::match_and_decode(&log) {
                    extract_withdraw_one_event(
                        &mut pool_events,
                        &blk,
                        trx,
                        log,
                        &pool,
                        withdraw.provider,
                        withdraw.token_amount,
                        withdraw.coin_amount,
                        &uniswap_prices,
                        &chainlink_prices,
                    );
                } else if let Some(fee_change) = ApplyNewFee1::match_and_decode(&log) {
                    fee_change_events.push(FeeChangeEvent {
                        transaction_hash: Hex::encode(&trx.hash),
                        tx_index: trx.index,
                        log_index: log.index,
                        log_ordinal: log.ordinal,
                        timestamp: blk.timestamp_seconds(),
                        block_number: blk.number,
                        fee: fee_change.fee.to_string(),
                        admin_fee: None,
                        pool_address: pool.address.clone(),
                    });
                } else if let Some(fee_change) = ApplyNewFee2::match_and_decode(&log) {
                    fee_change_events.push(FeeChangeEvent {
                        transaction_hash: Hex::encode(&trx.hash),
                        tx_index: trx.index,
                        log_index: log.index,
                        log_ordinal: log.ordinal,
                        timestamp: blk.timestamp_seconds(),
                        block_number: blk.number,
                        fee: fee_change.fee.to_string(),
                        admin_fee: None,
                        pool_address: pool.address.clone(),
                    });
                } else if let Some(fee_change) = NewFee1::match_and_decode(&log) {
                    fee_change_events.push(FeeChangeEvent {
                        transaction_hash: Hex::encode(&trx.hash),
                        tx_index: trx.index,
                        log_index: log.index,
                        log_ordinal: log.ordinal,
                        timestamp: blk.timestamp_seconds(),
                        block_number: blk.number,
                        fee: fee_change.fee.to_string(),
                        admin_fee: Some(fee_change.admin_fee.to_string()),
                        pool_address: pool.address.clone(),
                    });
                } else if let Some(fee_change) = NewFee2::match_and_decode(&log) {
                    fee_change_events.push(FeeChangeEvent {
                        transaction_hash: Hex::encode(&trx.hash),
                        tx_index: trx.index,
                        log_index: log.index,
                        log_ordinal: log.ordinal,
                        timestamp: blk.timestamp_seconds(),
                        block_number: blk.number,
                        fee: fee_change.fee.to_string(),
                        admin_fee: Some(fee_change.admin_fee.to_string()),
                        pool_address: pool.address.clone(),
                    });
                } else if let Some(fee_change) = NewParameters1::match_and_decode(&log) {
                    fee_change_events.push(FeeChangeEvent {
                        transaction_hash: Hex::encode(&trx.hash),
                        tx_index: trx.index,
                        log_index: log.index,
                        log_ordinal: log.ordinal,
                        timestamp: blk.timestamp_seconds(),
                        block_number: blk.number,
                        fee: fee_change.fee.to_string(),
                        admin_fee: Some(fee_change.admin_fee.to_string()),
                        pool_address: pool.address.clone(),
                    });
                } else if let Some(_fee_change) = NewParameters2::match_and_decode(&log) {
                    let (total_fee, admin_fee) = get_pool_fee_and_admin_fee(&pool.address_vec())?;

                    fee_change_events.push(FeeChangeEvent {
                        transaction_hash: Hex::encode(&trx.hash),
                        tx_index: trx.index,
                        log_index: log.index,
                        log_ordinal: log.ordinal,
                        timestamp: blk.timestamp_seconds(),
                        block_number: blk.number,
                        fee: total_fee.to_string(),
                        admin_fee: Some(admin_fee.to_string()),
                        pool_address: pool.address.clone(),
                    });
                } else if let Some(_fee_change) = NewParameters3::match_and_decode(&log) {
                    let (total_fee, admin_fee) = get_pool_fee_and_admin_fee(&pool.address_vec())?;

                    fee_change_events.push(FeeChangeEvent {
                        transaction_hash: Hex::encode(&trx.hash),
                        tx_index: trx.index,
                        log_index: log.index,
                        log_ordinal: log.ordinal,
                        timestamp: blk.timestamp_seconds(),
                        block_number: blk.number,
                        fee: total_fee.to_string(),
                        admin_fee: Some(admin_fee.to_string()),
                        pool_address: pool.address.clone(),
                    });
                } else if let Some(_fee_change) = NewParameters4::match_and_decode(&log) {
                    let (total_fee, admin_fee) = get_pool_fee_and_admin_fee(&pool.address_vec())?;

                    fee_change_events.push(FeeChangeEvent {
                        transaction_hash: Hex::encode(&trx.hash),
                        tx_index: trx.index,
                        log_index: log.index,
                        log_ordinal: log.ordinal,
                        timestamp: blk.timestamp_seconds(),
                        block_number: blk.number,
                        fee: total_fee.to_string(),
                        admin_fee: Some(admin_fee.to_string()),
                        pool_address: pool.address.clone(),
                    });
                }
            }
        }
    }
    events.pool_events = pool_events;
    events.fee_changes_events = fee_change_events;
    Ok(events)
}

fn extract_swap_event(
    pool_events: &mut Vec<PoolEvent>,
    blk: &eth::Block,
    trx: &TransactionTrace,
    log: &Log,
    pool: &Pool,
    sold_id: &BigInt,
    bought_id: &BigInt,
    tokens_sold: &BigInt,
    tokens_bought: &BigInt,
    buyer: &Vec<u8>,
    uniswap_prices: &StoreGetProto<Erc20Price>,
    chainlink_prices: &StoreGetBigDecimal,
) {
    let pool_address = &pool.address;
    substreams::log::info!(format!(
        "Extracting Swap from transaction {} and pool {}",
        Hex::encode(&trx.hash),
        &pool_address
    ));
    let in_address_index = sold_id.to_i32().to_usize().unwrap();
    let out_address_index = bought_id.to_i32().to_usize().unwrap();

    let in_address = pool.input_tokens_ordered[in_address_index].clone();
    let out_address = pool.input_tokens_ordered[in_address_index].clone();

    let token_in = pool
        .input_tokens
        .iter()
        .find(|t| t.address == in_address)
        .unwrap();
    let token_out = pool
        .input_tokens
        .iter()
        .find(|t| t.address == out_address)
        .unwrap();

    let (token_in_price, token_in_price_source) =
        get_token_usd_price(token_in, &uniswap_prices, &chainlink_prices);
    let (token_out_price, token_out_price_source) =
        get_token_usd_price(token_out, &uniswap_prices, &chainlink_prices);

    let token_amount_in = TokenAmount {
        token_address: pool.input_tokens_ordered[in_address_index].clone(),
        amount: tokens_sold.into(),
        amount_usd: (tokens_sold.to_decimal(token_in.decimals) * token_in_price.clone())
            .to_string(),
        source: TokenSource::Default as i32,
        token_price: token_in_price.to_string(),
        price_source: token_in_price_source as i32,
    };

    let token_amount_out = TokenAmount {
        token_address: pool.input_tokens_ordered[out_address_index].clone(),
        amount: tokens_bought.into(),
        amount_usd: (tokens_bought.to_decimal(token_out.decimals) * token_out_price.clone())
            .to_string(),
        source: TokenSource::Default as i32,
        token_price: token_out_price.to_string(),
        price_source: token_out_price_source as i32,
    };

    let swap_event = SwapEvent {
        token_in: Some(token_amount_in),
        token_out: Some(token_amount_out),
    };

    pool_events.push(PoolEvent {
        transaction_hash: Hex::encode(&trx.hash),
        tx_index: trx.index,
        log_index: log.index,
        log_ordinal: log.ordinal,
        to_address: pool_address.to_string(),
        from_address: Hex::encode(buyer),
        timestamp: blk.timestamp_seconds(),
        block_number: blk.number,
        pool_address: pool_address.to_string(),
        r#type: Some(Type::SwapEvent(swap_event)),
    })
}

fn extract_swap_underlying_event(
    pool_events: &mut Vec<PoolEvent>,
    blk: &eth::Block,
    trx: &TransactionTrace,
    log: &Log,
    pool: &Pool,
    sold_id: &BigInt,
    bought_id: &BigInt,
    tokens_sold: &BigInt,
    tokens_bought: &BigInt,
    buyer: &Vec<u8>,
    uniswap_prices: &StoreGetProto<Erc20Price>,
    chainlink_prices: &StoreGetBigDecimal,
) {
    if is_metapool(pool) {
        // Check if the pool is a metapool and retrieve the base pool's address.
        let base_pool_address = match &pool.pool_type {
            Some(PoolType::MetaPool(meta)) => meta.base_pool_address.clone(),
            _ => {
                substreams::log::debug!(
                    "Pool is not a metapool, skipping `TokenExchangeUnderlying` event processing."
                );
                return;
            }
        };

        if base_pool_address.is_empty() {
            substreams::log::debug!("Base pool address not found for metapool.");
            return;
        }

        substreams::log::info!(
            "Extracting Swap Underlying from transaction {} and pool {}",
            Hex::encode(&trx.hash),
            &pool.address
        );

        // Determine the addresses and sources of the input and output tokens
        // based on their indices and the type of the pool.
        let (token_in, token_out) = match determine_underlying_exchange_tokens(
            &pool,
            sold_id.to_i32(),
            bought_id.to_i32(),
            tokens_sold,
            tokens_bought,
            uniswap_prices,
            chainlink_prices,
        ) {
            Ok((token_in, token_out)) => (token_in, token_out),
            Err(e) => {
                substreams::log::debug!("Error determining exchange tokens: {:?}", e);
                return;
            }
        };

        // Determine if there's a change in the metapools base pool LP token balance. This could be a burn
        // or mint operation depending on the swap direction (metapool to base pool or vice versa).
        let base_pool_lp_token_address = match pool.input_tokens.iter().find(|t| &t.index == &"1") {
            Some(token) => token.address_vec(),
            None => {
                substreams::log::debug!("Error determining base pool lp token from metapool");
                return;
            }
        };

        let lp_token_change = if token_in.source() == TokenSource::MetaPool
            && token_out.source() == TokenSource::BasePool
        {
            extract_lp_token_change(trx, log, pool, &base_pool_lp_token_address, true)
        } else if token_in.source() == TokenSource::BasePool
            && token_out.source() == TokenSource::MetaPool
        {
            extract_lp_token_change(trx, log, pool, &base_pool_lp_token_address, false)
        } else {
            None
        };

        let swap_underlying_event = SwapUnderlyingMetaEvent {
            token_in: Some(token_in),
            token_out: Some(token_out),
            lp_token_change: lp_token_change,
            base_pool_address,
        };

        pool_events.push(PoolEvent {
            transaction_hash: Hex::encode(&trx.hash),
            tx_index: trx.index,
            log_index: log.index,
            log_ordinal: log.ordinal,
            to_address: pool.address.to_string(),
            from_address: Hex::encode(buyer),
            timestamp: blk.timestamp_seconds(),
            block_number: blk.number,
            pool_address: pool.address.to_string(),
            r#type: Some(Type::SwapUnderlyingMetaEvent(swap_underlying_event)),
        })
    } else if let Some(PoolType::LendingPool(lending_pool)) = &pool.pool_type {
        if let Some(lending_pool_type) = &lending_pool.lending_pool_type {
            let token_in = lending_pool
                .underlying_tokens
                .get(sold_id.to_i32() as usize)
                .map_or_else(
                    || {
                        substreams::log::info!(
                            "Sold token index {} is out of bounds for LendingPool",
                            sold_id
                        );
                        None
                    },
                    |token| Some(token),
                );

            let token_out = lending_pool
                .underlying_tokens
                .get(bought_id.to_i32() as usize)
                .map_or_else(
                    || {
                        substreams::log::info!(
                            "Bought token index {} is out of bounds for LendingPool",
                            bought_id
                        );
                        None
                    },
                    |token| Some(token),
                );

            if let (Some(token_in), Some(token_out)) = (token_in, token_out) {
                let (token_in_price, token_in_price_source) =
                    get_token_usd_price(token_in, &uniswap_prices, &chainlink_prices);
                let (token_out_price, token_out_price_source) =
                    get_token_usd_price(token_out, &uniswap_prices, &chainlink_prices);

                let skip_mint_for_token_in = should_skip_token_action(lending_pool_type, sold_id);
                let skip_burn_for_token_out =
                    should_skip_token_action(lending_pool_type, bought_id);

                let token_amount_in = TokenAmount {
                    token_address: token_in.address.clone(),
                    amount: tokens_sold.into(),
                    amount_usd: (tokens_sold.to_decimal(token_in.decimals)
                        * token_in_price.clone())
                    .to_string(),
                    source: if skip_mint_for_token_in {
                        TokenSource::LendingPool as i32
                    } else {
                        TokenSource::LendingProtcol as i32
                    },
                    token_price: token_in_price.to_string(),
                    price_source: token_in_price_source as i32,
                };

                let token_amount_out = TokenAmount {
                    token_address: token_out.address.clone(),
                    amount: tokens_bought.into(),
                    amount_usd: (tokens_bought.to_decimal(token_out.decimals)
                        * token_out_price.clone())
                    .to_string(),
                    source: if skip_burn_for_token_out {
                        TokenSource::LendingPool as i32
                    } else {
                        TokenSource::LendingProtcol as i32
                    },
                    token_price: token_out_price.to_string(),
                    price_source: token_out_price_source as i32,
                };

                let interest_token_in_address =
                    match Hex::decode(&pool.input_tokens_ordered[sold_id.to_i32() as usize]) {
                        Ok(address) => address,
                        Err(e) => {
                            substreams::log::info!(
                            "Failed to decode interest bearing token in address for sold_id {}: {}",
                            sold_id,
                            e
                        );
                            return; // Exit from the function, skipping this event
                        }
                    };

                let interest_token_out_address =
                    match Hex::decode(&pool.input_tokens_ordered[bought_id.to_i32() as usize]) {
                        Ok(address) => address,
                        Err(e) => {
                            substreams::log::info!(
                        "Failed to decode interest bearing token out address for bought_id {}: {}",
                        bought_id,
                        e
                    );
                            return; // Exit from the function, skipping this event
                        }
                    };

                // Token in will have a corresponding mint event
                let interest_token_in_change = if !skip_mint_for_token_in {
                    extract_lending_pool_token_change(
                        trx,
                        log,
                        &pool.address_vec(),
                        &interest_token_in_address,
                        false,
                        lending_pool_type,
                    )
                } else {
                    None
                };

                // Token out will have a corresponding burn event
                let interest_token_out_change = if !skip_burn_for_token_out {
                    extract_lending_pool_token_change(
                        trx,
                        log,
                        &pool.address_vec(),
                        &interest_token_out_address,
                        true,
                        lending_pool_type,
                    )
                } else {
                    None
                };

                let swap_underlying_event = SwapUnderlyingLendingEvent {
                    token_in: Some(token_amount_in),
                    token_out: Some(token_amount_out),
                    interest_bearing_token_in_action: interest_token_in_change,
                    interest_bearing_token_out_action: interest_token_out_change,
                };

                pool_events.push(PoolEvent {
                    transaction_hash: Hex::encode(&trx.hash),
                    tx_index: trx.index,
                    log_index: log.index,
                    log_ordinal: log.ordinal,
                    to_address: pool.address.to_string(),
                    from_address: Hex::encode(buyer),
                    timestamp: blk.timestamp_seconds(),
                    block_number: blk.number,
                    pool_address: pool.address.to_string(),
                    r#type: Some(Type::SwapUnderlyingLendingEvent(swap_underlying_event)),
                })
            }
        }
    }
}

fn determine_underlying_exchange_tokens(
    pool: &Pool,
    sold_id: i32,
    bought_id: i32,
    tokens_sold: &BigInt,
    tokens_bought: &BigInt,
    uniswap_prices: &StoreGetProto<Erc20Price>,
    chainlink_prices: &StoreGetBigDecimal,
) -> Result<(TokenAmount, TokenAmount), Error> {
    let get_token_info = |id: i32, pool: &Pool| -> Result<(Token, TokenSource), Error> {
        match &pool.pool_type {
            Some(PoolType::MetaPool(meta)) => {
                if id == 0 {
                    pool.input_tokens
                        .iter()
                        .find(|t| t.index == id.to_string())
                        .map(|t| (t.clone(), TokenSource::MetaPool))
                        .ok_or_else(|| anyhow!("Token not found in metapool"))
                } else {
                    let adjusted_id = id - meta.max_coin.to_i32().unwrap();
                    meta.underlying_tokens
                        .iter()
                        .find(|t| t.index == adjusted_id.to_string())
                        .map(|t| (t.clone(), TokenSource::BasePool))
                        .ok_or_else(|| anyhow!("Token not found in base pool"))
                }
            }
            _ => Err(anyhow!(
                "Unsupported pool type or metapool information not available"
            )),
        }
    };

    let (token_in, token_in_source) = get_token_info(sold_id, &pool)?;
    let (token_out, token_out_source) = get_token_info(bought_id, &pool)?;

    let (token_in_price, token_in_price_source) =
        get_token_usd_price(&token_in, &uniswap_prices, &chainlink_prices);
    let (token_out_price, token_out_price_source) =
        get_token_usd_price(&token_out, &uniswap_prices, &chainlink_prices);

    let token_in = TokenAmount {
        token_address: token_in.address,
        amount: tokens_sold.to_string(),
        amount_usd: (tokens_sold.to_decimal(token_in.decimals) * token_in_price.clone())
            .to_string(),
        source: token_in_source as i32,
        token_price: token_in_price.to_string(),
        price_source: token_in_price_source as i32,
    };

    let token_out = TokenAmount {
        token_address: token_out.address,
        amount: tokens_bought.to_string(),
        amount_usd: (tokens_bought.to_decimal(token_out.decimals) * token_out_price.clone())
            .to_string(),
        source: token_out_source as i32,
        token_price: token_out_price.to_string(),
        price_source: token_out_price_source as i32,
    };

    Ok((token_in, token_out))
}

fn extract_lp_token_change(
    trx: &TransactionTrace,
    log: &Log,
    pool: &Pool,
    lp_token_address: &Vec<u8>,
    is_burn: bool,
) -> Option<LpTokenChange> {
    let pool_address = pool.address_vec();
    let null_address = NULL_ADDRESS.to_vec();

    let from: &Vec<u8>;
    let to: &Vec<u8>;

    // Determine the 'from' and 'to' addresses based on whether it's a burn or mint event.
    if is_burn {
        from = &pool_address;
        to = &null_address;
    } else {
        from = &null_address;
        to = &pool_address;
    }

    // Attempt to extract the specific transfer event for the LP token.
    match event_extraction::extract_specific_transfer_event(
        trx,
        Some(lp_token_address),
        Some(from),
        Some(to),
        None,
        log.index,
    ) {
        Ok(transfer) => Some(LpTokenChange {
            token_address: Hex::encode(transfer.token_address),
            amount: transfer.transfer.value.into(),
            change_type: if is_burn {
                LpTokenChangeType::Burn
            } else {
                LpTokenChangeType::Mint
            } as i32,
        }),
        Err(e) => {
            substreams::log::debug!("Error extracting LP token change: {:?}", e);
            None
        }
    }
}

fn extract_lending_pool_token_change(
    trx: &TransactionTrace,
    log: &Log,
    pool_address: &Vec<u8>,
    token_address: &Vec<u8>,
    is_burn: bool,
    lending_pool_type: &LendingPoolType,
) -> Option<LpTokenChange> {
    let null_address = &NULL_ADDRESS.to_vec();

    // Determine the 'from' and 'to' addresses based on the lending pool
    //  type, and whether it's a burn or mint event.
    let (from, to) = match lending_pool_type {
        LendingPoolType::CompoundLending(_)
        | LendingPoolType::CompoundTetherLending(_)
        | LendingPoolType::IronbankLending(_) => {
            if is_burn {
                (pool_address, token_address)
            } else {
                (token_address, pool_address)
            }
        }
        LendingPoolType::AaveLending(_)
        | LendingPoolType::YIearnLending(_)
        | LendingPoolType::PaxLending(_) => {
            if is_burn {
                (pool_address, null_address)
            } else {
                (null_address, pool_address)
            }
        }
    };

    // Attempt to extract the specific transfer event for the LP token.
    match event_extraction::extract_specific_transfer_event(
        trx,
        Some(token_address),
        Some(from),
        Some(to),
        None,
        log.index,
    ) {
        Ok(transfer) => Some(LpTokenChange {
            token_address: Hex::encode(transfer.token_address),
            amount: transfer.transfer.value.into(),
            change_type: if is_burn {
                LpTokenChangeType::Burn
            } else {
                LpTokenChangeType::Mint
            } as i32,
        }),
        Err(e) => {
            substreams::log::debug!("Error extracting LP token change: {:?}", e);
            None
        }
    }
}

fn extract_deposit_event(
    pool_events: &mut Vec<PoolEvent>,
    blk: &eth::Block,
    trx: &TransactionTrace,
    log: &Log,
    pool: &Pool,
    token_amounts: Vec<BigInt>,
    fees: Vec<String>,
    provider: Vec<u8>,
    uniswap_prices: &StoreGetProto<Erc20Price>,
    chainlink_prices: &StoreGetBigDecimal,
) {
    substreams::log::info!(format!(
        "Extracting Deposit from transaction {} and pool {}",
        Hex::encode(&trx.hash),
        &pool.address
    ));
    let pool_address = match Hex::decode(&pool.output_token_ref().address.clone()) {
        Ok(address) => address,
        Err(e) => {
            substreams::log::debug!("Error in `extract_deposit_event`: {:?}", e);
            return;
        }
    };

    let input_token_amounts = pool
        .input_tokens
        .iter()
        .filter_map(|token| {
            token
                .index
                .parse::<usize>()
                .ok()
                .and_then(|index| token_amounts.get(index))
                .map(|amount| {
                    let (token_price, price_source) =
                        get_token_usd_price(token, &uniswap_prices, &chainlink_prices);
                    TokenAmount {
                        token_address: token.address.clone(),
                        amount: amount.into(),
                        amount_usd: (amount.to_decimal(token.decimals) * token_price.clone())
                            .to_string(),
                        source: TokenSource::Default as i32,
                        token_price: token_price.to_string(),
                        price_source: price_source as i32,
                    }
                })
        })
        .collect::<Vec<TokenAmount>>();

    let output_token_amount = event_extraction::extract_specific_transfer_event(
        &trx,
        Some(&pool_address),
        Some(&NULL_ADDRESS.to_vec()),
        Some(&provider),
        None,
        log.index,
    )
    .or_else(|_| {
        // If finding a `Transfer` event with the provider as `to` fails, it may involve a Deposit Zap contract.
        // In such cases, LP tokens are sent to the user interacting with the Deposit Zap, not the contract itself.
        // Hence, we retry without specifying the `to` address, though the initial attempt aims for precision when possible.
        event_extraction::extract_specific_transfer_event(
            &trx,
            Some(&pool_address),
            Some(&NULL_ADDRESS.to_vec()),
            None,
            None,
            log.index,
        )
    })
    .map(|transfer| transfer.transfer.value)
    .unwrap_or_else(|e| {
        substreams::log::debug!("Error in `map_extract_pool_events`: {:?}", e);
        BigInt::zero()
    });
    let (output_token_price, output_token_price_source) =
        get_token_usd_price(pool.output_token_ref(), &uniswap_prices, &chainlink_prices);

    let deposit_event = DepositEvent {
        input_tokens: input_token_amounts,
        output_token: Some(TokenAmount {
            token_address: pool.output_token_ref().address.clone(),
            amount: output_token_amount.clone().into(),
            amount_usd: (output_token_amount.to_decimal(pool.output_token_ref().decimals)
                * output_token_price.clone())
            .to_string(),
            source: TokenSource::Default as i32,
            token_price: output_token_price.to_string(),
            price_source: output_token_price_source as i32,
        }),
        fees,
    };
    pool_events.push(PoolEvent {
        transaction_hash: Hex::encode(&trx.hash),
        tx_index: trx.index,
        log_index: log.index,
        log_ordinal: log.ordinal,
        to_address: pool.address.clone(),
        from_address: Hex::encode(provider),
        timestamp: blk.timestamp_seconds(),
        block_number: blk.number,
        pool_address: pool.address.clone(),
        r#type: Some(Type::DepositEvent(deposit_event)),
    })
}

// Multiple Token Withdrawl
fn extract_withdraw_event(
    pool_events: &mut Vec<PoolEvent>,
    blk: &eth::Block,
    trx: &TransactionTrace,
    log: &Log,
    pool: &Pool,
    provider: Vec<u8>,
    token_amounts: Vec<BigInt>,
    fees: Vec<String>,
    uniswap_prices: &StoreGetProto<Erc20Price>,
    chainlink_prices: &StoreGetBigDecimal,
) {
    substreams::log::info!(format!(
        "Extracting Withdrawal from transaction {} and pool {}",
        Hex::encode(&trx.hash),
        &pool.address
    ));
    let pool_address = match Hex::decode(&pool.output_token_ref().address.clone()) {
        Ok(address) => address,
        Err(e) => {
            substreams::log::debug!("Error in `extract_withdraw_event`: {:?}", e);
            return;
        }
    };

    let input_token_amounts = pool
        .input_tokens
        .iter()
        .filter_map(|token| {
            token
                .index
                .parse::<usize>()
                .ok()
                .and_then(|index| token_amounts.get(index))
                .map(|amount| {
                    let (token_price, price_source) =
                        get_token_usd_price(token, &uniswap_prices, &chainlink_prices);
                    TokenAmount {
                        token_address: token.address.clone(),
                        amount: amount.into(),
                        amount_usd: (amount.to_decimal(token.decimals) * token_price.clone())
                            .to_string(),
                        source: TokenSource::Default as i32,
                        token_price: token_price.to_string(),
                        price_source: price_source as i32,
                    }
                })
        })
        .collect::<Vec<TokenAmount>>();

    let output_token_amount = match event_extraction::extract_specific_transfer_event(
        &trx,
        Some(&pool_address),
        Some(&provider),
        Some(&NULL_ADDRESS.to_vec()),
        None,
        log.index,
    ) {
        Ok(burn_transfer) => burn_transfer.transfer.value,
        Err(e) => {
            substreams::log::debug!("Error in `map_extract_pool_events`: {:?}", e);
            BigInt::zero()
        }
    };
    let (output_token_price, output_token_price_source) =
        get_token_usd_price(pool.output_token_ref(), &uniswap_prices, &chainlink_prices);

    let withdraw_event = WithdrawEvent {
        input_tokens: input_token_amounts,
        output_token: Some(TokenAmount {
            token_address: pool.output_token_ref().address.clone(),
            amount: output_token_amount.clone().into(),
            amount_usd: (output_token_amount.to_decimal(pool.output_token_ref().decimals)
                * output_token_price.clone())
            .to_string(),
            source: TokenSource::Default as i32,
            token_price: output_token_price.to_string(),
            price_source: output_token_price_source as i32,
        }),
        fees,
    };
    pool_events.push(PoolEvent {
        transaction_hash: Hex::encode(&trx.hash),
        tx_index: trx.index,
        log_index: log.index,
        log_ordinal: log.ordinal,
        to_address: pool.address.clone(),
        from_address: Hex::encode(provider),
        timestamp: blk.timestamp_seconds(),
        block_number: blk.number,
        pool_address: pool.address.clone(),
        r#type: Some(Type::WithdrawEvent(withdraw_event)),
    })
}

// Single Token Withdrawal
fn extract_withdraw_one_event(
    pool_events: &mut Vec<PoolEvent>,
    blk: &eth::Block,
    trx: &TransactionTrace,
    log: &Log,
    pool: &Pool,
    provider: Vec<u8>,
    token_amount: BigInt,
    coin_amount: BigInt,
    uniswap_prices: &StoreGetProto<Erc20Price>,
    chainlink_prices: &StoreGetBigDecimal,
) {
    let pool_address = &pool.address;
    substreams::log::info!(format!(
        "Extracting Withdraw from transaction {} and pool {}",
        Hex::encode(&trx.hash),
        pool_address
    ));

    let mut token_transfer = event_extraction::extract_specific_transfer_event(
        trx,
        None,
        Some(&pool.address_vec()),
        Some(&provider),
        Some(&coin_amount),
        log.index,
    );

    // If we cannot get the `Transfer` event from the initial condition, the transfer may originate
    // from a  zap and we should try again without specifying the `to` address.
    if token_transfer.is_err() {
        token_transfer = event_extraction::extract_specific_transfer_event(
            trx,
            None,
            Some(&pool.address_vec()),
            None,
            Some(&coin_amount),
            log.index,
        );
    }

    let mut is_eth_transfer = false;

    // Check if this transaction involves native ETH transfers (no token event).
    if token_transfer.is_err() {
        // let coin_amount_bytes_be = coin_amount.to_signed_bytes_be();
        trx.calls().for_each(|call| {
            if let Some(value) = &call.call.value {
                let eth_value: BigInt = value.into();
                if eth_value == coin_amount {
                    is_eth_transfer = true;
                }
            }
        });
    }

    let input_tokens = pool
        .input_tokens_ordered
        .iter()
        .map(|address| {
            if is_eth_transfer && address == &Hex::encode(&ETH_ADDRESS) {
                if let Some(token) = pool.input_tokens.iter().find(|t| &t.address == address) {
                    let (token_price, price_source) =
                        get_token_usd_price(token, &uniswap_prices, &chainlink_prices);
                    return TokenAmount {
                        token_address: address.clone(),
                        amount: coin_amount.clone().into(),
                        amount_usd: (coin_amount.to_decimal(token.decimals) * token_price.clone())
                            .to_string(),
                        source: TokenSource::Default as i32,
                        token_price: token_price.to_string(),
                        price_source: price_source as i32,
                    };
                }
            } else if let Ok(transfer) = &token_transfer {
                if &Hex::encode(&transfer.token_address) == address {
                    if let Some(token) = pool.input_tokens.iter().find(|t| &t.address == address) {
                        let (token_price, price_source) =
                            get_token_usd_price(token, &uniswap_prices, &chainlink_prices);

                        return TokenAmount {
                            token_address: address.clone(),
                            amount: coin_amount.clone().into(),
                            amount_usd: (coin_amount.to_decimal(token.decimals)
                                * token_price.clone())
                            .to_string(),
                            source: TokenSource::Default as i32,
                            token_price: token_price.to_string(),
                            price_source: price_source as i32,
                        };
                    }
                }
            }
            TokenAmount {
                token_address: address.clone(),
                amount: BigInt::zero().into(),
                amount_usd: "0".to_string(),
                source: TokenSource::Default as i32,
                token_price: "0".to_string(),
                price_source: PriceSource::Unknown as i32,
            }
        })
        .collect();

    let (output_token_price, output_token_price_source) =
        get_token_usd_price(pool.output_token_ref(), &uniswap_prices, &chainlink_prices);

    let withdraw_event = WithdrawEvent {
        input_tokens,
        output_token: Some(TokenAmount {
            token_address: pool.output_token_ref().address.clone(),
            amount: token_amount.clone().into(),
            amount_usd: (token_amount.to_decimal(pool.output_token_ref().decimals)
                * output_token_price.clone())
            .to_string(),
            source: TokenSource::Default as i32,
            token_price: output_token_price.to_string(),
            price_source: output_token_price_source as i32,
        }),
        fees: Vec::new(),
    };

    pool_events.push(PoolEvent {
        transaction_hash: Hex::encode(&trx.hash),
        tx_index: trx.index,
        log_index: log.index,
        log_ordinal: log.ordinal,
        to_address: pool_address.to_string(),
        from_address: Hex::encode(provider),
        timestamp: blk.timestamp_seconds(),
        block_number: blk.number,
        pool_address: pool_address.to_string(),
        r#type: Some(Type::WithdrawEvent(withdraw_event)),
    })
}

// Determines whether mint or burn actions should be skipped for a token based on its index and the pool type.
// For Pax/CompoundTether Lending Pools, if the ID matches below, it implies that there is no interaction
// required with the underlying lending protocol as these coins are part of the base lending pool.
fn should_skip_token_action(lending_pool_type: &LendingPoolType, token_index: &BigInt) -> bool {
    match lending_pool_type {
        LendingPoolType::CompoundTetherLending(_) => token_index.eq(&BigInt::from(2)),
        LendingPoolType::PaxLending(_) => token_index.eq(&BigInt::from(3)),
        _ => false,
    }
}
