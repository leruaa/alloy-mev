use alloy::{
    network::{eip2718::Encodable2718, Network},
    providers::{
        fillers::{FillProvider, TxFiller},
        Provider,
    },
    rpc::client::RpcCall,
    signers::Signer,
    transports::{http::Http, TransportErrorKind, TransportResult},
};
use alloy_rpc_types::mev::{
    BundleItem, SendBundleRequest, SendBundleResponse, SimBundleOverrides, SimBundleResponse,
};
use async_trait::async_trait;

use crate::MevHttp;

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
    async fn sim_mev_bundle(
        &self,
        bundle: SendBundleRequest,
        sim_overrides: SimBundleOverrides,
    ) -> TransportResult<SimBundleResponse>;
}

#[async_trait]
impl<F, P, N> MevShareProviderExt<N> for FillProvider<F, P, Http<reqwest::Client>, N>
where
    F: TxFiller<N>,
    P: Provider<Http<reqwest::Client>, N>,
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

    async fn send_mev_bundle<S>(
        &self,
        bundle: SendBundleRequest,
        signer: S,
    ) -> TransportResult<SendBundleResponse>
    where
        S: Signer + Clone + Send + Sync + 'static,
    {
        let request = self.client().make_request("mev_sendBundle", (bundle,));

        RpcCall::new(
            request,
            MevHttp::flashbots(self.client().transport().clone(), signer),
        )
        .await
    }

    async fn sim_mev_bundle(
        &self,
        bundle: SendBundleRequest,
        sim_overrides: SimBundleOverrides,
    ) -> TransportResult<SimBundleResponse> {
        let request = self
            .client()
            .make_request("mev_simBundle", (bundle, sim_overrides));

        RpcCall::new(
            request,
            MevHttp::new(
                "https://relay.flashbots.net".parse().unwrap(),
                self.client().transport().clone(),
                None,
            ),
        )
        .await
    }
}
