use alloy::providers::ProviderBuilder;

use crate::mev_capable_provider_builder::MevCapableProviderBuilder;

/// Extension trait for building a MEV capable provider.
pub trait MevCapableProviderBuilderExt<L, F, N, S> {
    /// Add the ability to send bundles to the stack being built.
    fn with_bundle_managment(self) -> MevCapableProviderBuilder<L, F, N, S>;
}

impl<L, F, N, S> MevCapableProviderBuilderExt<L, F, N, S> for ProviderBuilder<L, F, N> {
    fn with_bundle_managment(self) -> MevCapableProviderBuilder<L, F, N, S> {
        MevCapableProviderBuilder::new(self)
    }
}
