use std::sync::Arc;

use alloy_network::Network;
use alloy_providers::NetworkRpcClient;
use alloy_transport::{Transport, TransportResult};
use alloy_transport_http::Http;
use async_trait::async_trait;
use mev_share::rpc::{
    SendBundleRequest, SendBundleResponse, SimBundleOverrides, SimBundleResponse,
};

mod reqwest;

#[derive(Debug, Clone)]
pub struct Flashbots<T, Signer> {
    http: Http<T>,
    signer: Arc<Signer>,
}

impl<T, Signer> Flashbots<T, Signer> {
    /// Create a new [`Flashbots`] transport.
    pub fn new(http: Http<T>, signer: Arc<Signer>) -> Self {
        Self { http, signer }
    }
}

pub struct FlashbotsLayer<Signer> {
    signer: Arc<Signer>,
}

impl<Signer> FlashbotsLayer<Signer> {
    pub fn new(signer: Arc<Signer>) -> Self {
        Self { signer }
    }
}

#[async_trait]
pub trait FlashbotsProviderExt {
    async fn send_bundle(&self, bundle: SendBundleRequest) -> TransportResult<SendBundleResponse>;

    async fn sim_bundle(
        &self,
        bundle: SendBundleRequest,
        sim_overrides: SimBundleOverrides,
    ) -> TransportResult<SimBundleResponse>;
}

#[async_trait]
impl<N, C, S> FlashbotsProviderExt for NetworkRpcClient<N, Flashbots<C, S>>
where
    N: Network,
    Flashbots<C, S>: Transport + Clone + Send + Sync,
    C: Send + Sync,
    S: Send + Sync,
{
    async fn send_bundle(&self, bundle: SendBundleRequest) -> TransportResult<SendBundleResponse> {
        self.client.prepare("mev_sendBundle", (bundle,)).await
    }

    async fn sim_bundle(
        &self,
        bundle: SendBundleRequest,
        sim_overrides: SimBundleOverrides,
    ) -> TransportResult<SimBundleResponse> {
        self.client
            .prepare("mev_simBundle", (bundle, sim_overrides))
            .await
    }
}
