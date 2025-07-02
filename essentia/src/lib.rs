pub mod algorithm;
pub mod essentia;
pub use essentia_core::{data, parameter_map, pool};

pub use essentia_core::data::{
    ConversionError, DataContainer, DataType, InputOutputData, IntoDataContainer, ParameterData,
    PoolData, TryGetFromDataContainer, TryIntoDataContainer, phantom,
};

pub use algorithm::{Configured, Initialized};
pub use essentia::Essentia;

pub use pool::{Pool, PoolError};

// Error types
//pub use algorithm::{
//    ComputeError, ConfigurationError, InputError, OutputError, ParameterError, ResetError,
//};

// Centralized error handling (to be enabled)
// pub mod error;
// pub use error::EssentiaError;
