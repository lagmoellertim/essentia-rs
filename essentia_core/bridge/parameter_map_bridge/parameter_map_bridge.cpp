#include "parameter_map_bridge.h"

namespace essentia_bridge {

ParameterMapBridge::ParameterMapBridge() {
  _parameter_map = new essentia::ParameterMap();
}

ParameterMapBridge::~ParameterMapBridge() { delete _parameter_map; }

void ParameterMapBridge::add(rust::Str key,
                             std::unique_ptr<VariantData> variant_data) {
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
      map->add(key, essentia::Parameter(static_cast<int>(value)));
    }
    void operator()(std::int64_t value) {
      map->add(key, essentia::Parameter(static_cast<int>(value)));
    }
    void operator()(const essentia::StereoSample &value) {
      map->add(key, essentia::Parameter(value));
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
    void operator()(const std::vector<TNT::Array2D<float>> &value) {
      map->add(key, essentia::Parameter(value));
    }
    void operator()(const TNT::Array2D<float> &value) {
      map->add(key, essentia::Parameter(value));
    }
  };

  std::visit(Visitor{_parameter_map, str_key}, variant_data->data);
}

std::unique_ptr<ParameterMapBridge> create_parameter_map() {
  return std::make_unique<ParameterMapBridge>();
}

const essentia::ParameterMap &ParameterMapBridge::get_parameter_map() const {
  return *_parameter_map;
}

} // namespace essentia_bridge