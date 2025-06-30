#[cxx::bridge(namespace = "essentia_bridge")]
pub mod bridge {
    // ===== Helper Structs =====
    struct SliceFloat<'a> {
        slice: &'a [f32],
    }

    struct VecString {
        vec: Vec<String>,
    }

    struct SliceStereoSample<'a> {
        slice: &'a [StereoSample],
    }

    struct MatrixFloat<'a> {
        slice: &'a [f32],
        dim1: usize,
        dim2: usize,
    }

    struct TensorFloat<'a> {
        slice: &'a [f32],
        shape: &'a [usize],
    }

    struct MapEntryVectorFloat<'a> {
        key: String,
        value: &'a [f32],
    }

    struct MapEntryVectorString {
        key: String,
        value: Vec<String>,
    }

    struct MapEntryVectorInt<'a> {
        key: String,
        value: &'a [i32],
    }

    struct MapEntryFloat {
        key: String,
        value: f32,
    }

    #[derive(Clone, Debug)]
    struct StereoSample {
        left: f32,
        right: f32,
    }

    #[derive(Clone, Debug)]
    struct Complex {
        real: f32,
        imag: f32,
    }

    struct SliceComplex<'a> {
        slice: &'a [Complex],
    }

    struct VecComplex {
        vec: Vec<Complex>,
    }

    struct MapEntryVectorComplex<'a> {
        key: String,
        value: &'a [Complex],
    }

    // ===== Data Type Enum =====
    #[derive(Debug, Clone, Copy)]
    enum DataType {
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

    // ===== Introspection Structs =====
    struct ParameterInfo {
        name: String,
        data_type: DataType,
        constraint: String,
        description: String,
        default_value: String,
    }

    struct InputOutputInfo {
        name: String,
        data_type: DataType,
        description: String,
    }

    // ===== C++ Bridge =====
    unsafe extern "C++" {
        include!("bridge/bridge.h");

        // ===== Core types =====
        type AlgorithmBridge;
        type ParameterMapBridge;
        type PoolBridge;
        type VariantData;

        // ===== Essentia Initialization =====
        fn init_essentia();
        fn shutdown_essentia();

        // ===== Algorithm Bridge Creation =====
        fn get_algorithm_names() -> Vec<String>;
        fn create_algorithm_bridge(name: &str) -> Result<UniquePtr<AlgorithmBridge>>;

        // ===== Algorithm Bridge Introspection =====
        fn get_name(self: &AlgorithmBridge) -> String;
        fn get_category(self: &AlgorithmBridge) -> String;
        fn get_description(self: &AlgorithmBridge) -> String;
        fn get_parameter_infos(self: &AlgorithmBridge) -> Vec<ParameterInfo>;
        fn get_input_infos(self: &AlgorithmBridge) -> Vec<InputOutputInfo>;
        fn get_output_infos(self: &AlgorithmBridge) -> Vec<InputOutputInfo>;

        // ===== Algorithm Bridge Configuration & Execution =====
        fn configure(
            self: Pin<&mut AlgorithmBridge>,
            parameter_map_bridge: UniquePtr<ParameterMapBridge>,
        ) -> Result<()>;
        fn compute(self: Pin<&mut AlgorithmBridge>) -> Result<()>;
        fn reset(self: Pin<&mut AlgorithmBridge>) -> Result<()>;

        // ===== Algorithm Bridge Input/Output =====
        fn set_input(
            self: Pin<&mut AlgorithmBridge>,
            input_name: &str,
            variant_data: UniquePtr<VariantData>,
        ) -> Result<()>;
        fn setup_output(
            self: Pin<&mut AlgorithmBridge>,
            output_name: &str,
            data_type: DataType,
        ) -> Result<()>;
        fn get_output(self: &AlgorithmBridge, output_name: &str) -> Result<&VariantData>;

        // ===== Variant Data Constructors =====
        fn create_variant_data_from_bool(value: bool) -> UniquePtr<VariantData>;
        fn create_variant_data_from_string(value: &str) -> UniquePtr<VariantData>;
        fn create_variant_data_from_float(value: f32) -> UniquePtr<VariantData>;
        fn create_variant_data_from_int(value: i32) -> UniquePtr<VariantData>;
        fn create_variant_data_from_unsigned_int(value: u32) -> UniquePtr<VariantData>;
        fn create_variant_data_from_long(value: i64) -> UniquePtr<VariantData>;
        fn create_variant_data_from_stereo_sample(value: StereoSample) -> UniquePtr<VariantData>;
        fn create_variant_data_from_complex(value: Complex) -> UniquePtr<VariantData>;
        fn create_variant_data_from_vector_bool(value: &[bool]) -> UniquePtr<VariantData>;
        fn create_variant_data_from_vector_int(value: &[i32]) -> UniquePtr<VariantData>;
        fn create_variant_data_from_vector_string(value: &[&str]) -> UniquePtr<VariantData>;
        fn create_variant_data_from_vector_float(value: &[f32]) -> UniquePtr<VariantData>;
        fn create_variant_data_from_vector_stereo_sample(
            value: &[StereoSample],
        ) -> UniquePtr<VariantData>;
        fn create_variant_data_from_vector_complex(value: &[Complex]) -> UniquePtr<VariantData>;
        fn create_variant_data_from_vector_vector_float(
            value: Vec<SliceFloat>,
        ) -> UniquePtr<VariantData>;
        fn create_variant_data_from_matrix_float(value: MatrixFloat) -> UniquePtr<VariantData>;
        fn create_variant_data_from_tensor_float(value: TensorFloat) -> UniquePtr<VariantData>;
        fn create_variant_data_from_vector_vector_string(
            value: Vec<VecString>,
        ) -> UniquePtr<VariantData>;
        fn create_variant_data_from_vector_vector_stereo_sample(
            value: Vec<SliceStereoSample>,
        ) -> UniquePtr<VariantData>;
        fn create_variant_data_from_vector_vector_complex(
            value: Vec<VecComplex>,
        ) -> UniquePtr<VariantData>;
        fn create_variant_data_from_vector_matrix_float(
            value: Vec<MatrixFloat>,
        ) -> UniquePtr<VariantData>;
        fn create_variant_data_from_map_vector_float(
            value: Vec<MapEntryVectorFloat>,
        ) -> UniquePtr<VariantData>;
        fn create_variant_data_from_map_vector_string(
            value: Vec<MapEntryVectorString>,
        ) -> UniquePtr<VariantData>;
        fn create_variant_data_from_map_vector_int(
            value: Vec<MapEntryVectorInt>,
        ) -> UniquePtr<VariantData>;
        fn create_variant_data_from_map_vector_complex(
            value: Vec<MapEntryVectorComplex>,
        ) -> UniquePtr<VariantData>;
        fn create_variant_data_from_map_float(value: Vec<MapEntryFloat>) -> UniquePtr<VariantData>;
        fn create_variant_data_from_pool(value: UniquePtr<PoolBridge>) -> UniquePtr<VariantData>;

        // ===== Variant Data Introspection =====
        fn get_data_type(self: &VariantData) -> DataType;

        // ===== Variant Data Accessors =====
        fn get_bool(self: &VariantData) -> Result<bool>;
        fn get_string(self: &VariantData) -> Result<String>;
        fn get_float(self: &VariantData) -> Result<f32>;
        fn get_int(self: &VariantData) -> Result<i32>;
        fn get_unsigned_int(self: &VariantData) -> Result<u32>;
        fn get_long(self: &VariantData) -> Result<i64>;
        fn get_stereo_sample(self: &VariantData) -> Result<StereoSample>;
        fn get_complex(self: &VariantData) -> Result<Complex>;
        fn get_vector_bool(self: &VariantData) -> Result<Vec<bool>>;
        fn get_vector_int(self: &VariantData) -> Result<&[i32]>;
        fn get_vector_string(self: &VariantData) -> Result<Vec<String>>;
        fn get_vector_float(self: &VariantData) -> Result<&[f32]>;
        fn get_vector_stereo_sample(self: &VariantData) -> Result<&[StereoSample]>;
        fn get_vector_complex(self: &VariantData) -> Result<&[Complex]>;
        fn get_vector_vector_float(self: &VariantData) -> Result<Vec<SliceFloat>>;
        fn get_matrix_float(self: &VariantData) -> Result<MatrixFloat>;
        fn get_tensor_float(self: &VariantData) -> Result<TensorFloat>;
        fn get_vector_vector_string(self: &VariantData) -> Result<Vec<VecString>>;
        fn get_vector_vector_stereo_sample(self: &VariantData) -> Result<Vec<SliceStereoSample>>;
        fn get_vector_vector_complex(self: &VariantData) -> Result<Vec<VecComplex>>;
        fn get_vector_matrix_float(self: &VariantData) -> Result<Vec<MatrixFloat>>;
        fn get_map_vector_float(self: &VariantData) -> Result<Vec<MapEntryVectorFloat>>;
        fn get_map_vector_string(self: &VariantData) -> Result<Vec<MapEntryVectorString>>;
        fn get_map_vector_int(self: &VariantData) -> Result<Vec<MapEntryVectorInt>>;
        fn get_map_vector_complex(self: &VariantData) -> Result<Vec<MapEntryVectorComplex>>;
        fn get_map_float(self: &VariantData) -> Result<Vec<MapEntryFloat>>;
        fn get_pool(self: &VariantData) -> &PoolBridge;

        // ===== Parameter Map Bridge =====
        fn create_parameter_map_bridge() -> UniquePtr<ParameterMapBridge>;
        fn add(
            self: Pin<&mut ParameterMapBridge>,
            key: &str,
            variant_data: UniquePtr<VariantData>,
        ) -> Result<()>;

        // ===== Pool Bridge =====
        fn create_pool_bridge() -> UniquePtr<PoolBridge>;
        fn clone(self: &PoolBridge) -> UniquePtr<PoolBridge>;
        fn set(self: Pin<&mut PoolBridge>, key: &str, variant_data: UniquePtr<VariantData>);
        fn get(self: &PoolBridge, key: &str) -> Result<UniquePtr<VariantData>>;
        fn contains(self: &PoolBridge, key: &str) -> bool;
        fn keys(self: &PoolBridge) -> Vec<String>;
    }
}

pub use bridge::*;

// TODO Move this somewhere else
impl From<num::Complex<f32>> for Complex {
    fn from(c: num::Complex<f32>) -> Self {
        Complex {
            real: c.re,
            imag: c.im,
        }
    }
}

impl From<Complex> for num::Complex<f32> {
    fn from(c: Complex) -> Self {
        num::Complex::new(c.real, c.imag)
    }
}

impl From<&num::Complex<f32>> for Complex {
    fn from(c: &num::Complex<f32>) -> Self {
        Complex {
            real: c.re,
            imag: c.im,
        }
    }
}

impl From<&Complex> for num::Complex<f32> {
    fn from(c: &Complex) -> Self {
        num::Complex::new(c.real, c.imag)
    }
}
