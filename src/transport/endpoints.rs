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

    /// Adds Beaverbuild.
    pub fn beaverbuild(mut self) -> Self {
        self.endpoints.add(
            Http::with_client(
                reqwest::Client::new(),
                "https://rpc.beaverbuild.org".parse().unwrap(),
            )
            .boxed(),
        );

        self
    }

    /// Adds Titan.
    pub fn titan(mut self, bundle_signer: BundleSigner) -> Self {
        self.endpoints.add(
            MevHttp::new(
                "https://rpc.titanbuilder.xyz".parse().unwrap(),
                self.base_transport.clone(),
                bundle_signer,
            )
            .boxed(),
        );

        self
    }

    /// Adds Rsync.
    pub fn rsync(mut self) -> Self {
        self.endpoints.add(
            Http::with_client(
                reqwest::Client::new(),
                "https://rsync-builder.xyz/".parse().unwrap(),
            )
            .boxed(),
        );

        self
    }

    /// Adds Flashbots.
    pub fn flashbots(mut self, bundle_signer: BundleSigner) -> Self {
        self.endpoints.add(
            MevHttp::new(
                "https://relay.flashbots.net".parse().unwrap(),
                self.base_transport.clone(),
                bundle_signer,
            )
            .boxed(),
        );

        self
    }

    /// Returns the [`Endpoints`] struct.
    pub fn build(self) -> Endpoints {
        self.endpoints
    }
}
