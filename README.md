# Alloy MEV

Easily send transaction bundles using [Alloy].

[Alloy]: https://github.com/alloy-rs/alloy

## Installation

Add `alloy-mev` to your `Cargo.toml`.

```toml
alloy-mev = "0.1"
```

## Usage

```rust
use std::env;

use alloy_mev::{
    rpc::{Inclusion, SendBundleRequest},
    MevLayer, MevProviderExt, MevCapableProviderBuilderExt,
};
use alloy_primitives::{address, U256};
use alloy::network::{Ethereum, EthereumWallet};
use alloy::providers::ProviderBuilder;
use alloy::rpc::types::eth::TransactionRequest;
use alloy::signers::local::LocalSigner;
use anyhow::Result;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let eth_rpc = env::var("ETH_HTTP_RPC")?;

    // This is your searcher identity
    let bundle_signer = LocalSigner::random();

    // This signs transactions
    let tx_signer = EthereumWallet::from(LocalSigner::random());

    // Build a provider with MEV
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(tx_signer.clone())
        .with_bundle_management()
        .bundle_signer(bundle_signer)
        .on_http(eth_rpc.parse()?);

    // Pay Vitalik using a MEV-Share bundle!
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

## Credits

- [alloy]
- [ethers-flashbots]
- [mev-share-rs]

[alloy]: https://github.com/alloy-rs
[ethers-flashbots]: https://github.com/onbjerg/ethers-flashbots
[mev-share-rs]: https://github.com/paradigmxyz/mev-share-rs
