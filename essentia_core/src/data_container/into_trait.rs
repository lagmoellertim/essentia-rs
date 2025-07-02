use essentia_sys::ffi;
use ndarray::{Array2, Array4};
use std::collections::HashMap;

use crate::{
    data_container::{ConversionError, DataContainer, data_type},
    pool::Pool,
};

fn complex_to_ffi(complex: &num::Complex<f32>) -> ffi::Complex {
    ffi::Complex {
        real: complex.re,
        imag: complex.im,
    }
}

pub trait IntoDataContainer<T> {
    fn into_data_container(self) -> DataContainer<'static, T>;
}

pub trait TryIntoDataContainer<T> {
    fn try_into_data_container(self) -> Result<DataContainer<'static, T>, ConversionError>;
}

impl<T, V> TryIntoDataContainer<T> for V
where
    V: IntoDataContainer<T>,
{
    fn try_into_data_container(self) -> Result<DataContainer<'static, T>, ConversionError> {
        Ok(self.into_data_container())
    }
}

impl<'a, T> IntoDataContainer<T> for DataContainer<'a, T> {
    fn into_data_container(self) -> DataContainer<'static, T> {
        let owned_ptr = self.into_owned_ptr();
        DataContainer::new_owned(owned_ptr)
    }
}

impl IntoDataContainer<data_type::Bool> for bool {
    fn into_data_container(self) -> DataContainer<'static, data_type::Bool> {
        DataContainer::new_owned(ffi::create_data_container_from_bool(self))
    }
}

impl IntoDataContainer<data_type::String> for &str {
    fn into_data_container(self) -> DataContainer<'static, data_type::String> {
        DataContainer::new_owned(ffi::create_data_container_from_string(self))
    }
}

impl IntoDataContainer<data_type::Int> for i32 {
    fn into_data_container(self) -> DataContainer<'static, data_type::Int> {
        DataContainer::new_owned(ffi::create_data_container_from_int(self))
    }
}

impl IntoDataContainer<data_type::Float> for f32 {
    fn into_data_container(self) -> DataContainer<'static, data_type::Float> {
        DataContainer::new_owned(ffi::create_data_container_from_float(self))
    }
}

impl IntoDataContainer<data_type::UnsignedInt> for u32 {
    fn into_data_container(self) -> DataContainer<'static, data_type::UnsignedInt> {
        DataContainer::new_owned(ffi::create_data_container_from_unsigned_int(self))
    }
}

impl IntoDataContainer<data_type::Long> for i64 {
    fn into_data_container(self) -> DataContainer<'static, data_type::Long> {
        DataContainer::new_owned(ffi::create_data_container_from_long(self))
    }
}

impl IntoDataContainer<data_type::StereoSample> for ffi::StereoSample {
    fn into_data_container(self) -> DataContainer<'static, data_type::StereoSample> {
        DataContainer::new_owned(ffi::create_data_container_from_stereo_sample(self))
    }
}

impl IntoDataContainer<data_type::Complex> for num::Complex<f32> {
    fn into_data_container(self) -> DataContainer<'static, data_type::Complex> {
        DataContainer::new_owned(ffi::create_data_container_from_complex(complex_to_ffi(
            &self,
        )))
    }
}

impl<'a> IntoDataContainer<data_type::TensorFloat> for &'a Array4<f32> {
    fn into_data_container(self) -> DataContainer<'static, data_type::TensorFloat> {
        let slice = self.as_slice().expect("Array must be contiguous");
        let shape = [
            self.shape()[0],
            self.shape()[1],
            self.shape()[2],
            self.shape()[3],
        ];

        DataContainer::new_owned(ffi::create_data_container_from_tensor_float(
            ffi::TensorFloat {
                slice,
                shape: &shape,
            },
        ))
    }
}

impl<'a> IntoDataContainer<data_type::VectorBool> for &'a [bool] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorBool> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_bool(self))
    }
}

impl<'a> IntoDataContainer<data_type::VectorInt> for &'a [i32] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorInt> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_int(self))
    }
}

impl<'a> IntoDataContainer<data_type::VectorString> for &'a [&str] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorString> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_string(self))
    }
}

impl<'a> IntoDataContainer<data_type::VectorFloat> for &'a [f32] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorFloat> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_float(self))
    }
}

impl<'a> IntoDataContainer<data_type::VectorStereoSample> for &'a [ffi::StereoSample] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorStereoSample> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_stereo_sample(self))
    }
}

impl<'a> IntoDataContainer<data_type::VectorComplex> for &'a [num::Complex<f32>] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorComplex> {
        let ffi_vec: Vec<ffi::Complex> = self.iter().map(|c| complex_to_ffi(c)).collect();
        DataContainer::new_owned(ffi::create_data_container_from_vector_complex(&ffi_vec))
    }
}

impl<'a> IntoDataContainer<data_type::VectorVectorFloat> for &'a [Vec<f32>] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorVectorFloat> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_vector_float(
            self.iter()
                .map(|item| ffi::SliceFloat {
                    slice: item.as_slice(),
                })
                .collect(),
        ))
    }
}

impl<'a> IntoDataContainer<data_type::MatrixFloat> for &'a Array2<f32> {
    fn into_data_container(self) -> DataContainer<'static, data_type::MatrixFloat> {
        let slice = self.as_slice().expect("Array must be contiguous");
        let (dim1, dim2) = self.dim();

        DataContainer::new_owned(ffi::create_data_container_from_matrix_float(
            ffi::MatrixFloat { slice, dim1, dim2 },
        ))
    }
}

