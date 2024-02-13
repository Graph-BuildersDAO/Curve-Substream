use substreams::pb::substreams::Clock;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{
    StoreGet, StoreGetBigDecimal, StoreGetBigInt, StoreGetInt64, StoreGetProto, StoreGetString,
};
use substreams_entity_change::tables::Tables;

use crate::common::pool_utils::{get_input_token_balances, get_input_token_weights};
use crate::common::prices::get_token_usd_price;
use crate::key_management::entity_key_manager::EntityKey;
use crate::key_management::store_key_manager::StoreKey;
use crate::pb::curve::types::v1::Pool;
use crate::pb::uniswap_pricing::v1::Erc20Price;
use crate::types::snapshot::SnapshotType;

pub struct SnapshotCreator<'a> {
    tables: &'a mut Tables,
    clock: &'a Clock,
    pool_count_store: &'a StoreGetInt64,
    pool_addresses_store: &'a StoreGetString,
    pools_store: &'a StoreGetProto<Pool>,
    pool_tvl_store: &'a StoreGetBigDecimal,
    pool_volume_usd_store: &'a StoreGetBigDecimal,
    pool_volume_native_store: &'a StoreGetBigInt,
    protocol_tvl_store: &'a StoreGetBigDecimal,
    protocol_volume_store: &'a StoreGetBigDecimal,
    input_token_balances_store: &'a StoreGetBigInt,
    output_token_supply_store: &'a StoreGetBigInt,
    uniswap_prices: &'a StoreGetProto<Erc20Price>,
    chainlink_prices: &'a StoreGetBigDecimal,
}

impl<'a> SnapshotCreator<'a> {
    pub fn new(
        tables: &'a mut Tables,
        clock: &'a Clock,
        pool_count_store: &'a StoreGetInt64,
        pool_addresses_store: &'a StoreGetString,
        pools_store: &'a StoreGetProto<Pool>,
        pool_tvl_store: &'a StoreGetBigDecimal,
        pool_volume_usd_store: &'a StoreGetBigDecimal,
        pool_volume_native_store: &'a StoreGetBigInt,
        protocol_tvl_store: &'a StoreGetBigDecimal,
        protocol_volume_store: &'a StoreGetBigDecimal,
        input_token_balances_store: &'a StoreGetBigInt,
        output_token_supply_store: &'a StoreGetBigInt,
        uniswap_prices: &'a StoreGetProto<Erc20Price>,
        chainlink_prices: &'a StoreGetBigDecimal,
    ) -> Self {
        Self {
            tables,
            clock,
            pool_count_store,
            pool_addresses_store,
            pools_store,
            pool_tvl_store,
            pool_volume_usd_store,
            pool_volume_native_store,
            protocol_tvl_store,
            protocol_volume_store,
            input_token_balances_store,
            output_token_supply_store,
            uniswap_prices,
            chainlink_prices,
        }
    }

    pub fn create_protocol_financials_daily_snapshot(&mut self, day_id: &i64) {
        let tvl_usd = self
            .protocol_tvl_store
            .get_last(StoreKey::protocol_tvl_key())
            .unwrap();
        let daily_volume = self
            .protocol_volume_store
            .get_last(StoreKey::protocol_daily_volume_usd_key(&day_id))
            .unwrap();
        let cumulative_volume = self
            .protocol_volume_store
            .get_last(StoreKey::protocol_volume_usd_key())
            .unwrap();
        self.tables
            .create_row(
                "FinancialsDailySnapshot",
                EntityKey::protocol_daily_financials_key(&day_id),
            )
            .set("protocol", EntityKey::protocol_key())
            .set("totalValueLockedUSD", tvl_usd)
            .set("dailyVolumeUSD", daily_volume)
            .set("cumulativeVolumeUSD", cumulative_volume)
            .set("blockNumber", BigInt::from(self.clock.number))
            .set(
                "timestamp",
                BigInt::from(self.clock.timestamp.clone().unwrap().seconds),
            );
    }

