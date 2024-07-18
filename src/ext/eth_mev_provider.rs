use alloy::{
    network::Network,
    primitives::{Bytes, B256},
    rpc::types::mev::{
        CancelBundleRequest, EthCallBundle, EthCallBundleResponse, EthSendBundle,
        PrivateTransactionRequest, SendBundleResponse,
    },
    transports::TransportResult,
};
use async_trait::async_trait;

use crate::transport::{Endpoints, EndpointsBuilder};

/// Extension trait for sending and simulate eth bundles.
#[async_trait]
pub trait EthMevProviderExt<C, N>
where
    N: Network,
{
    /// Returns a [`EndpointsBuilder`] that can be used to build a new
    /// [`Endpoints`].
    fn endpoints_builder(&self) -> EndpointsBuilder<C>;

    /// Sign and encode a transaction request.
    async fn encode_request(&self, tx: N::TransactionRequest) -> TransportResult<Bytes>;

    /// Submits a bundle to one or more builder(s). It takes in a bundle and
    /// provides a bundle hash as a return value.
    async fn send_eth_bundle(
        &self,
        bundle: EthSendBundle,
        endpoints: &Endpoints,
    ) -> Vec<TransportResult<SendBundleResponse>>;

    /// Submits a single transaction to one or more builder(s). It takes in a
    /// bundle and provides a bundle hash as a return value.
    async fn send_eth_private_transaction(
        &self,
        request: PrivateTransactionRequest,
    ) -> TransportResult<B256>;

    /// simulates a bundle against a specific block number.
    async fn call_eth_bundle(
        &self,
        bundle: EthCallBundle,
        endpoints: &Endpoints,
    ) -> Vec<TransportResult<EthCallBundleResponse>>;

    /// Cancels a previously submitted bundle.
    async fn cancel_eth_bundle(&self, request: CancelBundleRequest) -> TransportResult<()>;
}
