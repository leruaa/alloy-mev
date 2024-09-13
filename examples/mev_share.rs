use std::env;

use alloy::{
    network::EthereumWallet,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{
        mev::{Inclusion, SendBundleRequest},
        TransactionRequest,
    },
    signers::local::PrivateKeySigner,
};
use alloy_mev::MevShareProviderExt;
use anyhow::Result;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let eth_rpc = env::var("ETH_HTTP_RPC")?;

    // This is your searcher identity
    let bundle_signer = PrivateKeySigner::random();

    // This signs transactions
    let tx_signer = EthereumWallet::from(PrivateKeySigner::random());

    // Build a provider with MEV
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(tx_signer.clone())
        .on_http(eth_rpc.parse()?);

    // Pay Vitalik using a MEV-Share bundle!
    let tx = TransactionRequest::default()
        .to(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045")) // vitalik.eth
        .value(U256::from(1000000000));

    // Build a bundle...
    let bundle = SendBundleRequest {
        bundle_body: vec![provider.build_bundle_item(tx, false).await?],
        inclusion: Inclusion::at_block(provider.get_block_number().await? + 1),
        ..Default::default()
    };

    // ... and send it!
    let response = provider.send_mev_bundle(bundle, bundle_signer).await?;

    println!("Bundle hash: {}", response.bundle_hash);

    Ok(())
}
