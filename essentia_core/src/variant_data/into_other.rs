use ndarray::{Array2, Array4};
use std::collections::HashMap;

use crate::{
    ffi,
    pool::Pool,
    variant_data::{ConversionError, VariantData, variant},
};

pub trait GetVariantData<T> {
    fn get(&self) -> T;
}

pub trait TryGetVariantData<T> {
    fn try_get(&self) -> Result<T, ConversionError>;
}

impl<'a> GetVariantData<bool> for VariantData<'a, variant::Bool> {
    fn get(&self) -> bool {
        self.data.as_ref().get_bool().unwrap()
    }
}

impl<'a> GetVariantData<String> for VariantData<'a, variant::String> {
    fn get(&self) -> String {
        self.data.as_ref().get_string().unwrap().to_string()
    }
}

impl<'a> GetVariantData<i32> for VariantData<'a, variant::Int> {
    fn get(&self) -> i32 {
        self.data.as_ref().get_int().unwrap()
    }
}

impl<'a> GetVariantData<f32> for VariantData<'a, variant::Float> {
    fn get(&self) -> f32 {
        self.data.as_ref().get_float().unwrap()
    }
}

impl<'a> GetVariantData<u32> for VariantData<'a, variant::UnsignedInt> {
    fn get(&self) -> u32 {
        self.data.as_ref().get_unsigned_int().unwrap()
    }
}

impl<'a> GetVariantData<i64> for VariantData<'a, variant::Long> {
    fn get(&self) -> i64 {
        self.data.as_ref().get_long().unwrap()
    }
}

impl<'a> GetVariantData<ffi::StereoSample> for VariantData<'a, variant::StereoSample> {
    fn get(&self) -> ffi::StereoSample {
        self.data.as_ref().get_stereo_sample().unwrap()
    }
}

impl<'a> GetVariantData<num::Complex<f32>> for VariantData<'a, variant::Complex> {
    fn get(&self) -> num::Complex<f32> {
        self.data.as_ref().get_complex().unwrap().into()
    }
}

impl<'a> GetVariantData<Array4<f32>> for VariantData<'a, variant::TensorFloat> {
    fn get(&self) -> Array4<f32> {
        let tensor = self.data.as_ref().get_tensor_float().unwrap();

        let shape = (
            tensor.shape[0],
            tensor.shape[1],
            tensor.shape[2],
            tensor.shape[3],
        );

        Array4::from_shape_vec(shape, tensor.slice.to_vec()).unwrap() // Safe because C++ guarantees correct dimensions
    }
}

impl<'a> GetVariantData<Vec<bool>> for VariantData<'a, variant::VectorBool> {
    fn get(&self) -> Vec<bool> {
        self.data.as_ref().get_vector_bool().unwrap()
    }
}

impl<'a> GetVariantData<Vec<i32>> for VariantData<'a, variant::VectorInt> {
    fn get(&self) -> Vec<i32> {
        self.data.as_ref().get_vector_int().unwrap().to_vec()
    }
}

impl<'a> GetVariantData<Vec<String>> for VariantData<'a, variant::VectorString> {
    fn get(&self) -> Vec<String> {
        self.data.as_ref().get_vector_string().unwrap()
    }
}

impl<'a> GetVariantData<Vec<f32>> for VariantData<'a, variant::VectorFloat> {
    fn get(&self) -> Vec<f32> {
        self.data.as_ref().get_vector_float().unwrap().to_vec()
    }
}

impl<'a> GetVariantData<Vec<ffi::StereoSample>> for VariantData<'a, variant::VectorStereoSample> {
    fn get(&self) -> Vec<ffi::StereoSample> {
        self.data
            .as_ref()
            .get_vector_stereo_sample()
            .unwrap()
            .to_vec()
    }
}

impl<'a> GetVariantData<Vec<num::Complex<f32>>> for VariantData<'a, variant::VectorComplex> {
    fn get(&self) -> Vec<num::Complex<f32>> {
        self.data
            .as_ref()
            .get_vector_complex()
            .unwrap()
            .iter()
            .map(|c| c.into())
            .collect()
    }
}

impl<'a> GetVariantData<Array2<f32>> for VariantData<'a, variant::MatrixFloat> {
    fn get(&self) -> Array2<f32> {
        let matrix_float = self.data.as_ref().get_matrix_float().unwrap();

        Array2::from_shape_vec(
            (matrix_float.dim1, matrix_float.dim2),
            matrix_float.slice.to_vec(),
        )
        .unwrap() // Safe because C++ guarantees correct dimensions
    }
}

