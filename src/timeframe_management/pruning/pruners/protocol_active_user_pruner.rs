use substreams::store::{StoreDelete, StoreSetIfNotExistsInt64};

use crate::{
    key_management::store_key_manager::StoreKey,
    timeframe_management::pruning::traits::ProtocolPruneAction, types::timeframe::Timeframe,
};

pub struct ProtocolActiveUserPruneAction<'a> {
    pub store: &'a StoreSetIfNotExistsInt64,
}

impl<'a> ProtocolPruneAction for ProtocolActiveUserPruneAction<'a> {
    // Prunes protocol active user data for a specific timeframe.
    fn prune_protocol(&self, prune_time_frame_id: &i64, timeframe: &Timeframe) {
        match timeframe {
            Timeframe::Daily => {
                let key = StoreKey::active_user_daily_prune_key(&prune_time_frame_id);
                self.store.delete_prefix(0, &key);
            }
            Timeframe::Hourly => {
                let key = StoreKey::active_user_hourly_prune_key(&prune_time_frame_id);
                self.store.delete_prefix(0, &key);
            }
        }
    }
}
