use substreams::pb::substreams::Clock;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{
    StoreGet, StoreGetBigDecimal, StoreGetBigInt, StoreGetInt64, StoreGetProto, StoreGetString,
};
use substreams_entity_change::tables::Tables;

use crate::common::conversion::convert_i64_to_i32;
use crate::common::pool_utils::{get_input_token_balances, get_input_token_weights};
use crate::common::prices::get_token_usd_price;
use crate::key_management::entity_key_manager::EntityKey;
use crate::key_management::store_key_manager::StoreKey;
use crate::pb::curve::types::v1::{Pool, PoolRewards};
use crate::pb::uniswap_pricing::v1::Erc20Price;
use crate::types::timeframe::Timeframe;

pub struct SnapshotCreator<'a> {
    tables: &'a mut Tables,
    clock: &'a Clock,
    usage_metrics_store: &'a StoreGetInt64,
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
    pool_rewards_store: &'a StoreGetProto<PoolRewards>,
    uniswap_prices: &'a StoreGetProto<Erc20Price>,
    chainlink_prices: &'a StoreGetBigDecimal,
}

impl<'a> SnapshotCreator<'a> {
    pub fn new(
        tables: &'a mut Tables,
        clock: &'a Clock,
        usage_metrics_store: &'a StoreGetInt64,
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
        pool_rewards_store: &'a StoreGetProto<PoolRewards>,
        uniswap_prices: &'a StoreGetProto<Erc20Price>,
        chainlink_prices: &'a StoreGetBigDecimal,
    ) -> Self {
        Self {
            tables,
            clock,
            usage_metrics_store,
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
            pool_rewards_store,
            uniswap_prices,
            chainlink_prices,
        }
    }

    pub fn create_usage_metrics_snapshots(
        &mut self,
        snapshot_type: &Timeframe,
        time_frame_id: &i64,
    ) {
        let timeframe_active_users = match snapshot_type {
            Timeframe::Daily => self
                .usage_metrics_store
                .get_last(StoreKey::active_user_daily_count_key(&time_frame_id))
                .unwrap_or_default(),
            Timeframe::Hourly => self
                .usage_metrics_store
                .get_last(StoreKey::active_user_hourly_count_key(&time_frame_id))
                .unwrap_or_default(),
        };

        let cumulative_active_users = self
            .usage_metrics_store
            .get_last(StoreKey::active_user_count_key())
            .unwrap_or_default();

        let timeframe_tx_count = match snapshot_type {
            Timeframe::Daily => self
                .usage_metrics_store
                .get_last(StoreKey::transaction_daily_count_key(&time_frame_id))
                .unwrap_or_default(),
            Timeframe::Hourly => self
                .usage_metrics_store
                .get_last(StoreKey::transaction_hourly_count_key(&time_frame_id))
                .unwrap_or_default(),
        };

        let timeframe_swap_count = match snapshot_type {
            Timeframe::Daily => self
                .usage_metrics_store
                .get_last(StoreKey::swap_daily_count_key(&time_frame_id))
                .unwrap_or_default(),
            Timeframe::Hourly => self
                .usage_metrics_store
                .get_last(StoreKey::swap_hourly_count_key(&time_frame_id))
                .unwrap_or_default(),
        };

        let timeframe_deposit_count = match snapshot_type {
            Timeframe::Daily => self
                .usage_metrics_store
                .get_last(StoreKey::deposit_daily_count_key(&time_frame_id))
                .unwrap_or_default(),
            Timeframe::Hourly => self
                .usage_metrics_store
                .get_last(StoreKey::deposit_hourly_count_key(&time_frame_id))
                .unwrap_or_default(),
        };

        let timeframe_withdraw_count = match snapshot_type {
            Timeframe::Daily => self
                .usage_metrics_store
                .get_last(StoreKey::withdraw_daily_count_key(&time_frame_id))
                .unwrap_or_default(),
            Timeframe::Hourly => self
                .usage_metrics_store
                .get_last(StoreKey::withdraw_hourly_count_key(&time_frame_id))
                .unwrap_or_default(),
        };

        let pool_count = self
            .pool_count_store
            .get_last(StoreKey::protocol_pool_count_key())
            .unwrap_or_default();

        match snapshot_type {
            Timeframe::Daily => Self::create_usage_metrics_daily_snapshot(
                self.tables,
                self.clock,
                time_frame_id,
                timeframe_active_users,
                cumulative_active_users,
                timeframe_tx_count,
                timeframe_swap_count,
                timeframe_deposit_count,
                timeframe_withdraw_count,
                pool_count,
            ),
            Timeframe::Hourly => Self::create_usage_metrics_hourly_snapshot(
                self.tables,
                self.clock,
                time_frame_id,
                timeframe_active_users,
                cumulative_active_users,
                timeframe_tx_count,
                timeframe_swap_count,
                timeframe_deposit_count,
                timeframe_withdraw_count,
                pool_count,
            ),
        }
    }

