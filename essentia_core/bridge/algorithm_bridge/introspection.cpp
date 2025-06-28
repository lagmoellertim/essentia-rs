#include "algorithm_bridge.h"
#include "essentia/algorithmfactory.h"
#include "essentia_core/src/ffi.rs.h"
#include <cstring>
#include <typeindex>
#include <typeinfo>

namespace essentia_bridge {

struct EssentiaParamTypeHasher {
  std::size_t operator()(essentia::Parameter::ParamType p) const noexcept {
    using U = std::underlying_type_t<essentia::Parameter::ParamType>;
    return static_cast<std::size_t>(static_cast<U>(p));
  }
};

static const std::unordered_map<essentia::Parameter::ParamType, DataType,
                                EssentiaParamTypeHasher>
    parameterTypeLookupMap = {
        {essentia::Parameter::REAL, DataType::Float},
        {essentia::Parameter::STRING, DataType::String},
        {essentia::Parameter::BOOL, DataType::Bool},
        {essentia::Parameter::INT, DataType::Int},
        {essentia::Parameter::STEREOSAMPLE, DataType::StereoSample},
        {essentia::Parameter::VECTOR_REAL, DataType::VectorFloat},
        {essentia::Parameter::VECTOR_STRING, DataType::VectorString},
        {essentia::Parameter::VECTOR_BOOL, DataType::VectorBool},
        {essentia::Parameter::VECTOR_INT, DataType::VectorInt},
        {essentia::Parameter::VECTOR_STEREOSAMPLE,
         DataType::VectorStereoSample},
        {essentia::Parameter::VECTOR_VECTOR_REAL, DataType::VectorVectorFloat},
        {essentia::Parameter::VECTOR_VECTOR_STRING,
         DataType::VectorVectorString},
        {essentia::Parameter::VECTOR_VECTOR_STEREOSAMPLE,
         DataType::VectorVectorStereoSample},
        {essentia::Parameter::VECTOR_MATRIX_REAL, DataType::VectorMatrixFloat},
        {essentia::Parameter::MAP_VECTOR_REAL, DataType::MapVectorFloat},
        {essentia::Parameter::MAP_VECTOR_STRING, DataType::MapVectorString},
        {essentia::Parameter::MAP_VECTOR_INT, DataType::MapVectorInt},
        {essentia::Parameter::MAP_REAL, DataType::MapFloat},
        {essentia::Parameter::MATRIX_REAL, DataType::MatrixFloat},
};

DataType
essentia_param_type_to_enum(essentia::Parameter::ParamType essentia_type) {
  const auto it = parameterTypeLookupMap.find(essentia_type);
  if (it == parameterTypeLookupMap.end())
    throw std::invalid_argument{
        "essentia_param_type_to_enum: unsupported parameter type"};

  return it->second;
}

static const std::unordered_map<std::type_index, DataType> ioTypeLookupMap = {
    {typeid(float), DataType::Float},
    {typeid(int), DataType::Int},
    {typeid(unsigned int), DataType::UnsignedInt},
    {typeid(long), DataType::Long},
    {typeid(std::vector<float>), DataType::VectorFloat},
    {typeid(std::vector<std::vector<float>>), DataType::VectorVectorFloat},
    {typeid(TNT::Array2D<float>), DataType::MatrixFloat}};

DataType type_info_to_io_enum(const std::type_info *type_info) {
  if (type_info == nullptr)
    throw std::invalid_argument{"type_info_to_io_enum: null pointer"};

  const auto it = ioTypeLookupMap.find(*type_info);
  if (it == ioTypeLookupMap.end())
    throw std::invalid_argument{"type_info_to_io_enum: unsupported type (" +
                                std::string(type_info->name()) + ')'};

  return it->second;
}

rust::String AlgorithmBridge::get_name() const {
  return rust::String(_algorithm->name());
}

rust::String AlgorithmBridge::get_category() const {
  try {
    const auto &info =
        essentia::standard::AlgorithmFactory::getInfo(_algorithm->name());
    return rust::String(info.category);
  } catch (...) {
    return rust::String("Unknown");
  }
}

rust::String AlgorithmBridge::get_description() const {
  try {
    const auto &info =
        essentia::standard::AlgorithmFactory::getInfo(_algorithm->name());
    return rust::String(info.description);
  } catch (...) {
    return rust::String("Description not available");
  }
}

rust::Vec<ParameterInfo> AlgorithmBridge::get_parameters() const {
  rust::Vec<ParameterInfo> param_infos;

  const auto &param_descriptions = _algorithm->parameterDescription;
  const auto &param_ranges = _algorithm->parameterRange;
  const auto &default_params = _algorithm->defaultParameters();

  for (const auto &param_desc : param_descriptions) {
    const std::string &param_name = param_desc.first;

    ParameterInfo info;
    info.name = param_name;
    info.description = param_desc.second;

    auto range_it = param_ranges.find(param_name);
    info.constraint = (range_it != param_ranges.end()) ? range_it->second : "";

    auto type_it = default_params.find(param_name);
    if (type_it != default_params.end()) {
      const auto &param = type_it->second;
      info.data_type = essentia_param_type_to_enum(param.type());

      try {
        info.default_value = param.toString();
      } catch (const essentia::EssentiaException &e) {
        info.default_value = "";
      }
    } else {
      throw std::runtime_error(std::string("Parameter type not found for: ") +
                               param_name);
    }

    param_infos.push_back(std::move(info));
  }

  return param_infos;
}

rust::Vec<InputOutputInfo> AlgorithmBridge::get_inputs() const {
  rust::Vec<InputOutputInfo> input_infos;

  auto input_names = _algorithm->inputNames();
  auto input_types = _algorithm->inputTypes();
  const auto &input_descriptions = _algorithm->inputDescription;

  for (size_t i = 0; i < input_names.size() && i < input_types.size(); ++i) {
    InputOutputInfo info;
    info.name = input_names[i];
    info.data_type = type_info_to_io_enum(input_types[i]);

    auto desc_it = input_descriptions.find(input_names[i]);
    info.description =
        (desc_it != input_descriptions.end()) ? desc_it->second : "";

    input_infos.push_back(std::move(info));
  }

  return input_infos;
}

rust::Vec<InputOutputInfo> AlgorithmBridge::get_outputs() const {
  rust::Vec<InputOutputInfo> output_infos;

  auto output_names = _algorithm->outputNames();
  auto output_types = _algorithm->outputTypes();
  const auto &output_descriptions = _algorithm->outputDescription;

  for (size_t i = 0; i < output_names.size() && i < output_types.size(); ++i) {
    InputOutputInfo info;
    info.name = output_names[i];
    info.data_type = type_info_to_io_enum(output_types[i]);

    auto desc_it = output_descriptions.find(output_names[i]);
    info.description =
        (desc_it != output_descriptions.end()) ? desc_it->second : "";

    output_infos.push_back(std::move(info));
  }

  return output_infos;
}
} // namespace essentia_bridge