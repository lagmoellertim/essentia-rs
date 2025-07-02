use crate::{data_container::data_type, parameter::ParameterType};

pub trait Parameter {
    fn parameter_type() -> ParameterType;
}

impl Parameter for data_type::Float {
    fn parameter_type() -> ParameterType {
        ParameterType::Float
    }
}

impl Parameter for data_type::String {
    fn parameter_type() -> ParameterType {
        ParameterType::String
    }
}

impl Parameter for data_type::Bool {
    fn parameter_type() -> ParameterType {
        ParameterType::Bool
    }
}

impl Parameter for data_type::Int {
    fn parameter_type() -> ParameterType {
        ParameterType::Int
    }
}

impl Parameter for data_type::StereoSample {
    fn parameter_type() -> ParameterType {
        ParameterType::StereoSample
    }
}

impl Parameter for data_type::VectorFloat {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorFloat
    }
}

impl Parameter for data_type::VectorString {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorString
    }
}

impl Parameter for data_type::VectorBool {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorBool
    }
}

impl Parameter for data_type::VectorInt {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorInt
    }
}

impl Parameter for data_type::VectorStereoSample {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorStereoSample
    }
}

impl Parameter for data_type::VectorVectorFloat {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorVectorFloat
    }
}

impl Parameter for data_type::VectorVectorString {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorVectorString
    }
}

impl Parameter for data_type::VectorVectorStereoSample {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorVectorStereoSample
    }
}

impl Parameter for data_type::VectorMatrixFloat {
    fn parameter_type() -> ParameterType {
        ParameterType::VectorMatrixFloat
    }
}

impl Parameter for data_type::MapVectorFloat {
    fn parameter_type() -> ParameterType {
        ParameterType::MapVectorFloat
    }
}

impl Parameter for data_type::MapVectorString {
    fn parameter_type() -> ParameterType {
        ParameterType::MapVectorString
    }
}

impl Parameter for data_type::MapVectorInt {
    fn parameter_type() -> ParameterType {
        ParameterType::MapVectorInt
    }
}

impl Parameter for data_type::MapFloat {
    fn parameter_type() -> ParameterType {
        ParameterType::MapFloat
    }
}

impl Parameter for data_type::MatrixFloat {
    fn parameter_type() -> ParameterType {
        ParameterType::MatrixFloat
    }
}
