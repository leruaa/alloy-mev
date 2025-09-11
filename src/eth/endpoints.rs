use std::{fmt::Debug, slice::Iter};

use alloy::signers::Signer;
use dyn_clone::DynClone;
use url::Url;

/// Stores a list of transports that can be used to broadcast a request to.
#[derive(Default, Debug)]
pub struct Endpoints(Vec<Endpoint>);

impl Endpoints {
    /// Returns the associated builder.
    pub fn builder() -> EndpointsBuilder {
        EndpointsBuilder::default()
    }

    /// Adds the given transport.
    pub fn add(&mut self, endpoint: Endpoint) {
        self.0.push(endpoint)
    }

    /// Returns an iterator over the transports.
    pub fn iter(&self) -> Iter<'_, Endpoint> {
        self.0.iter()
    }
}

pub trait ClonableSigner: Signer + DynClone + Send + Sync + Debug + 'static {}

impl<T> ClonableSigner for T where T: Signer + Clone + Send + Sync + Debug + 'static {}

dyn_clone::clone_trait_object!(ClonableSigner);

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub url: Url,
    pub signer: Option<Box<dyn ClonableSigner>>,
}

/// An [`Endpoints`] builder.
#[derive(Default, Debug)]
pub struct EndpointsBuilder {
    endpoints: Endpoints,
}

impl EndpointsBuilder {
    /// Adds a new transport to the [`Endpoints`] being built.
    pub fn endpoint(mut self, url: Url) -> Self {
        self.endpoints.add(Endpoint { url, signer: None });

        self
    }

    /// Adds a new transport to the [`Endpoints`] being built, using the given signer for header authentication.
    pub fn authenticated_endpoint<S: ClonableSigner>(mut self, url: Url, signer: S) -> Self {
        self.endpoints.add(Endpoint {
            url,
            signer: Some(Box::new(signer)),
        });

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
    pub fn titan<S: ClonableSigner>(self, bundle_signer: S) -> Self {
        self.authenticated_endpoint(
            "https://rpc.titanbuilder.xyz".parse().unwrap(),
            bundle_signer,
        )
    }

    /// Adds Titan, using the Europe RPC.
    pub fn titan_europe<S: ClonableSigner>(self, bundle_signer: S) -> Self {
        self.authenticated_endpoint(
            "https://eu.rpc.titanbuilder.xyz".parse().unwrap(),
            bundle_signer,
        )
    }

    /// Adds Titan, using the United States RPC.
    pub fn titan_united_states<S: ClonableSigner>(self, bundle_signer: S) -> Self {
        self.authenticated_endpoint(
            "https://us.rpc.titanbuilder.xyz".parse().unwrap(),
            bundle_signer,
        )
    }

    /// Adds Titan, using the Asia RPC.
    pub fn titan_asia<S: ClonableSigner>(self, bundle_signer: S) -> Self {
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
    pub fn flashbots<S: ClonableSigner>(self, bundle_signer: S) -> Self {
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
