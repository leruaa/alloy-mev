use std::{fmt::Debug, sync::Arc};

use alloy::{primitives::Address, signers::Signer};

/// A [`Signer`] wrapper to sign bundles.
#[derive(Clone)]
pub struct BundleSigner {
    /// The header name on which set the signature.
    pub header: String,
    /// The signer used to sign the bundle.
    pub signer: Arc<dyn Signer + Send + Sync>,
}

impl BundleSigner {
    /// Creates a new [`BundleSigner`]
    pub fn new<S>(header: String, signer: S) -> Self
    where
        S: Signer + Send + Sync + 'static,
    {
        Self {
            header,
            signer: Arc::new(signer),
        }
    }

    /// Creates a [`BundleSigner`] set up to add the Flashbots header.
    pub fn flashbots<S>(signer: S) -> Self
    where
        S: Signer + Send + Sync + 'static,
    {
        Self {
            header: "X-Flashbots-Signature".to_string(),
            signer: Arc::new(signer),
        }
    }

    /// Returns the signer address.
    pub fn address(&self) -> Address {
        self.signer.address()
    }
}

impl Debug for BundleSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BundleSigner")
            .field("header", &self.header)
            .field("signer_address", &self.signer.address())
            .finish()
    }
}
