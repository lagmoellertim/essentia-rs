#include "algorithm.h"
#include "essentia_core/src/ffi.rs.h"
#include <stdexcept>

void AlgorithmBridge::setup_output(rust::Str output_name, IOType io_type)
{
    switch (io_type)
    {
    case IOType::Real:
        generic_setup_output<float>(output_name);
        break;

    case IOType::Int:
        generic_setup_output<int>(output_name);
        break;

    case IOType::UnsignedInt:
        generic_setup_output<unsigned int>(output_name);
        break;

    case IOType::Long:
        generic_setup_output<std::int64_t>(output_name);
        break;

    case IOType::VectorReal:
        generic_setup_output<std::vector<float>>(output_name);
        break;

    case IOType::VectorVectorReal:
        generic_setup_output<std::vector<std::vector<float>>>(output_name);
        break;

    case IOType::MatrixReal:
        generic_setup_output<TNT::Array2D<float>>(output_name);
        break;

    default:
        throw std::invalid_argument{"AlgorithmWrapper::setup_output: "
                                    "unsupported IOType value"};
    }
}
