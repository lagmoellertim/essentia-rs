use cxx::UniquePtr;
use essentia_sys::ffi;

use crate::{
    data_container::{DataContainer, DataType, IntoDataContainer},
    pool::PoolError,
    pool_data::{PoolData, PoolDataType},
};

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

    pub fn set<T: PoolData>(&mut self, key: &str, value: impl IntoDataContainer<T>) {
        let data_container = value.into_data_container();
        self.inner
            .pin_mut()
            .set(key, data_container.into_owned_ptr());
    }

    pub fn get<T: PoolData>(&self, key: &str) -> Result<DataContainer<'static, T>, PoolError> {
        if !self.contains(key) {
            return Err(PoolError::KeyNotFound {
                key: key.to_string(),
            });
        }

        let data_container_ffi = self.inner.as_ref().unwrap().get(key)?;

        let expected_type = T::pool_data_type();
        let actual_data_type: DataType =
            data_container_ffi.as_ref().unwrap().get_data_type().into();

        let actual_pool_type = PoolDataType::try_from(actual_data_type).unwrap();

        if actual_pool_type != expected_type {
            return Err(PoolError::TypeMismatch {
                key: key.to_string(),
                expected: expected_type,
                actual: actual_pool_type,
            });
        }

        Ok(DataContainer::new_owned(data_container_ffi))
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
