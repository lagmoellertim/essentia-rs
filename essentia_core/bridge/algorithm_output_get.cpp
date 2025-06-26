#include "algorithm.h"
#include <cstring>
#include "essentia_core/src/ffi.rs.h"

float AlgorithmBridge::get_output_real(rust::Str output_name) const
{
    return std::get<float>(_outputs.at(std::string(output_name)));
}

int AlgorithmBridge::get_output_int(rust::Str output_name) const
{
    return std::get<int>(_outputs.at(std::string(output_name)));
}

unsigned int AlgorithmBridge::get_output_uint(rust::Str output_name) const
{
    return std::get<unsigned int>(_outputs.at(std::string(output_name)));
}

std::int64_t AlgorithmBridge::get_output_long(rust::Str output_name) const
{
    return std::get<std::int64_t>(_outputs.at(std::string(output_name)));
}

rust::Slice<const float> AlgorithmBridge::get_output_real_vector(rust::Str output_name) const
{
    const std::vector<float> &vec = std::get<std::vector<float>>(_outputs.at(std::string(output_name)));
    rust::Slice<const float> slice{vec.data(), vec.size()};

    return slice;
}

rust::Vec<FloatSlice> AlgorithmBridge::get_output_real_vector_vector(rust::Str output_name) const
{
    const auto &vec = std::get<std::vector<std::vector<float>>>(_outputs.at(std::string(output_name)));

    rust::Vec<FloatSlice> rust_vec;

    for (const auto &item : vec)
    {
        FloatSlice float_slice;
        float_slice.slice = rust::Slice{item.data(), item.size()};
        rust_vec.push_back(float_slice);
    }

    return rust_vec;
}

FloatMatrix2d AlgorithmBridge::get_output_real_matrix(rust::Str output_name) const
{
    const auto &matrix = std::get<TNT::Array2D<float>>(_outputs.at(std::string(output_name)));

    FloatMatrix2d rust_matrix;

    size_t dim1 = matrix.dim1();
    size_t dim2 = matrix.dim2();

    rust_matrix.dim1 = dim1;
    rust_matrix.dim2 = dim2;
    rust_matrix.slice = rust::Slice<const float>(&matrix[0][0], dim1 * dim2);

    return rust_matrix;
}