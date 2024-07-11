use std::slice::Iter;

use alloy::transports::{http::Http, BoxTransport, Transport};
use url::Url;

use crate::MevHttp;

use super::BundleSigner;

/// Stores a list of transports that can be used to broadcast a request to.
#[derive(Debug)]
pub struct Endpoints(Vec<BoxTransport>);

impl Endpoints {
    /// Returns the associated builder.
    pub const fn builder<C>(http: Http<C>) -> EndpointsBuilder<C>
    where
        C: Clone,
    {
        EndpointsBuilder::new(http)
    }

    /// Pushes the given transport.
    pub fn push(&mut self, transport: BoxTransport) {
        self.0.push(transport)
    }

    /// Returns an iterator over the transports.
    pub fn iter(&self) -> Iter<BoxTransport> {
        self.0.iter()
    }
}

/// An [`Endpoints`] builder.
#[derive(Debug)]
pub struct EndpointsBuilder<C> {
    base_transport: Http<C>,
    endpoints: Endpoints,
}

impl<C> EndpointsBuilder<C> {
    /// Creates a new builder.
    pub const fn new(base_transport: Http<C>) -> Self {
        Self {
            base_transport,
            endpoints: Endpoints(vec![]),
        }
    }
}

impl<C> EndpointsBuilder<C>
where
    C: Clone,
    MevHttp<C>: Transport,
{
    /// Pushes a new transport to the [`Endpoints`] being built, with the given
    /// signer used to build the header signature.
    pub fn push_with_bundle_signer(mut self, url: Url, bundle_signer: BundleSigner) -> Self {
        self.endpoints
            .push(MevHttp::new(url, self.base_transport.clone(), bundle_signer).boxed());

        self
    }

    /// Pushes a new transport to the [`Endpoints`] being built.
    pub fn push(mut self, url: Url) -> Self {
        self.endpoints
            .push(Http::with_client(reqwest::Client::new(), url).boxed());

        self
    }

    /// Returns the [`Endpoints`] struct.
    pub fn build(self) -> Endpoints {
        self.endpoints
    }
}
