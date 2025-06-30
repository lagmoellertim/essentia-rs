use thiserror::Error;

pub use crate::algorithm::error::*;
pub use crate::essentia::CreateAlgorithmError;

#[derive(Debug, Error)]
pub enum EssentiaError {
    #[error("algorithm creation failed: {0}")]
    CreateAlgorithm(#[from] CreateAlgorithmError),

    #[error("parameter error: {0}")]
    Parameter(#[from] ParameterError),

    #[error("configuration error: {0}")]
    Configuration(#[from] ConfigurationError),

    #[error("input error: {0}")]
    Input(#[from] InputError),

    #[error("computation error: {0}")]
    Computation(#[from] ComputeError),

    #[error("output error: {0}")]
    Output(#[from] OutputError),

    #[error("reset error: {0}")]
    Reset(#[from] ResetError),
}
