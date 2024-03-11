use alloy::network::{
    eip2718::Encodable2718, BuilderResult, Network, NetworkSigner, TransactionBuilder,
};
use async_trait::async_trait;

use crate::rpc::BundleItem;

/// Extension trait for sending and simulate bundles.
#[async_trait]
pub trait FlashbotsTransactionBuilderExt<N>
where
    N: Network,
{
    /// Build a [`BundleItem`] using the given [`NetworkSigner`] to sign the `tx`
    async fn build_bundle_item<NS: NetworkSigner<N>>(
        self,
        can_revert: bool,
        signer: &NS,
    ) -> BuilderResult<BundleItem>;
}

#[async_trait]
impl<T, N> FlashbotsTransactionBuilderExt<N> for T
where
    T: TransactionBuilder<N>,
    N: Network,
{
    async fn build_bundle_item<NS: NetworkSigner<N>>(
        self,
        can_revert: bool,
        signer: &NS,
    ) -> BuilderResult<BundleItem> {
        let envelope = self.build(signer).await?;

        let bundle_item = BundleItem::Tx {
            tx: envelope.encoded_2718().into(),
            can_revert,
        };

        Ok(bundle_item)
    }
}
