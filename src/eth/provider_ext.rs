use alloy::{
    network::Network,
    primitives::{Bytes, B256},
    providers::Provider,
    rpc::types::mev::{
        EthBundleHash, EthCallBundle, EthCallBundleResponse, EthCancelBundle, EthSendBundle,
        EthSendPrivateTransaction,
    },
    transports::{http::Http, Transport, TransportResult},
};
use async_trait::async_trait;

use crate::EthBundle;

use super::{Endpoints, EndpointsBuilderBox};

/// Extension trait for sending and simulate eth bundles.
#[async_trait]
pub trait EthMevProviderExt<C, N>: Provider<N> + Sized
where
    C: Clone,
    N: Network,
    Http<C>: Transport,
{
    /// Returns a [`EndpointsBuilder`] that can be used to build a new
    /// [`Endpoints`].
    fn endpoints_builder(&self) -> EndpointsBuilderBox;

    /// Sign and encode a transaction request.
    async fn encode_request(&self, tx: N::TransactionRequest) -> TransportResult<Bytes>;

    /// Returns a builder-style [`MevShareBundle`] that can be sent or simulated.
    fn build_bundle(&self) -> EthBundle<'_, Self, Http<C>, N>;

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
