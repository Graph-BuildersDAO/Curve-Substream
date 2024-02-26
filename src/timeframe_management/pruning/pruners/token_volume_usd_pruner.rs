use substreams::store::{StoreAddBigDecimal, StoreDelete};

use crate::{
    key_management::store_key_manager::StoreKey, timeframe_management::pruning::Pruner,
    types::timeframe::Timeframe,
};

pub struct TokenVolumeUsdPruner<'a> {
    pub store: &'a StoreAddBigDecimal,
}

impl<'a> Pruner for TokenVolumeUsdPruner<'a> {
    fn prune(&self, prune_time_frame_id: i64, timeframe: Timeframe) {
        let volume_usd_key = match timeframe {
            Timeframe::Daily => {
                StoreKey::pool_token_volume_usd_daily_prune_key(&prune_time_frame_id)
            }
            Timeframe::Hourly => {
                StoreKey::pool_token_volume_usd_hourly_prune_key(&prune_time_frame_id)
            }
        };
        self.store.delete_prefix(0, &volume_usd_key);
    }
}
