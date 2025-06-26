#include "bridge.h"
#include <essentia/essentia.h>
#include <essentia/algorithmfactory.h>

using namespace essentia::standard;

void init_essentia()
{
    essentia::init();
}

void shutdown_essentia()
{
    essentia::shutdown();
}

rust::Vec<rust::String> get_algorithm_names()
{
    std::vector<std::string> algorithm_names = AlgorithmFactory::keys();
    rust::Vec<rust::String> result;
    result.reserve(algorithm_names.size());

    for (const auto &algorithm_name : algorithm_names)
    {
        result.push_back(rust::String(algorithm_name));
    }

    return result;
}

std::unique_ptr<AlgorithmBridge> create_algorithm(rust::Str algorithm_name)
{
    Algorithm *algorithm = AlgorithmFactory::create(std::string(algorithm_name));
    return std::make_unique<AlgorithmBridge>(algorithm);
}
