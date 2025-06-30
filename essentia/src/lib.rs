// Generated algorithm specific error types
pub mod error;

// Include the generated algorithms module
pub mod algorithms {
    include!(concat!(env!("OUT_DIR"), "/algorithms/mod.rs"));
}

// Re-export the algorithms module for easier access
pub use algorithms::*;

// Re-export core types for convenience
pub use essentia_core::{
    algorithm::{Configured, Initialized},
    essentia::Essentia,
    variant_data::{VariantData, into_other::GetVariantData, variant},
};

// Re-export error types - essentia_core handles its own errors
pub use error::{
    // Domain-specific errors
    AlgorithmError,
    ComputeError,
    // Function-specific errors for generated algorithms
    ConfigureError,
    // Unified error
    EssentiaError,
    GetOutputError,
};

/// Trait for creating algorithms from Essentia
pub trait CreateAlgorithm<'a> {
    type Output;
    fn create(essentia: &'a essentia_core::essentia::Essentia) -> Self::Output;
}

/// Top-level convenience function
pub fn create<'a, T>(essentia: &'a essentia_core::essentia::Essentia) -> T::Output
where
    T: CreateAlgorithm<'a>,
{
    T::create(essentia)
}
