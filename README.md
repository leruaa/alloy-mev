# Alloy MEV

Easily send transaction bundles using [Alloy].

[Alloy]: https://github.com/alloy-rs/alloy

## Installation

Add `alloy-mev` to your `Cargo.toml`:

```toml
alloy-mev = "0.2"
```

## Features

### MEV-Share

This crate contains the [`MevShareProviderExt`] extension trait. When it's
in scope, it adds methods to send bundles to the Flashbots matchmaker on a
provider built on an HTTP transport.

### Blocks builders

This crate also contains the [`EthMevProviderExt`] extension trait that adds
methods to broadcast bundles to blocks builders on a provider built on an
HTTP transport.

## Credits

- [alloy]
- [ethers-flashbots]
- [mev-share-rs]

[alloy]: https://github.com/alloy-rs
[ethers-flashbots]: https://github.com/onbjerg/ethers-flashbots
[mev-share-rs]: https://github.com/paradigmxyz/mev-share-rs
[`MevShareProviderExt`]: https://docs.rs/alloy-mev/latest/alloy_mev/trait.MevShareProviderExt.html
[`EthMevProviderExt`]: https://docs.rs/alloy-mev/latest/alloy_mev/trait.EthMevProviderExt.html