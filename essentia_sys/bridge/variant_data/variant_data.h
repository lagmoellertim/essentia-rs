#pragma once

#include <complex>
#include <essentia/pool.h>
#include <essentia/types.h>
#include <essentia/utils/tnt/tnt_array2d.h>
#include <map>
#include <memory>
#include <rust/cxx.h>
#include <unsupported/Eigen/CXX11/Tensor>
#include <variant>
#include <vector>

namespace essentia_bridge {

enum class DataType : std::uint8_t;

struct SliceFloat;
struct MatrixFloat;
struct TensorFloat;
struct StereoSample;
struct Complex;
struct VecString;
struct SliceStereoSample;
struct SliceComplex;
struct VecComplex;
struct MapEntryVectorFloat;
struct MapEntryVectorString;
struct MapEntryVectorInt;
struct MapEntryVectorComplex;
struct MapEntryFloat;
class PoolBridge;

struct VariantData {
  using StorageType = std::variant<
      bool, std::string, float, int, unsigned int, std::int64_t,
      essentia::StereoSample, std::complex<essentia::Real>, std::vector<bool>,
      std::vector<int>, std::vector<std::string>, std::vector<float>,
      std::vector<essentia::StereoSample>,
      std::vector<std::complex<essentia::Real>>,
      std::vector<std::vector<float>>, TNT::Array2D<float>,
      essentia::Tensor<essentia::Real>, std::vector<std::vector<std::string>>,
      std::vector<std::vector<essentia::StereoSample>>,
      std::vector<std::vector<std::complex<essentia::Real>>>,
      std::vector<TNT::Array2D<float>>,
      std::map<std::string, std::vector<float>>,
      std::map<std::string, std::vector<std::string>>,
      std::map<std::string, std::vector<int>>,
      std::map<std::string, std::vector<std::complex<essentia::Real>>>,
      std::map<std::string, float>, essentia::Pool>;
  StorageType data;

  mutable std::unique_ptr<PoolBridge> pool_bridge_cache;

  VariantData();
  ~VariantData();
  VariantData(const VariantData &other);
  VariantData &operator=(const VariantData &other);
  VariantData(VariantData &&other) noexcept;
  VariantData &operator=(VariantData &&other) noexcept;

  template <typename T>
  explicit VariantData(T &&value) : data(std::forward<T>(value)) {}

public:
  DataType get_data_type() const;

  bool get_bool() const;
  rust::String get_string() const;
  float get_float() const;
  int get_int() const;
  unsigned int get_unsigned_int() const;
  std::int64_t get_long() const;
  StereoSample get_stereo_sample() const;
  Complex get_complex() const;
  rust::Vec<bool> get_vector_bool() const;
  rust::Slice<const int> get_vector_int() const;
  rust::Vec<rust::String> get_vector_string() const;
  rust::Slice<const float> get_vector_float() const;
  rust::Slice<const StereoSample> get_vector_stereo_sample() const;
  rust::Slice<const Complex> get_vector_complex() const;
  rust::Vec<SliceFloat> get_vector_vector_float() const;
  MatrixFloat get_matrix_float() const;
  TensorFloat get_tensor_float() const;
  rust::Vec<VecString> get_vector_vector_string() const;
  rust::Vec<SliceStereoSample> get_vector_vector_stereo_sample() const;
  rust::Vec<VecComplex> get_vector_vector_complex() const;
  rust::Vec<MatrixFloat> get_vector_matrix_float() const;
  rust::Vec<MapEntryVectorFloat> get_map_vector_float() const;
  rust::Vec<MapEntryVectorString> get_map_vector_string() const;
  rust::Vec<MapEntryVectorInt> get_map_vector_int() const;
  rust::Vec<MapEntryVectorComplex> get_map_vector_complex() const;
  rust::Vec<MapEntryFloat> get_map_float() const;
  const PoolBridge &get_pool() const;
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
std::unique_ptr<VariantData> create_variant_data_from_complex(Complex value);
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
create_variant_data_from_vector_complex(rust::Slice<const Complex> value);
std::unique_ptr<VariantData>
create_variant_data_from_vector_vector_float(rust::Vec<SliceFloat> value);
std::unique_ptr<VariantData>
create_variant_data_from_matrix_float(MatrixFloat value);
std::unique_ptr<VariantData>
create_variant_data_from_tensor_float(TensorFloat value);
std::unique_ptr<VariantData>
create_variant_data_from_vector_vector_string(rust::Vec<VecString> value);
std::unique_ptr<VariantData>
create_variant_data_from_vector_vector_stereo_sample(
    rust::Vec<SliceStereoSample> value);
std::unique_ptr<VariantData>
create_variant_data_from_vector_vector_complex(rust::Vec<VecComplex> value);
std::unique_ptr<VariantData>
create_variant_data_from_vector_matrix_float(rust::Vec<MatrixFloat> value);
std::unique_ptr<VariantData>
create_variant_data_from_map_vector_float(rust::Vec<MapEntryVectorFloat> value);
std::unique_ptr<VariantData> create_variant_data_from_map_vector_string(
    rust::Vec<MapEntryVectorString> value);
std::unique_ptr<VariantData>
create_variant_data_from_map_vector_int(rust::Vec<MapEntryVectorInt> value);
std::unique_ptr<VariantData> create_variant_data_from_map_vector_complex(
    rust::Vec<MapEntryVectorComplex> value);
std::unique_ptr<VariantData>
create_variant_data_from_map_float(rust::Vec<MapEntryFloat> value);
std::unique_ptr<VariantData>
create_variant_data_from_pool(std::unique_ptr<PoolBridge> pool_bridge);

} // namespace essentia_bridge