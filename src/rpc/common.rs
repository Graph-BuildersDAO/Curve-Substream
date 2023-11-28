use substreams::log;
use substreams_ethereum::{
    pb::eth::rpc::RpcResponse,
    rpc::{RPCDecodable, RpcBatch},
    Function,
};

pub fn decode_rpc_response<R, T: RPCDecodable<R> + Function>(
    response: &RpcResponse,
    log_message: &str,
) -> Option<R> {
    RpcBatch::decode::<_, T>(response).or_else(|| {
        log::debug!("{}", log_message);
        None
    })
}
