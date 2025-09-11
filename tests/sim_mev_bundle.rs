use std::env;

use alloy::{
    hex,
    network::EthereumWallet,
    providers::ProviderBuilder,
    rpc::types::mev::{BundleItem, Inclusion, MevSendBundle, SimBundleOverrides},
    signers::local::PrivateKeySigner,
};
use alloy_mev::MevShareProviderExt;
use dotenv::dotenv;

#[tokio::test]
async fn test_sim_mev_bundle() {
    dotenv().ok();
    let eth_rpc = env::var("ETH_HTTP_RPC").unwrap();
    let signer = PrivateKeySigner::random();
    let wallet = EthereumWallet::new(signer.clone());

    let provider = ProviderBuilder::new()
        .wallet(wallet.clone())
        .connect_http(eth_rpc.parse().unwrap());

    let block_number = 20247245;
    // tx 0x0722b12f3f46877a5251ecce105263ccf9f5390f9fab5ecc51e4858705fd8667
    let tx = hex!("02f876018204ed843b9aca0085012a05f20082a22794825001ac81d9348f71f2dadd717335ac0ab4a9fe89056a6418b50586000080c001a0e491ff34326cd113b9a1a34f2f82f57727d70dc78577a97ae54dd3a2b43b8583a06c956d5b1dae0514360d56186870c5d50771fc4b204931a5ace7e19baa7f0a86");

    let bundle = MevSendBundle {
        bundle_body: vec![BundleItem::Tx {
            tx: tx.into(),
            can_revert: false,
        }],
        inclusion: Inclusion::at_block(block_number),
        ..Default::default()
    };

    let x = provider
        .sim_mev_bundle(bundle, SimBundleOverrides::default(), signer)
        .await;

    println!("{x:?}");
}
