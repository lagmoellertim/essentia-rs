use crate::{data_container::data_type, input_output::InputOutputType};

pub trait InputOutput {
    fn input_output_type() -> InputOutputType;
}

impl InputOutput for data_type::Float {
    fn input_output_type() -> InputOutputType {
        InputOutputType::Float
    }
}

impl InputOutput for data_type::UnsignedInt {
    fn input_output_type() -> InputOutputType {
        InputOutputType::UnsignedInt
    }
}

impl InputOutput for data_type::Long {
    fn input_output_type() -> InputOutputType {
        InputOutputType::Long
    }
}

impl InputOutput for data_type::String {
    fn input_output_type() -> InputOutputType {
        InputOutputType::String
    }
}

impl InputOutput for data_type::Bool {
    fn input_output_type() -> InputOutputType {
        InputOutputType::Bool
    }
}

impl InputOutput for data_type::Int {
    fn input_output_type() -> InputOutputType {
        InputOutputType::Int
    }
}

impl InputOutput for data_type::StereoSample {
    fn input_output_type() -> InputOutputType {
        InputOutputType::StereoSample
    }
}

impl InputOutput for data_type::Complex {
    fn input_output_type() -> InputOutputType {
        InputOutputType::Complex
    }
}

impl InputOutput for data_type::VectorFloat {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorFloat
    }
}

impl InputOutput for data_type::VectorString {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorString
    }
}

impl InputOutput for data_type::VectorBool {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorBool
    }
}

impl InputOutput for data_type::VectorInt {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorInt
    }
}

impl InputOutput for data_type::VectorStereoSample {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorStereoSample
    }
}

impl InputOutput for data_type::VectorComplex {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorComplex
    }
}

impl InputOutput for data_type::VectorVectorFloat {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorVectorFloat
    }
}

impl InputOutput for data_type::VectorVectorString {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorVectorString
    }
}

impl InputOutput for data_type::VectorVectorStereoSample {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorVectorStereoSample
    }
}

impl InputOutput for data_type::VectorMatrixFloat {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorMatrixFloat
    }
}

impl InputOutput for data_type::MapVectorFloat {
    fn input_output_type() -> InputOutputType {
        InputOutputType::MapVectorFloat
    }
}

impl InputOutput for data_type::MapVectorString {
    fn input_output_type() -> InputOutputType {
        InputOutputType::MapVectorString
    }
}

impl InputOutput for data_type::MapVectorInt {
    fn input_output_type() -> InputOutputType {
        InputOutputType::MapVectorInt
    }
}

impl InputOutput for data_type::MapVectorComplex {
    fn input_output_type() -> InputOutputType {
        InputOutputType::MapVectorComplex
    }
}

impl InputOutput for data_type::MapFloat {
    fn input_output_type() -> InputOutputType {
        InputOutputType::MapFloat
    }
}

impl InputOutput for data_type::MatrixFloat {
    fn input_output_type() -> InputOutputType {
        InputOutputType::MatrixFloat
    }
}

impl InputOutput for data_type::VectorVectorComplex {
    fn input_output_type() -> InputOutputType {
        InputOutputType::VectorVectorComplex
    }
}

impl InputOutput for data_type::Pool {
    fn input_output_type() -> InputOutputType {
        InputOutputType::Pool
    }
}

impl InputOutput for data_type::TensorFloat {
    fn input_output_type() -> InputOutputType {
        InputOutputType::TensorFloat
    }
}
