use essentia_sys::ffi;
use ndarray::{Array2, Array4};
use std::collections::HashMap;

use crate::{ConversionError, DataContainer, phantom};

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

impl IntoDataContainer<phantom::Bool> for bool {
    fn into_data_container(self) -> DataContainer<'static, phantom::Bool> {
        DataContainer::new_owned(ffi::create_data_container_from_bool(self))
    }
}

impl IntoDataContainer<phantom::String> for &str {
    fn into_data_container(self) -> DataContainer<'static, phantom::String> {
        DataContainer::new_owned(ffi::create_data_container_from_string(self))
    }
}

impl IntoDataContainer<phantom::Int> for i32 {
    fn into_data_container(self) -> DataContainer<'static, phantom::Int> {
        DataContainer::new_owned(ffi::create_data_container_from_int(self))
    }
}

impl IntoDataContainer<phantom::Float> for f32 {
    fn into_data_container(self) -> DataContainer<'static, phantom::Float> {
        DataContainer::new_owned(ffi::create_data_container_from_float(self))
    }
}

impl IntoDataContainer<phantom::UnsignedInt> for u32 {
    fn into_data_container(self) -> DataContainer<'static, phantom::UnsignedInt> {
        DataContainer::new_owned(ffi::create_data_container_from_unsigned_int(self))
    }
}

impl IntoDataContainer<phantom::Long> for i64 {
    fn into_data_container(self) -> DataContainer<'static, phantom::Long> {
        DataContainer::new_owned(ffi::create_data_container_from_long(self))
    }
}

impl IntoDataContainer<phantom::StereoSample> for ffi::StereoSample {
    fn into_data_container(self) -> DataContainer<'static, phantom::StereoSample> {
        DataContainer::new_owned(ffi::create_data_container_from_stereo_sample(self))
    }
}

impl IntoDataContainer<phantom::Complex> for num::Complex<f32> {
    fn into_data_container(self) -> DataContainer<'static, phantom::Complex> {
        DataContainer::new_owned(ffi::create_data_container_from_complex(ffi::Complex {
            real: self.re,
            imag: self.im,
        }))
    }
}

impl<'a> IntoDataContainer<phantom::TensorFloat> for &'a Array4<f32> {
    fn into_data_container(self) -> DataContainer<'static, phantom::TensorFloat> {
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

impl<'a> IntoDataContainer<phantom::VectorBool> for &'a [bool] {
    fn into_data_container(self) -> DataContainer<'static, phantom::VectorBool> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_bool(self))
    }
}

impl<'a> IntoDataContainer<phantom::VectorInt> for &'a [i32] {
    fn into_data_container(self) -> DataContainer<'static, phantom::VectorInt> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_int(self))
    }
}

impl<'a> IntoDataContainer<phantom::VectorString> for &'a [&str] {
    fn into_data_container(self) -> DataContainer<'static, phantom::VectorString> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_string(self))
    }
}

impl<'a> IntoDataContainer<phantom::VectorFloat> for &'a [f32] {
    fn into_data_container(self) -> DataContainer<'static, phantom::VectorFloat> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_float(self))
    }
}

impl<'a> IntoDataContainer<phantom::VectorStereoSample> for &'a [ffi::StereoSample] {
    fn into_data_container(self) -> DataContainer<'static, phantom::VectorStereoSample> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_stereo_sample(self))
    }
}

impl<'a> IntoDataContainer<phantom::VectorComplex> for &'a [num::Complex<f32>] {
    fn into_data_container(self) -> DataContainer<'static, phantom::VectorComplex> {
        let ffi_vec: Vec<ffi::Complex> = self
            .iter()
            .map(|c| ffi::Complex {
                real: c.re,
                imag: c.im,
            })
            .collect();
        DataContainer::new_owned(ffi::create_data_container_from_vector_complex(&ffi_vec))
    }
}

impl<'a> IntoDataContainer<phantom::VectorVectorFloat> for &'a [Vec<f32>] {
    fn into_data_container(self) -> DataContainer<'static, phantom::VectorVectorFloat> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_vector_float(
            self.iter()
                .map(|item| ffi::SliceFloat {
                    slice: item.as_slice(),
                })
                .collect(),
        ))
    }
}

impl<'a> IntoDataContainer<phantom::MatrixFloat> for &'a Array2<f32> {
    fn into_data_container(self) -> DataContainer<'static, phantom::MatrixFloat> {
        let slice = self.as_slice().expect("Array must be contiguous");
        let (dim1, dim2) = self.dim();

