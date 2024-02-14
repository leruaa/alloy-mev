use alloy_transport_http::Http;

mod reqwest;

#[derive(Debug, Clone)]
pub struct Flashbots<T, S> {
    http: Http<T>,
    signer: S,
}

impl<T, S> Flashbots<T, S> {
    /// Create a new [`Flashbots`] transport.
    pub fn new(http: Http<T>, signer: S) -> Self {
        Self { http, signer }
    }
}
