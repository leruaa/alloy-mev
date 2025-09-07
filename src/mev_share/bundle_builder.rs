use std::marker::PhantomData;

use alloy::{
    eips::Encodable2718,
    network::Network,
    primitives::B256,
    providers::{
        fillers::{FillProvider, TxFiller},
        Provider,
    },
    rpc::types::mev::{BundleItem, MevSendBundle, Privacy, Validity},
    transports::{TransportErrorKind, TransportResult},
};

/// A MEV-Share bundle hat can be sent or simulated.
#[derive(Debug)]
pub struct MevShareBundleBuilder<'a, P, N>
where
    P: Provider<N>,
    N: Network,
{
    provider: &'a P,
    bundle: MevSendBundle,
    phantom: PhantomData<N>,
}

impl<'a, P, N> MevShareBundleBuilder<'a, P, N>
where
    P: Provider<N>,
    N: Network,
{
    /// Creates a new [`MevShareBundleBuilder`].
    pub fn new(provider: &'a P) -> Self {
        Self {
            provider,
            bundle: MevSendBundle::default(),
            phantom: PhantomData,
        }
    }

    /// Includes the given [`BundleItem`] in the bundle.
    ///
    /// **Note**: [`add_transaction`] can be used to construct a bundle item from a
    /// transaction request.
    ///
    /// [`add_transaction`]: MevShareBundleBuilder::add_transaction
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

    /// Builds the [`MevSendBundle`].
    pub fn build(self) -> MevSendBundle {
        self.bundle
    }
}

impl<'a, F, P, N> MevShareBundleBuilder<'a, FillProvider<F, P, N>, N>
where
    F: TxFiller<N>,
    P: Provider<N>,
    N: Network,
{
    /// Builds a bundle item from a transaction request, and includes it in the bundle.
    pub async fn add_transaction(
        self,
        tx: N::TransactionRequest,
        can_revert: bool,
    ) -> TransportResult<Self> {
        let sendable = self.provider.fill(tx).await?;

        if let Some(envelope) = sendable.as_envelope() {
            let bundle_item = BundleItem::Tx {
                tx: envelope.encoded_2718().into(),
                can_revert,
            };

            Ok(self.add_bundle_item(bundle_item))
        } else {
            Err(TransportErrorKind::custom_str("No signer has been setup"))
        }
    }
}
