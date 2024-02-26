use substreams::store::{StoreAddInt64, StoreDelete};

use crate::{
    key_management::store_key_manager::StoreKey, timeframe_management::pruning::Pruner,
    types::timeframe::Timeframe,
};

pub struct ProtocolUsageMetricsPruneAction<'a> {
    pub store: &'a StoreAddInt64,
}

impl<'a> Pruner for ProtocolUsageMetricsPruneAction<'a> {
    // Prunes protocol active user data for a specific timeframe.
    fn prune(&self, prune_time_frame_id: i64, timeframe: Timeframe) {
        match timeframe {
            Timeframe::Daily => {
                self.store.delete_prefix(
                    0,
                    &StoreKey::active_user_daily_count_key(&prune_time_frame_id),
                );
                self.store.delete_prefix(
                    0,
                    &StoreKey::transaction_daily_count_key(&prune_time_frame_id),
                );
                self.store
                    .delete_prefix(0, &StoreKey::swap_daily_count_key(&prune_time_frame_id));
                self.store
                    .delete_prefix(0, &StoreKey::deposit_daily_count_key(&prune_time_frame_id));
                self.store
                    .delete_prefix(0, &StoreKey::withdraw_daily_count_key(&prune_time_frame_id));
            }
            Timeframe::Hourly => {
                self.store.delete_prefix(
                    0,
                    &StoreKey::active_user_hourly_count_key(&prune_time_frame_id),
                );
                self.store.delete_prefix(
                    0,
                    &StoreKey::transaction_hourly_count_key(&prune_time_frame_id),
                );
                self.store
                    .delete_prefix(0, &StoreKey::swap_hourly_count_key(&prune_time_frame_id));
                self.store
                    .delete_prefix(0, &StoreKey::deposit_hourly_count_key(&prune_time_frame_id));
                self.store.delete_prefix(
                    0,
                    &StoreKey::withdraw_hourly_count_key(&prune_time_frame_id),
                );
            }
        }
    }
}
