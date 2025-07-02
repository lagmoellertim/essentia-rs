// ==============================================================================
// NEW UNIFIED DATA SYSTEM - COMPILE-TIME TYPE SAFETY
// ==============================================================================
pub mod data;

// ==============================================================================
// CORE MODULES (updated to use new data system)
// ==============================================================================
pub mod algorithm;
pub mod essentia;
pub mod parameter_map;
pub mod pool;

// ==============================================================================
// RE-EXPORTS - CLEAN API
// ==============================================================================

// Core data types with compile-time constraints
pub use data::{ConversionError, GetFromDataContainer, IntoDataContainer, TryIntoDataContainer};
pub use data::{DataContainer, DataType, phantom};
pub use data::{InputOutputData, ParameterData, PoolData};

// Algorithm and execution
pub use algorithm::{Algorithm, Configured, Initialized, Introspection};
pub use essentia::{CreateAlgorithmError, Essentia};
pub use pool::{Pool, PoolError};

// Error types
pub use algorithm::{
    ComputeError, ConfigurationError, InputError, OutputError, ParameterError, ResetError,
};

// Centralized error handling (to be enabled)
// pub mod error;
// pub use error::EssentiaError;
