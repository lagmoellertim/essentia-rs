#pragma once

#include "algorithm_bridge/algorithm_bridge.h"
#include "parameter_map_bridge/parameter_map_bridge.h"
#include "rust/cxx.h"
#include "variant_data/variant_data.h"
#include <essentia/algorithm.h>
#include <essentia/utils/tnt/tnt_array2d.h>
#include <memory>

// TODO: Add better error handling

namespace essentia_bridge
{

    void init_essentia();
    void shutdown_essentia();

    rust::Vec<rust::String> get_algorithm_names();
    std::unique_ptr<AlgorithmBridge> create_algorithm_bridge(rust::Str algorithm_name);
    std::unique_ptr<ParameterMapBridge> create_parameter_map_bridge();

} // namespace essentia_bridge
