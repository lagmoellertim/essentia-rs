use thiserror::Error;

pub use crate::algorithm::error::*;
use crate::essentia::CreateAlgorithmError;
pub use crate::variant_data::ConversionError;

#[derive(Debug, Error)]
pub enum EssentiaError {
    #[error("algorithm creation failed: {0}")]
    AlgorithmCreation(#[from] CreateAlgorithmError),

    #[error("configuration failed: {0}")]
    Configuration(#[from] ConfigurationError),

    #[error("input error: {0}")]
    Input(#[from] InputError),

    #[error("computation failed: {0}")]
    Computation(#[from] ComputationError),

    #[error("reset failed: {0}")]
    Reset(#[from] ResetError),

    #[error("output error: {0}")]
    Output(#[from] OutputError),

    #[error("parameter error: {0}")]
    Parameter(#[from] ParameterError),
}
