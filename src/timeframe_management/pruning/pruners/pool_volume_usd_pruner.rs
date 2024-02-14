use substreams::store::{StoreAddBigDecimal, StoreDelete};

use crate::{
    key_management::store_key_manager::StoreKey,
    timeframe_management::pruning::traits::PoolPruneAction, types::timeframe::Timeframe,
};

pub struct PoolVolumeUsdPruner<'a> {
    pub store: &'a StoreAddBigDecimal,
}

impl<'a> PoolPruneAction for PoolVolumeUsdPruner<'a> {
    // Prunes volume usd data for a specific token within a pool.
    fn prune_pool(&self, pool_address: &str, prune_time_frame_id: &i64, timeframe: &Timeframe) {
        // Prune daily/hourly pool volume data
        let pool_volume_usd_key = match timeframe {
            Timeframe::Daily => {
                StoreKey::pool_volume_usd_daily_key(&pool_address, &prune_time_frame_id)
            }
            Timeframe::Hourly => {
                StoreKey::pool_volume_usd_hourly_key(&pool_address, &prune_time_frame_id)
            }
        };
        self.store.delete_prefix(0, &pool_volume_usd_key);
    }
}
