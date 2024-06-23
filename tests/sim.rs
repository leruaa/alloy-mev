use std::env;

use alloy::network::EthereumWallet;
use alloy::primitives::address;
use alloy::primitives::U256;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::eth::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use alloy_mev::{
    rpc::mev::{Inclusion, SendBundleRequest, SimBundleOverrides},
    MevCapableProviderBuilderExt, MevProviderExt,
};
use dotenv::dotenv;

#[tokio::test]
async fn test_sim_bundle() {
    dotenv().ok();
    let eth_rpc = env::var("ETH_HTTP_RPC").unwrap();
    let signer = PrivateKeySigner::random();
    let wallet = EthereumWallet::new(signer.clone());

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .with_bundle_management()
        .bundle_signer(signer.clone())
        .on_http(eth_rpc.parse().unwrap());

    let block_number = provider.get_block_number().await.unwrap();

    let tx = TransactionRequest::default()
        .from(signer.address())
        .to(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"))
        .value(U256::from(1000000000));

    let bundle = SendBundleRequest {
        bundle_body: vec![provider.build_bundle_item(tx, false).await.unwrap()],
        inclusion: Inclusion::at_block(block_number + 1),
        ..Default::default()
    };

    let x = provider
        .sim_bundle(bundle, SimBundleOverrides::default())
        .await;

    println!("{x:?}");
}
