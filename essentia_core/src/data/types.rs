use essentia_sys::ffi;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    Float,
    String,
    Bool,
    Int,
    UnsignedInt,
    Long,
    StereoSample,
    Complex,
    TensorFloat,
    VectorFloat,
    VectorString,
    VectorBool,
    VectorInt,
    VectorStereoSample,
    VectorComplex,
    VectorVectorFloat,
    VectorVectorString,
    VectorVectorStereoSample,
    VectorVectorComplex,
    VectorMatrixFloat,
    MapVectorFloat,
    MapVectorString,
    MapVectorInt,
    MapVectorComplex,
    MapFloat,
    MatrixFloat,
    Pool,
}

impl DataType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DataType::Float => "Float",
            DataType::String => "String",
            DataType::Bool => "Bool",
            DataType::Int => "Int",
            DataType::UnsignedInt => "UnsignedInt",
            DataType::Long => "Long",
            DataType::StereoSample => "StereoSample",
            DataType::Complex => "Complex",
            DataType::TensorFloat => "TensorFloat",
            DataType::VectorFloat => "VectorFloat",
            DataType::VectorString => "VectorString",
            DataType::VectorBool => "VectorBool",
            DataType::VectorInt => "VectorInt",
            DataType::VectorStereoSample => "VectorStereoSample",
            DataType::VectorComplex => "VectorComplex",
            DataType::VectorVectorFloat => "VectorVectorFloat",
            DataType::VectorVectorString => "VectorVectorString",
            DataType::VectorVectorStereoSample => "VectorVectorStereoSample",
            DataType::VectorVectorComplex => "VectorVectorComplex",
            DataType::VectorMatrixFloat => "VectorMatrixFloat",
            DataType::MapVectorFloat => "MapVectorFloat",
            DataType::MapVectorString => "MapVectorString",
            DataType::MapVectorInt => "MapVectorInt",
            DataType::MapVectorComplex => "MapVectorComplex",
            DataType::MapFloat => "MapFloat",
            DataType::MatrixFloat => "MatrixFloat",
            DataType::Pool => "Pool",
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<ffi::DataType> for DataType {
    fn from(ffi_type: ffi::DataType) -> Self {
        match ffi_type {
            ffi::DataType::Float => DataType::Float,
            ffi::DataType::String => DataType::String,
            ffi::DataType::Bool => DataType::Bool,
            ffi::DataType::Int => DataType::Int,
            ffi::DataType::UnsignedInt => DataType::UnsignedInt,
            ffi::DataType::Long => DataType::Long,
            ffi::DataType::StereoSample => DataType::StereoSample,
            ffi::DataType::Complex => DataType::Complex,
            ffi::DataType::TensorFloat => DataType::TensorFloat,
            ffi::DataType::VectorFloat => DataType::VectorFloat,
            ffi::DataType::VectorString => DataType::VectorString,
            ffi::DataType::VectorBool => DataType::VectorBool,
            ffi::DataType::VectorInt => DataType::VectorInt,
            ffi::DataType::VectorStereoSample => DataType::VectorStereoSample,
            ffi::DataType::VectorComplex => DataType::VectorComplex,
            ffi::DataType::VectorVectorFloat => DataType::VectorVectorFloat,
            ffi::DataType::VectorVectorString => DataType::VectorVectorString,
            ffi::DataType::VectorVectorStereoSample => DataType::VectorVectorStereoSample,
            ffi::DataType::VectorVectorComplex => DataType::VectorVectorComplex,
            ffi::DataType::VectorMatrixFloat => DataType::VectorMatrixFloat,
            ffi::DataType::MapVectorFloat => DataType::MapVectorFloat,
            ffi::DataType::MapVectorString => DataType::MapVectorString,
            ffi::DataType::MapVectorInt => DataType::MapVectorInt,
            ffi::DataType::MapVectorComplex => DataType::MapVectorComplex,
            ffi::DataType::MapFloat => DataType::MapFloat,
            ffi::DataType::MatrixFloat => DataType::MatrixFloat,
            ffi::DataType::Pool => DataType::Pool,
            _ => panic!("Encountered unknown FFI DataType: {:?}", ffi_type),
        }
    }
}

impl From<DataType> for ffi::DataType {
    fn from(data_type: DataType) -> Self {
        match data_type {
            DataType::Float => ffi::DataType::Float,
            DataType::String => ffi::DataType::String,
            DataType::Bool => ffi::DataType::Bool,
            DataType::Int => ffi::DataType::Int,
            DataType::UnsignedInt => ffi::DataType::UnsignedInt,
            DataType::Long => ffi::DataType::Long,
            DataType::StereoSample => ffi::DataType::StereoSample,
            DataType::Complex => ffi::DataType::Complex,
            DataType::TensorFloat => ffi::DataType::TensorFloat,
            DataType::VectorFloat => ffi::DataType::VectorFloat,
            DataType::VectorString => ffi::DataType::VectorString,
            DataType::VectorBool => ffi::DataType::VectorBool,
            DataType::VectorInt => ffi::DataType::VectorInt,
            DataType::VectorStereoSample => ffi::DataType::VectorStereoSample,
            DataType::VectorComplex => ffi::DataType::VectorComplex,
            DataType::VectorVectorFloat => ffi::DataType::VectorVectorFloat,
            DataType::VectorVectorString => ffi::DataType::VectorVectorString,
            DataType::VectorVectorStereoSample => ffi::DataType::VectorVectorStereoSample,
            DataType::VectorVectorComplex => ffi::DataType::VectorVectorComplex,
            DataType::VectorMatrixFloat => ffi::DataType::VectorMatrixFloat,
            DataType::MapVectorFloat => ffi::DataType::MapVectorFloat,
            DataType::MapVectorString => ffi::DataType::MapVectorString,
            DataType::MapVectorInt => ffi::DataType::MapVectorInt,
            DataType::MapVectorComplex => ffi::DataType::MapVectorComplex,
            DataType::MapFloat => ffi::DataType::MapFloat,
            DataType::MatrixFloat => ffi::DataType::MatrixFloat,
            DataType::Pool => ffi::DataType::Pool,
        }
    }
}

