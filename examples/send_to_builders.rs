use std::env;

use alloy::{
    network::EthereumWallet,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{mev::EthSendBundle, TransactionRequest},
    signers::local::PrivateKeySigner,
};
use alloy_mev::{BundleSigner, EthMevProviderExt};
use anyhow::Result;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let eth_rpc = env::var("ETH_HTTP_RPC")?;
    let bundle_signer = PrivateKeySigner::random();
    let tx_signer = EthereumWallet::new(bundle_signer.clone());

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(tx_signer.clone())
        .on_http(eth_rpc.parse()?);

    // Select which builders the bundle will be sent to
    let endpoints = provider
        .endpoints_builder()
        .beaverbuild()
        .titan(BundleSigner::flashbots(bundle_signer.clone()))
        .build();

    let block_number: u64 = provider.get_block_number().await?;

    // Pay Vitalik using a MEV-Share bundle!
    let tx = TransactionRequest::default()
        .to(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045")) // vitalik.eth
        .value(U256::from(1000000000));

    // Broadcast the bundle to all builders setup above!
    let responses = provider
        .send_eth_bundle(
            EthSendBundle {
                txs: vec![provider.encode_request(tx).await?],
                block_number: block_number + 1,
                min_timestamp: None,
                max_timestamp: None,
                reverting_tx_hashes: vec![],
                replacement_uuid: None,
            },
            &endpoints,
        )
        .await;

    println!("{responses:#?}");

    Ok(())
}
