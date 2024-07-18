use alloy::{
    network::Network,
    rpc::types::mev::{
        BundleItem, SendBundleRequest, SendBundleResponse, SimBundleOverrides, SimBundleResponse,
    },
    signers::Signer,
    transports::TransportResult,
};
use async_trait::async_trait;

/// Extension trait for sending and simulate MEV-Share bundles.
#[async_trait]
pub trait MevShareProviderExt<N>
where
    N: Network,
{
    /// Builds a bundle item from a transaction request.
    async fn build_bundle_item(
        &self,
        tx: N::TransactionRequest,
        can_revert: bool,
    ) -> TransportResult<BundleItem>;

    /// Submits a bundle to the MEV-Share matchmaker. It takes in a bundle and
    /// provides a bundle hash as a return value.
    async fn send_mev_bundle<S>(
        &self,
        bundle: SendBundleRequest,
        signer: S,
    ) -> TransportResult<SendBundleResponse>
    where
        S: Signer + Clone + Send + Sync + 'static;

    /// Similar to `send_bundle` but instead of submitting a bundle to the
    /// matchmaker, it returns a simulation result. Only fully matched bundles
    /// can be simulated.
    async fn sim_mev_bundle<S>(
        &self,
        bundle: SendBundleRequest,
        sim_overrides: SimBundleOverrides,
        signer: S,
    ) -> TransportResult<SimBundleResponse>
    where
        S: Signer + Clone + Send + Sync + 'static;
}
