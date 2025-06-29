use crate::variant_data::DataType;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParameterType {
    Float,
    String,
    Bool,
    Int,
    StereoSample,
    VectorFloat,
    VectorString,
    VectorBool,
    VectorInt,
    VectorStereoSample,
    VectorVectorFloat,
    VectorVectorString,
    VectorVectorStereoSample,
    VectorMatrixFloat,
    MapVectorFloat,
    MapVectorString,
    MapVectorInt,
    MapFloat,
    MatrixFloat,
}

impl TryFrom<DataType> for ParameterType {
    type Error = String;

    fn try_from(data_type: DataType) -> Result<Self, Self::Error> {
        match data_type {
            DataType::Float => Ok(ParameterType::Float),
            DataType::String => Ok(ParameterType::String),
            DataType::Bool => Ok(ParameterType::Bool),
            DataType::Int => Ok(ParameterType::Int),
            DataType::StereoSample => Ok(ParameterType::StereoSample),
            DataType::VectorFloat => Ok(ParameterType::VectorFloat),
            DataType::VectorString => Ok(ParameterType::VectorString),
            DataType::VectorBool => Ok(ParameterType::VectorBool),
            DataType::VectorInt => Ok(ParameterType::VectorInt),
            DataType::VectorStereoSample => Ok(ParameterType::VectorStereoSample),
            DataType::VectorVectorFloat => Ok(ParameterType::VectorVectorFloat),
            DataType::VectorVectorString => Ok(ParameterType::VectorVectorString),
            DataType::VectorVectorStereoSample => Ok(ParameterType::VectorVectorStereoSample),
            DataType::VectorMatrixFloat => Ok(ParameterType::VectorMatrixFloat),
            DataType::MapVectorFloat => Ok(ParameterType::MapVectorFloat),
            DataType::MapVectorString => Ok(ParameterType::MapVectorString),
            DataType::MapVectorInt => Ok(ParameterType::MapVectorInt),
            DataType::MapFloat => Ok(ParameterType::MapFloat),
            DataType::MatrixFloat => Ok(ParameterType::MatrixFloat),
            // These types are not valid for parameters
            DataType::UnsignedInt => Err(format!("UnsignedInt is not a valid parameter type")),
            DataType::Long => Err(format!("Long is not a valid parameter type")),
        }
    }
}

impl From<ParameterType> for DataType {
    fn from(param_type: ParameterType) -> Self {
        match param_type {
            ParameterType::Float => DataType::Float,
            ParameterType::String => DataType::String,
            ParameterType::Bool => DataType::Bool,
            ParameterType::Int => DataType::Int,
            ParameterType::StereoSample => DataType::StereoSample,
            ParameterType::VectorFloat => DataType::VectorFloat,
            ParameterType::VectorString => DataType::VectorString,
            ParameterType::VectorBool => DataType::VectorBool,
            ParameterType::VectorInt => DataType::VectorInt,
            ParameterType::VectorStereoSample => DataType::VectorStereoSample,
            ParameterType::VectorVectorFloat => DataType::VectorVectorFloat,
            ParameterType::VectorVectorString => DataType::VectorVectorString,
            ParameterType::VectorVectorStereoSample => DataType::VectorVectorStereoSample,
            ParameterType::VectorMatrixFloat => DataType::VectorMatrixFloat,
            ParameterType::MapVectorFloat => DataType::MapVectorFloat,
            ParameterType::MapVectorString => DataType::MapVectorString,
            ParameterType::MapVectorInt => DataType::MapVectorInt,
            ParameterType::MapFloat => DataType::MapFloat,
            ParameterType::MatrixFloat => DataType::MatrixFloat,
        }
    }
}

impl fmt::Display for ParameterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
