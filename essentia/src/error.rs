use thiserror::Error;

pub use essentia_core::error::{
    ComputeError as CoreComputeError, ConfigurationError, CreateAlgorithmError,
    EssentiaError as CoreError, InputError, OutputError, ParameterError, ResetError,
};

#[derive(Debug, Error)]
pub enum ConfigureError {
    #[error("configuration error: {0}")]
    Configuration(#[from] ConfigurationError),
}

#[derive(Debug, Error)]
pub enum ComputeError {
    #[error("computation error: {0}")]
    Computation(#[from] CoreComputeError),
}

#[derive(Debug, Error)]
pub enum GetOutputError {
    #[error("output error: {0}")]
    Output(#[from] OutputError),
}

#[derive(Debug, Error)]
pub enum AlgorithmError {
    #[error("configuration error: {0}")]
    Configure(#[from] ConfigureError),

    #[error("computation error: {0}")]
    Compute(#[from] ComputeError),

    #[error("output error: {0}")]
    GetOutput(#[from] GetOutputError),
}

#[derive(Debug, Error)]
pub enum EssentiaError {
    #[error("core error: {0}")]
    Core(#[from] CoreError),

    #[error("algorithm error: {0}")]
    Algorithm(#[from] AlgorithmError),
}
