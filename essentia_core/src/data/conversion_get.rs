use essentia_sys::ffi;
use ndarray::{Array2, Array4};
use std::collections::HashMap;

use crate::{ConversionError, DataContainer, Pool, phantom};

pub trait GetFromDataContainer<T> {
    fn get(&self) -> T;
}

pub trait TryGetFromDataContainer<T> {
    fn try_get(&self) -> Result<T, ConversionError>;
}

impl<'a> GetFromDataContainer<bool> for DataContainer<'a, phantom::Bool> {
    fn get(&self) -> bool {
        self.inner.as_ref().get_bool().unwrap()
    }
}

impl<'a> GetFromDataContainer<String> for DataContainer<'a, phantom::String> {
    fn get(&self) -> String {
        self.inner.as_ref().get_string().unwrap().to_string()
    }
}

impl<'a> GetFromDataContainer<i32> for DataContainer<'a, phantom::Int> {
    fn get(&self) -> i32 {
        self.inner.as_ref().get_int().unwrap()
    }
}

impl<'a> GetFromDataContainer<f32> for DataContainer<'a, phantom::Float> {
    fn get(&self) -> f32 {
        self.inner.as_ref().get_float().unwrap()
    }
}

impl<'a> GetFromDataContainer<u32> for DataContainer<'a, phantom::UnsignedInt> {
    fn get(&self) -> u32 {
        self.inner.as_ref().get_unsigned_int().unwrap()
    }
}

impl<'a> GetFromDataContainer<i64> for DataContainer<'a, phantom::Long> {
    fn get(&self) -> i64 {
        self.inner.as_ref().get_long().unwrap()
    }
}

impl<'a> GetFromDataContainer<ffi::StereoSample> for DataContainer<'a, phantom::StereoSample> {
    fn get(&self) -> ffi::StereoSample {
        self.inner.as_ref().get_stereo_sample().unwrap()
    }
}

impl<'a> GetFromDataContainer<num::Complex<f32>> for DataContainer<'a, phantom::Complex> {
    fn get(&self) -> num::Complex<f32> {
        let ffi_complex = self.inner.as_ref().get_complex().unwrap();
        num::Complex::new(ffi_complex.real, ffi_complex.imag)
    }
}

