# Alloy Flashbots

An [Alloy] transport to send transaction bundles via [Flashbots].

[Alloy]: https://github.com/alloy-rs/alloy
[Flashbots]: https://docs.flashbots.net/

## Installation

Add `alloy-flashbots` to your `Cargo.toml`.

```toml
alloy-flashbots = "0.1"
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
    let wallet = EthereumSigner::from(LocalWallet::random());

    // Build a RPC client with the Flashbots layer...
    let client = RpcClient::builder()
        .layer(FlashbotsLayer::new(bundle_signer))
        .reqwest_http(eth_rpc.parse()?);

    // ... and a provider
    let provider = ProviderBuilder::<_, Ethereum>::new().on_client(client);

    // Pay Vitalik using a Flashbots bundle!
    let mut tx = TransactionRequest::default()
        .to(Some(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"))) // vitalik.eth
        .value(U256::from(1000000000));

    // Don't forget to populate nonce and gas fields on the tx ;)

    // Build a bundle...
    let bundle = SendBundleRequest {
        bundle_body: vec![tx.build_bundle_item(false, &wallet).await?],
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