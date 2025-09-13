# Alloy MEV

Easily send transaction bundles to MEV-Share or block builders using [Alloy].

You can have a look at [the docs] to help you get started. There are also
some [examples] available. 


[Alloy]: https://github.com/alloy-rs/alloy
[the docs]: https://docs.rs/alloy-mev/latest/alloy_mev/
[examples]: https://github.com/leruaa/alloy-mev/tree/main/examples

## Installation

Add `alloy-mev` to your `Cargo.toml`:

```toml
alloy-mev = "1.0.0"
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