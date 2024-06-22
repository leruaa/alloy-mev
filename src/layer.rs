use alloy::transports::http::Http;
use tower::Layer;
use url::Url;

use crate::MevHttp;

/// A Tower layer to decorate an Alloy [`Transport`] with [`MevHttp`].
/// `MevLayer` can be used with Alloy [`ClientBuilder`].
///
/// [`Transport`]: https://alloy-rs.github.io/alloy/alloy_transport/trait.Transport.html
/// [`ClientBuilder`]: https://alloy-rs.github.io/alloy/alloy_rpc_client/builder/struct.ClientBuilder.html
#[derive(Debug, Default)]
pub struct MevLayer<S> {
    /// The MEV-Share url to send `mev_*` bundles
    pub mev_share_url: Option<Url>,
    /// The signer used to sign bundles
    pub bundle_signer: Option<S>,
}

impl<S> MevLayer<S> {
    /// Create a new `MevLayer`, using the given [`Signer`] to produce
    /// the `X-Flashbots-Signature` header.
    ///
    /// [`Signer`]:  https://alloy-rs.github.io/alloy/alloy_signer/trait.Signer.html
    pub const fn new() -> Self {
        Self {
            mev_share_url: None,
            bundle_signer: None,
        }
    }
}

#[cfg(feature = "reqwest")]
impl<Signer: Clone> Layer<Http<reqwest::Client>> for MevLayer<Signer> {
    type Service = MevHttp<reqwest::Client, Signer>;

    fn layer(&self, inner: Http<reqwest::Client>) -> Self::Service {
        let mev_share_url = self
            .mev_share_url
            .clone()
            .unwrap_or("https://relay.flashbots.net".parse().unwrap());

        MevHttp::new(mev_share_url, inner, self.bundle_signer.clone())
    }
}
