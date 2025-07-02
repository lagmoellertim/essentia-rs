#include "type_mapping.h"
#include "essentia-sys/src/lib.rs.h"
#include <complex>
#include <essentia/pool.h>
#include <essentia/types.h>
#include <essentia/utils/tnt/tnt_array2d.h>
#include <map>
#include <stdexcept>
#include <string>
#include <typeinfo>
#include <unsupported/Eigen/CXX11/Tensor>
#include <vector>

namespace essentia_bridge {

const std::unordered_map<std::type_index, DataType> &get_type_mapping() {
  static const std::unordered_map<std::type_index, DataType> type_map = {
      {typeid(bool), DataType::Bool},
      {typeid(std::string), DataType::String},
      {typeid(float), DataType::Float},
      {typeid(int), DataType::Int},
      {typeid(unsigned int), DataType::UnsignedInt},
      {typeid(long), DataType::Long},
      {typeid(essentia::StereoSample), DataType::StereoSample},
      {typeid(std::complex<essentia::Real>), DataType::Complex},
      {typeid(essentia::Tensor<essentia::Real>), DataType::TensorFloat},
      {typeid(std::vector<bool>), DataType::VectorBool},
      {typeid(std::vector<int>), DataType::VectorInt},
      {typeid(std::vector<std::string>), DataType::VectorString},
      {typeid(std::vector<float>), DataType::VectorFloat},
      {typeid(std::vector<essentia::StereoSample>),
       DataType::VectorStereoSample},
      {typeid(std::vector<std::complex<essentia::Real>>),
       DataType::VectorComplex},
      {typeid(std::vector<std::vector<float>>), DataType::VectorVectorFloat},
      {typeid(std::vector<std::vector<std::string>>),
       DataType::VectorVectorString},
      {typeid(std::vector<std::vector<essentia::StereoSample>>),
       DataType::VectorVectorStereoSample},
      {typeid(std::vector<std::vector<std::complex<essentia::Real>>>),
       DataType::VectorVectorComplex},
      {typeid(std::vector<TNT::Array2D<float>>), DataType::VectorMatrixFloat},
      {typeid(TNT::Array2D<float>), DataType::MatrixFloat},
      {typeid(std::map<std::string, std::vector<float>>),
       DataType::MapVectorFloat},
      {typeid(std::map<std::string, std::vector<std::string>>),
       DataType::MapVectorString},
      {typeid(std::map<std::string, std::vector<int>>), DataType::MapVectorInt},
      {typeid(std::map<std::string, std::vector<std::complex<essentia::Real>>>),
       DataType::MapVectorComplex},
      {typeid(std::map<std::string, float>), DataType::MapFloat},
      {typeid(essentia::Pool), DataType::Pool}};
  return type_map;
}

DataType type_info_to_data_type(const std::type_info &type_info) {
  const auto &type_map = get_type_mapping();
  const auto it = type_map.find(type_info);

  if (it == type_map.end()) {
    throw std::invalid_argument("type_info_to_data_type: unsupported type (" +
                                std::string(type_info.name()) + ')');
  }

  return it->second;
}

} // namespace essentia_bridge