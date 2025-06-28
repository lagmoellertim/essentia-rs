#pragma once

#include <essentia/types.h>
#include <essentia/utils/tnt/tnt_array2d.h>
#include <memory>
#include <rust/cxx.h>
#include <variant>
#include <vector>

namespace essentia_bridge {

enum class DataType : std::uint8_t;

struct SliceFloat;
struct MatrixFloat;
struct StereoSample;
struct VecString;
struct SliceStereoSample;

struct VariantData {
  using StorageType =
      std::variant<bool, std::string, float, int, unsigned int, std::int64_t,
                   essentia::StereoSample, std::vector<bool>, std::vector<int>,
                   std::vector<std::string>, std::vector<float>,
                   std::vector<essentia::StereoSample>,
                   std::vector<std::vector<float>>, TNT::Array2D<float>,
                   std::vector<std::vector<std::string>>,
                   std::vector<std::vector<essentia::StereoSample>>,
                   std::vector<TNT::Array2D<float>>>;
  StorageType data;

  VariantData() = default;

  template <typename T>
  explicit VariantData(T &&value) : data(std::forward<T>(value)) {}

  DataType get_data_type() const;

  bool get_bool() const;
  rust::String get_string() const;
  float get_float() const;
  int get_int() const;
  unsigned int get_unsigned_int() const;
  std::int64_t get_long() const;
  StereoSample get_stereo_sample() const;
  rust::Vec<bool> get_vector_bool() const;
  rust::Slice<const int> get_vector_int() const;
  rust::Vec<rust::String> get_vector_string() const;
  rust::Slice<const float> get_vector_float() const;
  rust::Slice<const StereoSample> get_vector_stereo_sample() const;
  rust::Vec<SliceFloat> get_vector_vector_float() const;
  MatrixFloat get_matrix_float() const;
  rust::Vec<VecString> get_vector_vector_string() const;
  rust::Vec<SliceStereoSample> get_vector_vector_stereo_sample() const;
  rust::Vec<MatrixFloat> get_vector_matrix_float() const;
};

std::unique_ptr<VariantData> create_variant_data_from_bool(bool value);
std::unique_ptr<VariantData> create_variant_data_from_string(rust::Str value);
std::unique_ptr<VariantData> create_variant_data_from_float(float value);
std::unique_ptr<VariantData> create_variant_data_from_int(int value);
std::unique_ptr<VariantData>
create_variant_data_from_unsigned_int(unsigned int value);
std::unique_ptr<VariantData> create_variant_data_from_long(std::int64_t value);
std::unique_ptr<VariantData>
create_variant_data_from_stereo_sample(StereoSample value);
std::unique_ptr<VariantData>
create_variant_data_from_vector_bool(rust::Slice<const bool> value);
std::unique_ptr<VariantData>
create_variant_data_from_vector_int(rust::Slice<const int> value);
std::unique_ptr<VariantData>
create_variant_data_from_vector_string(rust::Slice<const rust::Str> value);
std::unique_ptr<VariantData>
create_variant_data_from_vector_float(rust::Slice<const float> value);
std::unique_ptr<VariantData> create_variant_data_from_vector_stereo_sample(
    rust::Slice<const StereoSample> value);
std::unique_ptr<VariantData>
create_variant_data_from_vector_vector_float(rust::Vec<SliceFloat> value);
std::unique_ptr<VariantData>
create_variant_data_from_matrix_float(MatrixFloat value);
std::unique_ptr<VariantData>
create_variant_data_from_vector_vector_string(rust::Vec<VecString> value);
std::unique_ptr<VariantData>
create_variant_data_from_vector_vector_stereo_sample(
    rust::Vec<SliceStereoSample> value);
std::unique_ptr<VariantData>
create_variant_data_from_vector_matrix_float(rust::Vec<MatrixFloat> value);

} // namespace essentia_bridge