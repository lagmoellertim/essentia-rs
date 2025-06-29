pub mod input_output_type;
pub use input_output_type::InputOutputType;

use crate::variant_data::variant;

pub trait InputOutput {
    fn input_output_type() -> InputOutputType;
}

impl InputOutput for variant::Float {
    fn input_output_type() -> InputOutputType {
        InputOutputType::Float
    }
}

impl InputOutput for variant::UnsignedInt {
    fn input_output_type() -> InputOutputType {
        InputOutputType::UnsignedInt
    }
}

impl InputOutput for variant::Long {
    fn input_output_type() -> InputOutputType {
        InputOutputType::Long
    }
}

impl InputOutput for variant::String {
    fn input_output_type() -> InputOutputType {
        InputOutputType::String
    }
}

impl InputOutput for variant::Bool {
    fn input_output_type() -> InputOutputType {
        InputOutputType::Bool
    }
}

impl InputOutput for variant::Int {
    fn input_output_type() -> InputOutputType {
        InputOutputType::Int
    }
}

impl InputOutput for variant::StereoSample {
    fn input_output_type() -> InputOutputType {
        InputOutputType::StereoSample
    }
}

impl InputOutput for variant::VectorFloat {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorFloat
    }
}

impl InputOutput for variant::VectorString {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorString
    }
}

impl InputOutput for variant::VectorBool {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorBool
    }
}

impl InputOutput for variant::VectorInt {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorInt
    }
}

impl InputOutput for variant::VectorStereoSample {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorStereoSample
    }
}

impl InputOutput for variant::VectorVectorFloat {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorVectorFloat
    }
}

impl InputOutput for variant::VectorVectorString {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorVectorString
    }
}

impl InputOutput for variant::VectorVectorStereoSample {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorVectorStereoSample
    }
}

impl InputOutput for variant::VectorMatrixFloat {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorMatrixFloat
    }
}

impl InputOutput for variant::MapVectorFloat {
    fn input_output_type() -> InputOutputType {
        InputOutputType::MapVectorFloat
    }
}

impl InputOutput for variant::MapVectorString {
    fn input_output_type() -> InputOutputType {
        InputOutputType::MapVectorString
    }
}

impl InputOutput for variant::MapVectorInt {
    fn input_output_type() -> InputOutputType {
        InputOutputType::MapVectorInt
    }
}

impl InputOutput for variant::MapFloat {
    fn input_output_type() -> InputOutputType {
        InputOutputType::MapFloat
    }
}

impl InputOutput for variant::MatrixFloat {
    fn input_output_type() -> InputOutputType {
        InputOutputType::MatrixFloat
    }
}
