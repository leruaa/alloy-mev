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
mod layer;
mod provider_builder_ext;
mod provider_ext;
pub mod rpc;

pub use http::FlashbotsHttp;
pub use layer::FlashbotsLayer;
pub use provider_builder_ext::FlashbotsProviderBuilderExt;
pub use provider_ext::FlashbotsProviderExt;
