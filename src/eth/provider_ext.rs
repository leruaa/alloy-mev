use alloy::{
    network::Network,
    primitives::B256,
    providers::Provider,
    rpc::types::mev::{
        EthBundleHash, EthCallBundle, EthCallBundleResponse, EthCancelBundle, EthSendBundle,
        EthSendPrivateTransaction,
    },
    transports::TransportResult,
};
use async_trait::async_trait;

use crate::{eth::EthBundleBuilder, BroadcastableCall};

use super::{Endpoints, EndpointsBuilder};

/// Extension trait for sending and simulate eth bundles.
#[async_trait]
pub trait EthMevProviderExt<N>: Provider<N> + Sized
where
    N: Network,
{
    /// Returns a [`EndpointsBuilder`] that can be used to build a new
    /// [`Endpoints`].
    fn endpoints_builder(&self) -> EndpointsBuilder;

    /// Returns a builder-style [`EthBundleBuilder`] that can be sent or simulated.
    fn bundle_builder(&self) -> EthBundleBuilder<'_, Self, N>;

    /// Submits a bundle to one or more builder(s). It takes in a bundle and
    /// provides a bundle hash as a return value.
    async fn send_eth_bundle(
        &self,
        bundle: EthSendBundle,
        endpoints: &Endpoints,
    ) -> Vec<TransportResult<EthBundleHash>>;

    /// Submits a single transaction to one or more builder(s). It takes in a
    /// bundle and provides a bundle hash as a return value.
    async fn send_eth_private_transaction(
        &self,
        request: EthSendPrivateTransaction,
    ) -> TransportResult<B256>;

    /// simulates a bundle against a specific block number.
    async fn call_eth_bundle(
        &self,
        bundle: EthCallBundle,
        endpoints: &Endpoints,
    ) -> Vec<TransportResult<EthCallBundleResponse>>;

    /// Cancels a previously submitted bundle.
    async fn cancel_eth_bundle(&self, request: EthCancelBundle) -> TransportResult<()>;
}

#[async_trait]
impl<P, N> EthMevProviderExt<N> for P
where
    P: Provider<N>,
    N: Network,
{
    fn endpoints_builder(&self) -> EndpointsBuilder {
        EndpointsBuilder::default()
    }

    fn bundle_builder(&self) -> EthBundleBuilder<'_, Self, N> {
        EthBundleBuilder::new(self)
    }

    async fn send_eth_bundle(
        &self,
        bundle: EthSendBundle,
        endpoints: &Endpoints,
    ) -> Vec<TransportResult<EthBundleHash>> {
        BroadcastableCall::new(
            endpoints,
            self.client().make_request("eth_sendBundle", (bundle,)),
        )
        .await
    }

    async fn send_eth_private_transaction(
        &self,
        request: EthSendPrivateTransaction,
    ) -> TransportResult<B256> {
        self.client()
            .request("eth_sendPrivateTransaction", (request,))
            .await
    }

    async fn call_eth_bundle(
        &self,
        bundle: EthCallBundle,
        endpoints: &Endpoints,
    ) -> Vec<TransportResult<EthCallBundleResponse>> {
        BroadcastableCall::new(
            endpoints,
            self.client().make_request("eth_callBundle", (bundle,)),
        )
        .await
    }

    async fn cancel_eth_bundle(&self, request: EthCancelBundle) -> TransportResult<()> {
        self.client().request("eth_cancelBundle", (request,)).await
    }
}
