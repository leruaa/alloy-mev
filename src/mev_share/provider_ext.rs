use alloy::{
    network::Network,
    providers::Provider,
    rpc::types::mev::{
        BundleItem, MevSendBundle, EthBundleHash, SimBundleOverrides, SimBundleResponse,
    },
    signers::Signer,
    transports::{http::Http, Transport, TransportResult},
};
use async_trait::async_trait;

use crate::MevShareBundle;

/// Extension trait for sending and simulate MEV-Share bundles.
#[async_trait]
pub trait MevShareProviderExt<C, N>: Provider<N> + Sized
where
    C: Clone,
    N: Network,
    Http<C>: Transport,
{
    /// Builds a bundle item from a transaction request.
    async fn build_bundle_item(
        &self,
        tx: N::TransactionRequest,
        can_revert: bool,
    ) -> TransportResult<BundleItem>;

    /// Returns a builder-style [`MevShareBundle`] that can be sent or simulated.
    fn build_bundle<S>(&self, bundle_signer: S) -> MevShareBundle<'_, Self, C, N, S>
    where
        S: Signer + Send + Sync + 'static;

    /// Submits a bundle to the MEV-Share matchmaker. It takes in a bundle and
    /// provides a bundle hash as a return value.
    async fn send_mev_bundle<S>(
        &self,
        bundle: MevSendBundle,
        signer: S,
    ) -> TransportResult<EthBundleHash>
    where
        S: Signer + Clone + Send + Sync + 'static;

    /// Similar to `send_bundle` but instead of submitting a bundle to the
    /// matchmaker, it returns a simulation result. Only fully matched bundles
    /// can be simulated.
    async fn sim_mev_bundle<S>(
        &self,
        bundle: MevSendBundle,
        sim_overrides: SimBundleOverrides,
        signer: S,
    ) -> TransportResult<SimBundleResponse>
    where
        S: Signer + Clone + Send + Sync + 'static;
}
