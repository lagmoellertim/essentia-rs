#pragma once

#include "../data_container/data_container.h"
#include <essentia/parameter.h>
#include <rust/cxx.h>

namespace essentia_bridge {

class ParameterMapBridge {
public:
  ParameterMapBridge();
  ~ParameterMapBridge();

  void add(rust::Str key, std::unique_ptr<DataContainer> data_container);

  essentia::ParameterMap get_parameter_map();

private:
  essentia::ParameterMap *_parameter_map;
};

} // namespace essentia_bridge