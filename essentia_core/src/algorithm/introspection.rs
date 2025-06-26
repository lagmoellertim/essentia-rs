use crate::ffi;
use core::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AlgorithmIntrospection {
    name: String,
    category: String,
    description: String,
    input_definitions: HashMap<String, InputOutputDefinition>,
    output_definitions: HashMap<String, InputOutputDefinition>,
    parameter_definitions: HashMap<String, ParameterDefinition>,
}

impl AlgorithmIntrospection {
    pub fn from_algorithm(ffi_bridge: &ffi::AlgorithmBridge) -> Self {
        let input_info = ffi_bridge
            .get_all_input_info()
            .into_iter()
            .map(|info| {
                let info: InputOutputDefinition = info.into();
                (info.name.clone(), info)
            })
            .collect();

        let output_info = ffi_bridge
            .get_all_output_info()
            .into_iter()
            .map(|info| {
                let info: InputOutputDefinition = info.into();
                (info.name.clone(), info)
            })
            .collect();

        let parameter_info = ffi_bridge
            .get_all_parameter_info()
            .into_iter()
            .map(|info| {
                let info: ParameterDefinition = info.into();
                (info.name.clone(), info)
            })
            .collect();

        Self {
            name: ffi_bridge.get_algorithm_name(),
            category: ffi_bridge.get_algorithm_category(),
            description: ffi_bridge.get_algorithm_description(),
            input_definitions: input_info,
            output_definitions: output_info,
            parameter_definitions: parameter_info,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn category(&self) -> &str {
        &self.category
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn inputs(&self) -> impl Iterator<Item = &InputOutputDefinition> {
        self.input_definitions.values()
    }
    pub fn outputs(&self) -> impl Iterator<Item = &InputOutputDefinition> {
        self.output_definitions.values()
    }
    pub fn parameters(&self) -> impl Iterator<Item = &ParameterDefinition> {
        self.parameter_definitions.values()
    }

    pub fn get_parameter(&self, name: &str) -> Option<&ParameterDefinition> {
        self.parameter_definitions.get(name)
    }
    pub fn get_input(&self, name: &str) -> Option<&InputOutputDefinition> {
        self.input_definitions.get(name)
    }
    pub fn get_output(&self, name: &str) -> Option<&InputOutputDefinition> {
        self.output_definitions.get(name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputOutputType {
    Real,
    Int,
    UnsignedInt,
    Long,
    VectorReal,
    VectorVectorReal,
    MatrixReal,
}

impl fmt::Display for InputOutputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone)]
pub struct InputOutputDefinition {
    name: String,
    data_type: InputOutputType,
    description: String,
}

impl InputOutputDefinition {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn data_type(&self) -> InputOutputType {
        self.data_type
    }
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl From<ffi::IOType> for InputOutputType {
    fn from(value: ffi::IOType) -> Self {
        match value {
            ffi::IOType::Real => InputOutputType::Real,
            ffi::IOType::Int => InputOutputType::Int,
            ffi::IOType::UnsignedInt => InputOutputType::UnsignedInt,
            ffi::IOType::Long => InputOutputType::Long,
            ffi::IOType::VectorReal => InputOutputType::VectorReal,
            ffi::IOType::VectorVectorReal => InputOutputType::VectorVectorReal,
            ffi::IOType::MatrixReal => InputOutputType::MatrixReal,
            _ => panic!("Encountered invalid IO type from C++ side: {:?}", value),
        }
    }
}

impl From<InputOutputType> for ffi::IOType {
    fn from(value: InputOutputType) -> Self {
        match value {
            InputOutputType::Real => ffi::IOType::Real,
            InputOutputType::Int => ffi::IOType::Int,
            InputOutputType::UnsignedInt => ffi::IOType::UnsignedInt,
            InputOutputType::Long => ffi::IOType::Long,
            InputOutputType::VectorReal => ffi::IOType::VectorReal,
            InputOutputType::VectorVectorReal => ffi::IOType::VectorVectorReal,
            InputOutputType::MatrixReal => ffi::IOType::MatrixReal,
        }
    }
}

impl From<ffi::IOInfo> for InputOutputDefinition {
    fn from(value: ffi::IOInfo) -> Self {
        InputOutputDefinition {
            name: value.name,
            data_type: value.type_.into(),
            description: value.description,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParameterType {
    Real,
    String,
    Bool,
    Int,
    StereoSample,
    VectorReal,
    VectorString,
    VectorBool,
    VectorInt,
    VectorStereoSample,
    VectorVectorReal,
    VectorVectorString,
    VectorVectorStereoSample,
    VectorMatrixReal,
    MapVectorReal,
    MapVectorString,
    MapVectorInt,
    MapReal,
    MatrixReal,
}

impl From<ffi::ParameterType> for ParameterType {
    fn from(value: ffi::ParameterType) -> Self {
        match value {
            ffi::ParameterType::Real => ParameterType::Real,
            ffi::ParameterType::String => ParameterType::String,
            ffi::ParameterType::Bool => ParameterType::Bool,
            ffi::ParameterType::Int => ParameterType::Int,
            ffi::ParameterType::StereoSample => ParameterType::StereoSample,
            ffi::ParameterType::VectorReal => ParameterType::VectorReal,
            ffi::ParameterType::VectorString => ParameterType::VectorString,
            ffi::ParameterType::VectorBool => ParameterType::VectorBool,
            ffi::ParameterType::VectorInt => ParameterType::VectorInt,
            ffi::ParameterType::VectorStereoSample => ParameterType::VectorStereoSample,
            ffi::ParameterType::VectorVectorReal => ParameterType::VectorVectorReal,
            ffi::ParameterType::VectorVectorString => ParameterType::VectorVectorString,
            ffi::ParameterType::VectorVectorStereoSample => ParameterType::VectorVectorStereoSample,
            ffi::ParameterType::VectorMatrixReal => ParameterType::VectorMatrixReal,
            ffi::ParameterType::MapVectorReal => ParameterType::MapVectorReal,
            ffi::ParameterType::MapVectorString => ParameterType::MapVectorString,
            ffi::ParameterType::MapVectorInt => ParameterType::MapVectorInt,
            ffi::ParameterType::MapReal => ParameterType::MapReal,
            ffi::ParameterType::MatrixReal => ParameterType::MatrixReal,
            _ => panic!(
                "Encountered invalid parameter type from C++ side: {:?}",
                value
            ),
        }
    }
}

impl From<ParameterType> for ffi::ParameterType {
    fn from(value: ParameterType) -> Self {
        match value {
            ParameterType::Real => ffi::ParameterType::Real,
            ParameterType::String => ffi::ParameterType::String,
            ParameterType::Bool => ffi::ParameterType::Bool,
            ParameterType::Int => ffi::ParameterType::Int,
            ParameterType::StereoSample => ffi::ParameterType::StereoSample,
            ParameterType::VectorReal => ffi::ParameterType::VectorReal,
            ParameterType::VectorString => ffi::ParameterType::VectorString,
            ParameterType::VectorBool => ffi::ParameterType::VectorBool,
            ParameterType::VectorInt => ffi::ParameterType::VectorInt,
            ParameterType::VectorStereoSample => ffi::ParameterType::VectorStereoSample,
            ParameterType::VectorVectorReal => ffi::ParameterType::VectorVectorReal,
            ParameterType::VectorVectorString => ffi::ParameterType::VectorVectorString,
            ParameterType::VectorVectorStereoSample => ffi::ParameterType::VectorVectorStereoSample,
            ParameterType::VectorMatrixReal => ffi::ParameterType::VectorMatrixReal,
            ParameterType::MapVectorReal => ffi::ParameterType::MapVectorReal,
            ParameterType::MapVectorString => ffi::ParameterType::MapVectorString,
            ParameterType::MapVectorInt => ffi::ParameterType::MapVectorInt,
            ParameterType::MapReal => ffi::ParameterType::MapReal,
            ParameterType::MatrixReal => ffi::ParameterType::MatrixReal,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterConstraint {
    Any,
    PositiveReal,
    NonNegativeReal,
    IntRange { min: i32, max: i32 },
    NonNegativeInt,
    PositiveInt,
    OneOf(Vec<String>),
    Custom(String),
}

impl From<&str> for ParameterConstraint {
    fn from(s: &str) -> Self {
        if s.is_empty() {
            return ParameterConstraint::Any;
        }
        match s {
            "(0,inf)" => ParameterConstraint::PositiveReal,
            "[0,inf)" => ParameterConstraint::NonNegativeReal,
            s if s.starts_with('{') && s.ends_with('}') => Self::parse_one_of_constraint(s),
            s if s.starts_with('[') && s.ends_with(']') => {
                Self::parse_int_range_constraint(s).unwrap_or_else(|| Self::Custom(s.to_string()))
            }
            _ => ParameterConstraint::Custom(s.to_string()),
        }
    }
}

impl ParameterConstraint {
    fn parse_one_of_constraint(s: &str) -> Self {
        let inner = &s[1..s.len() - 1];
        let values = inner.split(',').map(|v| v.trim().to_string()).collect();
        Self::OneOf(values)
    }
    fn parse_int_range_constraint(s: &str) -> Option<Self> {
        let inner = &s[1..s.len() - 1];
        let (min_str, max_str) = inner.split_once(',')?;
        let min = min_str.trim().parse::<i32>().ok()?;
        let max = max_str.trim().parse::<i32>().ok()?;
        Some(Self::IntRange { min, max })
    }
}

#[derive(Debug, Clone)]
pub struct ParameterDefinition {
    name: String,
    data_type: ParameterType,
    description: String,
    constraint: ParameterConstraint,
    default_value: String,
}

impl ParameterDefinition {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn type_(&self) -> ParameterType {
        self.data_type
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn constraint(&self) -> &ParameterConstraint {
        &self.constraint
    }
    pub fn default_value(&self) -> &str {
        &self.default_value
    }
    pub fn optional(&self) -> bool {
        self.default_value != ""
    }
}

impl From<ffi::ParameterInfo> for ParameterDefinition {
    fn from(value: ffi::ParameterInfo) -> Self {
        ParameterDefinition {
            name: value.name,
            data_type: value.type_.into(),
            description: value.description,
            constraint: value.range.as_str().into(),
            default_value: value.default_value,
        }
    }
}
