#include "essentia_core/src/ffi.rs.h"
#include "variant_data.h"
#include <cstring>
#include <memory>
#include <rust/cxx.h>
#include <string>

namespace essentia_bridge {

std::unique_ptr<VariantData> create_variant_data_from_bool(bool value) {
  return std::make_unique<VariantData>(value);
}

std::unique_ptr<VariantData> create_variant_data_from_string(rust::Str value) {
  return std::make_unique<VariantData>(std::string(value));
}

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
create_variant_data_from_stereo_sample(StereoSample value) {
  essentia::StereoSample sample;
  sample.first = value.left;
  sample.second = value.right;
  return std::make_unique<VariantData>(sample);
}

std::unique_ptr<VariantData>
create_variant_data_from_vector_bool(rust::Slice<const bool> value) {
  return std::make_unique<VariantData>(
      std::vector<bool>(value.begin(), value.end()));
}

std::unique_ptr<VariantData>
create_variant_data_from_vector_int(rust::Slice<const int> value) {
  return std::make_unique<VariantData>(
      std::vector<int>(value.begin(), value.end()));
}

std::unique_ptr<VariantData>
create_variant_data_from_vector_string(rust::Slice<const rust::Str> value) {
  std::vector<std::string> cpp_vec;
  cpp_vec.reserve(value.size());
  for (const auto &str : value) {
    cpp_vec.push_back(std::string(str));
  }
  return std::make_unique<VariantData>(std::move(cpp_vec));
}

std::unique_ptr<VariantData>
create_variant_data_from_vector_float(rust::Slice<const float> value) {
  return std::make_unique<VariantData>(
      std::vector<float>(value.begin(), value.end()));
}

std::unique_ptr<VariantData> create_variant_data_from_vector_stereo_sample(
    rust::Slice<const StereoSample> value) {
  std::vector<essentia::StereoSample> cpp_vec;
  cpp_vec.resize(value.size());

  static_assert(sizeof(StereoSample) == sizeof(essentia::StereoSample),
                "StereoSample sizes must match for memcpy");
  static_assert(std::is_trivially_copyable_v<StereoSample>,
                "StereoSample must be trivially copyable");
  static_assert(std::is_trivially_copyable_v<essentia::StereoSample>,
                "essentia::StereoSample must be trivially copyable");

  if (!value.empty()) {
    std::memcpy(cpp_vec.data(), value.data(),
                value.size() * sizeof(StereoSample));
  }

  return std::make_unique<VariantData>(std::move(cpp_vec));
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

std::unique_ptr<VariantData>
create_variant_data_from_vector_vector_string(rust::Vec<VecString> value) {
  std::vector<std::vector<std::string>> cpp_vec;
  cpp_vec.reserve(value.size());

  for (const auto &vec_string : value) {
    std::vector<std::string> inner_vec;
    inner_vec.reserve(vec_string.vec.size());
    for (const auto &str : vec_string.vec) {
      inner_vec.push_back(std::string(str));
    }
    cpp_vec.push_back(std::move(inner_vec));
  }

  return std::make_unique<VariantData>(std::move(cpp_vec));
}

std::unique_ptr<VariantData>
create_variant_data_from_vector_vector_stereo_sample(
    rust::Vec<SliceStereoSample> value) {
  std::vector<std::vector<essentia::StereoSample>> cpp_vec;
  cpp_vec.reserve(value.size());

  static_assert(sizeof(StereoSample) == sizeof(essentia::StereoSample),
                "StereoSample sizes must match for memcpy");
  static_assert(std::is_trivially_copyable_v<StereoSample>,
                "StereoSample must be trivially copyable");
  static_assert(std::is_trivially_copyable_v<essentia::StereoSample>,
                "essentia::StereoSample must be trivially copyable");

  for (const auto &slice_stereo_sample : value) {
    std::vector<essentia::StereoSample> inner_vec;
    inner_vec.resize(slice_stereo_sample.slice.size());

    if (!slice_stereo_sample.slice.empty()) {
      std::memcpy(inner_vec.data(), slice_stereo_sample.slice.data(),
                  slice_stereo_sample.slice.size() * sizeof(StereoSample));
    }

    cpp_vec.push_back(std::move(inner_vec));
  }

  return std::make_unique<VariantData>(std::move(cpp_vec));
}

std::unique_ptr<VariantData>
create_variant_data_from_vector_matrix_float(rust::Vec<MatrixFloat> value) {
  std::vector<TNT::Array2D<float>> cpp_vec;
  cpp_vec.reserve(value.size());

  for (const auto &matrix : value) {
    assert(matrix.slice.size() == matrix.dim1 * matrix.dim2);
    TNT::Array2D<float> array(matrix.dim1, matrix.dim2);
    std::memcpy(&array[0][0], matrix.slice.data(),
                matrix.slice.size() * sizeof(float));
    cpp_vec.push_back(std::move(array));
  }

  return std::make_unique<VariantData>(std::move(cpp_vec));
}

} // namespace essentia_bridge