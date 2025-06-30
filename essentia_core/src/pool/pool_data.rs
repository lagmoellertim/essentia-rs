use super::pool_data_type::PoolDataType;
use crate::variant_data::variant;

pub trait PoolData {
    fn pool_data_type() -> PoolDataType;
}

impl PoolData for variant::Float {
    fn pool_data_type() -> PoolDataType {
        PoolDataType::Float
    }
}

impl PoolData for variant::String {
    fn pool_data_type() -> PoolDataType {
        PoolDataType::String
    }
}

impl PoolData for variant::StereoSample {
    fn pool_data_type() -> PoolDataType {
        PoolDataType::StereoSample
    }
}

impl PoolData for variant::VectorFloat {
    fn pool_data_type() -> PoolDataType {
        PoolDataType::VectorFloat
    }
}

impl PoolData for variant::VectorString {
    fn pool_data_type() -> PoolDataType {
        PoolDataType::VectorString
    }
}

impl PoolData for variant::VectorStereoSample {
    fn pool_data_type() -> PoolDataType {
        PoolDataType::VectorStereoSample
    }
}

impl PoolData for variant::TensorFloat {
    fn pool_data_type() -> PoolDataType {
        PoolDataType::TensorFloat
    }
}
