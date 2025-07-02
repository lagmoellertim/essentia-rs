use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Type mismatch during conversion: {message}")]
    TypeMismatch { message: String },

    #[error("Invalid data format: {message}")]
    InvalidFormat { message: String },

    #[error("Conversion not supported: {message}")]
    NotSupported { message: String },
}
