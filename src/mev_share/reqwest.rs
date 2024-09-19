use alloy::{
    eips::eip2718::Encodable2718,
    network::{Ethereum, Network},
    providers::{
        fillers::{FillProvider, TxFiller},
        Provider,
    },
    rpc::{
        client::RpcCall,
        types::mev::{
            BundleItem, SendBundleRequest, SendBundleResponse, SimBundleOverrides,
            SimBundleResponse,
        },
    },
    signers::Signer,
    transports::{http::Http, TransportErrorKind, TransportResult},
};
use async_trait::async_trait;

use crate::{MevHttp, MevShareBundle, MevShareProviderExt};

/// A [`MevShareBundle`] on Ethereun network using Reqwest HTTP transport.
pub type EthereumReqwestMevShareBundle<'a, P, S> =
    MevShareBundle<'a, P, Http<reqwest::Client>, Ethereum, S>;

#[async_trait]
impl<F, P, N> MevShareProviderExt<reqwest::Client, N>
    for FillProvider<F, P, Http<reqwest::Client>, N>
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

    fn build_bundle<S>(&self, bundle_signer: S) -> MevShareBundle<'_, Self, reqwest::Client, N, S>
    where
        S: Signer + Send + Sync + 'static,
    {
        MevShareBundle::new(self, bundle_signer)
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

    async fn sim_mev_bundle<S>(
        &self,
        bundle: SendBundleRequest,
        sim_overrides: SimBundleOverrides,
        signer: S,
    ) -> TransportResult<SimBundleResponse>
    where
        S: Signer + Clone + Send + Sync + 'static,
    {
        let request = self
            .client()
            .make_request("mev_simBundle", (bundle, sim_overrides));

        RpcCall::new(
            request,
            MevHttp::flashbots(self.client().transport().clone(), signer),
        )
        .await
    }
}
