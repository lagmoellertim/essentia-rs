use thiserror::Error;

use crate::data::ConversionError;

#[derive(Debug, Error)]
pub enum ParameterError {
    #[error("Failed to convert data for parameter '{parameter}': {source}")]
    DataConversion {
        parameter: String,
        #[source]
        source: ConversionError,
    },
}

impl From<essentia_core::algorithm::ParameterError> for ParameterError {
    fn from(err: essentia_core::algorithm::ParameterError) -> Self {
        match err {
            essentia_core::algorithm::ParameterError::DataConversion { parameter, source } => {
                ParameterError::DataConversion { parameter, source }
            }
            _ => panic!("Unexpected error type"),
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigurationError {
    #[error("Configuration failed: {0}")]
    Internal(#[from] cxx::Exception),
}

impl From<essentia_core::algorithm::ConfigurationError> for ConfigurationError {
    fn from(err: essentia_core::algorithm::ConfigurationError) -> Self {
        match err {
            essentia_core::algorithm::ConfigurationError::Internal(e) => {
                ConfigurationError::Internal(e)
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum OutputError {
    #[error("Internal error for output '{output}': {source}")]
    Internal {
        output: String,
        #[source]
        source: cxx::Exception,
    },
}

impl From<essentia_core::algorithm::OutputError> for OutputError {
    fn from(err: essentia_core::algorithm::OutputError) -> Self {
        match err {
            essentia_core::algorithm::OutputError::Internal { output, source } => {
                OutputError::Internal { output, source }
            }
            _ => panic!("Unexpected error type"),
        }
    }
}

#[derive(Debug, Error)]
pub enum ComputeError {
    #[error("Failed to setup output '{output}': {source}")]
    OutputSetup {
        output: String,
        #[source]
        source: cxx::Exception,
    },

    #[error("Failed to convert data for input '{input}': {source}")]
    DataConversion {
        input: String,
        #[source]
        source: ConversionError,
    },

    #[error("Computation failed: {0}")]
    Compute(#[from] cxx::Exception),

    #[error("Internal error for input '{input}': {source}")]
    InputInternal {
        input: String,
        #[source]
        source: cxx::Exception,
    },
}

impl From<essentia_core::algorithm::ComputeError> for ComputeError {
    fn from(err: essentia_core::algorithm::ComputeError) -> Self {
        match err {
            essentia_core::algorithm::ComputeError::OutputSetup { output, source } => {
                ComputeError::OutputSetup { output, source }
            }
            essentia_core::algorithm::ComputeError::Compute(exception) => {
                ComputeError::Compute(exception)
            }
        }
    }
}

impl From<essentia_core::algorithm::InputError> for ComputeError {
    fn from(err: essentia_core::algorithm::InputError) -> Self {
        match err {
            essentia_core::algorithm::InputError::DataConversion { input, source } => {
                ComputeError::DataConversion { input, source }
            }
            essentia_core::algorithm::InputError::Internal { input, source } => {
                ComputeError::InputInternal { input, source }
            }
            _ => panic!("Unexpected error type"),
        }
    }
}

#[derive(Debug, Error)]
pub enum ResetError {
    #[error("Reset failed: {0}")]
    Internal(#[from] cxx::Exception),
}

impl From<essentia_core::algorithm::ResetError> for ResetError {
    fn from(err: essentia_core::algorithm::ResetError) -> Self {
        match err {
            essentia_core::algorithm::ResetError::Internal(exception) => {
                ResetError::Internal(exception)
            }
        }
    }
}
