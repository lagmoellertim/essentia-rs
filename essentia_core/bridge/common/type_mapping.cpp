#include "type_mapping.h"
#include "essentia_core/src/ffi.rs.h"
#include <essentia/utils/tnt/tnt_array2d.h>
#include <stdexcept>
#include <string>
#include <typeinfo>
#include <vector>

namespace essentia_bridge {

const std::unordered_map<std::type_index, DataType> &get_type_mapping() {
  static const std::unordered_map<std::type_index, DataType> type_map = {
      {typeid(float), DataType::Float},
      {typeid(int), DataType::Int},
      {typeid(unsigned int), DataType::UnsignedInt},
      {typeid(long), DataType::Long},
      {typeid(std::vector<float>), DataType::VectorFloat},
      {typeid(std::vector<std::vector<float>>), DataType::VectorVectorFloat},
      {typeid(TNT::Array2D<float>), DataType::MatrixFloat}};
  return type_map;
}

DataType type_info_to_data_type(const std::type_info &type_info) {
  const auto &type_map = get_type_mapping();
  const auto it = type_map.find(type_info);

  if (it == type_map.end()) {
    throw std::invalid_argument("type_info_to_data_type: unsupported type (" +
                                std::string(type_info.name()) + ')');
  }

  return it->second;
}

} // namespace essentia_bridge