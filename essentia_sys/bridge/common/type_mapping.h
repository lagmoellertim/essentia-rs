#pragma once

#include <cstdint>
#include <typeindex>
#include <unordered_map>

namespace essentia_bridge {

enum class DataType : std::uint8_t;

DataType type_info_to_data_type(const std::type_info &type_info);

} // namespace essentia_bridge