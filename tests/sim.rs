use std::env;

use alloy::network::{Ethereum, EthereumSigner};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::client::RpcClient;
use alloy::rpc::types::eth::TransactionRequest;
use alloy::signers::LocalWallet;
use alloy_flashbots::{
    rpc::{Inclusion, SendBundleRequest, SimBundleOverrides},
    FlashbotsLayer, FlashbotsProviderExt, FlashbotsTransactionBuilderExt,
};
use alloy_primitives::address;
use alloy_primitives::U256;
use dotenv::dotenv;

#[tokio::test]
async fn test_sim_bundle() {
    dotenv().ok();
    let eth_rpc = env::var("ETH_HTTP_RPC").unwrap();
    let wallet = LocalWallet::random();
    let signer = EthereumSigner::from(wallet.clone());

    let client = RpcClient::builder()
        .layer(FlashbotsLayer::new(wallet.clone()))
        .reqwest_http(eth_rpc.parse().unwrap());

    let provider = ProviderBuilder::<_, Ethereum>::new().on_client(client);

    let block_number = provider.get_block_number().await.unwrap();

    let mut tx = TransactionRequest::default()
        .to(Some(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045")))
        .value(U256::from(1000000000));

    let nonce = provider
        .get_transaction_count(wallet.address(), None)
        .await
        .unwrap();

    tx = tx.nonce(nonce.to());

    provider.populate_gas_eip1559(&mut tx, None).await.unwrap();
    provider.populate_gas(&mut tx, None).await.unwrap();

    let bundle = SendBundleRequest {
        bundle_body: vec![tx.build_bundle_item(false, &signer).await.unwrap()],
        inclusion: Inclusion::at_block(block_number + 1),
        ..Default::default()
    };

    let x = provider
        .sim_bundle(bundle, SimBundleOverrides::default())
        .await;

    println!("{x:?}");
}
