#[cxx::bridge]
pub mod bridge {
    struct FloatSlice<'a> {
        slice: &'a [f32],
    }

    struct FloatMatrix2d<'a> {
        slice: &'a [f32],
        dim1: usize,
        dim2: usize,
    }

    unsafe extern "C++" {
        include!("bridge/parameter_map.h");

        type ParameterMapBridge;

        fn add_string(self: Pin<&mut ParameterMapBridge>, key: &str, value: &str);
        fn add_real(self: Pin<&mut ParameterMapBridge>, key: &str, value: f32);
        fn add_int(self: Pin<&mut ParameterMapBridge>, key: &str, value: i32);
        fn add_bool(self: Pin<&mut ParameterMapBridge>, key: &str, value: bool);
    }

    #[derive(Debug, Clone, Copy)]
    enum ParameterType {
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

    struct ParameterInfo {
        name: String,
        type_: ParameterType,
        range: String,
        description: String,
        default_value: String,
    }

    #[derive(Debug, Clone, Copy)]
    enum IOType {
        Real,
        Int,
        UnsignedInt,
        Long,
        VectorReal,
        VectorVectorReal,
        MatrixReal,
    }

    struct IOInfo {
        name: String,
        type_: IOType,
        description: String,
    }

    unsafe extern "C++" {
        include!("bridge/bridge.h");

        type AlgorithmBridge;

        fn init_essentia();
        fn shutdown_essentia();
        fn get_algorithm_names() -> Vec<String>;

        fn create_algorithm(name: &str) -> UniquePtr<AlgorithmBridge>;
        fn create_parameter_map() -> UniquePtr<ParameterMapBridge>;

        // AlgorithmBridge methods
        fn configure(self: Pin<&mut AlgorithmBridge>, parameter_map: &ParameterMapBridge);
        fn compute(self: Pin<&mut AlgorithmBridge>);

        // Metadata
        fn get_algorithm_name(self: &AlgorithmBridge) -> String;
        fn get_algorithm_category(self: &AlgorithmBridge) -> String;
        fn get_algorithm_description(self: &AlgorithmBridge) -> String;
        fn get_all_parameter_info(self: &AlgorithmBridge) -> Vec<ParameterInfo>;
        fn get_all_input_info(self: &AlgorithmBridge) -> Vec<IOInfo>;
        fn get_all_output_info(self: &AlgorithmBridge) -> Vec<IOInfo>;

        // Input setter methods
        fn set_input_real(self: Pin<&mut AlgorithmBridge>, input_name: &str, value: f32);
        fn set_input_int(self: Pin<&mut AlgorithmBridge>, input_name: &str, value: i32);
        fn set_input_uint(self: Pin<&mut AlgorithmBridge>, input_name: &str, value: u32);
        fn set_input_long(self: Pin<&mut AlgorithmBridge>, input_name: &str, value: i64);
        fn set_input_real_vector(self: Pin<&mut AlgorithmBridge>, input_name: &str, value: &[f32]);
        fn set_input_real_vector_vector(
            self: Pin<&mut AlgorithmBridge>,
            input_name: &str,
            value: Vec<FloatSlice>,
        );
        fn set_input_real_matrix(
            self: Pin<&mut AlgorithmBridge>,
            input_name: &str,
            value: FloatMatrix2d,
        );

        // Output setup methods
        fn setup_output(self: Pin<&mut AlgorithmBridge>, output_name: &str, io_type: IOType);

        // Output getter methods
        fn get_output_real(self: &AlgorithmBridge, output_name: &str) -> f32;
        fn get_output_int(self: &AlgorithmBridge, output_name: &str) -> i32;
        fn get_output_uint(self: &AlgorithmBridge, output_name: &str) -> u32;
        fn get_output_long(self: &AlgorithmBridge, output_name: &str) -> i64;
        fn get_output_real_vector(self: &AlgorithmBridge, output_name: &str) -> &[f32];
        fn get_output_real_vector_vector(
            self: &AlgorithmBridge,
            output_name: &str,
        ) -> Vec<FloatSlice>;
        fn get_output_real_matrix(self: &AlgorithmBridge, output_name: &str) -> FloatMatrix2d;

        fn reset(self: Pin<&mut AlgorithmBridge>);
    }
}

// Re-export for convenience
pub use bridge::*;
