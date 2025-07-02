#[cxx::bridge(namespace = "essentia_bridge")]
pub mod ffi {

    // ===== Helper Structs =====
    pub struct SliceFloat<'a> {
        slice: &'a [f32],
    }

    pub struct VecString {
        vec: Vec<String>,
    }

    pub struct SliceStereoSample<'a> {
        slice: &'a [StereoSample],
    }

    pub struct MatrixFloat<'a> {
        slice: &'a [f32],
        dim1: usize,
        dim2: usize,
    }

    pub struct TensorFloat<'a> {
        slice: &'a [f32],
        shape: &'a [usize],
    }

    pub struct MapEntryVectorFloat<'a> {
        key: String,
        value: &'a [f32],
    }

    pub struct MapEntryVectorString {
        key: String,
        value: Vec<String>,
    }

    pub struct MapEntryVectorInt<'a> {
        key: String,
        value: &'a [i32],
    }

    pub struct MapEntryFloat {
        key: String,
        value: f32,
    }

    #[derive(Clone, Debug)]
    pub struct StereoSample {
        left: f32,
        right: f32,
    }

    #[derive(Clone, Debug)]
    pub struct Complex {
        real: f32,
        imag: f32,
    }

    pub struct VecComplex {
        vec: Vec<Complex>,
    }

    pub struct MapEntryVectorComplex<'a> {
        key: String,
        value: &'a [Complex],
    }

    // ===== Data Type Enum =====
    #[derive(Debug, Clone, Copy)]
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

    // ===== Introspection Structs =====
    pub struct ParameterInfo {
        name: String,
        data_type: DataType,
        constraint: String,
        description: String,
        default_value: String,
    }

    pub struct InputOutputInfo {
        name: String,
        data_type: DataType,
        description: String,
    }

    // ===== C++ Bridge =====
    unsafe extern "C++" {
        include!("bridge/bridge.h");

        // ===== Core types =====
        pub type AlgorithmBridge;
        pub type ParameterMapBridge;
        pub type PoolBridge;
        pub type DataContainer;

        // ===== Essentia Initialization =====
        pub fn init_essentia();
        pub fn shutdown_essentia();

        // ===== Algorithm Bridge Creation =====
        pub fn get_algorithm_names() -> Vec<String>;
        pub fn create_algorithm_bridge(name: &str) -> Result<UniquePtr<AlgorithmBridge>>;

        // ===== Algorithm Bridge Introspection =====
        pub fn get_name(self: &AlgorithmBridge) -> String;
        pub fn get_category(self: &AlgorithmBridge) -> String;
        pub fn get_description(self: &AlgorithmBridge) -> String;
        pub fn get_parameter_infos(self: &AlgorithmBridge) -> Vec<ParameterInfo>;
        pub fn get_input_infos(self: &AlgorithmBridge) -> Vec<InputOutputInfo>;
        pub fn get_output_infos(self: &AlgorithmBridge) -> Vec<InputOutputInfo>;

        // ===== Algorithm Bridge Configuration & Execution =====
        pub fn configure(
            self: Pin<&mut AlgorithmBridge>,
            parameter_map_bridge: UniquePtr<ParameterMapBridge>,
        ) -> Result<()>;
        pub fn compute(self: Pin<&mut AlgorithmBridge>) -> Result<()>;
        pub fn reset(self: Pin<&mut AlgorithmBridge>) -> Result<()>;

        // ===== Algorithm Bridge Input/Output =====
        pub fn set_input(
            self: Pin<&mut AlgorithmBridge>,
            input_name: &str,
            data_container: UniquePtr<DataContainer>,
        ) -> Result<()>;
        pub fn setup_output(
            self: Pin<&mut AlgorithmBridge>,
            output_name: &str,
            data_type: DataType,
        ) -> Result<()>;
        pub fn get_output(self: &AlgorithmBridge, output_name: &str) -> Result<&DataContainer>;

        // ===== Data Container Constructors =====
        pub fn create_data_container_from_bool(value: bool) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_string(value: &str) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_float(value: f32) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_int(value: i32) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_unsigned_int(value: u32) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_long(value: i64) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_stereo_sample(
            value: StereoSample,
        ) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_complex(value: Complex) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_vector_bool(value: &[bool]) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_vector_int(value: &[i32]) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_vector_string(value: &[&str])
        -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_vector_float(value: &[f32]) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_vector_stereo_sample(
            value: &[StereoSample],
        ) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_vector_complex(
            value: &[Complex],
        ) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_vector_vector_float(
            value: Vec<SliceFloat>,
        ) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_matrix_float(
            value: MatrixFloat,
        ) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_tensor_float(
            value: TensorFloat,
        ) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_vector_vector_string(
            value: Vec<VecString>,
        ) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_vector_vector_stereo_sample(
            value: Vec<SliceStereoSample>,
        ) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_vector_vector_complex(
            value: Vec<VecComplex>,
        ) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_vector_matrix_float(
            value: Vec<MatrixFloat>,
        ) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_map_vector_float(
            value: Vec<MapEntryVectorFloat>,
        ) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_map_vector_string(
            value: Vec<MapEntryVectorString>,
        ) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_map_vector_int(
            value: Vec<MapEntryVectorInt>,
        ) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_map_vector_complex(
            value: Vec<MapEntryVectorComplex>,
        ) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_map_float(
            value: Vec<MapEntryFloat>,
        ) -> UniquePtr<DataContainer>;
        pub fn create_data_container_from_pool(
            value: UniquePtr<PoolBridge>,
        ) -> UniquePtr<DataContainer>;

        // ===== Data Container Introspection =====
        pub fn get_data_type(self: &DataContainer) -> DataType;

        // ===== Data Container Accessors =====
        pub fn get_bool(self: &DataContainer) -> Result<bool>;
        pub fn get_string(self: &DataContainer) -> Result<String>;
        pub fn get_float(self: &DataContainer) -> Result<f32>;
        pub fn get_int(self: &DataContainer) -> Result<i32>;
        pub fn get_unsigned_int(self: &DataContainer) -> Result<u32>;
        pub fn get_long(self: &DataContainer) -> Result<i64>;
        pub fn get_stereo_sample(self: &DataContainer) -> Result<StereoSample>;
        pub fn get_complex(self: &DataContainer) -> Result<Complex>;
        pub fn get_vector_bool(self: &DataContainer) -> Result<Vec<bool>>;
        pub fn get_vector_int(self: &DataContainer) -> Result<&[i32]>;
        pub fn get_vector_string(self: &DataContainer) -> Result<Vec<String>>;
        pub fn get_vector_float(self: &DataContainer) -> Result<&[f32]>;
        pub fn get_vector_stereo_sample(self: &DataContainer) -> Result<&[StereoSample]>;
        pub fn get_vector_complex(self: &DataContainer) -> Result<&[Complex]>;
        pub fn get_vector_vector_float(self: &DataContainer) -> Result<Vec<SliceFloat>>;
        pub fn get_matrix_float(self: &DataContainer) -> Result<MatrixFloat>;
        pub fn get_tensor_float(self: &DataContainer) -> Result<TensorFloat>;
        pub fn get_vector_vector_string(self: &DataContainer) -> Result<Vec<VecString>>;
        pub fn get_vector_vector_stereo_sample(
            self: &DataContainer,
        ) -> Result<Vec<SliceStereoSample>>;
        pub fn get_vector_vector_complex(self: &DataContainer) -> Result<Vec<VecComplex>>;
        pub fn get_vector_matrix_float(self: &DataContainer) -> Result<Vec<MatrixFloat>>;
        pub fn get_map_vector_float(self: &DataContainer) -> Result<Vec<MapEntryVectorFloat>>;
        pub fn get_map_vector_string(self: &DataContainer) -> Result<Vec<MapEntryVectorString>>;
        pub fn get_map_vector_int(self: &DataContainer) -> Result<Vec<MapEntryVectorInt>>;
        pub fn get_map_vector_complex(self: &DataContainer) -> Result<Vec<MapEntryVectorComplex>>;
        pub fn get_map_float(self: &DataContainer) -> Result<Vec<MapEntryFloat>>;
        pub fn get_pool(self: &DataContainer) -> &PoolBridge;

        // ===== Parameter Map Bridge =====
        pub fn create_parameter_map_bridge() -> UniquePtr<ParameterMapBridge>;
        pub fn add(
            self: Pin<&mut ParameterMapBridge>,
            key: &str,
            data_container: UniquePtr<DataContainer>,
        ) -> Result<()>;

        // ===== Pool Bridge =====
        pub fn create_pool_bridge() -> UniquePtr<PoolBridge>;
        pub fn clone(self: &PoolBridge) -> UniquePtr<PoolBridge>;
        pub fn set(self: Pin<&mut PoolBridge>, key: &str, data_container: UniquePtr<DataContainer>);
        pub fn get(self: &PoolBridge, key: &str) -> Result<UniquePtr<DataContainer>>;
        pub fn contains(self: &PoolBridge, key: &str) -> bool;
        pub fn keys(self: &PoolBridge) -> Vec<String>;
    }
}
