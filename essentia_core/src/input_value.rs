use std::pin::Pin;

use ndarray::Array2;

use crate::ffi;

pub enum InputValue<'a> {
    Real(f32),
    Int(i32),
    UnsignedInt(u32),
    Long(i64),
    VectorReal(&'a [f32]),
    VectorVectorReal(Vec<ffi::FloatSlice<'a>>),
    MatrixReal(ffi::FloatMatrix2d<'a>),
}

impl<'a> InputValue<'a> {
    pub fn set_as_input(self, algorithm: Pin<&mut ffi::AlgorithmBridge>, input_name: &str) {
        match self {
            InputValue::Real(value) => algorithm.set_input_real(input_name, value),
            InputValue::Int(value) => algorithm.set_input_int(input_name, value),
            InputValue::UnsignedInt(value) => algorithm.set_input_uint(input_name, value),
            InputValue::Long(value) => algorithm.set_input_long(input_name, value),
            InputValue::VectorReal(value) => algorithm.set_input_real_vector(input_name, value),
            InputValue::VectorVectorReal(value) => {
                algorithm.set_input_real_vector_vector(input_name, value)
            }
            InputValue::MatrixReal(value) => algorithm.set_input_real_matrix(input_name, value),
        }
    }
}

impl<'a> From<f32> for InputValue<'a> {
    fn from(value: f32) -> Self {
        InputValue::Real(value)
    }
}

impl<'a> From<i32> for InputValue<'a> {
    fn from(value: i32) -> Self {
        InputValue::Int(value)
    }
}

impl<'a> From<u32> for InputValue<'a> {
    fn from(value: u32) -> Self {
        InputValue::UnsignedInt(value)
    }
}

impl<'a> From<i64> for InputValue<'a> {
    fn from(value: i64) -> Self {
        InputValue::Long(value)
    }
}

impl<'a> From<&'a [f32]> for InputValue<'a> {
    fn from(value: &'a [f32]) -> Self {
        InputValue::VectorReal(value)
    }
}

impl<'a> From<&'a [Vec<f32>]> for InputValue<'a> {
    fn from(value: &'a [Vec<f32>]) -> Self {
        InputValue::VectorVectorReal(
            value
                .iter()
                .map(|item| ffi::FloatSlice {
                    slice: item.as_slice(),
                })
                .collect(),
        )
    }
}

impl<'a> From<&'a Array2<f32>> for InputValue<'a> {
    fn from(value: &'a Array2<f32>) -> Self {
        let slice = value.as_slice().expect("Array must be contiguous");
        let (dim1, dim2) = value.dim();

        InputValue::MatrixReal(ffi::FloatMatrix2d { slice, dim1, dim2 })
    }
}
