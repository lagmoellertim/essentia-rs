use cxx::UniquePtr;
use essentia_sys::ffi;
use thiserror::Error;

use crate::IntoDataContainer;
use crate::data::types::HasDataType;
use crate::data::{DataContainer, DataType, GetFromDataContainer, PoolData};

pub struct Pool {
    inner: UniquePtr<ffi::PoolBridge>,
}

impl Default for Pool {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn set<T>(&mut self, key: &str, value: impl IntoDataContainer<T>) -> Result<(), PoolError>
    where
        T: PoolData + HasDataType,
    {
        let data_container = value.into_data_container();

        self.inner
            .pin_mut()
            .set(key, data_container.into_owned_ptr());

        Ok(())
    }

    pub fn get<T, R>(&self, key: &str) -> Result<R, PoolError>
    where
        T: PoolData + HasDataType,
        for<'a> DataContainer<'a, T>: GetFromDataContainer<R>,
    {
        if !self.contains(key) {
            return Err(PoolError::KeyNotFound {
                key: key.to_string(),
            });
        }

        let data_container_ffi =
            self.inner
                .as_ref()
                .unwrap()
                .get(key)
                .map_err(|exception| PoolError::Internal {
                    key: key.to_string(),
                    source: exception,
                })?;

        let data_container = DataContainer::new_borrowed(data_container_ffi.as_ref().unwrap());

        // Verify type safety at runtime (backup to compile-time checks)
        let expected_type = T::data_type();
        let actual_type = data_container.data_type();

        if actual_type != expected_type {
            return Err(PoolError::TypeMismatch {
                key: key.to_string(),
                expected: expected_type,
                actual: actual_type,
            });
        }

        Ok(data_container.get())
    }

    pub fn get_container<T>(&self, key: &str) -> Result<DataContainer<'static, T>, PoolError>
    where
        T: PoolData + HasDataType,
    {
        if !self.contains(key) {
            return Err(PoolError::KeyNotFound {
                key: key.to_string(),
            });
        }

        let data_container_ffi =
            self.inner
                .as_ref()
                .unwrap()
                .get(key)
                .map_err(|exception| PoolError::Internal {
                    key: key.to_string(),
                    source: exception,
                })?;

        let data_container = DataContainer::new_owned(data_container_ffi);

        // Verify type safety
        let expected_type = T::data_type();
        let actual_type = data_container.data_type();

        if actual_type != expected_type {
            return Err(PoolError::TypeMismatch {
                key: key.to_string(),
                expected: expected_type,
                actual: actual_type,
            });
        }

        Ok(data_container)
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

#[derive(Debug, Error)]
pub enum PoolError {
    #[error("Key '{key}' not found in pool")]
    KeyNotFound { key: String },

    #[error("Type mismatch for key '{key}': expected {expected}, found {actual}")]
    TypeMismatch {
        key: String,
        expected: DataType,
        actual: DataType,
    },

    #[error("Internal error for key '{key}': {source}")]
    Internal {
        key: String,
        #[source]
        source: cxx::Exception,
    },
}
