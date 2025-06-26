use std::pin::Pin;

use cxx::UniquePtr;

use crate::ffi::{self, ParameterMapBridge};

pub trait ParameterValue {
    fn set_parameter(&self, parameter_map: Pin<&mut ParameterMapBridge>, key: &str);
}

impl ParameterValue for &str {
    fn set_parameter(&self, parameter_map: Pin<&mut ParameterMapBridge>, key: &str) {
        parameter_map.add_string(key, self);
    }
}

impl ParameterValue for f32 {
    fn set_parameter(&self, parameter_map: Pin<&mut ParameterMapBridge>, key: &str) {
        parameter_map.add_real(key, *self);
    }
}

impl ParameterValue for i32 {
    fn set_parameter(&self, parameter_map: Pin<&mut ParameterMapBridge>, key: &str) {
        parameter_map.add_int(key, *self);
    }
}

impl ParameterValue for bool {
    fn set_parameter(&self, parameter_map: Pin<&mut ParameterMapBridge>, key: &str) {
        parameter_map.add_bool(key, *self);
    }
}

pub struct Parameters {
    pub(crate) inner: UniquePtr<ffi::ParameterMapBridge>,
}

impl Parameters {
    pub fn new() -> Self {
        let parameter_map = ffi::create_parameter_map();
        Self {
            inner: parameter_map,
        }
    }

    pub fn add_parameter(&mut self, key: &str, value: impl ParameterValue) {
        let parameter_map = self.inner.pin_mut();

        value.set_parameter(parameter_map, key);
    }
}
