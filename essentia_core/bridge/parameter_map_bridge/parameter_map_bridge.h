#pragma once

#include "../variant_data/variant_data.h"
#include <essentia/parameter.h>
#include <rust/cxx.h>

namespace essentia_bridge {

class ParameterMapBridge {
public:
  ParameterMapBridge();
  ~ParameterMapBridge();

  void add(rust::Str key, std::unique_ptr<VariantData> variant_data);

  const essentia::ParameterMap &get_parameter_map() const;

private:
  essentia::ParameterMap *_parameter_map;
};

} // namespace essentia_bridge