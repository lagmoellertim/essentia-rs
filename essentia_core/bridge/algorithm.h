#pragma once

#include "parameter_map.h"
#include "essentia/algorithm.h"
#include <rust/cxx.h>

enum class ParameterType : ::std::uint8_t;
enum class IOType : ::std::uint8_t;

struct FloatSlice;
struct FloatMatrix2d;
struct ParameterInfo;
struct IOInfo;

// Helper function declarations
ParameterType essentia_param_type_to_enum(essentia::Parameter::ParamType essentia_type);
IOType type_info_to_io_enum(const std::type_info *type_info);

class AlgorithmBridge
{
private:
    essentia::standard::Algorithm *_algorithm;

    // Define a unified variant type that can hold all possible input/output types
    using EssentiaVariant = std::variant<
        float,
        int,
        unsigned int,
        std::int64_t,

        std::vector<float>,

        std::vector<std::vector<float>>,

        TNT::Array2D<float>>;

    std::map<std::string, EssentiaVariant> _outputs;
    std::map<std::string, EssentiaVariant> _inputs;

    template <typename T>
    void set_and_store_input(rust::Str input_name, T value)
    {
        // TODO Investigate move
        std::string str_input_name(input_name);
        _inputs[str_input_name] = value;
        _algorithm->input(str_input_name).set(std::get<T>(_inputs[str_input_name]));
    }

    // Internal template method for type-agnostic binding
    template <typename T>
    void generic_setup_output(rust::Str output_name)
    {
        std::string str_output_name(output_name);
        _outputs[str_output_name] = T{};
        _algorithm->output(str_output_name).set(std::get<T>(_outputs[str_output_name]));
    }

public:
    AlgorithmBridge(essentia::standard::Algorithm *algorithm);
    ~AlgorithmBridge();

    void configure(const ParameterMapBridge &parameter_map_wrapper);
    void compute();

    // Introspection methods
    rust::String get_algorithm_name() const;
    rust::String get_algorithm_category() const;
    rust::String get_algorithm_description() const;
    rust::Vec<ParameterInfo> get_all_parameter_info() const;
    rust::Vec<IOInfo> get_all_input_info() const;
    rust::Vec<IOInfo> get_all_output_info() const;

    // Input setter methods
    void set_input_real(rust::Str input_name, float value);
    void set_input_int(rust::Str input_name, int value);
    void set_input_uint(rust::Str input_name, unsigned int value);
    void set_input_long(rust::Str input_name, std::int64_t value);
    void set_input_real_vector(rust::Str input_name, rust::Slice<const float> value);
    void set_input_real_vector_vector(rust::Str input_name, rust::Vec<FloatSlice> value);
    void set_input_real_matrix(rust::Str input_name, FloatMatrix2d value);

    void setup_output(rust::Str output_name, IOType io_type);

    float get_output_real(rust::Str output_name) const;
    int get_output_int(rust::Str output_name) const;
    unsigned int get_output_uint(rust::Str output_name) const;
    std::int64_t get_output_long(rust::Str output_name) const;
    rust::Slice<const float> get_output_real_vector(rust::Str output_name) const;
    rust::Vec<FloatSlice> get_output_real_vector_vector(rust::Str input_name) const;
    FloatMatrix2d get_output_real_matrix(rust::Str input_name) const;

    void reset();
};