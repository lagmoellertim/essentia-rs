use cxx::UniquePtr;
use essentia_sys::ffi;

use crate::data::DataContainer;

pub struct ParameterMap {
    pub(crate) parameter_map_bridge: UniquePtr<ffi::ParameterMapBridge>,
}

impl ParameterMap {
    pub fn new() -> Self {
        Self {
            parameter_map_bridge: ffi::create_parameter_map_bridge(),
        }
    }

    pub fn set_parameter<T>(&mut self, key: &str, value: DataContainer<'static, T>) {
        self.parameter_map_bridge
            .pin_mut()
            .add(key, value.into_owned_ptr())
            .unwrap();
    }
}
