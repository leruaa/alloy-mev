use alloy::{primitives::Address, signers::Signer, transports::http::Http};
use url::Url;

#[cfg(feature = "reqwest")]
mod reqwest;

/// An Alloy `Transport` that handle `mev_sendBundle` and `mev_simBundle`
/// requests and delegates all others to the inner `Transport`.
#[derive(Debug, Clone)]
pub struct MevHttp<T, S> {
    url: Url,
    http: Http<T>,
    bundle_signer: Option<BundleSigner<S>>,
}

impl<T, S> MevHttp<T, S> {
    /// Creates a new [`MevHttp`] transport.
    pub const fn new(url: Url, http: Http<T>, bundle_signer: Option<BundleSigner<S>>) -> Self {
        Self {
            url,
            http,
            bundle_signer,
        }
    }

    /// Creates a transport to send requests to flashbots.
    pub fn flashbots(http: Http<T>, bundle_signer: S) -> Self {
        Self {
            url: "https://relay.flashbots.net".parse().unwrap(),
            http,
            bundle_signer: Some(BundleSigner::new(
                "X-Flashbots-Signature".to_string(),
                bundle_signer,
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BundleSigner<S> {
    pub header: String,
    pub signer: S,
}

impl<S> BundleSigner<S> {
    pub const fn new(header: String, signer: S) -> Self {
        Self { header, signer }
    }
}

impl<S> BundleSigner<S>
where
    S: Signer,
{
    pub fn address(&self) -> Address {
        self.signer.address()
    }
}
