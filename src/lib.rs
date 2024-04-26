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

mod http;
pub use http::FlashbotsHttp;

mod layer;
pub use layer::FlashbotsLayer;

mod provider_builder_ext;
pub use provider_builder_ext::FlashbotsProviderBuilderExt;

mod provider_ext;
pub use provider_ext::FlashbotsProviderExt;

/// RPC types.
pub mod rpc;
