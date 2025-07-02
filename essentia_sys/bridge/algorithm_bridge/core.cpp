#include "algorithm_bridge.h"

namespace essentia_bridge {

AlgorithmBridge::AlgorithmBridge(essentia::standard::Algorithm *algorithm)
    : _algorithm(algorithm) {}

AlgorithmBridge::~AlgorithmBridge() { delete _algorithm; }

void AlgorithmBridge::configure(
    std::unique_ptr<ParameterMapBridge> parameter_map_bridge) {
  _algorithm->configure(parameter_map_bridge->get_parameter_map());
}

void AlgorithmBridge::compute() { _algorithm->compute(); }

void AlgorithmBridge::reset() { _algorithm->reset(); }

} // namespace essentia_bridge