        DataContainer::new_owned(ffi::create_data_container_from_matrix_float(
            ffi::MatrixFloat { slice, dim1, dim2 },
        ))
    }
}

impl<'a> IntoDataContainer<phantom::VectorVectorString> for &'a [&[&str]] {
    fn into_data_container(self) -> DataContainer<'static, phantom::VectorVectorString> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_vector_string(
            self.iter()
                .map(|item| ffi::VecString {
                    vec: item.iter().map(|s| s.to_string()).collect(),
                })
                .collect(),
        ))
    }
}

impl<'a> IntoDataContainer<phantom::VectorVectorStereoSample> for &'a [&[ffi::StereoSample]] {
    fn into_data_container(self) -> DataContainer<'static, phantom::VectorVectorStereoSample> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_vector_stereo_sample(
            self.iter()
                .map(|item| ffi::SliceStereoSample { slice: *item })
                .collect(),
        ))
    }
}

impl<'a> IntoDataContainer<phantom::VectorVectorComplex> for &'a [Vec<num::Complex<f32>>] {
    fn into_data_container(self) -> DataContainer<'static, phantom::VectorVectorComplex> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_vector_complex(
            self.iter()
                .map(|item| ffi::VecComplex {
                    vec: item
                        .iter()
                        .map(|c| ffi::Complex {
                            real: c.re,
                            imag: c.im,
                        })
                        .collect(),
                })
                .collect(),
        ))
    }
}

impl<'a> IntoDataContainer<phantom::VectorMatrixFloat> for &'a [Array2<f32>] {
    fn into_data_container(self) -> DataContainer<'static, phantom::VectorMatrixFloat> {
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

impl<'a> IntoDataContainer<phantom::MapVectorFloat> for &'a HashMap<String, Vec<f32>> {
    fn into_data_container(self) -> DataContainer<'static, phantom::MapVectorFloat> {
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

impl<'a> IntoDataContainer<phantom::MapVectorString> for &'a HashMap<String, Vec<String>> {
    fn into_data_container(self) -> DataContainer<'static, phantom::MapVectorString> {
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

impl<'a> IntoDataContainer<phantom::MapVectorInt> for &'a HashMap<String, Vec<i32>> {
    fn into_data_container(self) -> DataContainer<'static, phantom::MapVectorInt> {
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

impl<'a> IntoDataContainer<phantom::MapVectorComplex>
    for &'a HashMap<String, Vec<num::Complex<f32>>>
{
    fn into_data_container(self) -> DataContainer<'static, phantom::MapVectorComplex> {
        let converted_data: Vec<(String, Vec<ffi::Complex>)> = self
            .iter()
            .map(|(key, vec)| {
                (
                    key.clone(),
                    vec.iter()
                        .map(|c| ffi::Complex {
                            real: c.re,
                            imag: c.im,
                        })
                        .collect(),
                )
            })
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

impl<'a> IntoDataContainer<phantom::MapFloat> for &'a HashMap<String, f32> {
    fn into_data_container(self) -> DataContainer<'static, phantom::MapFloat> {
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

impl<'a> TryIntoDataContainer<phantom::MatrixFloat> for &'a [Vec<f32>] {
    fn try_into_data_container(
        self,
    ) -> Result<DataContainer<'static, phantom::MatrixFloat>, ConversionError> {
        if self.is_empty() {
            return Err(ConversionError::InvalidFormat {
                message: "Cannot create matrix from empty vector".to_string(),
            });
        }

        let expected_cols = self[0].len();
        if expected_cols == 0 {
            return Err(ConversionError::InvalidFormat {
                message: "Cannot create matrix from empty rows".to_string(),
            });
        }

        for (row_idx, row) in self.iter().enumerate() {
            if row.len() != expected_cols {
                return Err(ConversionError::InvalidFormat {
                    message: format!(
                        "Non-rectangular matrix: row {} has {} elements, expected {}",
                        row_idx,
                        row.len(),
                        expected_cols
                    ),
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

impl IntoDataContainer<phantom::Pool> for crate::pool::Pool {
    fn into_data_container(self) -> DataContainer<'static, phantom::Pool> {
        DataContainer::new_owned(ffi::create_data_container_from_pool(self.into_owned_ptr()))
    }
}
