use substreams::store::{StoreAddBigInt, StoreDelete};

use crate::{
    key_management::store_key_manager::StoreKey,
    timeframe_management::pruning::traits::TokenPruneAction, types::timeframe::Timeframe,
};

pub struct TokenVolumeNativePruner<'a> {
    pub store: &'a StoreAddBigInt,
}

impl<'a> TokenPruneAction for TokenVolumeNativePruner<'a> {
    // Prunes volume native data for a specific token within a pool.
    fn prune_token(
        &self,
        pool_address: &str,
        token_address: &str,
        prune_time_frame_id: &i64,
        timeframe: &Timeframe,
    ) {
        let volume_native_key = match timeframe {
            Timeframe::Daily => StoreKey::pool_token_volume_native_daily_key(
                pool_address,
                token_address,
                prune_time_frame_id,
            ),
            Timeframe::Hourly => StoreKey::pool_token_volume_native_hourly_key(
                pool_address,
                token_address,
                prune_time_frame_id,
            ),
        };
        self.store.delete_prefix(0, &volume_native_key);
    }
}
