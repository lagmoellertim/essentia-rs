use crate::variant_data::variant;

pub mod parameter_type;
pub use parameter_type::ParameterType;

pub mod parameter_map;
pub use parameter_map::ParameterMap;

pub trait Parameter {
    fn parameter_type() -> ParameterType;
}

impl Parameter for variant::Float {
    fn parameter_type() -> ParameterType {
        ParameterType::Float
    }
}

impl Parameter for variant::String {
    fn parameter_type() -> ParameterType {
        ParameterType::String
    }
}

impl Parameter for variant::Bool {
    fn parameter_type() -> ParameterType {
        ParameterType::Bool
    }
}

impl Parameter for variant::Int {
    fn parameter_type() -> ParameterType {
        ParameterType::Int
    }
}

impl Parameter for variant::StereoSample {
    fn parameter_type() -> ParameterType {
        ParameterType::StereoSample
    }
}

impl Parameter for variant::VectorFloat {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorFloat
    }
}

impl Parameter for variant::VectorString {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorString
    }
}

impl Parameter for variant::VectorBool {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorBool
    }
}

impl Parameter for variant::VectorInt {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorInt
    }
}

impl Parameter for variant::VectorStereoSample {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorStereoSample
    }
}

impl Parameter for variant::VectorVectorFloat {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorVectorFloat
    }
}

impl Parameter for variant::VectorVectorString {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorVectorString
    }
}

impl Parameter for variant::VectorVectorStereoSample {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorVectorStereoSample
    }
}

impl Parameter for variant::VectorMatrixFloat {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorMatrixFloat
    }
}

impl Parameter for variant::MapVectorFloat {
    fn parameter_type() -> ParameterType {
        ParameterType::MapVectorFloat
    }
}

impl Parameter for variant::MapVectorString {
    fn parameter_type() -> ParameterType {
        ParameterType::MapVectorString
    }
}

impl Parameter for variant::MapVectorInt {
    fn parameter_type() -> ParameterType {
        ParameterType::MapVectorInt
    }
}

impl Parameter for variant::MapFloat {
    fn parameter_type() -> ParameterType {
        ParameterType::MapFloat
    }
}

impl Parameter for variant::MatrixFloat {
    fn parameter_type() -> ParameterType {
        ParameterType::MatrixFloat
    }
}
