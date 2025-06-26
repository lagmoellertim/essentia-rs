use std::marker::PhantomData;

use cxx::UniquePtr;
use thiserror::Error;

use crate::{
    essentia::Essentia,
    ffi,
    input_value::InputValue,
    output_value::{OutputConvertError, OutputValue},
    parameter_map::Parameters,
};

mod introspection;

pub use introspection::*;

pub struct Initialized;
pub struct Configured;

pub struct Algorithm<'a, State> {
    ffi_bridge: UniquePtr<ffi::AlgorithmBridge>,
    introspection: AlgorithmIntrospection,
    _marker: PhantomData<(&'a Essentia, State)>,
}

impl<'a> Algorithm<'a, Initialized> {
    pub(crate) fn new(algorithm: UniquePtr<ffi::AlgorithmBridge>) -> Self {
        let introspection = AlgorithmIntrospection::from_algorithm(&algorithm);

        Self {
            ffi_bridge: algorithm,
            introspection,
            _marker: PhantomData,
        }
    }

    pub fn configure(mut self, parameter_map: &Parameters) -> Algorithm<'a, Configured> {
        self.ffi_bridge.pin_mut().configure(&parameter_map.inner);

        Algorithm {
            ffi_bridge: self.ffi_bridge,
            introspection: self.introspection,
            _marker: PhantomData,
        }
    }
}

impl<'a> Algorithm<'a, Configured> {
    pub fn input<T: Into<InputValue<'a>>>(&mut self, input_name: &str, value: T) -> &mut Self {
        value
            .into()
            .set_as_input(self.ffi_bridge.pin_mut(), input_name);

        self
    }

    pub fn compute(&mut self) -> ComputeResult<'a, '_> {
        for output in self.introspection.outputs() {
            self.ffi_bridge
                .pin_mut()
                .setup_output(output.name(), output.data_type().into());
        }

        self.ffi_bridge.pin_mut().compute();

        ComputeResult { algorithm: self }
    }

    pub fn reset(&mut self) {
        self.ffi_bridge.pin_mut().reset();
    }
}

#[derive(Debug, Error)]
pub enum OutputRetrievalError {
    #[error("no output named `{0}` found")]
    InvalidOutput(String),

    #[error(transparent)]
    Conversion(#[from] OutputConvertError),
}

pub struct ComputeResult<'algorithm, 'result> {
    algorithm: &'result Algorithm<'algorithm, Configured>,
}

impl<'algorithm, 'result> ComputeResult<'algorithm, 'result> {
    pub fn get<T: TryFrom<OutputValue<'result>, Error = OutputConvertError>>(
        &self,
        output_name: &str,
    ) -> Result<T, OutputRetrievalError> {
        let output_type = self
            .algorithm
            .introspection
            .get_output(output_name)
            .ok_or(OutputRetrievalError::InvalidOutput(output_name.to_string()))?
            .data_type();

        let result =
            OutputValue::get_from_output(&self.algorithm.ffi_bridge, output_name, output_type)
                .try_into()?;

        Ok(result)
    }
}
