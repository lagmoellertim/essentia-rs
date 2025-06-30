use cxx::UniquePtr;
use thiserror::Error;

use crate::{
    ffi,
    variant_data::{DataType, IntoVariantData, VariantData},
};

pub mod pool_data_type;
pub use pool_data_type::PoolDataType;

pub mod pool_data;
pub use pool_data::PoolData;

#[derive(Debug, Error)]
pub enum PoolError {
    #[error("Key '{key}' not found in pool")]
    KeyNotFound { key: String },

    #[error("Type mismatch for key '{key}': expected {expected}, found {actual}")]
    TypeMismatch {
        key: String,
        expected: PoolDataType,
        actual: PoolDataType,
    },

    #[error("Internal error: {0}")]
    Internal(#[from] cxx::Exception),
}

pub struct Pool {
    inner: UniquePtr<ffi::PoolBridge>,
}

impl Pool {
    pub fn new() -> Self {
        Self {
            inner: ffi::create_pool_bridge(),
        }
    }

    pub(crate) fn new_from_bridge(bridge: UniquePtr<ffi::PoolBridge>) -> Self {
        Self { inner: bridge }
    }

    pub fn set<T: PoolData>(&mut self, key: &str, value: impl IntoVariantData<T>) {
        let variant_data = value.into_variant_data();
        self.inner.pin_mut().set(key, variant_data.into_owned_ptr());
    }

    pub fn get<T: PoolData>(&self, key: &str) -> Result<VariantData<'static, T>, PoolError> {
        if !self.contains(key) {
            return Err(PoolError::KeyNotFound {
                key: key.to_string(),
            });
        }

        let variant_data = self.inner.as_ref().unwrap().get(key)?;

        let expected_type = T::pool_data_type();
        let actual_data_type: DataType = variant_data.as_ref().unwrap().get_data_type().into();

        let actual_pool_type = PoolDataType::try_from(actual_data_type).unwrap();

        if actual_pool_type != expected_type {
            return Err(PoolError::TypeMismatch {
                key: key.to_string(),
                expected: expected_type,
                actual: actual_pool_type,
            });
        }

        Ok(VariantData::new_owned(variant_data))
    }

    pub fn contains(&self, key: &str) -> bool {
        self.inner.as_ref().unwrap().contains(key)
    }

    pub fn keys(&self) -> Vec<String> {
        self.inner.as_ref().unwrap().keys()
    }

    pub fn len(&self) -> usize {
        self.keys().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub(crate) fn into_owned_ptr(self) -> UniquePtr<ffi::PoolBridge> {
        self.inner
    }
}

#[cfg(test)]
mod tests;
