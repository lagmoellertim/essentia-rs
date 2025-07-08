use essentia_sys::ffi;
use ndarray::{Array2, Array4};
use std::collections::HashMap;

use crate::{ConversionError, DataContainer, data_type};

pub trait IntoDataContainer<T> {
    fn into_data_container(self) -> DataContainer<'static, T>;
}

pub trait TryIntoDataContainer<T> {
    fn try_into_data_container(self) -> Result<DataContainer<'static, T>, ConversionError>;
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
        DataContainer::new_owned(ffi::create_data_container_from_complex(ffi::Complex {
            real: self.re,
            imag: self.im,
        }))
    }
}

impl IntoDataContainer<data_type::TensorFloat> for &Array4<f32> {
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

impl IntoDataContainer<data_type::VectorBool> for &[bool] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorBool> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_bool(self))
    }
}

impl IntoDataContainer<data_type::VectorInt> for &[i32] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorInt> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_int(self))
    }
}

impl IntoDataContainer<data_type::VectorString> for &[&str] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorString> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_string(self))
    }
}

impl IntoDataContainer<data_type::VectorFloat> for &[f32] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorFloat> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_float(self))
    }
}

impl IntoDataContainer<data_type::VectorStereoSample> for &[ffi::StereoSample] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorStereoSample> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_stereo_sample(self))
    }
}

impl IntoDataContainer<data_type::VectorComplex> for &[num::Complex<f32>] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorComplex> {
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

impl IntoDataContainer<data_type::VectorVectorFloat> for &[Vec<f32>] {
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

impl IntoDataContainer<data_type::MatrixFloat> for &Array2<f32> {
    fn into_data_container(self) -> DataContainer<'static, data_type::MatrixFloat> {
        let slice = self.as_slice().expect("Array must be contiguous");
        let (dim1, dim2) = self.dim();

        DataContainer::new_owned(ffi::create_data_container_from_matrix_float(
            ffi::MatrixFloat { slice, dim1, dim2 },
        ))
    }
}

impl IntoDataContainer<data_type::VectorVectorString> for &[&[&str]] {
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

impl IntoDataContainer<data_type::VectorVectorStereoSample> for &[&[ffi::StereoSample]] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorVectorStereoSample> {
        DataContainer::new_owned(ffi::create_data_container_from_vector_vector_stereo_sample(
            self.iter()
                .map(|item| ffi::SliceStereoSample { slice: item })
                .collect(),
        ))
    }
}

impl IntoDataContainer<data_type::VectorVectorComplex> for &[Vec<num::Complex<f32>>] {
    fn into_data_container(self) -> DataContainer<'static, data_type::VectorVectorComplex> {
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

impl IntoDataContainer<data_type::VectorMatrixFloat> for &[Array2<f32>] {
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

impl IntoDataContainer<data_type::MapVectorFloat> for &HashMap<String, Vec<f32>> {
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

impl IntoDataContainer<data_type::MapVectorString> for &HashMap<String, Vec<String>> {
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

impl IntoDataContainer<data_type::MapVectorInt> for &HashMap<String, Vec<i32>> {
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

impl IntoDataContainer<data_type::MapVectorComplex> for &HashMap<String, Vec<num::Complex<f32>>> {
    fn into_data_container(self) -> DataContainer<'static, data_type::MapVectorComplex> {
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

impl IntoDataContainer<data_type::MapFloat> for &HashMap<String, f32> {
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

impl TryIntoDataContainer<data_type::MatrixFloat> for &[Vec<f32>] {
    fn try_into_data_container(
        self,
    ) -> Result<DataContainer<'static, data_type::MatrixFloat>, ConversionError> {
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

impl IntoDataContainer<data_type::Pool> for crate::pool::Pool {
    fn into_data_container(self) -> DataContainer<'static, data_type::Pool> {
        DataContainer::new_owned(ffi::create_data_container_from_pool(self.into_owned_ptr()))
    }
}
