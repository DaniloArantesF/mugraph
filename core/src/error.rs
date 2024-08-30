use onlyerror::Error;
use serde::{Deserialize, Serialize};
use test_strategy::Arbitrary;

use crate::types::{Hash, Signature};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error, Clone, Serialize, Deserialize, Arbitrary, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[error("Server error: {reason}")]
    ServerError { reason: String },

    #[error("Storage error: {reason}")]
    StorageError { reason: String },

    #[error("Insufficient funds for {asset_id}, expected {expected} but got {got}")]
    InsufficientFunds {
        asset_id: Hash,
        expected: u64,
        got: u64,
    },

    #[error("Atom has already been spent: {signature}")]
    AlreadySpent { signature: Signature },

    #[error("Invalid signature {signature}: {reason}")]
    InvalidSignature {
        reason: String,
        signature: Signature,
    },

    #[error("Invalid public or secret key: {reason}")]
    InvalidKey { reason: String },

    #[error("Invalid hash: {reason}")]
    InvalidHash { reason: String },

    #[error("Atom is invalid: {reason}")]
    InvalidAtom { reason: String },

    #[error("Other error")]
    Other,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::StorageError {
            reason: e.to_string(),
        }
    }
}

impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    }
}
