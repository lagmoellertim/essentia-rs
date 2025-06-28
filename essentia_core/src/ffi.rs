#[cxx::bridge(namespace = "essentia_bridge")]
pub mod bridge {
    // ===== Helper Structs =====
    
    struct SliceFloat<'a> {
        slice: &'a [f32],
    }

    struct MatrixFloat<'a> {
        slice: &'a [f32],
        dim1: usize,
        dim2: usize,
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
        type VariantData;

        // ===== Essentia Initialization =====
        fn init_essentia();
        fn shutdown_essentia();

        // ===== Algorithm Creation =====
        fn get_algorithm_names() -> Vec<String>;
        fn create_algorithm(name: &str) -> Result<UniquePtr<AlgorithmBridge>>;

        // ===== Algorithm Introspection =====
        fn get_name(self: &AlgorithmBridge) -> String;
        fn get_category(self: &AlgorithmBridge) -> String;
        fn get_description(self: &AlgorithmBridge) -> String;
        fn get_parameters(self: &AlgorithmBridge) -> Vec<ParameterInfo>;
        fn get_inputs(self: &AlgorithmBridge) -> Vec<InputOutputInfo>;
        fn get_outputs(self: &AlgorithmBridge) -> Vec<InputOutputInfo>;

        // ===== Algorithm Configuration & Execution =====
        fn configure(self: Pin<&mut AlgorithmBridge>, parameter_map: &ParameterMapBridge) -> Result<()>;
        fn compute(self: Pin<&mut AlgorithmBridge>) -> Result<()>;
        fn reset(self: Pin<&mut AlgorithmBridge>) -> Result<()>;

        // ===== Algorithm Input/Output =====
        fn set_input(
            self: Pin<&mut AlgorithmBridge>,
            input_name: &str,
            variant_data: UniquePtr<VariantData>,
        ) -> Result<()>;
        fn setup_output(self: Pin<&mut AlgorithmBridge>, output_name: &str, data_type: DataType) -> Result<()>;
        fn get_output(self: &AlgorithmBridge, output_name: &str) -> Result<&VariantData>;

        // ===== VariantData Constructors =====
        fn create_variant_data_from_float(value: f32) -> UniquePtr<VariantData>;
        fn create_variant_data_from_int(value: i32) -> UniquePtr<VariantData>;
        fn create_variant_data_from_unsigned_int(value: u32) -> UniquePtr<VariantData>;
        fn create_variant_data_from_long(value: i64) -> UniquePtr<VariantData>;
        fn create_variant_data_from_vector_float(value: &[f32]) -> UniquePtr<VariantData>;
        fn create_variant_data_from_vector_vector_float(value: Vec<SliceFloat>) -> UniquePtr<VariantData>;
        fn create_variant_data_from_matrix_float(value: MatrixFloat) -> UniquePtr<VariantData>;

        // ===== VariantData Accessors =====
        fn get_float(self: &VariantData) -> Result<f32>;
        fn get_int(self: &VariantData) -> Result<i32>;
        fn get_unsigned_int(self: &VariantData) -> Result<u32>;
        fn get_long(self: &VariantData) -> Result<i64>;
        fn get_vector_float(self: &VariantData) -> Result<&[f32]>;
        fn get_vector_vector_float(self: &VariantData) -> Result<Vec<SliceFloat>>;
        fn get_matrix_float(self: &VariantData) -> Result<MatrixFloat>;

        // ===== ParameterMap =====
        fn create_parameter_map() -> UniquePtr<ParameterMapBridge>;
        fn add(self: Pin<&mut ParameterMapBridge>, key: &str, variant_data: UniquePtr<VariantData>) -> Result<()>;
    }
}

pub use bridge::*;
