use substreams::store::{StoreAddBigDecimal, StoreDelete};

use crate::{
    key_management::store_key_manager::StoreKey, timeframe_management::pruning::Pruner,
    types::timeframe::Timeframe,
};

pub struct ProtocolVolumeUsdPruneAction<'a> {
    pub store: &'a StoreAddBigDecimal,
}

impl<'a> Pruner for ProtocolVolumeUsdPruneAction<'a> {
    // Prunes volume usd data for a specific token within a pool.
    fn prune(&self, prune_time_frame_id: i64, timeframe: Timeframe) {
        match timeframe {
            Timeframe::Daily => {
                let key = StoreKey::protocol_daily_volume_usd_key(&prune_time_frame_id);
                self.store.delete_prefix(0, &key);
            }
            Timeframe::Hourly => {}
        }
    }
}
