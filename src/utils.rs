use alloy::rpc::client::RpcClient;
use url::Url;

pub(crate) fn build_rpc_client(url: Url) -> RpcClient {
    cfg_if::cfg_if! {
        if #[cfg(feature = "reqwest")] {
            RpcClient::new_http(url)
        } else if #[cfg(feature = "hyper")] {
            RpcClient::builder().new_hyper(url)
        } else {
            panic!("One of 'reqwest' or 'hyper' features must be enabled")
        }
    }
}
