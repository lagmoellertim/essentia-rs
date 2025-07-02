use core::fmt;

use crate::data_container::DataType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PoolDataType {
    Float,
    String,
    StereoSample,
    VectorFloat,
    VectorString,
    VectorStereoSample,
    TensorFloat,
}

impl TryFrom<DataType> for PoolDataType {
    type Error = String;

    fn try_from(data_type: DataType) -> Result<Self, Self::Error> {
        match data_type {
            DataType::Float => Ok(PoolDataType::Float),
            DataType::String => Ok(PoolDataType::String),
            DataType::StereoSample => Ok(PoolDataType::StereoSample),
            DataType::VectorFloat => Ok(PoolDataType::VectorFloat),
            DataType::VectorString => Ok(PoolDataType::VectorString),
            DataType::VectorStereoSample => Ok(PoolDataType::VectorStereoSample),
            DataType::TensorFloat => Ok(PoolDataType::TensorFloat),
            _ => Err(format!("Unsupported pool data type: {:?}", data_type)),
        }
    }
}

impl From<PoolDataType> for DataType {
    fn from(pool_type: PoolDataType) -> Self {
        match pool_type {
            PoolDataType::Float => DataType::Float,
            PoolDataType::String => DataType::String,
            PoolDataType::StereoSample => DataType::StereoSample,
            PoolDataType::VectorFloat => DataType::VectorFloat,
            PoolDataType::VectorString => DataType::VectorString,
            PoolDataType::VectorStereoSample => DataType::VectorStereoSample,
            PoolDataType::TensorFloat => DataType::TensorFloat,
        }
    }
}

impl fmt::Display for PoolDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
