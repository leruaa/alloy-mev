use alloy::{
    network::Network,
    providers::{fillers::TxFiller, ProviderBuilder, ProviderLayer, RootProvider},
    rpc::client::ClientBuilder,
    signers::Signer,
    transports::Transport,
};
use url::Url;

use crate::{layer::MevLayer, MevHttp};

/// A wrapper around [`ProviderBuilder`] allowing to configure a `Provider`to
/// send bundles.
#[derive(Debug)]
pub struct MevCapableProviderBuilder<L, F, N, S> {
    provider_builder: ProviderBuilder<L, F, N>,
    layer: MevLayer<S>,
}

impl<L, F, N, S> MevCapableProviderBuilder<L, F, N, S> {
    /// Creates a new [`MevCapableProviderBuilder`].
    pub const fn new(provider_builder: ProviderBuilder<L, F, N>) -> Self {
        Self {
            provider_builder,
            layer: MevLayer::<S>::new(),
        }
    }

    /// Sets the signer used for bundle signatures.
    pub fn bundle_signer(mut self, signer: S) -> Self {
        self.layer.bundle_signer = Some(signer);
        self
    }

    /// Sets the url to send MEV-Share bundles.
    /// Defaults to https://relay.flashbots.net
    pub fn mev_share_url(mut self, url: Url) -> Self {
        self.layer.mev_share_url = Some(url);
        self
    }
}

#[cfg(feature = "reqwest")]
impl<L, F, N, S> MevCapableProviderBuilder<L, F, N, S>
where
    L: ProviderLayer<RootProvider<MevHttp<reqwest::Client, S>, N>, MevHttp<reqwest::Client, S>, N>,
    F: TxFiller<N> + ProviderLayer<L::Provider, MevHttp<reqwest::Client, S>, N>,
    MevHttp<reqwest::Client, S>: Transport + Clone,
    S: Signer + Clone + Send + Sync,
    N: Network,
{
    /// Build this provider with an Reqwest HTTP transport.
    pub fn on_http(self, url: Url) -> F::Provider {
        let client = ClientBuilder::default().layer(self.layer).http(url);
        self.provider_builder.on_client(client)
    }
}
