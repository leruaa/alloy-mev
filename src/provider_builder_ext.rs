use alloy::{
    network::Network,
    providers::{fillers::TxFiller, ProviderBuilder, ProviderLayer, RootProvider},
    rpc::client::RpcClient,
    signers::Signer,
    transports::Transport,
};
use url::Url;

use crate::{FlashbotsHttp, FlashbotsLayer};

pub trait FlashbotsProviderBuilderExt<L, F, N> {
    #[cfg(feature = "reqwest")]
    fn on_http_with_flashbots<S>(self, url: Url, signer: S) -> F::Provider
    where
        L: ProviderLayer<
            RootProvider<FlashbotsHttp<reqwest::Client, S>, N>,
            FlashbotsHttp<reqwest::Client, S>,
            N,
        >,
        F: TxFiller<N> + ProviderLayer<L::Provider, FlashbotsHttp<reqwest::Client, S>, N>,
        FlashbotsHttp<reqwest::Client, S>: Transport + Clone,
        S: Signer + Clone + Send + Sync,
        N: Network;
}

impl<L, F, N> FlashbotsProviderBuilderExt<L, F, N> for ProviderBuilder<L, F, N> {
    #[cfg(feature = "reqwest")]
    fn on_http_with_flashbots<S>(self, url: Url, signer: S) -> F::Provider
    where
        L: ProviderLayer<
            RootProvider<FlashbotsHttp<reqwest::Client, S>, N>,
            FlashbotsHttp<reqwest::Client, S>,
            N,
        >,
        F: TxFiller<N> + ProviderLayer<L::Provider, FlashbotsHttp<reqwest::Client, S>, N>,
        FlashbotsHttp<reqwest::Client, S>: Transport + Clone,
        S: Signer + Clone + Send + Sync,
        N: Network,
    {
        let client = RpcClient::builder()
            .layer(FlashbotsLayer::new(signer))
            .http(url);

        self.on_client(client)
    }
}
