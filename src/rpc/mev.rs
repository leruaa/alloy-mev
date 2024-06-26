use std::collections::HashSet;

use alloy::{
    primitives::{Address, Bytes, TxHash, B256, U64},
    rpc::types::eth::{BlockId, Log},
};
use serde::{Deserialize, Serialize};

/// A bundle of transactions to send to the matchmaker.
///
/// Note: this is for `mev_sendBundle` and not `eth_sendBundle`.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SendBundleRequest {
    /// The version of the MEV-share API to use.
    #[serde(rename = "version")]
    pub protocol_version: ProtocolVersion,
    /// Data used by block builders to check if the bundle should be considered for inclusion.
    pub inclusion: Inclusion,
    /// The transactions to include in the bundle.
    #[serde(rename = "body")]
    pub bundle_body: Vec<BundleItem>,
    /// Requirements for the bundle to be included in the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<Validity>,
    /// Preferences on what data should be shared about the bundle and its transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Privacy>,
}

/// The version of the MEV-share API to use.
#[derive(Debug, Copy, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub enum ProtocolVersion {
    #[default]
    #[serde(rename = "beta-1")]
    /// The beta-1 version of the API.
    Beta1,
    /// The 0.1 version of the API.
    #[serde(rename = "v0.1")]
    V0_1,
}

/// Data used by block builders to check if the bundle should be considered for inclusion.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Inclusion {
    /// The first block the bundle is valid for.
    pub block: U64,
    /// The last block the bundle is valid for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_block: Option<U64>,
}

impl Inclusion {
    /// Creates a new inclusion with the given min block..
    pub fn at_block(block: u64) -> Self {
        Self {
            block: U64::from(block),
            max_block: None,
        }
    }
}

/// A bundle tx, which can either be a transaction hash, or a full tx.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum BundleItem {
    /// The hash of either a transaction or bundle we are trying to backrun.
    Hash {
        /// Tx hash.
        hash: TxHash,
    },
    /// A new signed transaction.
    #[serde(rename_all = "camelCase")]
    Tx {
        /// Bytes of the signed transaction.
        tx: Bytes,
        /// If true, the transaction can revert without the bundle being considered invalid.
        can_revert: bool,
    },
}

/// Requirements for the bundle to be included in the block.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Validity {
    /// Specifies the minimum percent of a given bundle's earnings to redistribute
    /// for it to be included in a builder's block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<Vec<Refund>>,
    /// Specifies what addresses should receive what percent of the overall refund for this bundle,
    /// if it is enveloped by another bundle (eg. a searcher backrun).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_config: Option<Vec<RefundConfig>>,
}

/// Specifies the minimum percent of a given bundle's earnings to redistribute
/// for it to be included in a builder's block.
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Refund {
    /// The index of the transaction in the bundle.
    pub body_idx: u64,
    /// The minimum percent of the bundle's earnings to redistribute.
    pub percent: u64,
}

/// Specifies what addresses should receive what percent of the overall refund for this bundle,
/// if it is enveloped by another bundle (eg. a searcher backrun).
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RefundConfig {
    /// The address to refund.
    pub address: Address,
    /// The minimum percent of the bundle's earnings to redistribute.
    pub percent: u64,
}

/// Preferences on what data should be shared about the bundle and its transactions
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Privacy {
    /// Hints on what data should be shared about the bundle and its transactions
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub hints: HashSet<PrivacyHint>,
    /// Names of the builders that should be allowed to see the bundle/transaction.
    /// https://github.com/flashbots/dowg/blob/main/builder-registrations.json
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub builders: Vec<String>,
}

/// Hints on what data should be shared about the bundle and its transactions
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PrivacyHint {
    /// The calldata of the bundle's transactions should be shared.
    Calldata,
    /// The address of the bundle's transactions should be shared.
    ContractAddress,
    /// The logs of the bundle's transactions should be shared.
    Logs,
    /// The function selector of the bundle's transactions should be shared.
    FunctionSelector,
    /// The hash of the bundle's transactions should be shared.
    Hash,
    /// The hash of the bundle should be shared.
    TxHash,
}

/// Response from the matchmaker after sending a bundle.
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SendBundleResponse {
    /// Hash of the bundle bodies.
    pub bundle_hash: B256,
}

/// Response from the matchmaker after sending a simulation request.
#[derive(Deserialize, Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SimBundleResponse {
    /// Whether the simulation was successful.
    pub success: bool,
    /// Error message if the simulation failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The block number of the simulated block.
    pub state_block: U64,
    /// The gas price of the simulated block.
    pub mev_gas_price: U64,
    /// The profit of the simulated block.
    pub profit: U64,
    /// The refundable value of the simulated block.
    pub refundable_value: U64,
    /// The gas used by the simulated block.
    pub gas_used: U64,
    /// Logs returned by mev_simBundle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<SimBundleLogs>>,
}

/// Logs returned by mev_simBundle.
#[derive(Deserialize, Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SimBundleLogs {
    /// Logs for transactions in bundle.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tx_logs: Vec<Log>,
    /// Logs for bundles in bundle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_logs: Option<Vec<SimBundleLogs>>,
}

/// Optional fields to override simulation state.
#[derive(Debug, Copy, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SimBundleOverrides {
    /// Block used for simulation state. Defaults to latest block.
    /// Block header data will be derived from parent block by default.
    /// Specify other params to override the default values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_block: Option<BlockId>,
    /// Block number used for simulation, defaults to parentBlock.number + 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<U64>,
    /// Coinbase used for simulation, defaults to parentBlock.coinbase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<Address>,
    /// Timestamp used for simulation, defaults to parentBlock.timestamp + 12
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<U64>,
    /// Gas limit used for simulation, defaults to parentBlock.gasLimit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<U64>,
    /// Base fee used for simulation, defaults to parentBlock.baseFeePerGas
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_fee: Option<U64>,
    /// Timeout in seconds, defaults to 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<U64>,
}
