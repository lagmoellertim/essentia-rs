use essentia_sys::ffi;
use ndarray::{Array2, Array4};
use std::collections::HashMap;

use crate::{
    data_container::{ConversionError, DataContainer, data_type},
    pool::Pool,
};

fn complex_from_ffi(complex: &ffi::Complex) -> num::Complex<f32> {
    num::Complex::new(complex.real, complex.imag)
}

pub trait TryGetFromDataContainer<T> {
    fn get(&self) -> T;
}

pub trait TryTryGetFromDataContainer<T> {
    fn try_get(&self) -> Result<T, ConversionError>;
}

impl<'a> TryGetFromDataContainer<bool> for DataContainer<'a, data_type::Bool> {
    fn get(&self) -> bool {
        self.inner.as_ref().get_bool().unwrap()
    }
}

impl<'a> TryGetFromDataContainer<String> for DataContainer<'a, data_type::String> {
    fn get(&self) -> String {
        self.inner.as_ref().get_string().unwrap().to_string()
    }
}

impl<'a> TryGetFromDataContainer<i32> for DataContainer<'a, data_type::Int> {
    fn get(&self) -> i32 {
        self.inner.as_ref().get_int().unwrap()
    }
}

impl<'a> TryGetFromDataContainer<f32> for DataContainer<'a, data_type::Float> {
    fn get(&self) -> f32 {
        self.inner.as_ref().get_float().unwrap()
    }
}

impl<'a> TryGetFromDataContainer<u32> for DataContainer<'a, data_type::UnsignedInt> {
    fn get(&self) -> u32 {
        self.inner.as_ref().get_unsigned_int().unwrap()
    }
}

impl<'a> TryGetFromDataContainer<i64> for DataContainer<'a, data_type::Long> {
    fn get(&self) -> i64 {
        self.inner.as_ref().get_long().unwrap()
    }
}

impl<'a> TryGetFromDataContainer<ffi::StereoSample> for DataContainer<'a, data_type::StereoSample> {
    fn get(&self) -> ffi::StereoSample {
        self.inner.as_ref().get_stereo_sample().unwrap()
    }
}

impl<'a> TryGetFromDataContainer<num::Complex<f32>> for DataContainer<'a, data_type::Complex> {
    fn get(&self) -> num::Complex<f32> {
        complex_from_ffi(&self.inner.as_ref().get_complex().unwrap())
    }
}

impl<'a> TryGetFromDataContainer<Array4<f32>> for DataContainer<'a, data_type::TensorFloat> {
    fn get(&self) -> Array4<f32> {
        let tensor = self.inner.as_ref().get_tensor_float().unwrap();

        let shape = (
            tensor.shape[0],
            tensor.shape[1],
            tensor.shape[2],
            tensor.shape[3],
        );

        Array4::from_shape_vec(shape, tensor.slice.to_vec()).unwrap() // Safe because C++ guarantees correct dimensions
    }
}

impl<'a> TryGetFromDataContainer<Vec<bool>> for DataContainer<'a, data_type::VectorBool> {
    fn get(&self) -> Vec<bool> {
        self.inner.as_ref().get_vector_bool().unwrap()
    }
}

impl<'a> TryGetFromDataContainer<Vec<i32>> for DataContainer<'a, data_type::VectorInt> {
    fn get(&self) -> Vec<i32> {
        self.inner.as_ref().get_vector_int().unwrap().to_vec()
    }
}

impl<'a> TryGetFromDataContainer<Vec<String>> for DataContainer<'a, data_type::VectorString> {
    fn get(&self) -> Vec<String> {
        self.inner.as_ref().get_vector_string().unwrap()
    }
}

impl<'a> TryGetFromDataContainer<Vec<f32>> for DataContainer<'a, data_type::VectorFloat> {
    fn get(&self) -> Vec<f32> {
        self.inner.as_ref().get_vector_float().unwrap().to_vec()
    }
}

impl<'a> TryGetFromDataContainer<Vec<ffi::StereoSample>>
    for DataContainer<'a, data_type::VectorStereoSample>
{
    fn get(&self) -> Vec<ffi::StereoSample> {
        self.inner
            .as_ref()
            .get_vector_stereo_sample()
            .unwrap()
            .to_vec()
    }
}

impl<'a> TryGetFromDataContainer<Vec<num::Complex<f32>>>
    for DataContainer<'a, data_type::VectorComplex>
{
    fn get(&self) -> Vec<num::Complex<f32>> {
        self.inner
            .as_ref()
            .get_vector_complex()
            .unwrap()
            .iter()
            .map(|c| complex_from_ffi(c))
            .collect()
    }
}

impl<'a> TryGetFromDataContainer<Array2<f32>> for DataContainer<'a, data_type::MatrixFloat> {
    fn get(&self) -> Array2<f32> {
        let matrix_float = self.inner.as_ref().get_matrix_float().unwrap();

        Array2::from_shape_vec(
            (matrix_float.dim1, matrix_float.dim2),
            matrix_float.slice.to_vec(),
        )
        .unwrap() // Safe because C++ guarantees correct dimensions
    }
}

impl<'a> TryGetFromDataContainer<Vec<Array2<f32>>>
    for DataContainer<'a, data_type::VectorMatrixFloat>
{
    fn get(&self) -> Vec<Array2<f32>> {
        let matrices = self.inner.as_ref().get_vector_matrix_float().unwrap();

        matrices
            .into_iter()
            .map(|matrix_float| {
                Array2::from_shape_vec(
                    (matrix_float.dim1, matrix_float.dim2),
                    matrix_float.slice.to_vec(),
                )
                .unwrap() // Safe because C++ guarantees correct dimensions
            })
            .collect()
    }
}

