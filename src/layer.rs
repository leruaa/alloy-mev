use std::sync::Arc;

use alloy_transport_http::Http;
use tower::Layer;

use crate::FlashbotsHttp;

/// A Tower layer to decorate an Alloy [`Transport`] with [`FlashbotsHttp`].
/// `FlashbotsLayer` can be used with Alloy [`ClientBuilder`].
///
/// [`Transport`]: https://alloy-rs.github.io/alloy/alloy_transport/trait.Transport.html
/// [`ClientBuilder`]: https://alloy-rs.github.io/alloy/alloy_rpc_client/builder/struct.ClientBuilder.html
pub struct FlashbotsLayer<Signer> {
    signer: Arc<Signer>,
}

impl<Signer> FlashbotsLayer<Signer> {
    /// Create a new `FlashbotsLayer`, using the given [`Signer`] to produce
    /// the `X-Flashbots-Signature` header.
    ///
    /// [`Signer`]:  https://alloy-rs.github.io/alloy/alloy_signer/trait.Signer.html
    pub fn new(signer: Arc<Signer>) -> Self {
        Self { signer }
    }
}

#[cfg(feature = "reqwest")]
impl<Signer> Layer<Http<reqwest::Client>> for FlashbotsLayer<Signer> {
    type Service = FlashbotsHttp<reqwest::Client, Signer>;

    fn layer(&self, inner: Http<reqwest::Client>) -> Self::Service {
        FlashbotsHttp::new(inner, self.signer.clone())
    }
}
