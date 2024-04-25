# Alloy Flashbots

An [Alloy] transport to send transaction bundles via [Flashbots].

[Alloy]: https://github.com/alloy-rs/alloy
[Flashbots]: https://docs.flashbots.net/

## Installation

Add `alloy-flashbots` to your `Cargo.toml`.

```toml
alloy-flashbots = { git = "https://github.com/leruaa/alloy-flashbots" }
```

## Usage

```rust
use std::env;

use alloy_flashbots::{
    rpc::{Inclusion, SendBundleRequest},
    FlashbotsLayer, FlashbotsProviderExt, FlashbotsTransactionBuilderExt,
};
use alloy_primitives::{address, U256};
use alloy::network::{Ethereum, EthereumSigner};
use alloy::providers::ProviderBuilder;
use alloy::rpc::client::RpcClient;
use alloy::rpc::types::eth::TransactionRequest;
use alloy::signers::LocalWallet;
use anyhow::Result;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let eth_rpc = env::var("ETH_HTTP_RPC")?;

    // This is your searcher identity
    let bundle_signer = LocalWallet::random();

    // This signs transactions
    let tx_signer = EthereumSigner::from(LocalWallet::random());

    // Build a provider with Flashbots
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(tx_signer.clone())
        .on_http_with_flashbots(eth_rpc.parse()?, bundle_signer.clone());

    // Pay Vitalik using a Flashbots bundle!
    let tx = TransactionRequest::default()
        .from(tx_signer.default_signer_address())
        .to(Some(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"))) // vitalik.eth
        .value(U256::from(1000000000));

    // Build a bundle...
    let bundle = SendBundleRequest {
        bundle_body: vec![provider.build_bundle_item(tx, false).await?],
        inclusion: Inclusion::at_block(provider.get_block_number().await? + 1),
        ..Default::default()
    };

    // ... and send it!
    let response = provider.send_bundle(bundle).await?;

    println!("Bundle hash: {}", response.bundle_hash);

    Ok(())
}
```

## TODO

- [x] Extension trait
- [x] Reqwest HTTP transport
- [ ] Hyper HTTP transport
- [x] Rewrite RPC types from `mev-share-rs` ones that depends on `ethers`
- [x] Add a method to build a `BundleItem` from a `TransactionRequest`

## Credits

- [alloy]
- [ethers-flashbots]
- [mev-share-rs]

[alloy]: https://github.com/alloy-rs
[ethers-flashbots]: https://github.com/onbjerg/ethers-flashbots
[mev-share-rs]: https://github.com/paradigmxyz/mev-share-rs