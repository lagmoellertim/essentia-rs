use crate::ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    Float,
    String,
    Bool,
    Int,
    UnsignedInt,
    Long,
    StereoSample,
    VectorFloat,
    VectorString,
    VectorBool,
    VectorInt,
    VectorStereoSample,
    VectorVectorFloat,
    VectorVectorString,
    VectorVectorStereoSample,
    VectorMatrixFloat,
    MapVectorFloat,
    MapVectorString,
    MapVectorInt,
    MapFloat,
    MatrixFloat,
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
            ffi::DataType::VectorFloat => DataType::VectorFloat,
            ffi::DataType::VectorString => DataType::VectorString,
            ffi::DataType::VectorBool => DataType::VectorBool,
            ffi::DataType::VectorInt => DataType::VectorInt,
            ffi::DataType::VectorStereoSample => DataType::VectorStereoSample,
            ffi::DataType::VectorVectorFloat => DataType::VectorVectorFloat,
            ffi::DataType::VectorVectorString => DataType::VectorVectorString,
            ffi::DataType::VectorVectorStereoSample => DataType::VectorVectorStereoSample,
            ffi::DataType::VectorMatrixFloat => DataType::VectorMatrixFloat,
            ffi::DataType::MapVectorFloat => DataType::MapVectorFloat,
            ffi::DataType::MapVectorString => DataType::MapVectorString,
            ffi::DataType::MapVectorInt => DataType::MapVectorInt,
            ffi::DataType::MapFloat => DataType::MapFloat,
            ffi::DataType::MatrixFloat => DataType::MatrixFloat,
            _ => panic!("Unknown FFI DataType variant: {:?}", ffi_type),
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
            DataType::VectorFloat => ffi::DataType::VectorFloat,
            DataType::VectorString => ffi::DataType::VectorString,
            DataType::VectorBool => ffi::DataType::VectorBool,
            DataType::VectorInt => ffi::DataType::VectorInt,
            DataType::VectorStereoSample => ffi::DataType::VectorStereoSample,
            DataType::VectorVectorFloat => ffi::DataType::VectorVectorFloat,
            DataType::VectorVectorString => ffi::DataType::VectorVectorString,
            DataType::VectorVectorStereoSample => ffi::DataType::VectorVectorStereoSample,
            DataType::VectorMatrixFloat => ffi::DataType::VectorMatrixFloat,
            DataType::MapVectorFloat => ffi::DataType::MapVectorFloat,
            DataType::MapVectorString => ffi::DataType::MapVectorString,
            DataType::MapVectorInt => ffi::DataType::MapVectorInt,
            DataType::MapFloat => ffi::DataType::MapFloat,
            DataType::MatrixFloat => ffi::DataType::MatrixFloat,
        }
    }
}
