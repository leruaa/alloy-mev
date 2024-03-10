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

TBD

## TODO

- [x] Extension trait
- [x] Reqwest HTTP transport
- [ ] Hyper HTTP transport
- [x] Rewrite RPC types from `mev-share-rs` ones that depends on `ethers`
- [x] Add a method to build a `BundleItem` from a `TransactionRequest` on the extension trait

## Credits

- [alloy]
- [ethers-flashbots]
- [mev-share-rs]

[alloy]: https://github.com/alloy-rs
[ethers-flashbots]: https://github.com/onbjerg/ethers-flashbots
[mev-share-rs]: https://github.com/paradigmxyz/mev-share-rs