impl<'a> IntoDataContainer<data_type::VectorVectorString> for &'a [&[&str]] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorVectorString> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_vector_string(
            self.iter()
                .map(|item| ffi::VecString {
                    vec: item.iter().map(|s| s.to_string()).collect(),
                })
                .collect(),
        ))
    }
}

impl<'a> IntoDataContainer<data_type::VectorVectorStereoSample> for &'a [&[ffi::StereoSample]] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorVectorStereoSample> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_vector_stereo_sample(
            self.iter()
                .map(|item| ffi::SliceStereoSample { slice: *item })
                .collect(),
        ))
    }
}

impl<'a> IntoDataContainer<data_type::VectorVectorComplex> for &'a [Vec<num::Complex<f32>>] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorVectorComplex> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_vector_complex(
            self.iter()
                .map(|item| ffi::VecComplex {
                    vec: item.iter().map(|c| complex_to_ffi(c)).collect(),
                })
                .collect(),
        ))
    }
}

impl<'a> IntoDataContainer<data_type::VectorMatrixFloat> for &'a [Array2<f32>] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorMatrixFloat> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_matrix_float(
            self.iter()
                .map(|array| {
                    let slice = array.as_slice().expect("Array must be contiguous");
                    let (dim1, dim2) = array.dim();
                    ffi::MatrixFloat { slice, dim1, dim2 }
                })
                .collect(),
        ))
    }
}

impl<'a> IntoDataContainer<data_type::MapVectorFloat> for &'a HashMap<String, Vec<f32>> {
    fn into_data_container(self) -> DataContainer<'static, data_type::MapVectorFloat> {
        DataContainer::new_owned(ffi::create_data_container_from_map_vector_float(
            self.iter()
                .map(|(key, vec)| ffi::MapEntryVectorFloat {
                    key: key.clone(),
                    value: vec.as_slice(),
                })
                .collect(),
        ))
    }
}

impl<'a> IntoDataContainer<data_type::MapVectorString> for &'a HashMap<String, Vec<String>> {
    fn into_data_container(self) -> DataContainer<'static, data_type::MapVectorString> {
        DataContainer::new_owned(ffi::create_data_container_from_map_vector_string(
            self.iter()
                .map(|(key, vec)| ffi::MapEntryVectorString {
                    key: key.clone(),
                    value: vec.clone(),
                })
                .collect(),
        ))
    }
}

impl<'a> IntoDataContainer<data_type::MapVectorInt> for &'a HashMap<String, Vec<i32>> {
    fn into_data_container(self) -> DataContainer<'static, data_type::MapVectorInt> {
        DataContainer::new_owned(ffi::create_data_container_from_map_vector_int(
            self.iter()
                .map(|(key, vec)| ffi::MapEntryVectorInt {
                    key: key.clone(),
                    value: vec.as_slice(),
                })
                .collect(),
        ))
    }
}

impl<'a> IntoDataContainer<data_type::MapVectorComplex>
    for &'a HashMap<String, Vec<num::Complex<f32>>>
{
    fn into_data_container(self) -> DataContainer<'static, data_type::MapVectorComplex> {
        // Convert all data first to avoid lifetime issues
        let converted_data: Vec<(String, Vec<ffi::Complex>)> = self
            .iter()
            .map(|(key, vec)| (key.clone(), vec.iter().map(|c| complex_to_ffi(c)).collect()))
            .collect();

        let entries: Vec<ffi::MapEntryVectorComplex> = converted_data
            .iter()
            .map(|(key, ffi_vec)| ffi::MapEntryVectorComplex {
                key: key.clone(),
                value: ffi_vec.as_slice(),
            })
            .collect();

        DataContainer::new_owned(ffi::create_data_container_from_map_vector_complex(entries))
    }
}

impl<'a> IntoDataContainer<data_type::MapFloat> for &'a HashMap<String, f32> {
    fn into_data_container(self) -> DataContainer<'static, data_type::MapFloat> {
        DataContainer::new_owned(ffi::create_data_container_from_map_float(
            self.iter()
                .map(|(key, &val)| ffi::MapEntryFloat {
                    key: key.clone(),
                    value: val,
                })
                .collect(),
        ))
    }
}

impl<'a> TryIntoDataContainer<data_type::MatrixFloat> for &'a [Vec<f32>] {
    fn try_into_data_container(
        self,
    ) -> Result<DataContainer<'static, data_type::MatrixFloat>, ConversionError> {
        if self.is_empty() {
            return Err(ConversionError::EmptyMatrix);
        }

        let expected_cols = self[0].len();
        if expected_cols == 0 {
            return Err(ConversionError::EmptyRows);
        }

        for (row_idx, row) in self.iter().enumerate() {
            if row.len() != expected_cols {
                return Err(ConversionError::NonRectangular {
                    row: row_idx,
                    expected: expected_cols,
                    actual: row.len(),
                });
            }
        }

        let mut flat_data = Vec::with_capacity(self.len() * expected_cols);
        for row in self {
            flat_data.extend(row);
        }

        let dim1 = flat_data.len() / expected_cols;
        let dim2 = expected_cols;

        Ok(DataContainer::new_owned(
            ffi::create_data_container_from_matrix_float(ffi::MatrixFloat {
                slice: &flat_data,
                dim1,
                dim2,
            }),
        ))
    }
}

impl IntoDataContainer<data_type::Pool> for Pool {
    fn into_data_container(self) -> DataContainer<'static, data_type::Pool> {
        DataContainer::new_owned(ffi::create_data_container_from_pool(self.into_owned_ptr()))
    }
}
