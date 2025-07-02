#pragma once

#include "../data_container/data_container.h"
#include "../parameter_map_bridge/parameter_map_bridge.h"
#include "essentia/algorithm.h"
#include <rust/cxx.h>

namespace essentia_bridge {

enum class DataType : ::std::uint8_t;

struct SliceFloat;
struct MatrixFloat;
struct ParameterInfo;
struct InputOutputInfo;

DataType
essentia_param_type_to_enum(essentia::Parameter::ParamType essentia_type);
DataType type_info_to_io_enum(const std::type_info *type_info);

class AlgorithmBridge {
private:
  essentia::standard::Algorithm *_algorithm;

  std::map<std::string, DataContainer> _inputs;
  std::map<std::string, DataContainer> _outputs;

  template <typename T> void generic_setup_output(rust::Str output_name) {
    std::string str_output_name(output_name);
    _outputs[str_output_name] = DataContainer(T{});
    _algorithm->output(str_output_name)
        .set(std::get<T>(_outputs[str_output_name].data));
  }

public:
  AlgorithmBridge(essentia::standard::Algorithm *algorithm);
  ~AlgorithmBridge();

  void configure(std::unique_ptr<ParameterMapBridge> parameter_map_bridge);
  void compute();

  rust::String get_name() const;
  rust::String get_category() const;
  rust::String get_description() const;
  rust::Vec<ParameterInfo> get_parameter_infos() const;
  rust::Vec<InputOutputInfo> get_input_infos() const;
  rust::Vec<InputOutputInfo> get_output_infos() const;

  void set_input(rust::Str input_name,
                 std::unique_ptr<DataContainer> data_container);

  void setup_output(rust::Str output_name, DataType data_type);

  const DataContainer &get_output(rust::Str output_name) const;

  void reset();
};

} // namespace essentia_bridge