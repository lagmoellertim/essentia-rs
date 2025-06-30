#include "../common/type_mapping.h"
#include "essentia-core/src/ffi.rs.h"
#include "variant_data.h"
#include <rust/cxx.h>
#include <stdexcept>
#include <variant>

namespace essentia_bridge {

DataType VariantData::get_data_type() const {
  return std::visit(
      [](const auto &value) -> DataType {
        return type_info_to_data_type(typeid(value));
      },
      data);
}

} // namespace essentia_bridge