# Alloy MEV

Easily send transaction bundles using [Alloy].

[Alloy]: https://github.com/alloy-rs/alloy

## Installation

Add `alloy-mev` to your `Cargo.toml`.

```toml
alloy-mev = "0.2"
```

## Features

### MEV-Share

This crate contains the `MevShareProviderExt` extension trait. When it's
in scope, it adds methods to send bundles to the Flashbots matchmaker on your
provider built on an HTTP transport.

```rust
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
    let response = provider.send_mev_bundle(bundle, bundle_signer).await?;

    println!("Bundle hash: {}", response.bundle_hash);

    Ok(())
}
```

### Blocks builders

This crate also contains the `EthMevProviderExt` extension trait that adds
methods to broadcast bundles to blocks builders on yourprovider built on an
HTTP transport.

```rust
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let eth_rpc = env::var("ETH_HTTP_RPC").unwrap();
    let bundle_signer = PrivateKeySigner::random();
    let tx_signer = EthereumWallet::new(signer.clone());

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(tx_signer.clone())
        .on_http(eth_rpc.parse().unwrap());

    // Select which builders the bundle will be sent to
    let endpoints = provider
        .endpoints_builder()
        .beaverbuild()
        .titan(BundleSigner::flashbots(bundle_signer.clone()))
        .build();

    let block_number: u64 = provider.get_block_number().await.unwrap().into();

    // Pay Vitalik using a MEV-Share bundle!
    let tx = TransactionRequest::default()
        .from(tx_signer.default_signer_address())
        .to(Some(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"))) // vitalik.eth
        .value(U256::from(1000000000));

    // Broadcast the bundle to all builders setup above!
    let responses = provider
        .send_eth_bundle(
            EthSendBundle {
                txs: vec![provider.encode_request(tx)],
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
}
```

## Credits

- [alloy]
- [ethers-flashbots]
- [mev-share-rs]

[alloy]: https://github.com/alloy-rs
[ethers-flashbots]: https://github.com/onbjerg/ethers-flashbots
[mev-share-rs]: https://github.com/paradigmxyz/mev-share-rs
