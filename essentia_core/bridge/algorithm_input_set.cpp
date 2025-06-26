#include "algorithm.h"
#include "essentia_core/src/ffi.rs.h"

void AlgorithmBridge::set_input_real(rust::Str input_name, float value)
{
    set_and_store_input(input_name, value);
}

void AlgorithmBridge::set_input_int(rust::Str input_name, int value)
{
    set_and_store_input(input_name, value);
}

void AlgorithmBridge::set_input_uint(rust::Str input_name, unsigned int value)
{
    set_and_store_input(input_name, value);
}

void AlgorithmBridge::set_input_long(rust::Str input_name, std::int64_t value)
{
    set_and_store_input(input_name, value);
}

void AlgorithmBridge::set_input_real_vector(rust::Str input_name, rust::Slice<const float> value)
{
    set_and_store_input(input_name, std::vector<float>(value.begin(), value.end()));
}

void AlgorithmBridge::set_input_real_vector_vector(rust::Str input_name, rust::Vec<FloatSlice> value)
{
    std::vector<std::vector<float>> cpp_vec;
    cpp_vec.reserve(value.size());

    for (const auto &float_slice : value)
    {
        const auto *data = float_slice.slice.data();
        size_t size = float_slice.slice.size();

        std::vector<float> inner_vec(data, data + size);
        cpp_vec.push_back(std::move(inner_vec));
    }

    set_and_store_input(input_name, cpp_vec);
}

void AlgorithmBridge::set_input_real_matrix(rust::Str input_name, FloatMatrix2d value)
{
    assert(value.slice.size() == value.dim1 * value.dim2);

    TNT::Array2D<float> array(value.dim1, value.dim2);

    std::memcpy(&array[0][0], value.slice.data(), value.slice.size() * sizeof(float));

    set_and_store_input(input_name, array);
}