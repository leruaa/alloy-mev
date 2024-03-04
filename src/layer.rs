use std::sync::Arc;

use alloy_transport_http::Http;
use tower::Layer;

use crate::FlashbotsHttp;

pub struct FlashbotsLayer<Signer> {
    signer: Arc<Signer>,
}

impl<Signer> FlashbotsLayer<Signer> {
    pub fn new(signer: Arc<Signer>) -> Self {
        Self { signer }
    }
}

impl<Signer> Layer<Http<reqwest::Client>> for FlashbotsLayer<Signer> {
    type Service = FlashbotsHttp<reqwest::Client, Signer>;

    fn layer(&self, inner: Http<reqwest::Client>) -> Self::Service {
        FlashbotsHttp::new(inner, self.signer.clone())
    }
}
