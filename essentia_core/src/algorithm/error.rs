use thiserror::Error;

use crate::data::DataType;

#[derive(Debug, Error)]
pub enum ParameterError {
    #[error("Parameter '{parameter}' not found")]
    ParameterNotFound { parameter: String },

    #[error("Type mismatch for parameter '{parameter}': expected {expected}, found {actual}")]
    TypeMismatch {
        parameter: String,
        expected: DataType,
        actual: DataType,
    },
}

#[derive(Debug, Error)]
pub enum ConfigurationError {
    #[error("Configuration failed: {0}")]
    Internal(#[from] cxx::Exception),
}

#[derive(Debug, Error)]
pub enum InputError {
    #[error("Input '{input}' not found")]
    InputNotFound { input: String },

    #[error("Type mismatch for input '{input}': expected {expected}, found {actual}")]
    TypeMismatch {
        input: String,
        expected: DataType,
        actual: DataType,
    },
}

#[derive(Debug, Error)]
pub enum OutputError {
    #[error("Output '{output}' not found")]
    OutputNotFound { output: String },

    #[error("Type mismatch for output '{output}': expected {expected}, found {actual}")]
    TypeMismatch {
        output: String,
        expected: DataType,
        actual: DataType,
    },
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
