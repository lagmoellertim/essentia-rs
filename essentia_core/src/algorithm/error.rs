use cxx::Exception;
use thiserror::Error;

use crate::{
    data_container::ConversionError, input_output::InputOutputType, parameter::ParameterType,
};

#[derive(Debug, Error)]
pub enum ParameterError {
    #[error("no parameter named '{parameter}' found")]
    ParameterNotFound { parameter: String },

    #[error("type mismatch for parameter '{parameter}': expected {expected:?}, got {actual:?}")]
    TypeMismatch {
        parameter: String,
        expected: ParameterType,
        actual: ParameterType,
    },

    #[error("failed to convert data for parameter '{parameter}': {source}")]
    DataConversion {
        parameter: String,
        #[source]
        source: ConversionError,
    },
}

#[derive(Debug, Error)]
pub enum ConfigurationError {
    #[error("algorithm configuration failed: {0}")]
    Internal(#[from] Exception),

    #[error("parameter validation failed: {0}")]
    Parameter(#[from] ParameterError),
}

#[derive(Debug, Error)]
pub enum InputError {
    #[error("no input named '{input}' found")]
    InputNotFound { input: String },

    #[error("type mismatch for input '{input}': expected {expected:?}, got {actual:?}")]
    TypeMismatch {
        input: String,
        expected: InputOutputType,
        actual: InputOutputType,
    },

    #[error("failed to convert data for input '{input}': {source}")]
    DataConversion {
        input: String,
        #[source]
        source: ConversionError,
    },

    #[error("failed to set input '{input}': internal error - {source}")]
    Internal { input: String, source: Exception },
}

#[derive(Debug, Error)]
pub enum ComputeError {
    #[error("failed to setup output '{output}': {source}")]
    OutputSetup { output: String, source: Exception },

    #[error("algorithm computation failed: {0}")]
    Compute(Exception),
}

#[derive(Debug, Error)]
pub enum ResetError {
    #[error("algorithm reset failed: {0}")]
    Internal(#[from] Exception),
}
#[derive(Debug, Error)]
pub enum OutputError {
    #[error("no output named '{output}' found")]
    OutputNotFound { output: String },

    #[error("type mismatch for output '{output}': expected {expected:?}, got {actual:?}")]
    TypeMismatch {
        output: String,
        expected: InputOutputType,
        actual: InputOutputType,
    },

    #[error("failed to retrieve output '{output}': {source}")]
    Internal { output: String, source: Exception },
}
