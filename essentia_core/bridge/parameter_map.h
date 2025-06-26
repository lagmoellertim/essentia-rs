#pragma once

#include "rust/cxx.h"
#include <essentia/parameter.h>

class ParameterMapBridge
{
public:
    ParameterMapBridge();
    ~ParameterMapBridge();

    void add_string(rust::Str key, rust::Str value);
    void add_real(rust::Str key, float value);
    void add_int(rust::Str key, int value);
    void add_bool(rust::Str key, bool value);

    const essentia::ParameterMap &get_parameter_map() const;

private:
    essentia::ParameterMap *_parameter_map;
};