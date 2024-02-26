use substreams::store::{StoreAddBigInt, StoreDelete};

use crate::{
    key_management::store_key_manager::StoreKey, timeframe_management::pruning::Pruner,
    types::timeframe::Timeframe,
};

pub struct TokenVolumeNativePruner<'a> {
    pub store: &'a StoreAddBigInt,
}

impl<'a> Pruner for TokenVolumeNativePruner<'a> {
    fn prune(&self, prune_time_frame_id: i64, timeframe: Timeframe) {
        let volume_native_key = match timeframe {
            Timeframe::Daily => {
                StoreKey::pool_token_volume_native_daily_prune_key(&prune_time_frame_id)
            }
            Timeframe::Hourly => {
                StoreKey::pool_token_volume_native_hourly_prune_key(&prune_time_frame_id)
            }
        };
        self.store.delete_prefix(0, &volume_native_key);
    }
}
