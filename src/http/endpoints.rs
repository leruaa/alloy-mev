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

    /// Adds the given transport.
    pub fn add(&mut self, transport: BoxTransport) {
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
    /// Adds a new transport to the [`Endpoints`] beiing built.
    pub fn add(mut self, url: Url, bundle_signer: Option<BundleSigner>) -> Self {
        self.endpoints
            .add(MevHttp::new(url, self.base_transport.clone(), bundle_signer).boxed());

        self
    }

    /// Returns the [`Endpoints`] struct.
    pub fn build(self) -> Endpoints {
        self.endpoints
    }
}
