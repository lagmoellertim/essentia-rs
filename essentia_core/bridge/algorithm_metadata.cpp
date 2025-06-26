#include "algorithm.h"
#include "essentia_core/src/ffi.rs.h"
#include "essentia/algorithmfactory.h"
#include <typeindex>
#include <typeinfo>
#include <cstring>

struct EssentiaParamTypeHasher
{
    std::size_t operator()(essentia::Parameter::ParamType p) const noexcept
    {
        using U = std::underlying_type_t<essentia::Parameter::ParamType>;
        return static_cast<std::size_t>(static_cast<U>(p));
    }
};

static const std::unordered_map<essentia::Parameter::ParamType, ParameterType, EssentiaParamTypeHasher> parameterTypeLookupMap = {
    {essentia::Parameter::REAL, ParameterType::Real},
    {essentia::Parameter::STRING, ParameterType::String},
    {essentia::Parameter::BOOL, ParameterType::Bool},
    {essentia::Parameter::INT, ParameterType::Int},
    {essentia::Parameter::STEREOSAMPLE, ParameterType::StereoSample},
    {essentia::Parameter::VECTOR_REAL, ParameterType::VectorReal},
    {essentia::Parameter::VECTOR_STRING, ParameterType::VectorString},
    {essentia::Parameter::VECTOR_BOOL, ParameterType::VectorBool},
    {essentia::Parameter::VECTOR_INT, ParameterType::VectorInt},
    {essentia::Parameter::VECTOR_STEREOSAMPLE, ParameterType::VectorStereoSample},
    {essentia::Parameter::VECTOR_VECTOR_REAL, ParameterType::VectorVectorReal},
    {essentia::Parameter::VECTOR_VECTOR_STRING, ParameterType::VectorVectorString},
    {essentia::Parameter::VECTOR_VECTOR_STEREOSAMPLE, ParameterType::VectorVectorStereoSample},
    {essentia::Parameter::VECTOR_MATRIX_REAL, ParameterType::VectorMatrixReal},
    {essentia::Parameter::MAP_VECTOR_REAL, ParameterType::MapVectorReal},
    {essentia::Parameter::MAP_VECTOR_STRING, ParameterType::MapVectorString},
    {essentia::Parameter::MAP_VECTOR_INT, ParameterType::MapVectorInt},
    {essentia::Parameter::MAP_REAL, ParameterType::MapReal},
    {essentia::Parameter::MATRIX_REAL, ParameterType::MatrixReal},
};

ParameterType essentia_param_type_to_enum(essentia::Parameter::ParamType essentia_type)
{
    const auto it = parameterTypeLookupMap.find(essentia_type);
    if (it == parameterTypeLookupMap.end())
        throw std::invalid_argument{"essentia_param_type_to_enum: unsupported parameter type"};

    return it->second;
}

static const std::unordered_map<std::type_index, IOType> ioTypeLookupMap = {
    {typeid(float), IOType::Real},
    {typeid(int), IOType::Int},
    {typeid(unsigned int), IOType::UnsignedInt},
    {typeid(long), IOType::Long},
    {typeid(std::vector<float>), IOType::VectorReal},
    {typeid(std::vector<std::vector<float>>), IOType::VectorVectorReal},
    {typeid(TNT::Array2D<float>), IOType::MatrixReal}};

IOType type_info_to_io_enum(const std::type_info *type_info)
{
    if (type_info == nullptr)
        throw std::invalid_argument{"type_info_to_io_enum: null pointer"};

    const auto it = ioTypeLookupMap.find(*type_info);
    if (it == ioTypeLookupMap.end())
        throw std::invalid_argument{
            "type_info_to_io_enum: unsupported type (" + std::string(type_info->name()) + ')'};

    return it->second;
}

rust::String AlgorithmBridge::get_algorithm_name() const
{
    return rust::String(_algorithm->name());
}

rust::String AlgorithmBridge::get_algorithm_category() const
{
    try
    {
        const auto &info = essentia::standard::AlgorithmFactory::getInfo(_algorithm->name());
        return rust::String(info.category);
    }
    catch (...)
    {
        return rust::String("Unknown");
    }
}

rust::String AlgorithmBridge::get_algorithm_description() const
{
    try
    {
        const auto &info = essentia::standard::AlgorithmFactory::getInfo(_algorithm->name());
        return rust::String(info.description);
    }
    catch (...)
    {
        return rust::String("Description not available");
    }
}

rust::Vec<ParameterInfo> AlgorithmBridge::get_all_parameter_info() const
{
    rust::Vec<ParameterInfo> param_infos;

    const auto &param_descriptions = _algorithm->parameterDescription;
    const auto &param_ranges = _algorithm->parameterRange;
    const auto &default_params = _algorithm->defaultParameters();

    for (const auto &param_desc : param_descriptions)
    {
        const std::string &param_name = param_desc.first;

        ParameterInfo info;
        info.name = param_name;
        info.description = param_desc.second;

        // Get range
        auto range_it = param_ranges.find(param_name);
        info.range = (range_it != param_ranges.end()) ? range_it->second : "";

        // Get type and default value
        auto type_it = default_params.find(param_name);
        if (type_it != default_params.end())
        {
            const auto &param = type_it->second;
            info.type_ = essentia_param_type_to_enum(param.type());

            // Try to get the default value, but handle cases where it's not available
            try
            {
                info.default_value = param.toString();
            }
            catch (const essentia::EssentiaException &e)
            {
                // If we can't get the default value, use an empty string
                info.default_value = "";
            }
        }
        else
        {
            throw std::runtime_error(std::string("Parameter type not found for: ") + param_name);
        }

        param_infos.push_back(std::move(info));
    }

    return param_infos;
}

rust::Vec<IOInfo> AlgorithmBridge::get_all_input_info() const
{
    rust::Vec<IOInfo> input_infos;

    auto input_names = _algorithm->inputNames();
    auto input_types = _algorithm->inputTypes();
    const auto &input_descriptions = _algorithm->inputDescription;

    for (size_t i = 0; i < input_names.size() && i < input_types.size(); ++i)
    {
        IOInfo info;
        info.name = input_names[i];
        info.type_ = type_info_to_io_enum(input_types[i]);

        // Get description if available
        auto desc_it = input_descriptions.find(input_names[i]);
        info.description = (desc_it != input_descriptions.end()) ? desc_it->second : "";

        input_infos.push_back(std::move(info));
    }

    return input_infos;
}

rust::Vec<IOInfo> AlgorithmBridge::get_all_output_info() const
{
    rust::Vec<IOInfo> output_infos;

    auto output_names = _algorithm->outputNames();
    auto output_types = _algorithm->outputTypes();
    const auto &output_descriptions = _algorithm->outputDescription;

    for (size_t i = 0; i < output_names.size() && i < output_types.size(); ++i)
    {
        IOInfo info;
        info.name = output_names[i];
        info.type_ = type_info_to_io_enum(output_types[i]);

        // Get description if available
        auto desc_it = output_descriptions.find(output_names[i]);
        info.description = (desc_it != output_descriptions.end()) ? desc_it->second : "";

        output_infos.push_back(std::move(info));
    }

    return output_infos;
}