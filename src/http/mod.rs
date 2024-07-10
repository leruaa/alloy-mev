use std::{fmt::Debug, sync::Arc};

use alloy::{primitives::Address, signers::Signer, transports::http::Http};
use url::Url;

mod broadcastable_call;
pub use broadcastable_call::BroadcastableCall;

mod endpoints;
pub use endpoints::{Endpoints, EndpointsBuilder};

#[cfg(feature = "reqwest")]
mod reqwest;

/// An Alloy `Transport` that handle `mev_sendBundle` and `mev_simBundle`
/// requests and delegates all others to the inner `Transport`.
#[derive(Debug, Clone)]
pub struct MevHttp<T> {
    url: Url,
    http: Http<T>,
    bundle_signer: Option<BundleSigner>,
}

impl<T> MevHttp<T> {
    /// Creates a new [`MevHttp`] transport.
    pub const fn new(url: Url, http: Http<T>, bundle_signer: Option<BundleSigner>) -> Self {
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
            bundle_signer: Some(BundleSigner::new(
                "X-Flashbots-Signature".to_string(),
                Box::new(signer),
            )),
        }
    }
}

#[derive(Clone)]
pub struct BundleSigner {
    pub header: String,
    pub signer: Arc<dyn Signer + Send + Sync>,
}

impl BundleSigner {
    pub fn new<S>(header: String, signer: S) -> Self
    where
        S: Signer + Send + Sync + 'static,
    {
        Self {
            header,
            signer: Arc::new(signer),
        }
    }

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
