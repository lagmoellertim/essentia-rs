#pragma once

#include <essentia/utils/tnt/tnt_array2d.h>
#include <memory>
#include <rust/cxx.h>
#include <variant>
#include <vector>

namespace essentia_bridge {

enum class DataType : std::uint8_t;

struct SliceFloat;
struct MatrixFloat;

struct VariantData {
  using StorageType =
      std::variant<float, int, unsigned int, std::int64_t, std::vector<float>,
                   std::vector<std::vector<float>>, TNT::Array2D<float>>;
  StorageType data;

  VariantData() = default;

  template <typename T>
  explicit VariantData(T &&value) : data(std::forward<T>(value)) {}

  DataType get_data_type() const;

  float get_float() const;
  int get_int() const;
  unsigned int get_unsigned_int() const;
  std::int64_t get_long() const;
  rust::Slice<const float> get_vector_float() const;
  rust::Vec<SliceFloat> get_vector_vector_float() const;
  MatrixFloat get_matrix_float() const;
};

std::unique_ptr<VariantData> create_variant_data_from_float(float value);
std::unique_ptr<VariantData> create_variant_data_from_int(int value);
std::unique_ptr<VariantData>
create_variant_data_from_unsigned_int(unsigned int value);
std::unique_ptr<VariantData> create_variant_data_from_long(std::int64_t value);
std::unique_ptr<VariantData>
create_variant_data_from_vector_float(rust::Slice<const float> value);
std::unique_ptr<VariantData>
create_variant_data_from_vector_vector_float(rust::Vec<SliceFloat> value);
std::unique_ptr<VariantData>
create_variant_data_from_matrix_float(MatrixFloat value);

} // namespace essentia_bridge