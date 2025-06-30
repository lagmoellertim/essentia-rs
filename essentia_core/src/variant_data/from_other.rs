use ndarray::{Array2, Array4};
use std::collections::HashMap;

use crate::{
    ffi,
    variant_data::{ConversionError, VariantData, variant},
};

pub trait IntoVariantData<T> {
    fn into_variant_data(self) -> VariantData<'static, T>;
}

pub trait TryIntoVariantData<T> {
    fn try_into_variant_data(self) -> Result<VariantData<'static, T>, ConversionError>;
}

impl<T, V> TryIntoVariantData<T> for V
where
    V: IntoVariantData<T>,
{
    fn try_into_variant_data(self) -> Result<VariantData<'static, T>, ConversionError> {
        Ok(self.into_variant_data())
    }
}

impl<'a, T> IntoVariantData<T> for VariantData<'a, T> {
    fn into_variant_data(self) -> VariantData<'static, T> {
        let owned_ptr = self.into_owned_ptr();
        VariantData::new_owned(owned_ptr)
    }
}

impl IntoVariantData<variant::Bool> for bool {
    fn into_variant_data(self) -> VariantData<'static, variant::Bool> {
        VariantData::new_owned(ffi::create_variant_data_from_bool(self))
    }
}

impl IntoVariantData<variant::String> for &str {
    fn into_variant_data(self) -> VariantData<'static, variant::String> {
        VariantData::new_owned(ffi::create_variant_data_from_string(self))
    }
}

impl IntoVariantData<variant::Int> for i32 {
    fn into_variant_data(self) -> VariantData<'static, variant::Int> {
        VariantData::new_owned(ffi::create_variant_data_from_int(self))
    }
}

impl IntoVariantData<variant::Float> for f32 {
    fn into_variant_data(self) -> VariantData<'static, variant::Float> {
        VariantData::new_owned(ffi::create_variant_data_from_float(self))
    }
}

impl IntoVariantData<variant::UnsignedInt> for u32 {
    fn into_variant_data(self) -> VariantData<'static, variant::UnsignedInt> {
        VariantData::new_owned(ffi::create_variant_data_from_unsigned_int(self))
    }
}

impl IntoVariantData<variant::Long> for i64 {
    fn into_variant_data(self) -> VariantData<'static, variant::Long> {
        VariantData::new_owned(ffi::create_variant_data_from_long(self))
    }
}

impl IntoVariantData<variant::StereoSample> for ffi::StereoSample {
    fn into_variant_data(self) -> VariantData<'static, variant::StereoSample> {
        VariantData::new_owned(ffi::create_variant_data_from_stereo_sample(self))
    }
}

impl IntoVariantData<variant::Complex> for num::Complex<f32> {
    fn into_variant_data(self) -> VariantData<'static, variant::Complex> {
        VariantData::new_owned(ffi::create_variant_data_from_complex(self.into()))
    }
}

impl<'a> IntoVariantData<variant::TensorFloat> for &'a Array4<f32> {
    fn into_variant_data(self) -> VariantData<'static, variant::TensorFloat> {
        let slice = self.as_slice().expect("Array must be contiguous");
        let shape = [
            self.shape()[0],
            self.shape()[1],
            self.shape()[2],
            self.shape()[3],
        ];

        VariantData::new_owned(ffi::create_variant_data_from_tensor_float(
            ffi::TensorFloat {
                slice,
                shape: &shape,
            },
        ))
    }
}

impl<'a> IntoVariantData<variant::VectorBool> for &'a [bool] {
    fn into_variant_data(self) -> VariantData<'static, variant::VectorBool> {
        VariantData::new_owned(ffi::create_variant_data_from_vector_bool(self))
    }
}

impl<'a> IntoVariantData<variant::VectorInt> for &'a [i32] {
    fn into_variant_data(self) -> VariantData<'static, variant::VectorInt> {
        VariantData::new_owned(ffi::create_variant_data_from_vector_int(self))
    }
}

impl<'a> IntoVariantData<variant::VectorString> for &'a [&str] {
    fn into_variant_data(self) -> VariantData<'static, variant::VectorString> {
        VariantData::new_owned(ffi::create_variant_data_from_vector_string(self))
    }
}

impl<'a> IntoVariantData<variant::VectorFloat> for &'a [f32] {
    fn into_variant_data(self) -> VariantData<'static, variant::VectorFloat> {
        VariantData::new_owned(ffi::create_variant_data_from_vector_float(self))
    }
}

impl<'a> IntoVariantData<variant::VectorStereoSample> for &'a [ffi::StereoSample] {
    fn into_variant_data(self) -> VariantData<'static, variant::VectorStereoSample> {
        VariantData::new_owned(ffi::create_variant_data_from_vector_stereo_sample(self))
    }
}

impl<'a> IntoVariantData<variant::VectorComplex> for &'a [num::Complex<f32>] {
    fn into_variant_data(self) -> VariantData<'static, variant::VectorComplex> {
        let ffi_vec: Vec<ffi::Complex> = self.iter().map(|c| c.into()).collect();
        VariantData::new_owned(ffi::create_variant_data_from_vector_complex(&ffi_vec))
    }
}

impl<'a> IntoVariantData<variant::VectorVectorFloat> for &'a [Vec<f32>] {
    fn into_variant_data(self) -> VariantData<'static, variant::VectorVectorFloat> {
        VariantData::new_owned(ffi::create_variant_data_from_vector_vector_float(
            self.iter()
                .map(|item| ffi::SliceFloat {
                    slice: item.as_slice(),
                })
                .collect(),
        ))
    }
}

