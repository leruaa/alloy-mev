use std::marker::PhantomData;
use std::{env, sync::Arc};

use alloy_flashbots::{FlashbotsLayer, FlashbotsProviderExt};
use alloy_network::Ethereum;
use alloy_providers::NetworkRpcClient;
use alloy_rpc_client::RpcClient;
use alloy_signer::LocalWallet;
use dotenv::dotenv;
use mev_share::rpc::SendBundleRequest;
use mev_share::rpc::SimBundleOverrides;

#[tokio::test]
async fn test_sim_bundle() {
    dotenv().ok();
    let eth_rpc = env::var("ETH_HTTP_RPC").unwrap();
    let signer = LocalWallet::random();

    let client = RpcClient::builder()
        .layer(FlashbotsLayer::new(Arc::new(signer)))
        .reqwest_http(eth_rpc.parse().unwrap());

    let provider = NetworkRpcClient::<Ethereum, _> {
        network: PhantomData,
        client,
    };

    let bundle_body = vec![];

    let bundle = SendBundleRequest {
        bundle_body,
        ..Default::default()
    };

    let x = provider
        .sim_bundle(bundle, SimBundleOverrides::default())
        .await;

    println!("{x:?}");
}
