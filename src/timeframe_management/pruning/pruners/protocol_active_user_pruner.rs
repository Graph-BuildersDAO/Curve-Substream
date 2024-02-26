use substreams::store::{StoreDelete, StoreSetIfNotExistsInt64};

use crate::{
    key_management::store_key_manager::StoreKey, timeframe_management::pruning::Pruner,
    types::timeframe::Timeframe,
};

pub struct ProtocolActiveUserPruneAction<'a> {
    pub store: &'a StoreSetIfNotExistsInt64,
}

impl<'a> Pruner for ProtocolActiveUserPruneAction<'a> {
    // Prunes protocol active user data for a specific timeframe.
    fn prune(&self, prune_time_frame_id: i64, timeframe: Timeframe) {
        let active_user_key = match timeframe {
            Timeframe::Daily => StoreKey::active_user_daily_prune_key(&prune_time_frame_id),
            Timeframe::Hourly => StoreKey::active_user_hourly_prune_key(&prune_time_frame_id),
        };
        self.store.delete_prefix(0, &active_user_key);
    }
}