impl<'a> GetVariantData<Vec<Array2<f32>>> for VariantData<'a, variant::VectorMatrixFloat> {
    fn get(&self) -> Vec<Array2<f32>> {
        let matrices = self.data.as_ref().get_vector_matrix_float().unwrap();

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

impl<'a> GetVariantData<Vec<Vec<f32>>> for VariantData<'a, variant::VectorVectorFloat> {
    fn get(&self) -> Vec<Vec<f32>> {
        self.data
            .as_ref()
            .get_vector_vector_float()
            .unwrap()
            .into_iter()
            .map(|float_slice| float_slice.slice.to_vec())
            .collect()
    }
}

impl<'a> TryGetVariantData<Array2<f32>> for VariantData<'a, variant::VectorVectorFloat> {
    fn try_get(&self) -> Result<Array2<f32>, ConversionError> {
        let vec_vec_data = self.data.as_ref().get_vector_vector_float().unwrap();

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

impl<'a> GetVariantData<Vec<Vec<String>>> for VariantData<'a, variant::VectorVectorString> {
    fn get(&self) -> Vec<Vec<String>> {
        self.data
            .as_ref()
            .get_vector_vector_string()
            .unwrap()
            .into_iter()
            .map(|vec_string| vec_string.vec)
            .collect()
    }
}

impl<'a> GetVariantData<Vec<Vec<ffi::StereoSample>>>
    for VariantData<'a, variant::VectorVectorStereoSample>
{
    fn get(&self) -> Vec<Vec<ffi::StereoSample>> {
        self.data
            .as_ref()
            .get_vector_vector_stereo_sample()
            .unwrap()
            .into_iter()
            .map(|slice_stereo_sample| slice_stereo_sample.slice.to_vec())
            .collect()
    }
}

impl<'a> GetVariantData<Vec<Vec<num::Complex<f32>>>>
    for VariantData<'a, variant::VectorVectorComplex>
{
    fn get(&self) -> Vec<Vec<num::Complex<f32>>> {
        self.data
            .as_ref()
            .get_vector_vector_complex()
            .unwrap()
            .into_iter()
            .map(|vec_complex| vec_complex.vec.into_iter().map(|c| c.into()).collect())
            .collect()
    }
}

impl<'a> GetVariantData<HashMap<String, f32>> for VariantData<'a, variant::MapFloat> {
    fn get(&self) -> HashMap<String, f32> {
        self.data
            .as_ref()
            .get_map_float()
            .unwrap()
            .into_iter()
            .map(|entry| (entry.key.to_string(), entry.value))
            .collect()
    }
}

impl<'a> GetVariantData<HashMap<String, Vec<f32>>> for VariantData<'a, variant::MapVectorFloat> {
    fn get(&self) -> HashMap<String, Vec<f32>> {
        self.data
            .as_ref()
            .get_map_vector_float()
            .unwrap()
            .into_iter()
            .map(|entry| (entry.key.to_string(), entry.value.to_vec()))
            .collect()
    }
}

impl<'a> GetVariantData<HashMap<String, Vec<String>>>
    for VariantData<'a, variant::MapVectorString>
{
    fn get(&self) -> HashMap<String, Vec<String>> {
        self.data
            .as_ref()
            .get_map_vector_string()
            .unwrap()
            .into_iter()
            .map(|entry| (entry.key.to_string(), entry.value))
            .collect()
    }
}

impl<'a> GetVariantData<HashMap<String, Vec<i32>>> for VariantData<'a, variant::MapVectorInt> {
    fn get(&self) -> HashMap<String, Vec<i32>> {
        self.data
            .as_ref()
            .get_map_vector_int()
            .unwrap()
            .into_iter()
            .map(|entry| (entry.key.to_string(), entry.value.to_vec()))
            .collect()
    }
}

impl<'a> GetVariantData<HashMap<String, Vec<num::Complex<f32>>>>
    for VariantData<'a, variant::MapVectorComplex>
{
    fn get(&self) -> HashMap<String, Vec<num::Complex<f32>>> {
        self.data
            .as_ref()
            .get_map_vector_complex()
            .unwrap()
            .into_iter()
            .map(|entry| {
                (
                    entry.key.to_string(),
                    entry.value.iter().map(|c| c.into()).collect(),
                )
            })
            .collect()
    }
}

// TODO Maybe the Pool should be take a reference to the PoolBridge?
impl<'a> GetVariantData<Pool> for VariantData<'a, variant::Pool> {
    fn get(&self) -> Pool {
        let pool_bridge_ref = self.data.as_ref().get_pool();
        let cloned_bridge = pool_bridge_ref.clone();
        Pool::new_from_bridge(cloned_bridge)
    }
}
