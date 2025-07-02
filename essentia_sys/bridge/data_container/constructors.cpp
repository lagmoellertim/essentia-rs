#include "../pool_bridge/pool_bridge.h"
#include "data_container.h"
#include "essentia-sys/src/lib.rs.h"
#include <cassert>
#include <complex>
#include <cstring>
#include <essentia/types.h>
#include <essentia/utils/tnt/tnt_array2d.h>
#include <map>
#include <memory>
#include <rust/cxx.h>
#include <stdexcept>
#include <string>
#include <unsupported/Eigen/CXX11/Tensor>
#include <vector>

namespace essentia_bridge {

std::unique_ptr<DataContainer> create_data_container_from_bool(bool value) {
  return std::make_unique<DataContainer>(value);
}

std::unique_ptr<DataContainer>
create_data_container_from_string(rust::Str value) {
  return std::make_unique<DataContainer>(std::string(value));
}

std::unique_ptr<DataContainer> create_data_container_from_float(float value) {
  return std::make_unique<DataContainer>(value);
}

std::unique_ptr<DataContainer> create_data_container_from_int(int value) {
  return std::make_unique<DataContainer>(value);
}

std::unique_ptr<DataContainer>
create_data_container_from_unsigned_int(unsigned int value) {
  return std::make_unique<DataContainer>(value);
}

std::unique_ptr<DataContainer>
create_data_container_from_long(std::int64_t value) {
  return std::make_unique<DataContainer>(value);
}

std::unique_ptr<DataContainer>
create_data_container_from_stereo_sample(StereoSample value) {
  essentia::StereoSample sample;
  sample.first = value.left;
  sample.second = value.right;
  return std::make_unique<DataContainer>(sample);
}

std::unique_ptr<DataContainer>
create_data_container_from_vector_bool(rust::Slice<const bool> value) {
  return std::make_unique<DataContainer>(
      std::vector<bool>(value.begin(), value.end()));
}

std::unique_ptr<DataContainer>
create_data_container_from_vector_int(rust::Slice<const int> value) {
  return std::make_unique<DataContainer>(
      std::vector<int>(value.begin(), value.end()));
}

std::unique_ptr<DataContainer>
create_data_container_from_vector_string(rust::Slice<const rust::Str> value) {
  std::vector<std::string> cpp_vec;
  cpp_vec.reserve(value.size());
  for (const auto &str : value) {
    cpp_vec.push_back(std::string(str));
  }
  return std::make_unique<DataContainer>(std::move(cpp_vec));
}

std::unique_ptr<DataContainer>
create_data_container_from_vector_float(rust::Slice<const float> value) {
  return std::make_unique<DataContainer>(
      std::vector<float>(value.begin(), value.end()));
}

std::unique_ptr<DataContainer> create_data_container_from_vector_stereo_sample(
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

  return std::make_unique<DataContainer>(std::move(cpp_vec));
}

std::unique_ptr<DataContainer>
create_data_container_from_vector_vector_float(rust::Vec<SliceFloat> value) {
  std::vector<std::vector<float>> cpp_vec;
  cpp_vec.reserve(value.size());

  for (const auto &float_slice : value) {
    const auto *data = float_slice.slice.data();
    size_t size = float_slice.slice.size();
    cpp_vec.push_back(std::vector<float>(data, data + size));
  }

  return std::make_unique<DataContainer>(std::move(cpp_vec));
}

std::unique_ptr<DataContainer>
create_data_container_from_matrix_float(MatrixFloat value) {
  assert(value.slice.size() == value.dim1 * value.dim2);
  TNT::Array2D<float> array(value.dim1, value.dim2);
  std::memcpy(&array[0][0], value.slice.data(),
              value.slice.size() * sizeof(float));
  return std::make_unique<DataContainer>(std::move(array));
}

std::unique_ptr<DataContainer>
create_data_container_from_tensor_float(TensorFloat value) {
  // Essentia requires all tensors to be exactly 4D (TENSORRANK = 4)
  if (value.shape.size() != 4) {
    throw std::invalid_argument("Tensor must be exactly 4-dimensional. Got " +
                                std::to_string(value.shape.size()) +
                                " dimensions. ");
  }

  essentia::Tensor<essentia::Real> tensor(
      static_cast<long>(value.shape[0]), static_cast<long>(value.shape[1]),
      static_cast<long>(value.shape[2]), static_cast<long>(value.shape[3]));

  size_t total_size = 1;
  for (size_t dim : value.shape) {
    total_size *= dim;
  }

  if (value.slice.size() != total_size) {
    throw std::invalid_argument(
        "Tensor data size (" + std::to_string(value.slice.size()) +
        ") doesn't match expected size (" + std::to_string(total_size) + ")");
  }
  static_assert(sizeof(essentia::Real) == sizeof(float),
                "essentia::Real and float sizes must match for memcpy");

  std::memcpy(tensor.data(), value.slice.data(),
              value.slice.size() * sizeof(float));

  return std::make_unique<DataContainer>(std::move(tensor));
}

std::unique_ptr<DataContainer>
create_data_container_from_vector_vector_string(rust::Vec<VecString> value) {
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

  return std::make_unique<DataContainer>(std::move(cpp_vec));
}

std::unique_ptr<DataContainer>
create_data_container_from_vector_vector_stereo_sample(
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

  return std::make_unique<DataContainer>(std::move(cpp_vec));
}

std::unique_ptr<DataContainer>
create_data_container_from_vector_matrix_float(rust::Vec<MatrixFloat> value) {
  std::vector<TNT::Array2D<float>> cpp_vec;
  cpp_vec.reserve(value.size());

  for (const auto &matrix : value) {
    assert(matrix.slice.size() == matrix.dim1 * matrix.dim2);
    TNT::Array2D<float> array(matrix.dim1, matrix.dim2);
    std::memcpy(&array[0][0], matrix.slice.data(),
                matrix.slice.size() * sizeof(float));
    cpp_vec.push_back(std::move(array));
  }

  return std::make_unique<DataContainer>(std::move(cpp_vec));
}

std::unique_ptr<DataContainer> create_data_container_from_map_vector_float(
    rust::Vec<MapEntryVectorFloat> value) {
  std::map<std::string, std::vector<float>> cpp_map;

  for (const auto &entry : value) {
    std::string key(entry.key);
    const auto *data = entry.value.data();
    size_t size = entry.value.size();
    cpp_map[key] = std::vector<float>(data, data + size);
  }

  return std::make_unique<DataContainer>(std::move(cpp_map));
}

std::unique_ptr<DataContainer> create_data_container_from_map_vector_string(
    rust::Vec<MapEntryVectorString> value) {
  std::map<std::string, std::vector<std::string>> cpp_map;

  for (const auto &entry : value) {
    std::string key(entry.key);
    std::vector<std::string> values;
    values.reserve(entry.value.size());
    for (const auto &str : entry.value) {
      values.push_back(std::string(str));
    }
    cpp_map[key] = std::move(values);
  }

  return std::make_unique<DataContainer>(std::move(cpp_map));
}

std::unique_ptr<DataContainer>
create_data_container_from_map_vector_int(rust::Vec<MapEntryVectorInt> value) {
  std::map<std::string, std::vector<int>> cpp_map;

  for (const auto &entry : value) {
    std::string key(entry.key);
    const auto *data = entry.value.data();
    size_t size = entry.value.size();
    cpp_map[key] = std::vector<int>(data, data + size);
  }

  return std::make_unique<DataContainer>(std::move(cpp_map));
}

std::unique_ptr<DataContainer> create_data_container_from_map_vector_complex(
    rust::Vec<MapEntryVectorComplex> value) {
  std::map<std::string, std::vector<std::complex<essentia::Real>>> cpp_map;

  static_assert(sizeof(Complex) == sizeof(std::complex<essentia::Real>),
                "Complex sizes must match for memcpy");
  static_assert(std::is_trivially_copyable_v<Complex>,
                "Complex must be trivially copyable");
  static_assert(std::is_trivially_copyable_v<std::complex<essentia::Real>>,
                "std::complex<essentia::Real> must be trivially copyable");

  for (const auto &entry : value) {
    std::string key(entry.key);
    std::vector<std::complex<essentia::Real>> values;
    values.resize(entry.value.size());

    if (!entry.value.empty()) {
      std::memcpy(values.data(), entry.value.data(),
                  entry.value.size() * sizeof(Complex));
    }

    cpp_map[key] = std::move(values);
  }

  return std::make_unique<DataContainer>(std::move(cpp_map));
}

std::unique_ptr<DataContainer>
create_data_container_from_map_float(rust::Vec<MapEntryFloat> value) {
  std::map<std::string, float> cpp_map;

  for (const auto &entry : value) {
    cpp_map[std::string(entry.key)] = entry.value;
  }

  return std::make_unique<DataContainer>(std::move(cpp_map));
}

std::unique_ptr<DataContainer>
create_data_container_from_complex(Complex value) {
  std::complex<essentia::Real> complex_val(value.real, value.imag);
  return std::make_unique<DataContainer>(complex_val);
}

std::unique_ptr<DataContainer>
create_data_container_from_vector_complex(rust::Slice<const Complex> value) {
  std::vector<std::complex<essentia::Real>> cpp_vec;
  cpp_vec.resize(value.size());

  static_assert(sizeof(Complex) == sizeof(std::complex<essentia::Real>),
                "Complex sizes must match for memcpy");
  static_assert(std::is_trivially_copyable_v<Complex>,
                "Complex must be trivially copyable");
  static_assert(std::is_trivially_copyable_v<std::complex<essentia::Real>>,
                "std::complex<essentia::Real> must be trivially copyable");

  if (!value.empty()) {
    std::memcpy(cpp_vec.data(), value.data(), value.size() * sizeof(Complex));
  }

  return std::make_unique<DataContainer>(std::move(cpp_vec));
}

std::unique_ptr<DataContainer>
create_data_container_from_vector_vector_complex(rust::Vec<VecComplex> value) {
  std::vector<std::vector<std::complex<essentia::Real>>> cpp_vec;
  cpp_vec.reserve(value.size());

  static_assert(sizeof(Complex) == sizeof(std::complex<essentia::Real>),
                "Complex sizes must match for memcpy");
  static_assert(std::is_trivially_copyable_v<Complex>,
                "Complex must be trivially copyable");
  static_assert(std::is_trivially_copyable_v<std::complex<essentia::Real>>,
                "std::complex<essentia::Real> must be trivially copyable");

  for (const auto &vec_complex : value) {
    std::vector<std::complex<essentia::Real>> inner_vec;
    inner_vec.resize(vec_complex.vec.size());

    if (!vec_complex.vec.empty()) {
      std::memcpy(inner_vec.data(), vec_complex.vec.data(),
                  vec_complex.vec.size() * sizeof(Complex));
    }

    cpp_vec.push_back(std::move(inner_vec));
  }

  return std::make_unique<DataContainer>(std::move(cpp_vec));
}

std::unique_ptr<DataContainer>
create_data_container_from_pool(std::unique_ptr<PoolBridge> pool_bridge) {
  essentia::Pool pool = pool_bridge->into_pool();
  return std::make_unique<DataContainer>(std::move(pool));
}

} // namespace essentia_bridge