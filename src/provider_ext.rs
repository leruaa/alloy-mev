use alloy_network::{eip2718::Encodable2718, Network, NetworkSigner, TransactionBuilder};
use alloy_providers::Provider;
use alloy_transport::{Transport, TransportErrorKind, TransportResult};
use async_trait::async_trait;

use crate::{
    http::FlashbotsHttp,
    rpc::{
        BundleItem, SendBundleRequest, SendBundleResponse, SimBundleOverrides, SimBundleResponse,
    },
};

/// Extension trait for sending and simulate bundles.
#[async_trait]
pub trait FlashbotsProviderExt<N, C, S>
where
    N: Network,
    FlashbotsHttp<C, S>: Transport + Clone + Send + Sync,
    C: Send + Sync,
    S: Send + Sync,
{
    /// Submit a bundle to the relay. It takes in a bundle and provides
    /// a bundle hash as a return value.
    async fn send_bundle(&self, bundle: SendBundleRequest) -> TransportResult<SendBundleResponse>;

    /// Similar to `send_bundle` but instead of submitting a bundle to the
    /// relay, it returns a simulation result. Only fully matched bundles
    /// can be simulated.
    async fn sim_bundle(
        &self,
        bundle: SendBundleRequest,
        sim_overrides: SimBundleOverrides,
    ) -> TransportResult<SimBundleResponse>;

    /// Build a [`BundleItem`] from a [`TransactionRequest`], using the given
    /// [`NetworkSigner`] to sign the `tx`
    async fn build_bundle_item<NS: NetworkSigner<N>>(
        &self,
        tx: <N as Network>::TransactionRequest,
        can_revert: bool,
        signer: &NS,
    ) -> TransportResult<BundleItem> {
        let envelope = tx.build(signer).await.map_err(TransportErrorKind::custom)?;

        let bundle_item = BundleItem::Tx {
            tx: envelope.encoded_2718().into(),
            can_revert,
        };

        Ok(bundle_item)
    }
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
