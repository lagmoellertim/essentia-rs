#include "algorithm_bridge.h"
#include "essentia_core/src/ffi.rs.h"
#include <stdexcept>

namespace essentia_bridge {

void AlgorithmBridge::set_input(rust::Str input_name,
                                std::unique_ptr<VariantData> variant_data) {
  std::string key(input_name);

  auto &input = _inputs[key] = std::move(*variant_data);

  std::visit([&](auto &value) { _algorithm->input(key).set(value); },
             input.data);
}

void AlgorithmBridge::setup_output(rust::Str output_name, DataType data_type) {
  switch (data_type) {
  case DataType::Float:
    generic_setup_output<float>(output_name);
    break;

  case DataType::Int:
    generic_setup_output<int>(output_name);
    break;

  case DataType::UnsignedInt:
    generic_setup_output<unsigned int>(output_name);
    break;

  case DataType::Long:
    generic_setup_output<std::int64_t>(output_name);
    break;

  case DataType::VectorFloat:
    generic_setup_output<std::vector<float>>(output_name);
    break;

  case DataType::VectorVectorFloat:
    generic_setup_output<std::vector<std::vector<float>>>(output_name);
    break;

  case DataType::MatrixFloat:
    generic_setup_output<TNT::Array2D<float>>(output_name);
    break;

  default:
    throw std::invalid_argument{"AlgorithmWrapper::setup_output: "
                                "unsupported DataType value"};
  }
}

const VariantData &AlgorithmBridge::get_output(rust::Str output_name) const {
  return _outputs.at(std::string(output_name));
}

} // namespace essentia_bridge