    pub fn create_liquidity_pool_snapshots(
        &mut self,
        snapshot_type: &SnapshotType,
        time_frame_id: &i64,
    ) {
        let pool_count = self
            .pool_count_store
            .get_last(StoreKey::protocol_pool_count_key())
            .unwrap();

        for i in 1..=pool_count {
            let pool_address = self
                .pool_addresses_store
                .get_last(StoreKey::pool_address_key(&i))
                .unwrap();

            let pool = self
                .pools_store
                .get_last(StoreKey::pool_key(&pool_address))
                .unwrap();

            let pool_tvl_usd = self
                .pool_tvl_store
                .get_last(StoreKey::pool_tvl_key(&pool.address))
                .unwrap();

            // Get the volume in the pool for a given timeframe (Daily/Hourly)
            let pool_volume = match snapshot_type {
                SnapshotType::Daily => self
                    .pool_volume_usd_store
                    .get_last(StoreKey::pool_volume_usd_daily_key(
                        &pool.address,
                        &time_frame_id,
                    ))
                    .unwrap(),
                SnapshotType::Hourly => self
                    .pool_volume_usd_store
                    .get_last(StoreKey::pool_volume_usd_hourly_key(
                        &pool.address,
                        &time_frame_id,
                    ))
                    .unwrap(),
            };

            // Get the volumes of each pool input token for a given timeframe (Daily/Hourly)
            let (volume_by_token_native, volume_by_token_usd) = get_pool_token_volumes_in_timeframe(
                &pool,
                time_frame_id,
                snapshot_type,
                self.pool_volume_native_store,
                self.pool_volume_usd_store,
            );

            let pool_cumulative_volume_usd = self
                .pool_volume_usd_store
                .get_last(StoreKey::pool_volume_usd_key(&pool.address))
                .unwrap();

            let input_token_balances = get_input_token_balances(
                &pool_address,
                &pool.input_tokens,
                &self.input_token_balances_store,
            );

            let input_token_weights = get_input_token_weights(&pool, &self.pool_tvl_store);

            let output_token_supply = self
                .output_token_supply_store
                .get_last(StoreKey::output_token_supply_key(&pool_address))
                .unwrap_or_else(|| BigInt::zero());

            let output_token_price = get_token_usd_price(
                pool.output_token_ref(),
                &self.uniswap_prices,
                &self.chainlink_prices,
            );

            // Create the relevant timeframe snapshot
            match snapshot_type {
                SnapshotType::Daily => Self::create_pool_daily_snapshot(
                    self.tables,
                    self.clock,
                    time_frame_id,
                    &pool_address,
                    &pool_tvl_usd,
                    &pool_volume,
                    &volume_by_token_native,
                    &volume_by_token_usd,
                    &pool_cumulative_volume_usd,
                    &input_token_balances,
                    &input_token_weights,
                    &output_token_supply,
                    &output_token_price,
                ),
                SnapshotType::Hourly => Self::create_pool_hourly_snapshot(
                    self.tables,
                    self.clock,
                    time_frame_id,
                    &pool_address,
                    &pool_tvl_usd,
                    &pool_volume,
                    &volume_by_token_native,
                    &volume_by_token_usd,
                    &pool_cumulative_volume_usd,
                    &input_token_balances,
                    &input_token_weights,
                    &output_token_supply,
                    &output_token_price,
                ),
            }
        }
    }

    fn create_pool_daily_snapshot(
        tables: &mut Tables,
        clock: &Clock,
        day_id: &i64,
        pool_address: &str,
        pool_tvl_usd: &BigDecimal,
        pool_volume_daily: &BigDecimal,
        volume_by_token_native: &Vec<BigInt>,
        volume_by_token_usd: &Vec<BigDecimal>,
        pool_cumulative_volume_usd: &BigDecimal,
        input_token_balances: &Vec<BigInt>,
        input_token_weights: &Vec<BigDecimal>,
        output_token_supply: &BigInt,
        output_token_price: &BigDecimal,
    ) {
        tables
            .create_row(
                "LiquidityPoolDailySnapshot",
                EntityKey::pool_daily_snapshot_key(pool_address, day_id),
            )
            .set("protocol", EntityKey::protocol_key())
            .set("pool", EntityKey::liquidity_pool_key(pool_address))
            .set("blockNumber", BigInt::from(clock.number))
            .set(
                "timestamp",
                BigInt::from(clock.timestamp.clone().unwrap().seconds),
            )
            .set("totalValueLockedUSD", pool_tvl_usd)
            .set("dailyVolumeUSD", pool_volume_daily)
            .set("dailyVolumeByTokenAmount", volume_by_token_native)
            .set("dailyVolumeByTokenUSD", volume_by_token_usd)
            .set("cumulativeVolumeUSD", pool_cumulative_volume_usd)
            .set("inputTokenBalances", input_token_balances)
            .set("inputTokenWeights", input_token_weights)
            .set("outputTokenSupply", output_token_supply)
            .set("outputTokenPriceUSD", output_token_price);
    }

