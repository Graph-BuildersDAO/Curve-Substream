use crate::types::timeframe::Timeframe;

pub trait ProtocolPruneAction {
    fn prune_protocol(&self, prune_time_frame_id: &i64, timeframe: &Timeframe);
}
pub trait PoolPruneAction {
    fn prune_pool(&self, pool_address: &str, prune_time_frame_id: &i64, timeframe: &Timeframe);
}

pub trait TokenPruneAction {
    fn prune_token(
        &self,
        pool_address: &str,
        token_address: &str,
        prune_time_frame_id: &i64,
        timeframe: &Timeframe,
    );
}
