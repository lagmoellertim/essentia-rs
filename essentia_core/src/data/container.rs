use cxx::UniquePtr;
use essentia_sys::ffi;
use std::marker::PhantomData;
use thiserror::Error;

use super::types::{DataType, HasDataType};

pub enum DataContainerInner<'a> {
    Owned(UniquePtr<ffi::DataContainer>),
    Borrowed(&'a ffi::DataContainer),
}

impl<'a> AsRef<ffi::DataContainer> for DataContainerInner<'a> {
    fn as_ref(&self) -> &ffi::DataContainer {
        match self {
            DataContainerInner::Owned(ptr) => ptr.as_ref().expect("UniquePtr should not be null"),
            DataContainerInner::Borrowed(reference) => reference,
        }
    }
}

pub struct DataContainer<'a, T> {
    pub(crate) inner: DataContainerInner<'a>,
    _marker: PhantomData<T>,
}

impl<'a, T> DataContainer<'a, T> {
    pub(crate) fn new_owned(inner: UniquePtr<ffi::DataContainer>) -> Self {
        Self {
            inner: DataContainerInner::Owned(inner),
            _marker: PhantomData,
        }
    }

    pub(crate) fn new_borrowed(inner: &'a ffi::DataContainer) -> Self {
        Self {
            inner: DataContainerInner::Borrowed(inner),
            _marker: PhantomData,
        }
    }

    pub fn into_any(self) -> DataContainer<'a, super::types::phantom::Any> {
        DataContainer {
            inner: self.inner,
            _marker: PhantomData,
        }
    }

    pub fn data_type(&self) -> DataType {
        self.inner.as_ref().get_data_type().into()
    }

    pub fn into_owned_ptr(self) -> UniquePtr<ffi::DataContainer> {
        match self.inner {
            DataContainerInner::Owned(ptr) => ptr,
            DataContainerInner::Borrowed(borrowed) => copy_to_owned(borrowed),
        }
    }
}

impl<'a, T: HasDataType> DataContainer<'a, T> {
    pub fn compile_time_data_type() -> DataType {
        T::data_type()
    }

    pub fn verify_type(&self) -> Result<(), TypeMismatchError> {
        let runtime_type = self.data_type();
        let compile_time_type = Self::compile_time_data_type();

        if runtime_type == compile_time_type {
            Ok(())
        } else {
            Err(TypeMismatchError {
                expected: compile_time_type,
                actual: runtime_type,
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
#[error("Type mismatch: expected {expected}, got {actual}")]
pub struct TypeMismatchError {
    pub expected: DataType,
    pub actual: DataType,
}

fn copy_to_owned(data: &ffi::DataContainer) -> UniquePtr<ffi::DataContainer> {
    let data_type = data.get_data_type();

    match data_type {
        ffi::DataType::Bool => {
            let value = data.get_bool().unwrap();
            ffi::create_data_container_from_bool(value)
        }
        ffi::DataType::String => {
            let value = data.get_string().unwrap();
            ffi::create_data_container_from_string(&value)
        }
        ffi::DataType::Float => {
            let value = data.get_float().unwrap();
            ffi::create_data_container_from_float(value)
        }
        ffi::DataType::Int => {
            let value = data.get_int().unwrap();
            ffi::create_data_container_from_int(value)
        }
        ffi::DataType::UnsignedInt => {
            let value = data.get_unsigned_int().unwrap();
            ffi::create_data_container_from_unsigned_int(value)
        }
        ffi::DataType::Long => {
            let value = data.get_long().unwrap();
            ffi::create_data_container_from_long(value)
        }
        ffi::DataType::StereoSample => {
            let value = data.get_stereo_sample().unwrap();
            ffi::create_data_container_from_stereo_sample(value)
        }
        ffi::DataType::VectorBool => {
            let value = data.get_vector_bool().unwrap();
            ffi::create_data_container_from_vector_bool(&value)
        }
        ffi::DataType::VectorInt => {
            let value = data.get_vector_int().unwrap();
            ffi::create_data_container_from_vector_int(value)
        }
        ffi::DataType::VectorString => {
            let strings = data.get_vector_string().unwrap();
            let str_refs: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
            ffi::create_data_container_from_vector_string(&str_refs)
        }
        ffi::DataType::VectorFloat => {
            let value = data.get_vector_float().unwrap();
            ffi::create_data_container_from_vector_float(value)
        }
        ffi::DataType::VectorStereoSample => {
            let value = data.get_vector_stereo_sample().unwrap();
            ffi::create_data_container_from_vector_stereo_sample(value)
        }
        ffi::DataType::VectorVectorFloat => {
            let value = data.get_vector_vector_float().unwrap();
            ffi::create_data_container_from_vector_vector_float(value)
        }
        ffi::DataType::MatrixFloat => {
            let value = data.get_matrix_float().unwrap();
            ffi::create_data_container_from_matrix_float(value)
        }
        ffi::DataType::VectorVectorString => {
            let value = data.get_vector_vector_string().unwrap();
            ffi::create_data_container_from_vector_vector_string(value)
        }
        ffi::DataType::VectorVectorStereoSample => {
            let value = data.get_vector_vector_stereo_sample().unwrap();
            ffi::create_data_container_from_vector_vector_stereo_sample(value)
        }
        ffi::DataType::VectorMatrixFloat => {
            let value = data.get_vector_matrix_float().unwrap();
            ffi::create_data_container_from_vector_matrix_float(value)
        }
        ffi::DataType::MapVectorFloat => {
            let value = data.get_map_vector_float().unwrap();
            ffi::create_data_container_from_map_vector_float(value)
        }
        ffi::DataType::MapVectorString => {
            let value = data.get_map_vector_string().unwrap();
            ffi::create_data_container_from_map_vector_string(value)
        }
        ffi::DataType::MapVectorInt => {
            let value = data.get_map_vector_int().unwrap();
            ffi::create_data_container_from_map_vector_int(value)
        }
        ffi::DataType::MapFloat => {
            let value = data.get_map_float().unwrap();
            ffi::create_data_container_from_map_float(value)
        }
        ffi::DataType::Pool => {
            let pool_bridge_ref = data.get_pool();
            let cloned_pool = pool_bridge_ref.clone();
            ffi::create_data_container_from_pool(cloned_pool)
        }
        data_type => {
            panic!(
                "Unsupported data type: {:?}. This indicates a bug - the Rust code is out of sync with the C++ data types.",
                data_type
            )
        }
    }
}
