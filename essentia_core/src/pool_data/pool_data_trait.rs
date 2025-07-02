use crate::data_container::data_type;

use super::pool_data_type::PoolDataType;

pub trait PoolData {
    fn pool_data_type() -> PoolDataType;
}

impl PoolData for data_type::Float {
    fn pool_data_type() -> PoolDataType {
        PoolDataType::Float
    }
}

impl PoolData for data_type::String {
    fn pool_data_type() -> PoolDataType {
        PoolDataType::String
    }
}

impl PoolData for data_type::StereoSample {
    fn pool_data_type() -> PoolDataType {
        PoolDataType::StereoSample
    }
}

impl PoolData for data_type::VectorFloat {
    fn pool_data_type() -> PoolDataType {
        PoolDataType::VectorFloat
    }
}

impl PoolData for data_type::VectorString {
    fn pool_data_type() -> PoolDataType {
        PoolDataType::VectorString
    }
}

impl PoolData for data_type::VectorStereoSample {
    fn pool_data_type() -> PoolDataType {
        PoolDataType::VectorStereoSample
    }
}

impl PoolData for data_type::TensorFloat {
    fn pool_data_type() -> PoolDataType {
        PoolDataType::TensorFloat
    }
}
