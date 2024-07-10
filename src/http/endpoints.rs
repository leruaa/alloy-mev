use std::slice::Iter;

use alloy::transports::http::Http;
use url::Url;

use crate::MevHttp;

use super::BundleSigner;

/// Stores a list of transports that can be used to broadcast a request to.
#[derive(Debug)]
pub struct Endpoints<C>(Vec<MevHttp<C>>);

impl<C> Endpoints<C>
where
    C: Clone,
{
    /// Returns the associated builder.
    pub const fn builder(http: Http<C>) -> EndpointsBuilder<C> {
        EndpointsBuilder::new(http)
    }

    /// Adds the given transport.
    pub fn add(&mut self, mev_http: MevHttp<C>) {
        self.0.push(mev_http)
    }

    /// Returns an iterator over the transports.
    pub fn iter(&self) -> Iter<MevHttp<C>> {
        self.0.iter()
    }
}

/// An [`Endpoints`] builder.
#[derive(Debug)]
pub struct EndpointsBuilder<C> {
    base_transport: Http<C>,
    endpoints: Endpoints<C>,
}

impl<C> EndpointsBuilder<C>
where
    C: Clone,
{
    /// Creates a new builder.
    pub const fn new(http: Http<C>) -> Self {
        Self {
            base_transport: http,
            endpoints: Endpoints(vec![]),
        }
    }

    /// Adds a new transport to the [`Endpoints`] beiing built.
    pub fn add(mut self, url: Url, bundle_signer: Option<BundleSigner>) -> Self {
        self.endpoints.add(MevHttp::new(
            url,
            self.base_transport.clone(),
            bundle_signer,
        ));

        self
    }

    /// Returns the [`Endpoints`] struct.
    pub fn build(self) -> Endpoints<C> {
        self.endpoints
    }
}
