use alloy::{
    network::Network,
    providers::{ext::MevBuilder, Provider},
    rpc::{
        client::RpcClient,
        types::mev::{EthBundleHash, MevSendBundle, SimBundleOverrides, SimBundleResponse},
    },
    signers::Signer,
    transports::TransportResult,
};
use async_trait::async_trait;

use crate::mev_share::{MevShareBundleBuilder, FLASHBOTS_RELAY_RPC_URL};

/// Extension trait for sending and simulate MEV-Share bundles.
#[async_trait]
pub trait MevShareProviderExt<N>: Provider<N> + Sized
where
    N: Network,
{
    /// Returns a builder-style [`MevShareBundleBuilder`] that can be sent or simulated.
    fn bundle_builder(&self) -> MevShareBundleBuilder<'_, Self, N>;

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

#[async_trait]
impl<P, N> MevShareProviderExt<N> for P
where
    N: Network,
    P: Provider<N>,
{
    fn bundle_builder(&self) -> MevShareBundleBuilder<'_, Self, N> {
        MevShareBundleBuilder::new(self)
    }

    async fn send_mev_bundle<S>(
        &self,
        bundle: MevSendBundle,
        signer: S,
    ) -> TransportResult<EthBundleHash>
    where
        S: Signer + Clone + Send + Sync + 'static,
    {
        let client = RpcClient::new_http(FLASHBOTS_RELAY_RPC_URL.parse().unwrap());
        let request = client.request("mev_sendBundle", (bundle,));

        MevBuilder::new_rpc(request).with_auth(signer).await
    }

    async fn sim_mev_bundle<S>(
        &self,
        bundle: MevSendBundle,
        sim_overrides: SimBundleOverrides,
        signer: S,
    ) -> TransportResult<SimBundleResponse>
    where
        S: Signer + Clone + Send + Sync + 'static,
    {
        let client = RpcClient::new_http(FLASHBOTS_RELAY_RPC_URL.parse().unwrap());
        let request = client.request("mev_simBundle", (bundle, sim_overrides));

        MevBuilder::new_rpc(request).with_auth(signer).await
    }
}