impl<'a> IntoVariantData<variant::MatrixFloat> for &'a Array2<f32> {
    fn into_variant_data(self) -> VariantData<'static, variant::MatrixFloat> {
        let slice = self.as_slice().expect("Array must be contiguous");
        let (dim1, dim2) = self.dim();

        VariantData::new_owned(ffi::create_variant_data_from_matrix_float(
            ffi::MatrixFloat { slice, dim1, dim2 },
        ))
    }
}

impl<'a> IntoVariantData<variant::VectorVectorString> for &'a [&[&str]] {
    fn into_variant_data(self) -> VariantData<'static, variant::VectorVectorString> {
        VariantData::new_owned(ffi::create_variant_data_from_vector_vector_string(
            self.iter()
                .map(|item| ffi::VecString {
                    vec: item.iter().map(|s| s.to_string()).collect(),
                })
                .collect(),
        ))
    }
}

impl<'a> IntoVariantData<variant::VectorVectorStereoSample> for &'a [&[ffi::StereoSample]] {
    fn into_variant_data(self) -> VariantData<'static, variant::VectorVectorStereoSample> {
        VariantData::new_owned(ffi::create_variant_data_from_vector_vector_stereo_sample(
            self.iter()
                .map(|item| ffi::SliceStereoSample { slice: *item })
                .collect(),
        ))
    }
}

impl<'a> IntoVariantData<variant::VectorVectorComplex> for &'a [Vec<num::Complex<f32>>] {
    fn into_variant_data(self) -> VariantData<'static, variant::VectorVectorComplex> {
        VariantData::new_owned(ffi::create_variant_data_from_vector_vector_complex(
            self.iter()
                .map(|item| ffi::VecComplex {
                    vec: item.iter().map(|c| c.into()).collect(),
                })
                .collect(),
        ))
    }
}

impl<'a> IntoVariantData<variant::VectorMatrixFloat> for &'a [Array2<f32>] {
    fn into_variant_data(self) -> VariantData<'static, variant::VectorMatrixFloat> {
        VariantData::new_owned(ffi::create_variant_data_from_vector_matrix_float(
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

impl<'a> IntoVariantData<variant::MapVectorFloat> for &'a HashMap<String, Vec<f32>> {
    fn into_variant_data(self) -> VariantData<'static, variant::MapVectorFloat> {
        VariantData::new_owned(ffi::create_variant_data_from_map_vector_float(
            self.iter()
                .map(|(key, vec)| ffi::MapEntryVectorFloat {
                    key: key.clone(),
                    value: vec.as_slice(),
                })
                .collect(),
        ))
    }
}

impl<'a> IntoVariantData<variant::MapVectorString> for &'a HashMap<String, Vec<String>> {
    fn into_variant_data(self) -> VariantData<'static, variant::MapVectorString> {
        VariantData::new_owned(ffi::create_variant_data_from_map_vector_string(
            self.iter()
                .map(|(key, vec)| ffi::MapEntryVectorString {
                    key: key.clone(),
                    value: vec.clone(),
                })
                .collect(),
        ))
    }
}

impl<'a> IntoVariantData<variant::MapVectorInt> for &'a HashMap<String, Vec<i32>> {
    fn into_variant_data(self) -> VariantData<'static, variant::MapVectorInt> {
        VariantData::new_owned(ffi::create_variant_data_from_map_vector_int(
            self.iter()
                .map(|(key, vec)| ffi::MapEntryVectorInt {
                    key: key.clone(),
                    value: vec.as_slice(),
                })
                .collect(),
        ))
    }
}

impl<'a> IntoVariantData<variant::MapVectorComplex>
    for &'a HashMap<String, Vec<num::Complex<f32>>>
{
    fn into_variant_data(self) -> VariantData<'static, variant::MapVectorComplex> {
        // Convert all data first to avoid lifetime issues
        let converted_data: Vec<(String, Vec<ffi::Complex>)> = self
            .iter()
            .map(|(key, vec)| (key.clone(), vec.iter().map(|c| c.into()).collect()))
            .collect();

        let entries: Vec<ffi::MapEntryVectorComplex> = converted_data
            .iter()
            .map(|(key, ffi_vec)| ffi::MapEntryVectorComplex {
                key: key.clone(),
                value: ffi_vec.as_slice(),
            })
            .collect();

        VariantData::new_owned(ffi::create_variant_data_from_map_vector_complex(entries))
    }
}

impl<'a> IntoVariantData<variant::MapFloat> for &'a HashMap<String, f32> {
    fn into_variant_data(self) -> VariantData<'static, variant::MapFloat> {
        VariantData::new_owned(ffi::create_variant_data_from_map_float(
            self.iter()
                .map(|(key, &val)| ffi::MapEntryFloat {
                    key: key.clone(),
                    value: val,
                })
                .collect(),
        ))
    }
}

impl<'a> TryIntoVariantData<variant::MatrixFloat> for &'a [Vec<f32>] {
    fn try_into_variant_data(
        self,
    ) -> Result<VariantData<'static, variant::MatrixFloat>, ConversionError> {
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

        Ok(VariantData::new_owned(
            ffi::create_variant_data_from_matrix_float(ffi::MatrixFloat {
                slice: &flat_data,
                dim1,
                dim2,
            }),
        ))
    }
}

impl IntoVariantData<variant::Pool> for crate::pool::Pool {
    fn into_variant_data(self) -> VariantData<'static, variant::Pool> {
        VariantData::new_owned(ffi::create_variant_data_from_pool(self.into_owned_ptr()))
    }
}
