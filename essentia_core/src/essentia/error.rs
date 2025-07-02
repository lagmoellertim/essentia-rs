use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateAlgorithmError {
    #[error("algorithm not found: {name}")]
    AlgorithmNotFound { name: String },

    #[error("failed to create algorithm: {0}")]
    Internal(#[from] cxx::Exception),
}
