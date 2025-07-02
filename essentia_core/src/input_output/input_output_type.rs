use crate::data_container::DataType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputOutputType {
    Float,
    UnsignedInt,
    Long,
    String,
    Bool,
    Int,
    StereoSample,
    Complex,
    TensorFloat,
    VectorFloat,
    VectorString,
    VectorBool,
    VectorInt,
    VectorStereoSample,
    VectorComplex,
    VectorVectorFloat,
    VectorVectorString,
    VectorVectorStereoSample,
    VectorVectorComplex,
    VectorMatrixFloat,
    MapVectorFloat,
    MapVectorString,
    MapVectorInt,
    MapVectorComplex,
    MapFloat,
    MatrixFloat,
    Pool,
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
            DataType::Complex => Ok(InputOutputType::Complex),
            DataType::TensorFloat => Ok(InputOutputType::TensorFloat),
            DataType::VectorFloat => Ok(InputOutputType::VectorFloat),
            DataType::VectorString => Ok(InputOutputType::VectorString),
            DataType::VectorBool => Ok(InputOutputType::VectorBool),
            DataType::VectorInt => Ok(InputOutputType::VectorInt),
            DataType::VectorStereoSample => Ok(InputOutputType::VectorStereoSample),
            DataType::VectorComplex => Ok(InputOutputType::VectorComplex),
            DataType::VectorVectorFloat => Ok(InputOutputType::VectorVectorFloat),
            DataType::VectorVectorString => Ok(InputOutputType::VectorVectorString),
            DataType::VectorVectorStereoSample => Ok(InputOutputType::VectorVectorStereoSample),
            DataType::VectorVectorComplex => Ok(InputOutputType::VectorVectorComplex),
            DataType::VectorMatrixFloat => Ok(InputOutputType::VectorMatrixFloat),
            DataType::MapVectorFloat => Ok(InputOutputType::MapVectorFloat),
            DataType::MapVectorString => Ok(InputOutputType::MapVectorString),
            DataType::MapVectorInt => Ok(InputOutputType::MapVectorInt),
            DataType::MapVectorComplex => Ok(InputOutputType::MapVectorComplex),
            DataType::MapFloat => Ok(InputOutputType::MapFloat),
            DataType::MatrixFloat => Ok(InputOutputType::MatrixFloat),
            DataType::Pool => Ok(InputOutputType::Pool),
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
            InputOutputType::Complex => DataType::Complex,
            InputOutputType::TensorFloat => DataType::TensorFloat,
            InputOutputType::VectorFloat => DataType::VectorFloat,
            InputOutputType::VectorString => DataType::VectorString,
            InputOutputType::VectorBool => DataType::VectorBool,
            InputOutputType::VectorInt => DataType::VectorInt,
            InputOutputType::VectorStereoSample => DataType::VectorStereoSample,
            InputOutputType::VectorComplex => DataType::VectorComplex,
            InputOutputType::VectorVectorFloat => DataType::VectorVectorFloat,
            InputOutputType::VectorVectorString => DataType::VectorVectorString,
            InputOutputType::VectorVectorStereoSample => DataType::VectorVectorStereoSample,
            InputOutputType::VectorVectorComplex => DataType::VectorVectorComplex,
            InputOutputType::VectorMatrixFloat => DataType::VectorMatrixFloat,
            InputOutputType::MapVectorFloat => DataType::MapVectorFloat,
            InputOutputType::MapVectorString => DataType::MapVectorString,
            InputOutputType::MapVectorInt => DataType::MapVectorInt,
            InputOutputType::MapVectorComplex => DataType::MapVectorComplex,
            InputOutputType::MapFloat => DataType::MapFloat,
            InputOutputType::MatrixFloat => DataType::MatrixFloat,
            InputOutputType::Pool => DataType::Pool,
        }
    }
}
