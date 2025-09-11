use std::env;

use alloy::{
    hex,
    network::EthereumWallet,
    providers::ProviderBuilder,
    rpc::types::{mev::EthCallBundle, BlockNumberOrTag},
    signers::local::PrivateKeySigner,
};
use alloy_mev::EthMevProviderExt;
use dotenv::dotenv;

#[cfg(feature = "reqwest")]
#[tokio::test]
async fn test_call_eth_bundle() {
    dotenv().ok();
    let eth_rpc = env::var("ETH_HTTP_RPC").unwrap();
    let signer = PrivateKeySigner::random();
    let wallet = EthereumWallet::new(signer.clone());

    let provider = ProviderBuilder::new()
        .wallet(wallet.clone())
        .connect_http(eth_rpc.parse().unwrap());

    let endpoints = provider
        .endpoints_builder()
        .flashbots(signer.clone())
        .build();

    let block_number = 20247245;

    let x = provider
        .call_eth_bundle(
            EthCallBundle {
                // tx 0x0722b12f3f46877a5251ecce105263ccf9f5390f9fab5ecc51e4858705fd8667
                txs: vec![hex!("02f876018204ed843b9aca0085012a05f20082a22794825001ac81d9348f71f2dadd717335ac0ab4a9fe89056a6418b50586000080c001a0e491ff34326cd113b9a1a34f2f82f57727d70dc78577a97ae54dd3a2b43b8583a06c956d5b1dae0514360d56186870c5d50771fc4b204931a5ace7e19baa7f0a86").into()],
                block_number,
                state_block_number: BlockNumberOrTag::Number(block_number - 1),
                transaction_index: None,
                coinbase: None,
                timestamp: None,
                timeout: None,
                gas_limit: None,
                difficulty: None,
                base_fee: None,
            },
            &endpoints,
        )
        .await;

    println!("{x:#?}");
}