impl<'a> GetFromDataContainer<Array4<f32>> for DataContainer<'a, phantom::TensorFloat> {
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

impl<'a> GetFromDataContainer<Vec<bool>> for DataContainer<'a, phantom::VectorBool> {
    fn get(&self) -> Vec<bool> {
        self.inner.as_ref().get_vector_bool().unwrap()
    }
}

impl<'a> GetFromDataContainer<Vec<i32>> for DataContainer<'a, phantom::VectorInt> {
    fn get(&self) -> Vec<i32> {
        self.inner.as_ref().get_vector_int().unwrap().to_vec()
    }
}

impl<'a> GetFromDataContainer<Vec<String>> for DataContainer<'a, phantom::VectorString> {
    fn get(&self) -> Vec<String> {
        self.inner.as_ref().get_vector_string().unwrap()
    }
}

impl<'a> GetFromDataContainer<Vec<f32>> for DataContainer<'a, phantom::VectorFloat> {
    fn get(&self) -> Vec<f32> {
        self.inner.as_ref().get_vector_float().unwrap().to_vec()
    }
}

impl<'a> GetFromDataContainer<Vec<ffi::StereoSample>>
    for DataContainer<'a, phantom::VectorStereoSample>
{
    fn get(&self) -> Vec<ffi::StereoSample> {
        self.inner
            .as_ref()
            .get_vector_stereo_sample()
            .unwrap()
            .to_vec()
    }
}

impl<'a> GetFromDataContainer<Vec<num::Complex<f32>>>
    for DataContainer<'a, phantom::VectorComplex>
{
    fn get(&self) -> Vec<num::Complex<f32>> {
        self.inner
            .as_ref()
            .get_vector_complex()
            .unwrap()
            .iter()
            .map(|c| num::Complex::new(c.real, c.imag))
            .collect()
    }
}

impl<'a> GetFromDataContainer<Array2<f32>> for DataContainer<'a, phantom::MatrixFloat> {
    fn get(&self) -> Array2<f32> {
        let matrix_float = self.inner.as_ref().get_matrix_float().unwrap();

        Array2::from_shape_vec(
            (matrix_float.dim1, matrix_float.dim2),
            matrix_float.slice.to_vec(),
        )
        .unwrap() // Safe because C++ guarantees correct dimensions
    }
}

impl<'a> GetFromDataContainer<Vec<Array2<f32>>> for DataContainer<'a, phantom::VectorMatrixFloat> {
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

impl<'a> GetFromDataContainer<Vec<Vec<f32>>> for DataContainer<'a, phantom::VectorVectorFloat> {
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

impl<'a> TryGetFromDataContainer<Array2<f32>> for DataContainer<'a, phantom::VectorVectorFloat> {
    fn try_get(&self) -> Result<Array2<f32>, ConversionError> {
        let vec_vec_data = self.inner.as_ref().get_vector_vector_float().unwrap();

        if vec_vec_data.is_empty() {
            return Err(ConversionError::InvalidFormat {
                message: "Cannot create matrix from empty vector".to_string(),
            });
        }

        let expected_cols = vec_vec_data[0].slice.len();
        if expected_cols == 0 {
            return Err(ConversionError::InvalidFormat {
                message: "Cannot create matrix from empty rows".to_string(),
            });
        }

        for (row_idx, row_data) in vec_vec_data.iter().enumerate() {
            if row_data.slice.len() != expected_cols {
                return Err(ConversionError::InvalidFormat {
                    message: format!(
                        "Non-rectangular matrix: row {} has {} elements, expected {}",
                        row_idx,
                        row_data.slice.len(),
                        expected_cols
                    ),
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

impl<'a> GetFromDataContainer<Vec<Vec<String>>> for DataContainer<'a, phantom::VectorVectorString> {
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

impl<'a> GetFromDataContainer<Vec<Vec<ffi::StereoSample>>>
    for DataContainer<'a, phantom::VectorVectorStereoSample>
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

impl<'a> GetFromDataContainer<Vec<Vec<num::Complex<f32>>>>
    for DataContainer<'a, phantom::VectorVectorComplex>
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
                    .map(|c| num::Complex::new(c.real, c.imag))
                    .collect()
            })
            .collect()
    }
}

impl<'a> GetFromDataContainer<HashMap<String, f32>> for DataContainer<'a, phantom::MapFloat> {
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

impl<'a> GetFromDataContainer<HashMap<String, Vec<f32>>>
    for DataContainer<'a, phantom::MapVectorFloat>
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

impl<'a> GetFromDataContainer<HashMap<String, Vec<String>>>
    for DataContainer<'a, phantom::MapVectorString>
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

impl<'a> GetFromDataContainer<HashMap<String, Vec<i32>>>
    for DataContainer<'a, phantom::MapVectorInt>
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

impl<'a> GetFromDataContainer<HashMap<String, Vec<num::Complex<f32>>>>
    for DataContainer<'a, phantom::MapVectorComplex>
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
                    entry
                        .value
                        .iter()
                        .map(|c| num::Complex::new(c.real, c.imag))
                        .collect(),
                )
            })
            .collect()
    }
}

// TODO Maybe the Pool should be take a reference to the PoolBridge?
impl<'a> GetFromDataContainer<Pool> for DataContainer<'a, phantom::Pool> {
    fn get(&self) -> Pool {
        let pool_bridge_ref = self.inner.as_ref().get_pool();
        let cloned_bridge = pool_bridge_ref.clone();
        Pool::new_from_bridge(cloned_bridge)
    }
}
