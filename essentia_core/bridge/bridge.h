#pragma once

#include <essentia/algorithm.h>
#include "rust/cxx.h"
#include "algorithm.h"
#include "parameter_map.h"

void init_essentia();
void shutdown_essentia();

rust::Vec<rust::String> get_algorithm_names();
std::unique_ptr<AlgorithmBridge> create_algorithm(rust::Str algorithm_name);
std::unique_ptr<ParameterMapBridge> create_parameter_map();