impl<'a> TryGetFromDataContainer<Vec<Vec<f32>>>
    for DataContainer<'a, data_type::VectorVectorFloat>
{
    fn get(&self) -> Vec<Vec<f32>> {
        self.inner
            .as_ref()
            .get_vector_vector_float()
            .unwrap()
            .into_iter()
            .map(|float_slice| float_slice.slice.to_vec())
            .collect()
    }
}

impl<'a> TryTryGetFromDataContainer<Array2<f32>>
    for DataContainer<'a, data_type::VectorVectorFloat>
{
    fn try_get(&self) -> Result<Array2<f32>, ConversionError> {
        let vec_vec_data = self.inner.as_ref().get_vector_vector_float().unwrap();

        if vec_vec_data.is_empty() {
            return Err(ConversionError::EmptyMatrix);
        }

        let expected_cols = vec_vec_data[0].slice.len();
        if expected_cols == 0 {
            return Err(ConversionError::EmptyRows);
        }

        for (row_idx, row_data) in vec_vec_data.iter().enumerate() {
            if row_data.slice.len() != expected_cols {
                return Err(ConversionError::NonRectangular {
                    row: row_idx,
                    expected: expected_cols,
                    actual: row_data.slice.len(),
                });
            }
        }

        let mut flat_data = Vec::with_capacity(vec_vec_data.len() * expected_cols);
        for row_data in &vec_vec_data {
            flat_data.extend_from_slice(row_data.slice);
        }

        let dim1 = vec_vec_data.len();
        let dim2 = expected_cols;

        Ok(Array2::from_shape_vec((dim1, dim2), flat_data).unwrap())
    }
}

impl<'a> TryGetFromDataContainer<Vec<Vec<String>>>
    for DataContainer<'a, data_type::VectorVectorString>
{
    fn get(&self) -> Vec<Vec<String>> {
        self.inner
            .as_ref()
            .get_vector_vector_string()
            .unwrap()
            .into_iter()
            .map(|vec_string| vec_string.vec)
            .collect()
    }
}

impl<'a> TryGetFromDataContainer<Vec<Vec<ffi::StereoSample>>>
    for DataContainer<'a, data_type::VectorVectorStereoSample>
{
    fn get(&self) -> Vec<Vec<ffi::StereoSample>> {
        self.inner
            .as_ref()
            .get_vector_vector_stereo_sample()
            .unwrap()
            .into_iter()
            .map(|slice_stereo_sample| slice_stereo_sample.slice.to_vec())
            .collect()
    }
}

impl<'a> TryGetFromDataContainer<Vec<Vec<num::Complex<f32>>>>
    for DataContainer<'a, data_type::VectorVectorComplex>
{
    fn get(&self) -> Vec<Vec<num::Complex<f32>>> {
        self.inner
            .as_ref()
            .get_vector_vector_complex()
            .unwrap()
            .into_iter()
            .map(|vec_complex| {
                vec_complex
                    .vec
                    .into_iter()
                    .map(|c| complex_from_ffi(&c))
                    .collect()
            })
            .collect()
    }
}

impl<'a> TryGetFromDataContainer<HashMap<String, f32>> for DataContainer<'a, data_type::MapFloat> {
    fn get(&self) -> HashMap<String, f32> {
        self.inner
            .as_ref()
            .get_map_float()
            .unwrap()
            .into_iter()
            .map(|entry| (entry.key.to_string(), entry.value))
            .collect()
    }
}

impl<'a> TryGetFromDataContainer<HashMap<String, Vec<f32>>>
    for DataContainer<'a, data_type::MapVectorFloat>
{
    fn get(&self) -> HashMap<String, Vec<f32>> {
        self.inner
            .as_ref()
            .get_map_vector_float()
            .unwrap()
            .into_iter()
            .map(|entry| (entry.key.to_string(), entry.value.to_vec()))
            .collect()
    }
}

impl<'a> TryGetFromDataContainer<HashMap<String, Vec<String>>>
    for DataContainer<'a, data_type::MapVectorString>
{
    fn get(&self) -> HashMap<String, Vec<String>> {
        self.inner
            .as_ref()
            .get_map_vector_string()
            .unwrap()
            .into_iter()
            .map(|entry| (entry.key.to_string(), entry.value))
            .collect()
    }
}

impl<'a> TryGetFromDataContainer<HashMap<String, Vec<i32>>>
    for DataContainer<'a, data_type::MapVectorInt>
{
    fn get(&self) -> HashMap<String, Vec<i32>> {
        self.inner
            .as_ref()
            .get_map_vector_int()
            .unwrap()
            .into_iter()
            .map(|entry| (entry.key.to_string(), entry.value.to_vec()))
            .collect()
    }
}

impl<'a> TryGetFromDataContainer<HashMap<String, Vec<num::Complex<f32>>>>
    for DataContainer<'a, data_type::MapVectorComplex>
{
    fn get(&self) -> HashMap<String, Vec<num::Complex<f32>>> {
        self.inner
            .as_ref()
            .get_map_vector_complex()
            .unwrap()
            .into_iter()
            .map(|entry| {
                (
                    entry.key.to_string(),
                    entry.value.iter().map(|c| complex_from_ffi(c)).collect(),
                )
            })
            .collect()
    }
}

// TODO Maybe the Pool should be take a reference to the PoolBridge?
impl<'a> TryGetFromDataContainer<Pool> for DataContainer<'a, data_type::Pool> {
    fn get(&self) -> Pool {
        let pool_bridge_ref = self.inner.as_ref().get_pool();
        let cloned_bridge = pool_bridge_ref.clone();
        Pool::new_from_bridge(cloned_bridge)
    }
}
