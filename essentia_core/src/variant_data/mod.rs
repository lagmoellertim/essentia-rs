use std::marker::PhantomData;
use thiserror::Error;

use cxx::UniquePtr;

use crate::ffi;

pub mod data_type;
pub mod from_other;
pub mod into_other;
pub mod variants;

pub use data_type::DataType;
pub use from_other::{IntoVariantData, TryIntoVariantData};
pub use variants::*;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("matrix data is not rectangular: row {row} has {actual} elements, expected {expected}")]
    NonRectangular {
        row: usize,
        expected: usize,
        actual: usize,
    },

    #[error("matrix cannot be empty")]
    EmptyMatrix,

    #[error("matrix rows cannot be empty")]
    EmptyRows,
}

pub enum VariantDataContainer<'a> {
    Owned(UniquePtr<ffi::VariantData>),
    Borrowed(&'a ffi::VariantData),
}

pub struct VariantData<'a, T> {
    data: VariantDataContainer<'a>,
    _marker: PhantomData<T>,
}

impl<'a, T> VariantData<'a, T> {
    pub(crate) fn new_owned(inner: UniquePtr<ffi::VariantData>) -> Self {
        Self {
            data: VariantDataContainer::Owned(inner),
            _marker: PhantomData,
        }
    }

    pub(crate) fn new_borrowed(inner: &'a ffi::VariantData) -> Self {
        Self {
            data: VariantDataContainer::Borrowed(inner),
            _marker: PhantomData,
        }
    }

    pub fn into_any(self) -> VariantData<'a, variant::Any> {
        VariantData {
            data: self.data,
            _marker: PhantomData,
        }
    }

    pub fn data_type(&self) -> DataType {
        self.data.as_ref().get_data_type().into()
    }

    pub fn into_owned_ptr(self) -> UniquePtr<ffi::VariantData> {
        match self.data {
            VariantDataContainer::Owned(ptr) => ptr,
            VariantDataContainer::Borrowed(borrowed) => copy_to_owned(borrowed),
        }
    }
}

impl<'a> AsRef<ffi::VariantData> for VariantDataContainer<'a> {
    fn as_ref(&self) -> &ffi::VariantData {
        match self {
            VariantDataContainer::Owned(ptr) => ptr.as_ref().expect("UniquePtr should not be null"),
            VariantDataContainer::Borrowed(reference) => reference,
        }
    }
}

fn copy_to_owned(data: &ffi::VariantData) -> UniquePtr<ffi::VariantData> {
    match data.get_data_type() {
        ffi::DataType::Bool => {
            let value = data.get_bool().unwrap();
            ffi::create_variant_data_from_bool(value)
        }
        ffi::DataType::String => {
            let value = data.get_string().unwrap();
            ffi::create_variant_data_from_string(&value)
        }
        ffi::DataType::Float => {
            let value = data.get_float().unwrap();
            ffi::create_variant_data_from_float(value)
        }
        ffi::DataType::Int => {
            let value = data.get_int().unwrap();
            ffi::create_variant_data_from_int(value)
        }
        ffi::DataType::UnsignedInt => {
            let value = data.get_unsigned_int().unwrap();
            ffi::create_variant_data_from_unsigned_int(value)
        }
        ffi::DataType::Long => {
            let value = data.get_long().unwrap();
            ffi::create_variant_data_from_long(value)
        }
        ffi::DataType::StereoSample => {
            let value = data.get_stereo_sample().unwrap();
            ffi::create_variant_data_from_stereo_sample(value)
        }
        ffi::DataType::VectorBool => {
            let value = data.get_vector_bool().unwrap();
            ffi::create_variant_data_from_vector_bool(&value)
        }
        ffi::DataType::VectorInt => {
            let value = data.get_vector_int().unwrap();
            ffi::create_variant_data_from_vector_int(value)
        }
        ffi::DataType::VectorString => {
            let strings = data.get_vector_string().unwrap();
            let str_refs: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
            ffi::create_variant_data_from_vector_string(&str_refs)
        }
        ffi::DataType::VectorFloat => {
            let value = data.get_vector_float().unwrap();
            ffi::create_variant_data_from_vector_float(value)
        }
        ffi::DataType::VectorStereoSample => {
            let value = data.get_vector_stereo_sample().unwrap();
            ffi::create_variant_data_from_vector_stereo_sample(value)
        }
        ffi::DataType::VectorVectorFloat => {
            let value = data.get_vector_vector_float().unwrap();
            ffi::create_variant_data_from_vector_vector_float(value)
        }
        ffi::DataType::MatrixFloat => {
            let value = data.get_matrix_float().unwrap();
            ffi::create_variant_data_from_matrix_float(value)
        }
        ffi::DataType::VectorVectorString => {
            let value = data.get_vector_vector_string().unwrap();
            ffi::create_variant_data_from_vector_vector_string(value)
        }
        ffi::DataType::VectorVectorStereoSample => {
            let value = data.get_vector_vector_stereo_sample().unwrap();
            ffi::create_variant_data_from_vector_vector_stereo_sample(value)
        }
        ffi::DataType::VectorMatrixFloat => {
            let value = data.get_vector_matrix_float().unwrap();
            ffi::create_variant_data_from_vector_matrix_float(value)
        }
        ffi::DataType::MapVectorFloat => {
            let value = data.get_map_vector_float().unwrap();
            ffi::create_variant_data_from_map_vector_float(value)
        }
        ffi::DataType::MapVectorString => {
            let value = data.get_map_vector_string().unwrap();
            ffi::create_variant_data_from_map_vector_string(value)
        }
        ffi::DataType::MapVectorInt => {
            let value = data.get_map_vector_int().unwrap();
            ffi::create_variant_data_from_map_vector_int(value)
        }
        ffi::DataType::MapFloat => {
            let value = data.get_map_float().unwrap();
            ffi::create_variant_data_from_map_float(value)
        }
        unsupported => {
            panic!(
                "Unsupported data type: {:?}. This indicates a bug - the Rust code is out of sync with the C++ data types.",
                unsupported
            )
        }
    }
}
