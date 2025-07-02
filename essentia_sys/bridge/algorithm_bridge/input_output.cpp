#include "algorithm_bridge.h"
#include "essentia-sys/src/lib.rs.h"
#include <stdexcept>

namespace essentia_bridge {

void AlgorithmBridge::set_input(rust::Str input_name,
                                std::unique_ptr<DataContainer> data_container) {
  std::string key(input_name);

  auto &input = _inputs[key] = std::move(*data_container);

  std::visit([&](auto &value) { _algorithm->input(key).set(value); },
             input.data);
}

void AlgorithmBridge::setup_output(rust::Str output_name, DataType data_type) {
  switch (data_type) {
  case DataType::Bool:
    generic_setup_output<bool>(output_name);
    break;

  case DataType::String:
    generic_setup_output<std::string>(output_name);
    break;

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

  case DataType::StereoSample:
    generic_setup_output<essentia::StereoSample>(output_name);
    break;

  case DataType::VectorBool:
    generic_setup_output<std::vector<bool>>(output_name);
    break;

  case DataType::VectorInt:
    generic_setup_output<std::vector<int>>(output_name);
    break;

  case DataType::VectorString:
    generic_setup_output<std::vector<std::string>>(output_name);
    break;

  case DataType::VectorFloat:
    generic_setup_output<std::vector<float>>(output_name);
    break;

  case DataType::VectorStereoSample:
    generic_setup_output<std::vector<essentia::StereoSample>>(output_name);
    break;

  case DataType::VectorVectorFloat:
    generic_setup_output<std::vector<std::vector<float>>>(output_name);
    break;

  case DataType::VectorVectorString:
    generic_setup_output<std::vector<std::vector<std::string>>>(output_name);
    break;

  case DataType::VectorVectorStereoSample:
    generic_setup_output<std::vector<std::vector<essentia::StereoSample>>>(
        output_name);
    break;

  case DataType::VectorMatrixFloat:
    generic_setup_output<std::vector<TNT::Array2D<float>>>(output_name);
    break;

  case DataType::MatrixFloat:
    generic_setup_output<TNT::Array2D<float>>(output_name);
    break;

  case DataType::MapVectorFloat:
    generic_setup_output<std::map<std::string, std::vector<float>>>(
        output_name);
    break;

  case DataType::MapVectorString:
    generic_setup_output<std::map<std::string, std::vector<std::string>>>(
        output_name);
    break;

  case DataType::MapVectorInt:
    generic_setup_output<std::map<std::string, std::vector<int>>>(output_name);
    break;

  case DataType::MapFloat:
    generic_setup_output<std::map<std::string, float>>(output_name);
    break;

  default:
    throw std::invalid_argument{"AlgorithmWrapper::setup_output: "
                                "unsupported DataType value"};
  }
}

const DataContainer &AlgorithmBridge::get_output(rust::Str output_name) const {
  return _outputs.at(std::string(output_name));
}

} // namespace essentia_bridge