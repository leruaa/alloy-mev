use std::slice::Iter;

use alloy::transports::{http::Http, BoxTransport, Transport};
use url::Url;

use crate::{BundleSigner, MevHttp};

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
    /// Adds a new transport to the [`Endpoints`] being built, with the given
    /// signer used to build the header signature.
    pub fn endpoint_with_bundle_signer(mut self, url: Url, bundle_signer: BundleSigner) -> Self {
        self.endpoints
            .add(MevHttp::new(url, self.base_transport.clone(), bundle_signer).boxed());

        self
    }

    /// Adds a new transport to the [`Endpoints`] being built.
    pub fn endpoint(mut self, url: Url) -> Self {
        self.endpoints
            .add(Http::with_client(reqwest::Client::new(), url).boxed());

        self
    }

    /// Adds a new transport to the [`Endpoints`] being built, using the given signer for header authentication.
    pub fn authenticated_endpoint(mut self, url: Url, bundle_signer: BundleSigner) -> Self {
        self.endpoints
            .add(MevHttp::new(url, self.base_transport.clone(), bundle_signer).boxed());

        self
    }

    /// Adds Beaverbuild.
    pub fn beaverbuild(self) -> Self {
        self.endpoint("https://rpc.beaverbuild.org".parse().unwrap())
    }

    /// Adds Titan using AWS geo-routing to find the best RPC to send to.
    /// Instead, the following methods that uses geo-located RPC can be used:
    /// * [`titan_europe`]
    /// * [`titan_united_states`]
    /// * [`titan_asia`]
    ///
    /// [`titan_europe`]: EndpointsBuilder::titan_europe
    /// [`titan_united_states`]: EndpointsBuilder::titan_united_states
    /// [`titan_asia`]: EndpointsBuilder::titan_asia
    pub fn titan(self, bundle_signer: BundleSigner) -> Self {
        self.authenticated_endpoint(
            "https://rpc.titanbuilder.xyz".parse().unwrap(),
            bundle_signer,
        )
    }

    /// Adds Titan, using the Europe RPC.
    pub fn titan_europe(self, bundle_signer: BundleSigner) -> Self {
        self.authenticated_endpoint(
            "https://eu.rpc.titanbuilder.xyz".parse().unwrap(),
            bundle_signer,
        )
    }

    /// Adds Titan, using the United States RPC.
    pub fn titan_united_states(self, bundle_signer: BundleSigner) -> Self {
        self.authenticated_endpoint(
            "https://us.rpc.titanbuilder.xyz".parse().unwrap(),
            bundle_signer,
        )
    }

    /// Adds Titan, using the Asia RPC.
    pub fn titan_asia(self, bundle_signer: BundleSigner) -> Self {
        self.authenticated_endpoint(
            "https://as.rpc.titanbuilder.xyz".parse().unwrap(),
            bundle_signer,
        )
    }

    /// Adds Rsync.
    pub fn rsync(self) -> Self {
        self.endpoint("https://rsync-builder.xyz".parse().unwrap())
    }

    /// Adds Flashbots.
    pub fn flashbots(self, bundle_signer: BundleSigner) -> Self {
        self.authenticated_endpoint(
            "https://relay.flashbots.net".parse().unwrap(),
            bundle_signer,
        )
    }

    /// Returns the [`Endpoints`] struct.
    pub fn build(self) -> Endpoints {
        self.endpoints
    }
}
