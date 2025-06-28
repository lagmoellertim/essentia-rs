#include "essentia_core/src/ffi.rs.h"
#include "variant_data.h"
#include <rust/cxx.h>

namespace essentia_bridge {

std::unique_ptr<VariantData> create_variant_data_from_float(float value) {
  return std::make_unique<VariantData>(value);
}

std::unique_ptr<VariantData> create_variant_data_from_int(int value) {
  return std::make_unique<VariantData>(value);
}

std::unique_ptr<VariantData>
create_variant_data_from_unsigned_int(unsigned int value) {
  return std::make_unique<VariantData>(value);
}

std::unique_ptr<VariantData> create_variant_data_from_long(std::int64_t value) {
  return std::make_unique<VariantData>(value);
}

std::unique_ptr<VariantData>
create_variant_data_from_vector_float(rust::Slice<const float> value) {
  return std::make_unique<VariantData>(
      std::vector<float>(value.begin(), value.end()));
}

std::unique_ptr<VariantData>
create_variant_data_from_vector_vector_float(rust::Vec<SliceFloat> value) {
  std::vector<std::vector<float>> cpp_vec;
  cpp_vec.reserve(value.size());

  for (const auto &float_slice : value) {
    const auto *data = float_slice.slice.data();
    size_t size = float_slice.slice.size();
    cpp_vec.push_back(std::vector<float>(data, data + size));
  }

  return std::make_unique<VariantData>(std::move(cpp_vec));
}

std::unique_ptr<VariantData>
create_variant_data_from_matrix_float(MatrixFloat value) {
  assert(value.slice.size() == value.dim1 * value.dim2);
  TNT::Array2D<float> array(value.dim1, value.dim2);
  std::memcpy(&array[0][0], value.slice.data(),
              value.slice.size() * sizeof(float));
  return std::make_unique<VariantData>(std::move(array));
}

} // namespace essentia_bridge