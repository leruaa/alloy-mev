#![doc = include_str!("../README.md")]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

mod builders;
pub use builders::{EthBundle, EthereumReqwestMevShareBundle, MevShareBundle};

mod ext;
pub use ext::{EthMevProviderExt, MevShareProviderExt};

mod transport;
pub use transport::{BroadcastableCall, BundleSigner, Endpoints, EndpointsBuilder, MevHttp};
