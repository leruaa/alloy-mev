use std::sync::Arc;

use alloy_transport_http::Http;

mod reqwest;

#[derive(Debug, Clone)]
pub struct FlashbotsHttp<T, Signer> {
    http: Http<T>,
    signer: Arc<Signer>,
}

impl<T, Signer> FlashbotsHttp<T, Signer> {
    /// Create a new [`Flashbots`] transport.
    pub fn new(http: Http<T>, signer: Arc<Signer>) -> Self {
        Self { http, signer }
    }
}
