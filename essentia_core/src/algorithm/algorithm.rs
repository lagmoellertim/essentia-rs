use cxx::UniquePtr;
use essentia_sys::ffi;
use std::marker::PhantomData;

use crate::{
    IntoDataContainer,
    algorithm::{
        ComputeError, ConfigurationError, InputError, Introspection, OutputError, ParameterError,
        ResetError,
    },
    data::{DataContainer, InputOutputData, ParameterData, types::HasDataType},
    essentia::Essentia,
    parameter_map::ParameterMap,
};

pub struct Initialized {
    parameter_map: ParameterMap,
}

pub struct Configured;

pub struct Algorithm<'a, State = Initialized> {
    algorithm_bridge: UniquePtr<ffi::AlgorithmBridge>,
    state: State,
    introspection: Introspection,
    _marker: PhantomData<&'a Essentia>,
}

impl<'a, State> Algorithm<'a, State> {
    pub fn introspection(&self) -> &Introspection {
        &self.introspection
    }
}

impl<'a> Algorithm<'a, Initialized> {
    pub(crate) fn new(algorithm_bridge: UniquePtr<ffi::AlgorithmBridge>) -> Self {
        let introspection = Introspection::from_algorithm_bridge(&algorithm_bridge);

        Self {
            algorithm_bridge,
            state: Initialized {
                parameter_map: ParameterMap::new(),
            },
            introspection,
            _marker: PhantomData,
        }
    }

    pub fn parameter<T>(
        mut self,
        key: &str,
        value: impl IntoDataContainer<T>,
    ) -> Result<Self, ParameterError>
    where
        T: ParameterData + HasDataType,
    {
        self.set_parameter(key, value)?;
        Ok(self)
    }

    pub fn set_parameter<T>(
        &mut self,
        key: &str,
        value: impl IntoDataContainer<T>,
    ) -> Result<(), ParameterError>
    where
        T: ParameterData + HasDataType,
    {
        let param_info = self.introspection.get_parameter(key).ok_or_else(|| {
            ParameterError::ParameterNotFound {
                parameter: key.to_string(),
            }
        })?;

        let expected_type = T::data_type();
        let param_data_type = param_info.parameter_type();

        if param_data_type != expected_type {
            return Err(ParameterError::TypeMismatch {
                parameter: key.to_string(),
                expected: expected_type,
                actual: param_data_type,
            });
        }

        let data_container = value.into_data_container();

        self.state.parameter_map.set_parameter(key, data_container);

        Ok(())
    }

    pub fn configure(mut self) -> Result<Algorithm<'a, Configured>, ConfigurationError> {
        self.algorithm_bridge
            .pin_mut()
            .configure(self.state.parameter_map.parameter_map_bridge)?;

        Ok(Algorithm {
            algorithm_bridge: self.algorithm_bridge,
            state: Configured,
            introspection: self.introspection,
            _marker: PhantomData,
        })
    }
}

impl<'a> Algorithm<'a, Configured> {
    pub fn input<T>(
        mut self,
        key: &str,
        value: impl IntoDataContainer<T>,
    ) -> Result<Self, InputError>
    where
        T: InputOutputData + HasDataType,
    {
        self.set_input(key, value)?;
        Ok(self)
    }

    pub fn set_input<T>(
        &mut self,
        key: &str,
        value: impl IntoDataContainer<T>,
    ) -> Result<(), InputError>
    where
        T: InputOutputData + HasDataType,
    {
        let input_info =
            self.introspection
                .get_input(key)
                .ok_or_else(|| InputError::InputNotFound {
                    input: key.to_string(),
                })?;

        let expected_type = T::data_type();
        let input_data_type = input_info.input_output_type();

        if input_data_type != expected_type {
            return Err(InputError::TypeMismatch {
                input: key.to_string(),
                expected: expected_type,
                actual: input_data_type,
            });
        }

        let data_container = value.into_data_container();

        let owned_ptr = data_container.into_owned_ptr();

        self.algorithm_bridge
            .pin_mut()
            .set_input(key, owned_ptr)
            .expect(&format!("failed to set input '{}' after validation", key));

        Ok(())
    }

    pub fn compute(&mut self) -> Result<ComputeResult<'a, '_>, ComputeError> {
        for output in self.introspection.outputs() {
            let data_type = output.input_output_type();

            self.algorithm_bridge
                .pin_mut()
                .setup_output(output.name(), data_type.into())
                .expect(&format!(
                    "failed to setup output '{}' after validation",
                    &output.name()
                ));
        }

        self.algorithm_bridge
            .pin_mut()
            .compute()
            .map_err(ComputeError::Compute)?;

        Ok(ComputeResult { algorithm: self })
    }

    pub fn reset(&mut self) -> Result<(), ResetError> {
        self.algorithm_bridge
            .pin_mut()
            .reset()
            .map_err(ResetError::Internal)
    }
}

pub struct ComputeResult<'algorithm, 'result> {
    algorithm: &'result Algorithm<'algorithm, Configured>,
}

impl<'algorithm, 'result> ComputeResult<'algorithm, 'result> {
    pub fn output<T>(&self, key: &str) -> Result<DataContainer<'result, T>, OutputError>
    where
        T: InputOutputData + HasDataType,
    {
        let output_info = self
            .algorithm
            .introspection
            .get_output(key)
            .ok_or_else(|| OutputError::OutputNotFound {
                output: key.to_string(),
            })?;

        let expected_type = T::data_type();
        let output_data_type = output_info.input_output_type();

        if output_data_type != expected_type {
            return Err(OutputError::TypeMismatch {
                output: key.to_string(),
                expected: expected_type,
                actual: output_data_type,
            });
        }

        let data_container = self
            .algorithm
            .algorithm_bridge
            .get_output(key)
            .map(|ffi_data_container| DataContainer::new_borrowed(ffi_data_container))
            .expect(&format!("failed to get output '{}' after validation", key));

        Ok(data_container)
    }
}
