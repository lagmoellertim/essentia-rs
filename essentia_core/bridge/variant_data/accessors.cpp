#include "essentia_core/src/ffi.rs.h"
#include "variant_data.h"
#include <cstring>
#include <rust/cxx.h>
#include <string>

namespace essentia_bridge {

bool VariantData::get_bool() const { return std::get<bool>(data); }

rust::String VariantData::get_string() const {
  return rust::String(std::get<std::string>(data));
}

float VariantData::get_float() const { return std::get<float>(data); }

int VariantData::get_int() const { return std::get<int>(data); }

unsigned int VariantData::get_unsigned_int() const {
  return std::get<unsigned int>(data);
}

std::int64_t VariantData::get_long() const {
  return std::get<std::int64_t>(data);
}

StereoSample VariantData::get_stereo_sample() const {
  const essentia::StereoSample &sample = std::get<essentia::StereoSample>(data);
  return StereoSample{sample.first, sample.second};
}

rust::Vec<bool> VariantData::get_vector_bool() const {
  const std::vector<bool> &vec = std::get<std::vector<bool>>(data);
  rust::Vec<bool> result;
  result.reserve(vec.size());
  for (bool value : vec) {
    result.push_back(value);
  }
  return result;
}

rust::Slice<const int> VariantData::get_vector_int() const {
  const std::vector<int> &vec = std::get<std::vector<int>>(data);
  return rust::Slice<const int>(vec.data(), vec.size());
}

rust::Vec<rust::String> VariantData::get_vector_string() const {
  const std::vector<std::string> &vec =
      std::get<std::vector<std::string>>(data);
  rust::Vec<rust::String> result;
  result.reserve(vec.size());
  for (const auto &str : vec) {
    result.push_back(rust::String(str));
  }
  return result;
}

rust::Slice<const float> VariantData::get_vector_float() const {
  const std::vector<float> &vec = std::get<std::vector<float>>(data);
  rust::Slice<const float> slice{vec.data(), vec.size()};

  return slice;
}

rust::Slice<const StereoSample> VariantData::get_vector_stereo_sample() const {
  const auto &vec = std::get<std::vector<essentia::StereoSample>>(data);

  static_assert(sizeof(StereoSample) == sizeof(essentia::StereoSample),
                "StereoSample sizes must match for direct casting");
  static_assert(std::is_trivially_copyable_v<StereoSample>,
                "StereoSample must be trivially copyable");
  static_assert(std::is_trivially_copyable_v<essentia::StereoSample>,
                "essentia::StereoSample must be trivially copyable");

  return rust::Slice<const StereoSample>(
      reinterpret_cast<const StereoSample *>(vec.data()), vec.size());
}

rust::Vec<SliceFloat> VariantData::get_vector_vector_float() const {
  const auto &vec = std::get<std::vector<std::vector<float>>>(data);

  rust::Vec<SliceFloat> rust_vec;
  rust_vec.reserve(vec.size());

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

rust::Vec<VecString> VariantData::get_vector_vector_string() const {
  const auto &vec = std::get<std::vector<std::vector<std::string>>>(data);

  rust::Vec<VecString> rust_vec;
  rust_vec.reserve(vec.size());

  for (const auto &string_vec : vec) {
    VecString vec_string;
    vec_string.vec = rust::Vec<rust::String>();
    vec_string.vec.reserve(string_vec.size());
    for (const auto &str : string_vec) {
      vec_string.vec.push_back(rust::String(str));
    }
    rust_vec.push_back(vec_string);
  }

  return rust_vec;
}

rust::Vec<SliceStereoSample>
VariantData::get_vector_vector_stereo_sample() const {
  const auto &vec =
      std::get<std::vector<std::vector<essentia::StereoSample>>>(data);

  rust::Vec<SliceStereoSample> rust_vec;
  rust_vec.reserve(vec.size());

  static_assert(sizeof(StereoSample) == sizeof(essentia::StereoSample),
                "StereoSample sizes must match for direct casting");
  static_assert(std::is_trivially_copyable_v<StereoSample>,
                "StereoSample must be trivially copyable");
  static_assert(std::is_trivially_copyable_v<essentia::StereoSample>,
                "essentia::StereoSample must be trivially copyable");

  for (const auto &stereo_vec : vec) {
    SliceStereoSample slice_stereo_sample;
    slice_stereo_sample.slice = rust::Slice<const StereoSample>(
        reinterpret_cast<const StereoSample *>(stereo_vec.data()),
        stereo_vec.size());
    rust_vec.push_back(slice_stereo_sample);
  }

  return rust_vec;
}

rust::Vec<MatrixFloat> VariantData::get_vector_matrix_float() const {
  const auto &vec = std::get<std::vector<TNT::Array2D<float>>>(data);

  rust::Vec<MatrixFloat> rust_vec;
  rust_vec.reserve(vec.size());

  for (const auto &matrix : vec) {
    MatrixFloat rust_matrix;

    size_t dim1 = matrix.dim1();
    size_t dim2 = matrix.dim2();

    rust_matrix.dim1 = dim1;
    rust_matrix.dim2 = dim2;
    rust_matrix.slice = rust::Slice<const float>(&matrix[0][0], dim1 * dim2);

    rust_vec.push_back(rust_matrix);
  }

  return rust_vec;
}

} // namespace essentia_bridge