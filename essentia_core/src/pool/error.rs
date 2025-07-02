use thiserror::Error;

use crate::pool_data::PoolDataType;

#[derive(Debug, Error)]
pub enum PoolError {
    #[error("Key '{key}' not found in pool")]
    KeyNotFound { key: String },

    #[error("Type mismatch for key '{key}': expected {expected}, found {actual}")]
    TypeMismatch {
        key: String,
        expected: PoolDataType,
        actual: PoolDataType,
    },

    #[error("Internal error: {0}")]
    Internal(#[from] cxx::Exception),
}