    fn create_usage_metrics_daily_snapshot(
        tables: &mut Tables,
        clock: &Clock,
        day_id: &i64,
        active_users: i64,
        cumulative_users: i64,
        tx_count: i64,
        swap_count: i64,
        deposit_count: i64,
        withdraw_count: i64,
        pool_count: i64,
    ) {
        tables
            .create_row("UsageMetricsDailySnapshot", day_id.to_string())
            .set("protocol", EntityKey::protocol_key())
            .set("dailyActiveUsers", convert_i64_to_i32(active_users))
            .set(
                "cumulativeUniqueUsers",
                convert_i64_to_i32(cumulative_users),
            )
            .set("dailyTransactionCount", convert_i64_to_i32(tx_count))
            .set("dailyDepositCount", convert_i64_to_i32(deposit_count))
            .set("dailyWithdrawCount", convert_i64_to_i32(withdraw_count))
            .set("dailySwapCount", convert_i64_to_i32(swap_count))
            .set("totalPoolCount", convert_i64_to_i32(pool_count))
            .set("blockNumber", BigInt::from(clock.number))
            .set(
                "timestamp",
                BigInt::from(clock.timestamp.clone().unwrap().seconds),
            );
    }

    fn create_usage_metrics_hourly_snapshot(
        tables: &mut Tables,
        clock: &Clock,
        hour_id: &i64,
        active_users: i64,
        cumulative_users: i64,
        tx_count: i64,
        swap_count: i64,
        deposit_count: i64,
        withdraw_count: i64,
        pool_count: i64,
    ) {
        tables
            .create_row("UsageMetricsHourlySnapshot", hour_id.to_string())
            .set("protocol", EntityKey::protocol_key())
            .set("hourlyActiveUsers", convert_i64_to_i32(active_users))
            .set(
                "cumulativeUniqueUsers",
                convert_i64_to_i32(cumulative_users),
            )
            .set("hourlyTransactionCount", convert_i64_to_i32(tx_count))
            .set("hourlyDepositCount", convert_i64_to_i32(deposit_count))
            .set("hourlyWithdrawCount", convert_i64_to_i32(withdraw_count))
            .set("hourlySwapCount", convert_i64_to_i32(swap_count))
            .set("totalPoolCount", convert_i64_to_i32(pool_count))
            .set("blockNumber", BigInt::from(clock.number))
            .set(
                "timestamp",
                BigInt::from(clock.timestamp.clone().unwrap().seconds),
            );
    }

