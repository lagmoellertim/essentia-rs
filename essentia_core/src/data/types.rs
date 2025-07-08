use essentia_sys::ffi;
use std::fmt;

/// Macro to generate the DataType enum and its string representation
macro_rules! define_data_types {
    ($($variant:ident),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum DataType {
            $($variant,)+
        }

        impl DataType {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(DataType::$variant => stringify!($variant),)+
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
                    $(ffi::DataType::$variant => DataType::$variant,)+
                    _ => panic!("Encountered unknown FFI DataType: {:?}", ffi_type),
                }
            }
        }

        impl From<DataType> for ffi::DataType {
            fn from(data_type: DataType) -> Self {
                match data_type {
                    $(DataType::$variant => ffi::DataType::$variant,)+
                }
            }
        }
    };
}

// Define all data types using the macro
define_data_types! {
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

pub mod phantom {
    /// Macro to define phantom types
    macro_rules! define_phantom_types {
        ($($type_name:ident),+ $(,)?) => {
            pub struct Any;
            $(pub struct $type_name;)+
        };
    }

    define_phantom_types! {
        // Scalar types
        Bool,
        String,
        Int,
        Float,
        UnsignedInt,
        Long,
        StereoSample,
        Complex,
        TensorFloat,
        
        // Vector types
        VectorBool,
        VectorString,
        VectorInt,
        VectorFloat,
        VectorStereoSample,
        VectorComplex,
        
        // Nested vector types
        VectorVectorFloat,
        VectorVectorString,
        VectorVectorStereoSample,
        VectorVectorComplex,
        VectorMatrixFloat,
        
        // Matrix types
        MatrixFloat,
        
        // Map types
        MapVectorFloat,
        MapVectorString,
        MapVectorInt,
        MapVectorComplex,
        MapFloat,
        
        Pool,
    }
}

pub trait HasDataType {
    const DATA_TYPE: DataType;

    fn data_type() -> DataType {
        Self::DATA_TYPE
    }
}

/// Macro to implement HasDataType for phantom types
macro_rules! impl_has_data_type {
    ($($phantom_type:ident => $data_type:ident),+ $(,)?) => {
        $(
            impl HasDataType for phantom::$phantom_type {
                const DATA_TYPE: DataType = DataType::$data_type;
            }
        )+
    };
}

impl_has_data_type! {
    Bool => Bool,
    String => String,
    Int => Int,
    Float => Float,
    UnsignedInt => UnsignedInt,
    Long => Long,
    StereoSample => StereoSample,
    Complex => Complex,
    TensorFloat => TensorFloat,
    VectorBool => VectorBool,
    VectorString => VectorString,
    VectorInt => VectorInt,
    VectorFloat => VectorFloat,
    VectorStereoSample => VectorStereoSample,
    VectorComplex => VectorComplex,
    VectorVectorFloat => VectorVectorFloat,
    VectorVectorString => VectorVectorString,
    VectorVectorStereoSample => VectorVectorStereoSample,
    VectorVectorComplex => VectorVectorComplex,
    VectorMatrixFloat => VectorMatrixFloat,
    MatrixFloat => MatrixFloat,
    MapVectorFloat => MapVectorFloat,
    MapVectorString => MapVectorString,
    MapVectorInt => MapVectorInt,
    MapVectorComplex => MapVectorComplex,
    MapFloat => MapFloat,
    Pool => Pool,
}
