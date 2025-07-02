#include "../common/type_mapping.h"
#include "data_container.h"
#include "essentia-sys/src/lib.rs.h"
#include <rust/cxx.h>
#include <stdexcept>
#include <variant>

namespace essentia_bridge {

DataType DataContainer::get_data_type() const {
  return std::visit(
      [](const auto &value) -> DataType {
        return type_info_to_data_type(typeid(value));
      },
      data);
}

} // namespace essentia_bridge