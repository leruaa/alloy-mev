use alloy::network::eip2718::Encodable2718;
use alloy::network::Network;
use alloy::providers::fillers::{FillProvider, TxFiller};
use alloy::providers::Provider;
use alloy::transports::{Transport, TransportErrorKind, TransportResult};
use async_trait::async_trait;

use crate::rpc::BundleItem;
use crate::rpc::{SendBundleRequest, SendBundleResponse, SimBundleOverrides, SimBundleResponse};

/// Extension trait for sending and simulate bundles.
#[async_trait]
pub trait FlashbotsProviderExt<N>
where
    N: Network,
{
    async fn build_bundle_item(
        &self,
        tx: N::TransactionRequest,
        can_revert: bool,
    ) -> TransportResult<BundleItem>;

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
}

#[async_trait]
impl<F, P, T, N> FlashbotsProviderExt<N> for FillProvider<F, P, T, N>
where
    F: TxFiller<N>,
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
    <N as Network>::TxEnvelope: Encodable2718 + Clone,
{
    async fn build_bundle_item(
        &self,
        tx: N::TransactionRequest,
        can_revert: bool,
    ) -> TransportResult<BundleItem> {
        let sendable = self.fill(tx).await?;

        if let Some(envelope) = sendable.as_envelope() {
            let bundle_item = BundleItem::Tx {
                tx: envelope.encoded_2718().into(),
                can_revert,
            };

            Ok(bundle_item)
        } else {
            Err(TransportErrorKind::custom_str("No signer has been setup"))
        }
    }

    async fn send_bundle(&self, bundle: SendBundleRequest) -> TransportResult<SendBundleResponse> {
        self.client().request("mev_sendBundle", (bundle,)).await
    }

    async fn sim_bundle(
        &self,
        bundle: SendBundleRequest,
        sim_overrides: SimBundleOverrides,
    ) -> TransportResult<SimBundleResponse> {
        self.client()
            .request("mev_simBundle", (bundle, sim_overrides))
            .await
    }
}
