use std::{fmt::Debug, sync::Arc};

use alloy::{primitives::Address, signers::Signer, transports::http::Http};
use url::Url;

mod broadcastable_call;
pub use broadcastable_call::BroadcastableCall;

mod endpoints;
pub use endpoints::{Endpoints, EndpointsBuilder};

#[cfg(feature = "reqwest")]
mod reqwest;

/// An Alloy `Transport` that add a signature in the headers for `mev_*` and
/// `eth_*` requests and delegates all others to the inner [`Transport`].
#[derive(Debug, Clone)]
pub struct MevHttp<T> {
    url: Url,
    http: Http<T>,
    bundle_signer: BundleSigner,
}

impl<T> MevHttp<T> {
    /// Creates a new [`MevHttp`] transport.
    pub const fn new(url: Url, http: Http<T>, bundle_signer: BundleSigner) -> Self {
        Self {
            url,
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
            url: "https://relay.flashbots.net".parse().unwrap(),
            http,
            bundle_signer: BundleSigner::flashbots(Box::new(signer)),
        }
    }
}

/// A [`Signer`] wrapper to sign bundles.
#[derive(Clone)]
pub struct BundleSigner {
    /// The header name on which set the signature.
    pub header: String,
    /// The signer used to sign the bundle.
    pub signer: Arc<dyn Signer + Send + Sync>,
}

impl BundleSigner {
    /// Creates a new [`BundleSigner`]
    pub fn new<S>(header: String, signer: S) -> Self
    where
        S: Signer + Send + Sync + 'static,
    {
        Self {
            header,
            signer: Arc::new(signer),
        }
    }

    /// Creates a [`BundleSigner`] set up to add the Flashbots header.
    pub fn flashbots<S>(signer: S) -> Self
    where
        S: Signer + Send + Sync + 'static,
    {
        Self {
            header: "X-Flashbots-Signature".to_string(),
            signer: Arc::new(signer),
        }
    }

    /// Returns the signer address.
    pub fn address(&self) -> Address {
        self.signer.address()
    }
}

impl Debug for BundleSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BundleSigner")
            .field("header", &self.header)
            .field("signer_address", &self.signer.address())
            .finish()
    }
}
