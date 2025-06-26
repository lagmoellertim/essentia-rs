#include "parameter_map.h"

ParameterMapBridge::ParameterMapBridge()
{
    _parameter_map = new essentia::ParameterMap();
}

ParameterMapBridge::~ParameterMapBridge()
{
    delete _parameter_map;
}

void ParameterMapBridge::add_string(rust::Str key, rust::Str value)
{
    std::string str_key(key);
    std::string str_value(value);
    _parameter_map->add(str_key, essentia::Parameter(str_value));
}

void ParameterMapBridge::add_real(rust::Str key, float value)
{
    std::string str_key(key);
    _parameter_map->add(str_key, essentia::Parameter(value));
}

void ParameterMapBridge::add_int(rust::Str key, int value)
{
    std::string str_key(key);
    _parameter_map->add(str_key, essentia::Parameter(value));
}

void ParameterMapBridge::add_bool(rust::Str key, bool value)
{
    std::string str_key(key);
    _parameter_map->add(str_key, essentia::Parameter(value));
}

const essentia::ParameterMap &ParameterMapBridge::get_parameter_map() const
{
    return *_parameter_map;
}

std::unique_ptr<ParameterMapBridge> create_parameter_map()
{
    return std::make_unique<ParameterMapBridge>();
}