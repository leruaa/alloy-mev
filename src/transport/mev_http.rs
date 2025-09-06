use alloy::{
    rpc::json_rpc::{RequestPacket, ResponsePacket},
    signers::Signer, 
    transports::{http::Http, BoxTransport, TransportError, TransportFut}
};
use std::task;
use tower::Service;
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

/// A version of MevHttp that works with BoxTransport
#[derive(Debug, Clone)]
pub struct MevHttpBox {
    /// The endpoint to send requests.
    pub endpoint: Url,
    /// The inner transport to send non-MEV requests.
    pub transport: BoxTransport,
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

impl MevHttpBox {
    /// Creates a new [`MevHttpBox`] transport.
    pub const fn new(endpoint: Url, transport: BoxTransport, bundle_signer: BundleSigner) -> Self {
        Self {
            endpoint,
            transport,
            bundle_signer,
        }
    }

    /// Creates a transport to send requests to flashbots using BoxTransport.
    pub fn flashbots<S>(transport: BoxTransport, signer: S) -> Self
    where
        S: Signer + Send + Sync + 'static,
    {
        Self {
            endpoint: "https://relay.flashbots.net".parse().unwrap(),
            transport,
            bundle_signer: BundleSigner::flashbots(Box::new(signer)),
        }
    }
}

impl Service<RequestPacket> for MevHttpBox {
    type Response = ResponsePacket;
    type Error = TransportError;
    type Future = TransportFut<'static>;

    fn poll_ready(&mut self, cx: &mut task::Context<'_>) -> task::Poll<Result<(), Self::Error>> {
        self.transport.poll_ready(cx)
    }

    fn call(&mut self, req: RequestPacket) -> Self::Future {
        // TODO: Add MEV-specific header signing logic here
        // For now, just delegate to the inner transport
        self.transport.call(req)
    }
}

