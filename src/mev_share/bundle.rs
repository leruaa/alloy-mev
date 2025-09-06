use std::marker::PhantomData;

use alloy::{
    network::Network,
    primitives::B256,
    providers::Provider,
    rpc::{
        client::RpcCall,
        types::mev::{
            BundleItem, Privacy, MevSendBundle, EthBundleHash, SimBundleOverrides,
            SimBundleResponse, Validity,
        },
    },
    signers::Signer,
    transports::{http::Http, Transport, TransportResult},
};

use crate::transport::MevHttpBox;

/// A MEV-Share bundle hat can be sent or simulated.
#[derive(Debug)]
pub struct MevShareBundle<'a, P, C, N, S>
where
    P: Provider<N>,
    C: Clone,
    N: Network,
    S: Signer + Send + Sync + 'static,
    Http<C>: Transport,
{
    provider: &'a P,
    bundle: MevSendBundle,
    bundle_signer: S,
    phantom: PhantomData<(C, N)>,
}

impl<'a, P, C, N, S> MevShareBundle<'a, P, C, N, S>
where
    P: Provider<N>,
    C: Clone,
    N: Network,
    S: Signer + Send + Sync + 'static,
    Http<C>: Transport,
    MevHttpBox: Transport,
{
    /// Creates a new [`MevShareBundle`].
    pub fn new(provider: &'a P, bundle_signer: S) -> Self {
        Self {
            provider,
            bundle: MevSendBundle::default(),
            bundle_signer,
            phantom: PhantomData,
        }
    }

    /// Includes the given [`BundleItem`] in the bundle.
    ///
    /// **Note**: [`build_bundle_item`] on the extension trait can be used to construct a
    /// bundle item from a transaction request.
    ///
    /// [`build_bundle_item`]: crate::MevShareProviderExt::build_bundle_item
    pub fn add_bundle_item(mut self, bundle_item: BundleItem) -> Self {
        self.bundle.bundle_body.push(bundle_item);

        self
    }

    /// Includes a tx hash in the bundle.
    pub fn add_tx_hash(mut self, hash: B256) -> Self {
        self.bundle.bundle_body.push(BundleItem::Hash { hash });

        self
    }

    /// Adds the data used by block builders to check if the bundle should be considered for inclusion.
    pub const fn with_inclusion(mut self, block: u64, max_block: Option<u64>) -> Self {
        self.bundle.inclusion.block = block;
        self.bundle.inclusion.max_block = max_block;

        self
    }

    /// Adds te requirements for the bundle to be included in the block.
    pub fn with_validity(mut self, validity: Validity) -> Self {
        self.bundle.validity = Some(validity);

        self
    }

    /// Adds the preferences on what data should be shared about the bundle and its transactions.
    pub fn with_privacy(mut self, privacy: Privacy) -> Self {
        self.bundle.privacy = Some(privacy);

        self
    }

    /// Submits a bundle to the MEV-Share matchmaker. It takes in a bundle and
    /// provides a bundle hash as a return value.
    pub async fn send(self) -> TransportResult<EthBundleHash> {
        let request = self
            .provider
            .client()
            .make_request("mev_sendBundle", (self.bundle,));

        RpcCall::new(
            request,
            MevHttpBox::flashbots(
                self.provider.client().transport().clone(),
                self.bundle_signer,
            ),
        )
        .await
    }

    /// Similar to `send_bundle` but instead of submitting a bundle to the
    /// matchmaker, it returns a simulation result. Only fully matched bundles
    /// can be simulated.
    pub async fn sim(
        self,
        sim_overrides: SimBundleOverrides,
    ) -> TransportResult<SimBundleResponse> {
        let request = self
            .provider
            .client()
            .make_request("mev_simBundle", (self.bundle, sim_overrides));

        RpcCall::new(
            request,
            MevHttpBox::flashbots(
                self.provider.client().transport().clone(),
                self.bundle_signer,
            ),
        )
        .await
    }
}
