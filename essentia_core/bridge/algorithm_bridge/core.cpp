#include "algorithm_bridge.h"

namespace essentia_bridge {

AlgorithmBridge::AlgorithmBridge(essentia::standard::Algorithm *algorithm)
    : _algorithm(algorithm) {}

AlgorithmBridge::~AlgorithmBridge() { delete _algorithm; }

void AlgorithmBridge::configure(
    const ParameterMapBridge &parameter_map_wrapper) {
  _algorithm->configure(parameter_map_wrapper.get_parameter_map());
}

void AlgorithmBridge::compute() { _algorithm->compute(); }

void AlgorithmBridge::reset() { _algorithm->reset(); }

} // namespace essentia_bridge