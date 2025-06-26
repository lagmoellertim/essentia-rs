use ndarray::{Array2, ShapeError};
use thiserror::Error;

use crate::{algorithm::InputOutputType, ffi};

pub enum OutputValue<'a> {
    Real(f32),
    Int(i32),
    UnsignedInt(u32),
    Long(i64),
    VectorReal(&'a [f32]),
    VectorVectorReal(Vec<ffi::FloatSlice<'a>>),
    MatrixReal(ffi::FloatMatrix2d<'a>),
}

#[derive(Debug, Error)]
pub enum OutputConvertError {
    #[error("expected `{expected}`, found `{found}`")]
    UnexpectedType {
        expected: InputOutputType,
        found: InputOutputType,
    },

    #[error(transparent)]
    Shape(#[from] ShapeError),
}

impl<'a> OutputValue<'a> {
    pub fn get_from_output(
        algorithm: &'a ffi::AlgorithmBridge,
        output_name: &str,
        output_type: InputOutputType,
    ) -> OutputValue<'a> {
        match output_type {
            InputOutputType::Real => OutputValue::Real(algorithm.get_output_real(output_name)),
            InputOutputType::Int => OutputValue::Int(algorithm.get_output_int(output_name)),
            InputOutputType::UnsignedInt => {
                OutputValue::UnsignedInt(algorithm.get_output_uint(output_name))
            }
            InputOutputType::Long => OutputValue::Long(algorithm.get_output_long(output_name)),
            InputOutputType::VectorReal => {
                OutputValue::VectorReal(algorithm.get_output_real_vector(output_name))
            }
            InputOutputType::VectorVectorReal => {
                OutputValue::VectorVectorReal(algorithm.get_output_real_vector_vector(output_name))
            }
            InputOutputType::MatrixReal => {
                OutputValue::MatrixReal(algorithm.get_output_real_matrix(output_name))
            }
        }
    }
}

impl<'a> OutputValue<'a> {
    pub fn data_type(&self) -> InputOutputType {
        match self {
            OutputValue::Real(_) => InputOutputType::Real,
            OutputValue::Int(_) => InputOutputType::Int,
            OutputValue::UnsignedInt(_) => InputOutputType::UnsignedInt,
            OutputValue::Long(_) => InputOutputType::Long,
            OutputValue::VectorReal(_) => InputOutputType::VectorReal,
            OutputValue::VectorVectorReal(_) => InputOutputType::VectorVectorReal,
            OutputValue::MatrixReal(_) => InputOutputType::MatrixReal,
        }
    }
}

impl<'a> TryFrom<OutputValue<'a>> for f32 {
    type Error = OutputConvertError;

    fn try_from(value: OutputValue<'a>) -> Result<Self, Self::Error> {
        match value {
            OutputValue::Real(value) => Ok(value),
            other => Err(OutputConvertError::UnexpectedType {
                expected: InputOutputType::Real,
                found: other.data_type(),
            }),
        }
    }
}

impl<'a> TryFrom<OutputValue<'a>> for i32 {
    type Error = OutputConvertError;

    fn try_from(value: OutputValue<'a>) -> Result<Self, Self::Error> {
        match value {
            OutputValue::Int(value) => Ok(value),
            other => Err(OutputConvertError::UnexpectedType {
                expected: InputOutputType::Int,
                found: other.data_type(),
            }),
        }
    }
}

impl<'a> TryFrom<OutputValue<'a>> for u32 {
    type Error = OutputConvertError;

    fn try_from(value: OutputValue<'a>) -> Result<Self, Self::Error> {
        match value {
            OutputValue::UnsignedInt(value) => Ok(value),
            other => Err(OutputConvertError::UnexpectedType {
                expected: InputOutputType::UnsignedInt,
                found: other.data_type(),
            }),
        }
    }
}

impl<'a> TryFrom<OutputValue<'a>> for i64 {
    type Error = OutputConvertError;

    fn try_from(value: OutputValue<'a>) -> Result<Self, Self::Error> {
        match value {
            OutputValue::Long(value) => Ok(value),
            other => Err(OutputConvertError::UnexpectedType {
                expected: InputOutputType::Long,
                found: other.data_type(),
            }),
        }
    }
}

impl<'a> TryFrom<OutputValue<'a>> for Vec<f32> {
    type Error = OutputConvertError;

    fn try_from(value: OutputValue<'a>) -> Result<Self, Self::Error> {
        match value {
            OutputValue::VectorReal(value) => Ok(value.to_vec()),
            other => Err(OutputConvertError::UnexpectedType {
                expected: InputOutputType::VectorReal,
                found: other.data_type(),
            }),
        }
    }
}

impl<'a> TryFrom<OutputValue<'a>> for Vec<Vec<f32>> {
    type Error = OutputConvertError;

    fn try_from(value: OutputValue<'a>) -> Result<Self, Self::Error> {
        match value {
            OutputValue::VectorVectorReal(value) => Ok(value
                .into_iter()
                .map(|float_slice| float_slice.slice.to_vec())
                .collect()),
            other => Err(OutputConvertError::UnexpectedType {
                expected: InputOutputType::VectorVectorReal,
                found: other.data_type(),
            }),
        }
    }
}

impl<'a> TryFrom<OutputValue<'a>> for Array2<f32> {
    type Error = OutputConvertError;

    fn try_from(value: OutputValue<'a>) -> Result<Self, Self::Error> {
        match value {
            OutputValue::MatrixReal(value) => {
                let result =
                    Array2::from_shape_vec((value.dim1, value.dim2), value.slice.to_vec())?;
                Ok(result)
            }
            other => Err(OutputConvertError::UnexpectedType {
                expected: InputOutputType::VectorVectorReal,
                found: other.data_type(),
            }),
        }
    }
}
