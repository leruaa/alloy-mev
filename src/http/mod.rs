use alloy_transport_http::Http;

#[cfg(feature = "reqwest")]
mod reqwest;

/// An Alloy `Transport` that handle `mev_sendBundle` and `mev_simBundle`
/// requests and delegates all others to the inner `Transport`.
#[derive(Debug, Clone)]
pub struct FlashbotsHttp<T, S> {
    http: Http<T>,
    signer: S,
}

impl<T, S> FlashbotsHttp<T, S> {
    /// Create a new [`FlashbotsHttp`] transport.
    pub fn new(http: Http<T>, signer: S) -> Self {
        Self { http, signer }
    }
}