    fn create_pool_hourly_snapshot(
        tables: &mut Tables,
        clock: &Clock,
        hour_id: &i64,
        pool_address: &str,
        pool_tvl_usd: &BigDecimal,
        pool_volume_hourly: &BigDecimal,
        volume_by_token_native: &Vec<BigInt>,
        volume_by_token_usd: &Vec<BigDecimal>,
        pool_cumulative_volume_usd: &BigDecimal,
        input_token_balances: &Vec<BigInt>,
        input_token_weights: &Vec<BigDecimal>,
        output_token_supply: &BigInt,
        output_token_price: &BigDecimal,
    ) {
        tables
            .create_row(
                "LiquidityPoolHourlySnapshot",
                EntityKey::pool_hourly_snapshot_key(&pool_address, hour_id),
            )
            .set("protocol", EntityKey::protocol_key())
            .set("pool", EntityKey::liquidity_pool_key(pool_address))
            .set("blockNumber", BigInt::from(clock.number))
            .set(
                "timestamp",
                BigInt::from(clock.timestamp.clone().unwrap().seconds),
            )
            .set("totalValueLockedUSD", pool_tvl_usd)
            .set("hourlyVolumeUSD", pool_volume_hourly)
            .set("hourlyVolumeByTokenAmount", volume_by_token_native)
            .set("hourlyVolumeByTokenUSD", volume_by_token_usd)
            .set("cumulativeVolumeUSD", pool_cumulative_volume_usd)
            .set("inputTokenBalances", input_token_balances)
            .set("inputTokenWeights", input_token_weights)
            .set("outputTokenSupply", output_token_supply)
            .set("outputTokenPriceUSD", output_token_price);
    }
}

fn get_pool_token_volumes_in_timeframe(
    pool: &Pool,
    time_frame_id: &i64,
    snapshot_type: &SnapshotType,
    pool_volume_native_store: &StoreGetBigInt,
    pool_volume_usd_store: &StoreGetBigDecimal,
) -> (Vec<BigInt>, Vec<BigDecimal>) {
    let mut pool_volume_by_token_native: Vec<BigInt> = Vec::new();
    let mut pool_volume_by_token_usd: Vec<BigDecimal> = Vec::new();

    for token in &pool.input_tokens {
        let native_volume_key = match snapshot_type {
            SnapshotType::Daily => StoreKey::pool_token_volume_native_daily_key(
                &pool.address,
                &token.address,
                &time_frame_id,
            ),
            SnapshotType::Hourly => StoreKey::pool_token_volume_native_hourly_key(
                &pool.address,
                &token.address,
                &time_frame_id,
            ),
        };
        let usd_volume_key = match snapshot_type {
            SnapshotType::Daily => StoreKey::pool_token_volume_usd_daily_key(
                &pool.address,
                &token.address,
                &time_frame_id,
            ),
            SnapshotType::Hourly => StoreKey::pool_token_volume_usd_hourly_key(
                &pool.address,
                &token.address,
                &time_frame_id,
            ),
        };

        // Fetch and push the native volume for the token
        let token_native_volume = pool_volume_native_store
            .get_last(&native_volume_key)
            .unwrap_or_else(|| BigInt::from(0)); // Default to 0 if not found
        pool_volume_by_token_native.push(token_native_volume);

        // Fetch and push the USD volume for the token
        let token_usd_volume = pool_volume_usd_store
            .get_last(&usd_volume_key)
            .unwrap_or_else(|| BigDecimal::from(0)); // Default to 0 if not found
        pool_volume_by_token_usd.push(token_usd_volume);
    }

    (pool_volume_by_token_native, pool_volume_by_token_usd)
}
