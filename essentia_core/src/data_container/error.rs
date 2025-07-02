use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("matrix data is not rectangular: row {row} has {actual} elements, expected {expected}")]
    NonRectangular {
        row: usize,
        expected: usize,
        actual: usize,
    },

    #[error("matrix cannot be empty")]
    EmptyMatrix,

    #[error("matrix rows cannot be empty")]
    EmptyRows,
}
