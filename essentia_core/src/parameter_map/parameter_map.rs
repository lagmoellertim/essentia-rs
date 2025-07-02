use cxx::UniquePtr;
use essentia_sys::ffi;

use crate::{data_container::IntoDataContainer, parameter::Parameter};

pub struct ParameterMap {
    pub(crate) parameter_map_bridge: UniquePtr<ffi::ParameterMapBridge>,
}

impl ParameterMap {
    pub fn new() -> Self {
        Self {
            parameter_map_bridge: ffi::create_parameter_map_bridge(),
        }
    }

    pub fn set_parameter<T: Parameter>(&mut self, key: &str, value: impl IntoDataContainer<T>) {
        let data_container = value.into_data_container();

        self.parameter_map_bridge
            .pin_mut()
            .add(key, data_container.into_owned_ptr())
            .unwrap();
    }
}
