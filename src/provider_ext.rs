use alloy_network::Network;
use alloy_providers::Provider;
use alloy_transport::{Transport, TransportResult};
use async_trait::async_trait;
use mev_share::rpc::{
    SendBundleRequest, SendBundleResponse, SimBundleOverrides, SimBundleResponse,
};

use crate::http::FlashbotsHttp;

/// Extension trait for sending and simulate bundles.
#[async_trait]
pub trait FlashbotsProviderExt<N, C, S>
where
    N: Network,
    FlashbotsHttp<C, S>: Transport + Clone + Send + Sync,
    C: Send + Sync,
    S: Send + Sync,
{
    async fn send_bundle(&self, bundle: SendBundleRequest) -> TransportResult<SendBundleResponse>;

    async fn sim_bundle(
        &self,
        bundle: SendBundleRequest,
        sim_overrides: SimBundleOverrides,
    ) -> TransportResult<SimBundleResponse>;
}

#[async_trait]
impl<T, N, C, S> FlashbotsProviderExt<N, C, S> for T
where
    T: Provider<N, FlashbotsHttp<C, S>>,
    N: Network,
    FlashbotsHttp<C, S>: Transport + Clone + Send + Sync,
    C: Send + Sync,
    S: Send + Sync,
{
    async fn send_bundle(&self, bundle: SendBundleRequest) -> TransportResult<SendBundleResponse> {
        self.client().prepare("mev_sendBundle", (bundle,)).await
    }

    async fn sim_bundle(
        &self,
        bundle: SendBundleRequest,
        sim_overrides: SimBundleOverrides,
    ) -> TransportResult<SimBundleResponse> {
        self.client()
            .prepare("mev_simBundle", (bundle, sim_overrides))
            .await
    }
}
