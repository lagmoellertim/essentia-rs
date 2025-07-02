use crate::data::types::{DataType, HasDataType, phantom};

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

impl ParameterData for phantom::Float {}
impl ParameterData for phantom::String {}
impl ParameterData for phantom::Bool {}
impl ParameterData for phantom::Int {}
impl ParameterData for phantom::StereoSample {}
impl ParameterData for phantom::VectorFloat {}
impl ParameterData for phantom::VectorString {}
impl ParameterData for phantom::VectorBool {}
impl ParameterData for phantom::VectorInt {}
impl ParameterData for phantom::VectorStereoSample {}
impl ParameterData for phantom::VectorVectorFloat {}
impl ParameterData for phantom::VectorVectorString {}
impl ParameterData for phantom::VectorVectorStereoSample {}
impl ParameterData for phantom::VectorMatrixFloat {}
impl ParameterData for phantom::MapVectorFloat {}
impl ParameterData for phantom::MapVectorString {}
impl ParameterData for phantom::MapVectorInt {}
impl ParameterData for phantom::MapFloat {}
impl ParameterData for phantom::MatrixFloat {}

impl InputOutputData for phantom::Float {}
impl InputOutputData for phantom::UnsignedInt {}
impl InputOutputData for phantom::Long {}
impl InputOutputData for phantom::String {}
impl InputOutputData for phantom::Bool {}
impl InputOutputData for phantom::Int {}
impl InputOutputData for phantom::StereoSample {}
impl InputOutputData for phantom::Complex {}
impl InputOutputData for phantom::TensorFloat {}
impl InputOutputData for phantom::VectorFloat {}
impl InputOutputData for phantom::VectorString {}
impl InputOutputData for phantom::VectorBool {}
impl InputOutputData for phantom::VectorInt {}
impl InputOutputData for phantom::VectorStereoSample {}
impl InputOutputData for phantom::VectorComplex {}
impl InputOutputData for phantom::VectorVectorFloat {}
impl InputOutputData for phantom::VectorVectorString {}
impl InputOutputData for phantom::VectorVectorStereoSample {}
impl InputOutputData for phantom::VectorVectorComplex {}
impl InputOutputData for phantom::VectorMatrixFloat {}
impl InputOutputData for phantom::MapVectorFloat {}
impl InputOutputData for phantom::MapVectorString {}
impl InputOutputData for phantom::MapVectorInt {}
impl InputOutputData for phantom::MapVectorComplex {}
impl InputOutputData for phantom::MapFloat {}
impl InputOutputData for phantom::MatrixFloat {}
impl InputOutputData for phantom::Pool {}

impl PoolData for phantom::Float {}
impl PoolData for phantom::String {}
impl PoolData for phantom::StereoSample {}
impl PoolData for phantom::VectorFloat {}
impl PoolData for phantom::VectorString {}
impl PoolData for phantom::VectorStereoSample {}
impl PoolData for phantom::TensorFloat {}

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
