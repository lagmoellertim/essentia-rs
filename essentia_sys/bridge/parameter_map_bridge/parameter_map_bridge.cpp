#include "parameter_map_bridge.h"
#include <stdexcept>

namespace essentia_bridge {

ParameterMapBridge::ParameterMapBridge() {
  _parameter_map = new essentia::ParameterMap();
}

ParameterMapBridge::~ParameterMapBridge() { delete _parameter_map; }

void ParameterMapBridge::add(rust::Str key,
                             std::unique_ptr<DataContainer> data_container) {
  std::string str_key(key);

  struct Visitor {
    essentia::ParameterMap *map;
    const std::string &key;

    void operator()(bool value) { map->add(key, essentia::Parameter(value)); }
    void operator()(const std::string &value) {
      map->add(key, essentia::Parameter(value));
    }
    void operator()(float value) { map->add(key, essentia::Parameter(value)); }
    void operator()(int value) { map->add(key, essentia::Parameter(value)); }
    void operator()(unsigned int value) {
      throw std::runtime_error("Unsigned Int is not supported as parameter");
    }
    void operator()(std::int64_t value) {
      throw std::runtime_error("Long is not supported as parameter");
    }
    void operator()(const essentia::StereoSample &value) {
      map->add(key, essentia::Parameter(value));
    }
    void operator()(const std::complex<essentia::Real> &value) {
      throw std::runtime_error("Complex types are not supported as parameters");
    }
    void operator()(const std::vector<bool> &value) {
      map->add(key, essentia::Parameter(value));
    }
    void operator()(const std::vector<int> &value) {
      map->add(key, essentia::Parameter(value));
    }
    void operator()(const std::vector<std::string> &value) {
      map->add(key, essentia::Parameter(value));
    }
    void operator()(const std::vector<float> &value) {
      map->add(key, essentia::Parameter(value));
    }
    void operator()(const std::vector<essentia::StereoSample> &value) {
      map->add(key, essentia::Parameter(value));
    }
    void operator()(const std::vector<std::complex<essentia::Real>> &value) {
      throw std::runtime_error("Complex types are not supported as parameters");
    }
    void operator()(const std::vector<std::vector<float>> &value) {
      map->add(key, essentia::Parameter(value));
    }
    void operator()(const std::vector<std::vector<std::string>> &value) {
      map->add(key, essentia::Parameter(value));
    }
    void
    operator()(const std::vector<std::vector<essentia::StereoSample>> &value) {
      map->add(key, essentia::Parameter(value));
    }
    void operator()(
        const std::vector<std::vector<std::complex<essentia::Real>>> &value) {
      throw std::runtime_error("Complex types are not supported as parameters");
    }
    void operator()(const std::vector<TNT::Array2D<float>> &value) {
      map->add(key, essentia::Parameter(value));
    }
    void operator()(const TNT::Array2D<float> &value) {
      map->add(key, essentia::Parameter(value));
    }
    void operator()(const std::map<std::string, std::vector<float>> &value) {
      map->add(key, essentia::Parameter(value));
    }
    void
    operator()(const std::map<std::string, std::vector<std::string>> &value) {
      map->add(key, essentia::Parameter(value));
    }
    void operator()(const std::map<std::string, std::vector<int>> &value) {
      map->add(key, essentia::Parameter(value));
    }
    void operator()(
        const std::map<std::string, std::vector<std::complex<essentia::Real>>>
            &value) {
      throw std::runtime_error("Complex types are not supported as parameters");
    }
    void operator()(const std::map<std::string, float> &value) {
      map->add(key, essentia::Parameter(value));
    }
    void operator()(const essentia::Pool &value) {
      throw std::runtime_error("Pool is not supported in ParameterMap");
    }
    void operator()(const essentia::Tensor<essentia::Real> &value) {
      throw std::runtime_error("Tensor types are not supported as parameters");
    }
  };

  std::visit(Visitor{_parameter_map, str_key}, data_container->data);
}

std::unique_ptr<ParameterMapBridge> create_parameter_map_bridge() {
  return std::make_unique<ParameterMapBridge>();
}

essentia::ParameterMap ParameterMapBridge::get_parameter_map() {
  essentia::ParameterMap result = std::move(*_parameter_map);
  delete _parameter_map;
  _parameter_map = nullptr;
  return result;
}

} // namespace essentia_bridge