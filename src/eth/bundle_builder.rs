use std::marker::PhantomData;

use alloy::{
    eips::Encodable2718,
    network::Network,
    primitives::{Bytes, B256},
    providers::{
        fillers::{FillProvider, TxFiller},
        Provider,
    },
    rpc::types::mev::EthSendBundle,
    transports::{TransportErrorKind, TransportResult},
};

/// A bundle that can be sent to one or more builder(s).
#[derive(Debug)]
pub struct EthBundleBuilder<'a, P, N>
where
    P: Provider<N>,
    N: Network,
{
    provider: &'a P,
    bundle: EthSendBundle,
    phantom: PhantomData<N>,
}

impl<'a, P, N> EthBundleBuilder<'a, P, N>
where
    P: Provider<N>,
    N: Network,
{
    /// Creates a new [`EthBundleBuilder`].
    pub fn new(provider: &'a P) -> Self {
        Self {
            provider,
            bundle: EthSendBundle::default(),
            phantom: PhantomData,
        }
    }

    /// Adds a hex-encoded signed transaction.
    pub fn add_signed_transaction(mut self, tx: Bytes) -> Self {
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

    /// Builds a [`EthSendBundle`].
    pub fn build(self) -> EthSendBundle {
        self.bundle
    }
}

impl<'a, F, P, N> EthBundleBuilder<'a, FillProvider<F, P, N>, N>
where
    F: TxFiller<N>,
    P: Provider<N>,
    N: Network,
{
    /// Sign and encode a transaction request, and then add it to the bundle.
    pub async fn add_transaction_request(self, tx: N::TransactionRequest) -> TransportResult<Self> {
        let sendable = self.provider.fill(tx).await?;

        if let Some(envelope) = sendable.as_envelope() {
            let encoded = envelope.encoded_2718().into();
            Ok(self.add_signed_transaction(encoded))
        } else {
            Err(TransportErrorKind::custom_str("No signer has been setup"))
        }
    }
}
