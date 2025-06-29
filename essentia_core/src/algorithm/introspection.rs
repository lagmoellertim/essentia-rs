use crate::{ffi, input_output::InputOutputType, parameter::ParameterType, variant_data::DataType};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AlgorithmIntrospection {
    name: String,
    category: String,
    description: String,
    input_infos: HashMap<String, InputOutputInfo>,
    output_infos: HashMap<String, InputOutputInfo>,
    parameter_infos: HashMap<String, ParameterInfo>,
}

impl AlgorithmIntrospection {
    pub fn from_algorithm_bridge(algorithm_bridge: &ffi::AlgorithmBridge) -> Self {
        let input_info = algorithm_bridge
            .get_input_infos()
            .into_iter()
            .map(|info| {
                let info: InputOutputInfo = info.into();
                (info.name.clone(), info)
            })
            .collect();

        let output_info = algorithm_bridge
            .get_output_infos()
            .into_iter()
            .map(|info| {
                let info: InputOutputInfo = info.into();
                (info.name.clone(), info)
            })
            .collect();

        let parameter_info = algorithm_bridge
            .get_parameter_infos()
            .into_iter()
            .map(|info| {
                let info: ParameterInfo = info.into();
                (info.name.clone(), info)
            })
            .collect();

        Self {
            name: algorithm_bridge.get_name(),
            category: algorithm_bridge.get_category(),
            description: algorithm_bridge.get_description(),
            input_infos: input_info,
            output_infos: output_info,
            parameter_infos: parameter_info,
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
    pub fn inputs(&self) -> impl Iterator<Item = &InputOutputInfo> {
        self.input_infos.values()
    }
    pub fn outputs(&self) -> impl Iterator<Item = &InputOutputInfo> {
        self.output_infos.values()
    }
    pub fn parameters(&self) -> impl Iterator<Item = &ParameterInfo> {
        self.parameter_infos.values()
    }

    pub fn get_parameter(&self, name: &str) -> Option<&ParameterInfo> {
        self.parameter_infos.get(name)
    }
    pub fn get_input(&self, name: &str) -> Option<&InputOutputInfo> {
        self.input_infos.get(name)
    }
    pub fn get_output(&self, name: &str) -> Option<&InputOutputInfo> {
        self.output_infos.get(name)
    }
}

#[derive(Debug, Clone)]
pub struct InputOutputInfo {
    name: String,
    input_output_type: InputOutputType,
    description: String,
}

impl InputOutputInfo {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn input_output_type(&self) -> InputOutputType {
        self.input_output_type
    }
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl From<ffi::InputOutputInfo> for InputOutputInfo {
    fn from(value: ffi::InputOutputInfo) -> Self {
        let data_type = DataType::from(value.data_type);
        let input_output_type =
            InputOutputType::try_from(data_type).expect("Invalid input/output type");

        InputOutputInfo {
            name: value.name,
            input_output_type: input_output_type,
            description: value.description,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    Any,
    PositiveReal,
    NonNegativeReal,
    IntRange { min: i32, max: i32 },
    NonNegativeInt,
    PositiveInt,
    OneOf(Vec<String>),
    Custom(String),
}

impl From<&str> for Constraint {
    fn from(s: &str) -> Self {
        if s.is_empty() {
            return Constraint::Any;
        }
        match s {
            "(0,inf)" => Constraint::PositiveReal,
            "[0,inf)" => Constraint::NonNegativeReal,
            s if s.starts_with('{') && s.ends_with('}') => Self::parse_one_of_constraint(s),
            s if s.starts_with('[') && s.ends_with(']') => {
                Self::parse_int_range_constraint(s).unwrap_or_else(|| Self::Custom(s.to_string()))
            }
            _ => Constraint::Custom(s.to_string()),
        }
    }
}

impl Constraint {
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
pub struct ParameterInfo {
    name: String,
    parameter_type: ParameterType,
    description: String,
    constraint: Constraint,
    default_value: String,
}

impl ParameterInfo {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn parameter_type(&self) -> ParameterType {
        self.parameter_type
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn constraint(&self) -> &Constraint {
        &self.constraint
    }
    pub fn default_value(&self) -> &str {
        &self.default_value
    }
    pub fn optional(&self) -> bool {
        self.default_value != ""
    }
}

impl From<ffi::ParameterInfo> for ParameterInfo {
    fn from(value: ffi::ParameterInfo) -> Self {
        let data_type = DataType::from(value.data_type);
        let parameter_type = ParameterType::try_from(data_type).expect("Invalid parameter type");

        ParameterInfo {
            name: value.name,
            parameter_type,
            description: value.description,
            constraint: value.constraint.as_str().into(),
            default_value: value.default_value,
        }
    }
}
