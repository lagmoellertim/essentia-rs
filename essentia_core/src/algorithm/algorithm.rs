use std::marker::PhantomData;

use cxx::UniquePtr;
use essentia_sys::ffi;

use crate::{
    algorithm::{
        ComputeError, ConfigurationError, InputError, Introspection, OutputError, ParameterError,
        ResetError,
    },
    data_container::{DataContainer, DataType, TryIntoDataContainer},
    essentia::Essentia,
    input_output::InputOutput,
    parameter::Parameter,
};

pub struct Initialized {
    parameter_map: ParameterMap,
}

pub struct Configured;

pub struct Algorithm<'a, State> {
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

    pub fn parameter<T: Parameter>(
        mut self,
        key: &str,
        value: impl TryIntoDataContainer<T>,
    ) -> Result<Self, ParameterError> {
        self.set_parameter(key, value)?;
        Ok(self)
    }

    pub fn set_parameter<T: Parameter>(
        &mut self,
        key: &str,
        value: impl TryIntoDataContainer<T>,
    ) -> Result<(), ParameterError> {
        let param_info = self.introspection.get_parameter(key).ok_or_else(|| {
            ParameterError::ParameterNotFound {
                parameter: key.to_string(),
            }
        })?;

        if param_info.parameter_type() != T::parameter_type() {
            return Err(ParameterError::TypeMismatch {
                parameter: key.to_string(),
                expected: T::parameter_type(),
                actual: param_info.parameter_type(),
            });
        }

        let variant_data =
            value
                .try_into_data_container()
                .map_err(|error| ParameterError::DataConversion {
                    parameter: key.to_string(),
                    source: error,
                })?;

        self.state.parameter_map.set_parameter(key, variant_data);

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
    pub fn input<T: InputOutput>(
        mut self,
        key: &str,
        value: impl TryIntoDataContainer<T>,
    ) -> Result<Self, InputError> {
        self.set_input(key, value)?;
        Ok(self)
    }

    pub fn set_input<T: InputOutput>(
        &mut self,
        key: &str,
        value: impl TryIntoDataContainer<T>,
    ) -> Result<(), InputError> {
        let input_info =
            self.introspection
                .get_input(key)
                .ok_or_else(|| InputError::InputNotFound {
                    input: key.to_string(),
                })?;

        if input_info.input_output_type() != T::input_output_type() {
            return Err(InputError::TypeMismatch {
                input: key.to_string(),
                expected: T::input_output_type(),
                actual: input_info.input_output_type(),
            });
        }

        let variant_data =
            value
                .try_into_data_container()
                .map_err(|error| InputError::DataConversion {
                    input: key.to_string(),
                    source: error,
                })?;

        let owned_ptr = variant_data.into_owned_ptr();

        self.algorithm_bridge
            .pin_mut()
            .set_input(key, owned_ptr)
            .map_err(|exception| InputError::Internal {
                input: key.to_string(),
                source: exception,
            })?;

        Ok(())
    }

    pub fn compute(&mut self) -> Result<ComputeResult<'a, '_>, ComputeError> {
        for output in self.introspection.outputs() {
            let data_type = DataType::from(output.input_output_type());

            self.algorithm_bridge
                .pin_mut()
                .setup_output(output.name(), data_type.into())
                .map_err(|exception| ComputeError::OutputSetup {
                    output: output.name().to_string(),
                    source: exception,
                })?;
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
        T: InputOutput,
    {
        let output_info = self
            .algorithm
            .introspection
            .get_output(key)
            .ok_or_else(|| OutputError::OutputNotFound {
                output: key.to_string(),
            })?;

        if output_info.input_output_type() != T::input_output_type() {
            return Err(OutputError::TypeMismatch {
                output: key.to_string(),
                expected: T::input_output_type(),
                actual: output_info.input_output_type(),
            });
        }

        let variant_data = self
            .algorithm
            .algorithm_bridge
            .get_output(key)
            .map(|ffi_variant_data| DataContainer::new_borrowed(ffi_variant_data))
            .map_err(|exception| OutputError::Internal {
                output: key.to_string(),
                source: exception,
            })?;

        Ok(variant_data)
    }
}
