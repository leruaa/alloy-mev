use std::env;

use alloy::{
    network::EthereumWallet,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use alloy_mev::EthMevProviderExt;
use anyhow::Result;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let eth_rpc = env::var("ETH_HTTP_RPC")?;
    let bundle_signer = PrivateKeySigner::random();
    let tx_signer = EthereumWallet::new(bundle_signer.clone());

    let provider = ProviderBuilder::new()
        .wallet(tx_signer.clone())
        .connect_http(eth_rpc.parse()?);

    // Select which builders the bundle will be sent to
    let endpoints = provider
        .endpoints_builder()
        .beaverbuild()
        .titan(bundle_signer.clone())
        .build();

    let block_number: u64 = provider.get_block_number().await?;

    // Pay Vitalik using a MEV-Share bundle!
    let tx = TransactionRequest::default()
        .to(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045")) // vitalik.eth
        .value(U256::from(1000000000));

    let bundle = provider
        .bundle_builder()
        .on_block(block_number + 1)
        .add_transaction_request(tx)
        .await?
        .build();

    // Broadcast the bundle to all builders setup above!
    let responses = provider.send_eth_bundle(bundle, &endpoints).await;

    println!("{responses:#?}");

    Ok(())
}
