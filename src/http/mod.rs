use std::sync::Arc;

use alloy_transport_http::Http;

#[cfg(feature = "reqwest")]
mod reqwest;

/// An Alloy `Transport` that handle `mev_sendBundle` and `mev_simBundle`
/// requests and delegates all others to the inner `Transport`.
#[derive(Debug, Clone)]
pub struct FlashbotsHttp<T, Signer> {
    http: Http<T>,
    signer: Arc<Signer>,
}

impl<T, Signer> FlashbotsHttp<T, Signer> {
    /// Create a new [`FlashbotsHttp`] transport.
    pub fn new(http: Http<T>, signer: Arc<Signer>) -> Self {
        Self { http, signer }
    }
}
