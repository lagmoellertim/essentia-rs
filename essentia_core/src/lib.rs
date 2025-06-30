pub mod algorithm;
pub mod error;
pub mod essentia;
pub mod ffi;
pub mod input_output;
pub mod parameter;
pub mod pool;
pub mod variant_data;

// Re-export commonly used types
pub use error::EssentiaError;
pub use essentia::Essentia;

#[cfg(test)]
mod pool_tests {
    include!("pool/tests.rs");
}
