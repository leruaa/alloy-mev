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
pub use http::{BroadcastableCall, Endpoints, EndpointsBuilder, MevHttp};

mod eth_provider_ext;
pub use eth_provider_ext::EthProviderExt;

mod mev_share_provider_ext;
pub use mev_share_provider_ext::MevShareProviderExt;
