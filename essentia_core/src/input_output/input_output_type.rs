use crate::variant_data::DataType;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputOutputType {
    Float,
    UnsignedInt,
    Long,
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

impl TryFrom<DataType> for InputOutputType {
    type Error = String;

    fn try_from(data_type: DataType) -> Result<Self, Self::Error> {
        match data_type {
            DataType::Float => Ok(InputOutputType::Float),
            DataType::UnsignedInt => Ok(InputOutputType::UnsignedInt),
            DataType::Long => Ok(InputOutputType::Long),
            DataType::String => Ok(InputOutputType::String),
            DataType::Bool => Ok(InputOutputType::Bool),
            DataType::Int => Ok(InputOutputType::Int),
            DataType::StereoSample => Ok(InputOutputType::StereoSample),
            DataType::VectorFloat => Ok(InputOutputType::VectorFloat),
            DataType::VectorString => Ok(InputOutputType::VectorString),
            DataType::VectorBool => Ok(InputOutputType::VectorBool),
            DataType::VectorInt => Ok(InputOutputType::VectorInt),
            DataType::VectorStereoSample => Ok(InputOutputType::VectorStereoSample),
            DataType::VectorVectorFloat => Ok(InputOutputType::VectorVectorFloat),
            DataType::VectorVectorString => Ok(InputOutputType::VectorVectorString),
            DataType::VectorVectorStereoSample => Ok(InputOutputType::VectorVectorStereoSample),
            DataType::VectorMatrixFloat => Ok(InputOutputType::VectorMatrixFloat),
            DataType::MapVectorFloat => Ok(InputOutputType::MapVectorFloat),
            DataType::MapVectorString => Ok(InputOutputType::MapVectorString),
            DataType::MapVectorInt => Ok(InputOutputType::MapVectorInt),
            DataType::MapFloat => Ok(InputOutputType::MapFloat),
            DataType::MatrixFloat => Ok(InputOutputType::MatrixFloat),
        }
    }
}

impl From<InputOutputType> for DataType {
    fn from(param_type: InputOutputType) -> Self {
        match param_type {
            InputOutputType::Float => DataType::Float,
            InputOutputType::UnsignedInt => DataType::UnsignedInt,
            InputOutputType::Long => DataType::Long,
            InputOutputType::String => DataType::String,
            InputOutputType::Bool => DataType::Bool,
            InputOutputType::Int => DataType::Int,
            InputOutputType::StereoSample => DataType::StereoSample,
            InputOutputType::VectorFloat => DataType::VectorFloat,
            InputOutputType::VectorString => DataType::VectorString,
            InputOutputType::VectorBool => DataType::VectorBool,
            InputOutputType::VectorInt => DataType::VectorInt,
            InputOutputType::VectorStereoSample => DataType::VectorStereoSample,
            InputOutputType::VectorVectorFloat => DataType::VectorVectorFloat,
            InputOutputType::VectorVectorString => DataType::VectorVectorString,
            InputOutputType::VectorVectorStereoSample => DataType::VectorVectorStereoSample,
            InputOutputType::VectorMatrixFloat => DataType::VectorMatrixFloat,
            InputOutputType::MapVectorFloat => DataType::MapVectorFloat,
            InputOutputType::MapVectorString => DataType::MapVectorString,
            InputOutputType::MapVectorInt => DataType::MapVectorInt,
            InputOutputType::MapFloat => DataType::MapFloat,
            InputOutputType::MatrixFloat => DataType::MatrixFloat,
        }
    }
}

impl fmt::Display for InputOutputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
