use cxx::UniquePtr;

use crate::{ffi, parameter::Parameter, variant_data::from_other::IntoVariantData};

pub struct ParameterMap {
    pub(crate) parameter_map_bridge: UniquePtr<ffi::ParameterMapBridge>,
}

impl ParameterMap {
    pub fn new() -> Self {
        Self {
            parameter_map_bridge: ffi::create_parameter_map_bridge(),
        }
    }

    pub fn set_parameter<T: Parameter>(&mut self, key: &str, value: impl IntoVariantData<T>) {
        let variant_data = value.into_variant_data();

        self.parameter_map_bridge
            .pin_mut()
            .add(key, variant_data.into_owned_ptr())
            .unwrap();
    }
}