    pub fn create_protocol_financials_daily_snapshot(&mut self, day_id: &i64) {
        let tvl_usd = self
            .protocol_tvl_store
            .get_last(StoreKey::protocol_tvl_key())
            .unwrap_or_else(|| BigDecimal::zero());
        let daily_volume = self
            .protocol_volume_store
            .get_last(StoreKey::protocol_daily_volume_usd_key(&day_id))
            .unwrap_or_else(|| BigDecimal::zero());
        let cumulative_volume = self
            .protocol_volume_store
            .get_last(StoreKey::protocol_volume_usd_key())
            .unwrap_or_else(|| BigDecimal::zero());
        self.tables
            .create_row(
                "FinancialsDailySnapshot",
                EntityKey::protocol_daily_financials_key(&day_id),
            )
            .set("protocol", EntityKey::protocol_key())
            .set("totalValueLockedUSD", tvl_usd)
            .set("dailyVolumeUSD", daily_volume)
            .set("cumulativeVolumeUSD", cumulative_volume)
            .set("dailySupplySideRevenueUSD", BigDecimal::zero())
            .set("cumulativeSupplySideRevenueUSD", BigDecimal::zero())
            .set("dailyProtocolSideRevenueUSD", BigDecimal::zero())
            .set("cumulativeProtocolSideRevenueUSD", BigDecimal::zero())
            .set("dailyTotalRevenueUSD", BigDecimal::zero())
            .set("cumulativeTotalRevenueUSD", BigDecimal::zero())
            .set("blockNumber", BigInt::from(self.clock.number))
            .set(
                "timestamp",
                BigInt::from(self.clock.timestamp.clone().unwrap().seconds),
            );
    }

