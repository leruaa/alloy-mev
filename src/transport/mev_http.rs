use alloy::{signers::Signer, transports::http::Http};
use url::Url;

use super::bundle_signer::BundleSigner;

/// An Alloy `Transport` that add a signature in the headers for `mev_*` and
/// `eth_*` requests and delegates all others to the inner [`Transport`].
#[derive(Debug, Clone)]
pub struct MevHttp<T> {
    /// The endpoint to send requests.
    pub endpoint: Url,
    /// The inner transport to send non-MEV requests.
    pub http: Http<T>,
    /// The signer used to build bundles signatures.
    pub bundle_signer: BundleSigner,
}

impl<T> MevHttp<T> {
    /// Creates a new [`MevHttp`] transport.
    pub const fn new(endpoint: Url, http: Http<T>, bundle_signer: BundleSigner) -> Self {
        Self {
            endpoint,
            http,
            bundle_signer,
        }
    }

    /// Creates a transport to send requests to flashbots.
    pub fn flashbots<S>(http: Http<T>, signer: S) -> Self
    where
        S: Signer + Send + Sync + 'static,
    {
        Self {
            endpoint: "https://relay.flashbots.net".parse().unwrap(),
            http,
            bundle_signer: BundleSigner::flashbots(Box::new(signer)),
        }
    }
}
