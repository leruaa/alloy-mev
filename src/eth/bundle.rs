use std::marker::PhantomData;

use alloy::{
    network::Network,
    primitives::{Bytes, B256},
    providers::Provider,
    rpc::types::mev::{EthSendBundle, EthBundleHash},
    transports::{Transport, TransportResult},
};

use crate::{BroadcastableCall, Endpoints};

/// A bundle that can be sent to one or more builder(s).
#[derive(Debug)]
pub struct EthBundle<'a, P, T, N>
where
    P: Provider<N>,
    T: Transport + Clone,
    N: Network,
{
    provider: &'a P,
    bundle: EthSendBundle,
    phantom: PhantomData<(T, N)>,
}

impl<'a, P, T, N> EthBundle<'a, P, T, N>
where
    P: Provider<N>,
    T: Transport + Clone,
    N: Network,
{
    /// Creates a new [`EthBundle`]
    pub fn new(provider: &'a P) -> Self {
        Self {
            provider,
            bundle: EthSendBundle::default(),
            phantom: PhantomData,
        }
    }

    /// Adds a hex-encoded signed transaction.
    pub fn add_tx(mut self, tx: Bytes) -> Self {
        self.bundle.txs.push(tx);

        self
    }

    /// Adds a hashes of a possibly reverting tx.
    pub fn add_reverting_tx(mut self, hash: B256) -> Self {
        self.bundle.reverting_tx_hashes.push(hash);

        self
    }

    /// Sets the block number for which this bundle is valid.
    pub const fn on_block(mut self, block: u64) -> Self {
        self.bundle.block_number = block;

        self
    }

    /// Sets the unix timestamp when this bundle becomes active.
    pub const fn with_min_timestamp(mut self, min_timestamp: u64) -> Self {
        self.bundle.min_timestamp = Some(min_timestamp);

        self
    }

    /// Sets the unix timestamp how long this bundle stays valid.
    pub const fn with_max_timestamp(mut self, max_timestamp: u64) -> Self {
        self.bundle.max_timestamp = Some(max_timestamp);

        self
    }

    /// Sets the UUID that can be used to cancel/replace this bundle.
    pub fn with_replacement_uuid(mut self, replacement_uuid: String) -> Self {
        self.bundle.replacement_uuid = Some(replacement_uuid);

        self
    }

    /// Submits a bundle to the given endpoints. It takes in a bundle and
    /// provides a bundle hash as a return value.
    pub async fn send(self, endpoints: &Endpoints) -> Vec<TransportResult<EthBundleHash>> {
        BroadcastableCall::new(
            endpoints,
            self.provider
                .client()
                .make_request("eth_sendBundle", (self.bundle,)),
        )
        .await
    }
}