    pub fn create_liquidity_pool_snapshots(
        &mut self,
        snapshot_type: &Timeframe,
        time_frame_id: &i64,
    ) {
        let pool_count = match self
            .pool_count_store
            .get_last(StoreKey::protocol_pool_count_key())
        {
            Some(count) => count,
            None => return,
        };

        for i in 1..=pool_count {
            let pool_address = match self
                .pool_addresses_store
                .get_last(StoreKey::pool_address_key(&i))
            {
                Some(address) => address,
                None => return,
            };

            let pool = match self.pools_store.get_last(StoreKey::pool_key(&pool_address)) {
                Some(pool) => pool,
                None => return,
            };

            let pool_tvl_usd = self
                .pool_tvl_store
                .get_last(StoreKey::pool_tvl_key(&pool.address))
                .unwrap_or_else(|| BigDecimal::zero());

            // Get the volume in the pool for a given timeframe (Daily/Hourly)
            let pool_volume = match snapshot_type {
                Timeframe::Daily => self
                    .pool_volume_usd_store
                    .get_last(StoreKey::pool_volume_usd_daily_key(
                        &time_frame_id,
                        &pool.address,
                    ))
                    .unwrap_or_else(|| BigDecimal::zero()),
                Timeframe::Hourly => self
                    .pool_volume_usd_store
                    .get_last(StoreKey::pool_volume_usd_hourly_key(
                        &time_frame_id,
                        &pool.address,
                    ))
                    .unwrap_or_else(|| BigDecimal::zero()),
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
                .unwrap_or_else(|| BigDecimal::zero());

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

            let (output_token_price, _) = get_token_usd_price(
                pool.output_token_ref(),
                &self.uniswap_prices,
                &self.chainlink_prices,
            );

            let pool_rewards = match self
                .pool_rewards_store
                .get_last(StoreKey::pool_rewards_key(&pool_address))
            {
                Some(rewards) => rewards,
                // Provide a default representing no rewards
                None => PoolRewards {
                    staked_output_token_amount: "0".to_string(),
                    reward_token_emissions_native: Vec::new(),
                    reward_token_emissions_usd: Vec::new(),
                },
            };

            // Create the relevant timeframe snapshot
            match snapshot_type {
                Timeframe::Daily => Self::create_pool_daily_snapshot(
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
                    &pool_rewards,
                ),
                Timeframe::Hourly => Self::create_pool_hourly_snapshot(
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
                    &pool_rewards,
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
        pool_rewards: &PoolRewards,
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
            // Revenue related data is currently set to a default of zero until fees are implemented
            .set("cumulativeSupplySideRevenueUSD", BigDecimal::zero())
            .set("dailySupplySideRevenueUSD", BigDecimal::zero())
            .set("cumulativeProtocolSideRevenueUSD", BigDecimal::zero())
            .set("dailyProtocolSideRevenueUSD", BigDecimal::zero())
            .set("cumulativeTotalRevenueUSD", BigDecimal::zero())
            .set("dailyTotalRevenueUSD", BigDecimal::zero())
            .set("dailyVolumeUSD", pool_volume_daily)
            .set("dailyVolumeByTokenAmount", volume_by_token_native)
            .set("dailyVolumeByTokenUSD", volume_by_token_usd)
            .set("cumulativeVolumeUSD", pool_cumulative_volume_usd)
            .set("inputTokenBalances", input_token_balances)
            .set("inputTokenWeights", input_token_weights)
            .set("outputTokenSupply", output_token_supply)
            .set("outputTokenPriceUSD", output_token_price)
            .set(
                "stakedOutputTokenAmount",
                pool_rewards.parse_staked_output_token_amount(),
            )
            .set(
                "rewardTokenEmissionsAmount",
                pool_rewards.parse_reward_token_emissions_native(),
            )
            .set(
                "rewardTokenEmissionsUSD",
                pool_rewards.parse_reward_token_emissions_usd(),
            );
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
        pool_rewards: &PoolRewards,
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
            // Revenue related data is currently set to a default of zero until fees are implemented
            .set("cumulativeSupplySideRevenueUSD", BigDecimal::zero())
            .set("hourlySupplySideRevenueUSD", BigDecimal::zero())
            .set("cumulativeProtocolSideRevenueUSD", BigDecimal::zero())
            .set("hourlyProtocolSideRevenueUSD", BigDecimal::zero())
            .set("cumulativeTotalRevenueUSD", BigDecimal::zero())
            .set("hourlyTotalRevenueUSD", BigDecimal::zero())
            .set("hourlyVolumeUSD", pool_volume_hourly)
            .set("hourlyVolumeByTokenAmount", volume_by_token_native)
            .set("hourlyVolumeByTokenUSD", volume_by_token_usd)
            .set("cumulativeVolumeUSD", pool_cumulative_volume_usd)
            .set("inputTokenBalances", input_token_balances)
            .set("inputTokenWeights", input_token_weights)
            .set("outputTokenSupply", output_token_supply)
            .set("outputTokenPriceUSD", output_token_price)
            .set(
                "stakedOutputTokenAmount",
                pool_rewards.parse_staked_output_token_amount(),
            )
            .set(
                "rewardTokenEmissionsAmount",
                pool_rewards.parse_reward_token_emissions_native(),
            )
            .set(
                "rewardTokenEmissionsUSD",
                pool_rewards.parse_reward_token_emissions_usd(),
            );
    }
}

fn get_pool_token_volumes_in_timeframe(
    pool: &Pool,
    time_frame_id: &i64,
    snapshot_type: &Timeframe,
    pool_volume_native_store: &StoreGetBigInt,
    pool_volume_usd_store: &StoreGetBigDecimal,
) -> (Vec<BigInt>, Vec<BigDecimal>) {
    let mut pool_volume_by_token_native: Vec<BigInt> = Vec::new();
    let mut pool_volume_by_token_usd: Vec<BigDecimal> = Vec::new();

    for token in &pool.input_tokens {
        let native_volume_key = match snapshot_type {
            Timeframe::Daily => StoreKey::pool_token_volume_native_daily_key(
                &time_frame_id,
                &pool.address,
                &token.address,
            ),
            Timeframe::Hourly => StoreKey::pool_token_volume_native_hourly_key(
                &time_frame_id,
                &pool.address,
                &token.address,
            ),
        };
        let usd_volume_key = match snapshot_type {
            Timeframe::Daily => StoreKey::pool_token_volume_usd_daily_key(
                &time_frame_id,
                &pool.address,
                &token.address,
            ),
            Timeframe::Hourly => StoreKey::pool_token_volume_usd_hourly_key(
                &time_frame_id,
                &pool.address,
                &token.address,
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
