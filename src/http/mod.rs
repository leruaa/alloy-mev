use alloy::transports::http::Http;
use url::Url;

#[cfg(feature = "reqwest")]
mod reqwest;

/// An Alloy `Transport` that handle `mev_sendBundle` and `mev_simBundle`
/// requests and delegates all others to the inner `Transport`.
#[derive(Debug, Clone)]
pub struct MevHttp<T, S> {
    mev_share_url: Url,
    http: Http<T>,
    signer: Option<S>,
}

impl<T, S> MevHttp<T, S> {
    /// Create a new [`MevHttp`] transport.
    pub const fn new(mev_share_url: Url, http: Http<T>, signer: Option<S>) -> Self {
        Self {
            mev_share_url,
            http,
            signer,
        }
    }
}
