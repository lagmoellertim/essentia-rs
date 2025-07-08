use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigurationError {
    #[error("Configuration failed: {0}")]
    Internal(#[from] cxx::Exception),
}

#[derive(Debug, Error)]
pub enum ComputeError {
    #[error("Computation failed: {0}")]
    Compute(#[from] cxx::Exception),
}

#[derive(Debug, Error)]
pub enum ResetError {
    #[error("Reset failed: {0}")]
    Internal(#[from] cxx::Exception),
}
