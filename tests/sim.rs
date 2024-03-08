use std::{env, sync::Arc};

use alloy_flashbots::{FlashbotsLayer, FlashbotsProviderExt};
use alloy_network::eip2718::Encodable2718;
use alloy_network::Ethereum;
use alloy_network::EthereumSigner;
use alloy_network::TransactionBuilder;
use alloy_primitives::address;
use alloy_primitives::U256;
use alloy_providers::ProviderBuilder;
use alloy_rpc_client::RpcClient;
use alloy_rpc_types::TransactionRequest;
use alloy_signer::LocalWallet;
use dotenv::dotenv;
use mev_share::rpc::BundleItem;
use mev_share::rpc::SendBundleRequest;
use mev_share::rpc::SimBundleOverrides;

#[tokio::test]
async fn test_sim_bundle() {
    dotenv().ok();
    let eth_rpc = env::var("ETH_HTTP_RPC").unwrap();
    let wallet = LocalWallet::random();
    let signer = EthereumSigner::from(wallet.clone());

    let client = RpcClient::builder()
        .layer(FlashbotsLayer::new(Arc::new(wallet)))
        .reqwest_http(eth_rpc.parse().unwrap());

    let provider = ProviderBuilder::<_, Ethereum>::new().on_client(client);

    let envelope = TransactionRequest::default()
        .to(Some(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045")))
        .value(U256::from(1000000000))
        .build(&signer)
        .await
        .unwrap();

    let bundle_body = vec![BundleItem::Tx {
        tx: envelope.encoded_2718().into(),
        can_revert: false,
    }];

    let bundle = SendBundleRequest {
        bundle_body,
        ..Default::default()
    };

    let x = provider
        .sim_bundle(bundle, SimBundleOverrides::default())
        .await;

    println!("{x:?}");
}