pub mod data_type {
    pub struct Any;

    // Scalar types
    pub struct Bool;
    pub struct String;
    pub struct Int;
    pub struct Float;
    pub struct UnsignedInt;
    pub struct Long;
    pub struct StereoSample;
    pub struct Complex;
    pub struct TensorFloat;

    // Vector types
    pub struct VectorBool;
    pub struct VectorString;
    pub struct VectorInt;
    pub struct VectorFloat;
    pub struct VectorStereoSample;
    pub struct VectorComplex;

    // Nested vector types
    pub struct VectorVectorFloat;
    pub struct VectorVectorString;
    pub struct VectorVectorStereoSample;
    pub struct VectorVectorComplex;
    pub struct VectorMatrixFloat;

    // Matrix types
    pub struct MatrixFloat;

    // Map types
    pub struct MapVectorFloat;
    pub struct MapVectorString;
    pub struct MapVectorInt;
    pub struct MapVectorComplex;
    pub struct MapFloat;

    pub struct Pool;
}

pub trait HasDataType {
    const DATA_TYPE: DataType;

    fn data_type() -> DataType {
        Self::DATA_TYPE
    }
}

impl HasDataType for data_type::Bool {
    const DATA_TYPE: DataType = DataType::Bool;
}

impl HasDataType for data_type::String {
    const DATA_TYPE: DataType = DataType::String;
}

impl HasDataType for data_type::Int {
    const DATA_TYPE: DataType = DataType::Int;
}

impl HasDataType for data_type::Float {
    const DATA_TYPE: DataType = DataType::Float;
}

impl HasDataType for data_type::UnsignedInt {
    const DATA_TYPE: DataType = DataType::UnsignedInt;
}

impl HasDataType for data_type::Long {
    const DATA_TYPE: DataType = DataType::Long;
}

impl HasDataType for data_type::StereoSample {
    const DATA_TYPE: DataType = DataType::StereoSample;
}

impl HasDataType for data_type::Complex {
    const DATA_TYPE: DataType = DataType::Complex;
}

impl HasDataType for data_type::TensorFloat {
    const DATA_TYPE: DataType = DataType::TensorFloat;
}

impl HasDataType for data_type::VectorBool {
    const DATA_TYPE: DataType = DataType::VectorBool;
}

impl HasDataType for data_type::VectorString {
    const DATA_TYPE: DataType = DataType::VectorString;
}

impl HasDataType for data_type::VectorInt {
    const DATA_TYPE: DataType = DataType::VectorInt;
}

impl HasDataType for data_type::VectorFloat {
    const DATA_TYPE: DataType = DataType::VectorFloat;
}

impl HasDataType for data_type::VectorStereoSample {
    const DATA_TYPE: DataType = DataType::VectorStereoSample;
}

impl HasDataType for data_type::VectorComplex {
    const DATA_TYPE: DataType = DataType::VectorComplex;
}

impl HasDataType for data_type::VectorVectorFloat {
    const DATA_TYPE: DataType = DataType::VectorVectorFloat;
}

impl HasDataType for data_type::VectorVectorString {
    const DATA_TYPE: DataType = DataType::VectorVectorString;
}

impl HasDataType for data_type::VectorVectorStereoSample {
    const DATA_TYPE: DataType = DataType::VectorVectorStereoSample;
}

impl HasDataType for data_type::VectorVectorComplex {
    const DATA_TYPE: DataType = DataType::VectorVectorComplex;
}

impl HasDataType for data_type::VectorMatrixFloat {
    const DATA_TYPE: DataType = DataType::VectorMatrixFloat;
}

impl HasDataType for data_type::MatrixFloat {
    const DATA_TYPE: DataType = DataType::MatrixFloat;
}

impl HasDataType for data_type::MapVectorFloat {
    const DATA_TYPE: DataType = DataType::MapVectorFloat;
}

impl HasDataType for data_type::MapVectorString {
    const DATA_TYPE: DataType = DataType::MapVectorString;
}

impl HasDataType for data_type::MapVectorInt {
    const DATA_TYPE: DataType = DataType::MapVectorInt;
}

impl HasDataType for data_type::MapVectorComplex {
    const DATA_TYPE: DataType = DataType::MapVectorComplex;
}

impl HasDataType for data_type::MapFloat {
    const DATA_TYPE: DataType = DataType::MapFloat;
}

impl HasDataType for data_type::Pool {
    const DATA_TYPE: DataType = DataType::Pool;
}
