#include "essentia_core/src/ffi.rs.h"
#include "variant_data.h"
#include <rust/cxx.h>

namespace essentia_bridge {

float VariantData::get_float() const { return std::get<float>(data); }

int VariantData::get_int() const { return std::get<int>(data); }

unsigned int VariantData::get_unsigned_int() const {
  return std::get<unsigned int>(data);
}

std::int64_t VariantData::get_long() const {
  return std::get<std::int64_t>(data);
}

rust::Slice<const float> VariantData::get_vector_float() const {
  const std::vector<float> &vec = std::get<std::vector<float>>(data);
  rust::Slice<const float> slice{vec.data(), vec.size()};

  return slice;
}

rust::Vec<SliceFloat> VariantData::get_vector_vector_float() const {
  const auto &vec = std::get<std::vector<std::vector<float>>>(data);

  rust::Vec<SliceFloat> rust_vec;

  for (const auto &item : vec) {
    SliceFloat slice_float;
    slice_float.slice = rust::Slice{item.data(), item.size()};
    rust_vec.push_back(slice_float);
  }

  return rust_vec;
}

MatrixFloat VariantData::get_matrix_float() const {
  const auto &matrix = std::get<TNT::Array2D<float>>(data);

  MatrixFloat rust_matrix;

  size_t dim1 = matrix.dim1();
  size_t dim2 = matrix.dim2();

  rust_matrix.dim1 = dim1;
  rust_matrix.dim2 = dim2;
  rust_matrix.slice = rust::Slice<const float>(&matrix[0][0], dim1 * dim2);

  return rust_matrix;
}

} // namespace essentia_bridge