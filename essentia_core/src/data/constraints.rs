use crate::data::types::{DataType, HasDataType, data_type};

pub trait ParameterData: HasDataType {}

pub trait InputOutputData: HasDataType {}

pub trait PoolData: HasDataType {}

pub trait ValidateConstraint<T> {
    const IS_VALID: bool;

    fn validate() -> Result<(), &'static str> {
        if Self::IS_VALID {
            Ok(())
        } else {
            Err("Type constraint violation")
        }
    }
}

impl ParameterData for data_type::Float {}
impl ParameterData for data_type::String {}
impl ParameterData for data_type::Bool {}
impl ParameterData for data_type::Int {}
impl ParameterData for data_type::StereoSample {}
impl ParameterData for data_type::VectorFloat {}
impl ParameterData for data_type::VectorString {}
impl ParameterData for data_type::VectorBool {}
impl ParameterData for data_type::VectorInt {}
impl ParameterData for data_type::VectorStereoSample {}
impl ParameterData for data_type::VectorVectorFloat {}
impl ParameterData for data_type::VectorVectorString {}
impl ParameterData for data_type::VectorVectorStereoSample {}
impl ParameterData for data_type::VectorMatrixFloat {}
impl ParameterData for data_type::MapVectorFloat {}
impl ParameterData for data_type::MapVectorString {}
impl ParameterData for data_type::MapVectorInt {}
impl ParameterData for data_type::MapFloat {}
impl ParameterData for data_type::MatrixFloat {}

impl InputOutputData for data_type::Float {}
impl InputOutputData for data_type::UnsignedInt {}
impl InputOutputData for data_type::Long {}
impl InputOutputData for data_type::String {}
impl InputOutputData for data_type::Bool {}
impl InputOutputData for data_type::Int {}
impl InputOutputData for data_type::StereoSample {}
impl InputOutputData for data_type::Complex {}
impl InputOutputData for data_type::TensorFloat {}
impl InputOutputData for data_type::VectorFloat {}
impl InputOutputData for data_type::VectorString {}
impl InputOutputData for data_type::VectorBool {}
impl InputOutputData for data_type::VectorInt {}
impl InputOutputData for data_type::VectorStereoSample {}
impl InputOutputData for data_type::VectorComplex {}
impl InputOutputData for data_type::VectorVectorFloat {}
impl InputOutputData for data_type::VectorVectorString {}
impl InputOutputData for data_type::VectorVectorStereoSample {}
impl InputOutputData for data_type::VectorVectorComplex {}
impl InputOutputData for data_type::VectorMatrixFloat {}
impl InputOutputData for data_type::MapVectorFloat {}
impl InputOutputData for data_type::MapVectorString {}
impl InputOutputData for data_type::MapVectorInt {}
impl InputOutputData for data_type::MapVectorComplex {}
impl InputOutputData for data_type::MapFloat {}
impl InputOutputData for data_type::MatrixFloat {}
impl InputOutputData for data_type::Pool {}

impl PoolData for data_type::Float {}
impl PoolData for data_type::String {}
impl PoolData for data_type::StereoSample {}
impl PoolData for data_type::VectorFloat {}
impl PoolData for data_type::VectorString {}
impl PoolData for data_type::VectorStereoSample {}
impl PoolData for data_type::TensorFloat {}

pub struct ParameterConstraint<T>(std::marker::PhantomData<T>);

impl<T: ParameterData> ValidateConstraint<T> for ParameterConstraint<T> {
    const IS_VALID: bool = true;
}

pub struct InputOutputConstraint<T>(std::marker::PhantomData<T>);

impl<T: InputOutputData> ValidateConstraint<T> for InputOutputConstraint<T> {
    const IS_VALID: bool = true;
}

pub struct PoolConstraint<T>(std::marker::PhantomData<T>);

impl<T: PoolData> ValidateConstraint<T> for PoolConstraint<T> {
    const IS_VALID: bool = true;
}

pub const fn is_valid_parameter_type(data_type: DataType) -> bool {
    matches!(
        data_type,
        DataType::Float
            | DataType::String
            | DataType::Bool
            | DataType::Int
            | DataType::StereoSample
            | DataType::VectorFloat
            | DataType::VectorString
            | DataType::VectorBool
            | DataType::VectorInt
            | DataType::VectorStereoSample
            | DataType::VectorVectorFloat
            | DataType::VectorVectorString
            | DataType::VectorVectorStereoSample
            | DataType::VectorMatrixFloat
            | DataType::MapVectorFloat
            | DataType::MapVectorString
            | DataType::MapVectorInt
            | DataType::MapFloat
            | DataType::MatrixFloat
    )
}

pub const fn is_valid_input_output_type(data_type: DataType) -> bool {
    matches!(
        data_type,
        DataType::Float
            | DataType::UnsignedInt
            | DataType::Long
            | DataType::String
            | DataType::Bool
            | DataType::Int
            | DataType::StereoSample
            | DataType::Complex
            | DataType::TensorFloat
            | DataType::VectorFloat
            | DataType::VectorString
            | DataType::VectorBool
            | DataType::VectorInt
            | DataType::VectorStereoSample
            | DataType::VectorComplex
            | DataType::VectorVectorFloat
            | DataType::VectorVectorString
            | DataType::VectorVectorStereoSample
            | DataType::VectorVectorComplex
            | DataType::VectorMatrixFloat
            | DataType::MapVectorFloat
            | DataType::MapVectorString
            | DataType::MapVectorInt
            | DataType::MapVectorComplex
            | DataType::MapFloat
            | DataType::MatrixFloat
            | DataType::Pool
    )
}

pub const fn is_valid_pool_type(data_type: DataType) -> bool {
    matches!(
        data_type,
        DataType::Float
            | DataType::String
            | DataType::StereoSample
            | DataType::VectorFloat
            | DataType::VectorString
            | DataType::VectorStereoSample
            | DataType::TensorFloat
    )
}
