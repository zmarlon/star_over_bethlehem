#include "shader_compiler.hpp"

#include "../utils.hpp"

namespace sob {
    [[nodiscard]] static LPCWSTR target_from_stage(RhiShaderStage stage) {
        switch (stage) {
            case RHI_SHADER_STAGE_VERTEX:
                return L"vs_6_8";
            case RHI_SHADER_STAGE_AMPLIFICATION:
                return L"as_6_8";
            case RHI_SHADER_STAGE_MESH:
                return L"ms_6_8";
            case RHI_SHADER_STAGE_PIXEL:
                return L"ps_6_8";
            case RHI_SHADER_STAGE_COMPUTE:
                return L"cs_6_8";
        }
    }

    RhiResult ShaderCompiler::init() {
        auto result = DxcCreateInstance(CLSID_DxcUtils, IID_PPV_ARGS(&_utils));
        if (FAILED(result)) {
            return RHI_SHADER_COMPILER_ERROR;
        }

        result = DxcCreateInstance(CLSID_DxcCompiler, IID_PPV_ARGS(&_compiler));
        if (FAILED(result)) {
            return RHI_SHADER_COMPILER_ERROR;
        }

        return RHI_SUCCESS;
    }

    RhiResult ShaderCompiler::compile(const RhiShaderDesc* desc, std::vector<uint8_t>& out) const {
        const auto& source = desc->source.hlsl;

        DxcBuffer src_buffer = {
            .Ptr = source.source.data,
            .Size = source.source.length,
            .Encoding = 0
        };

        const auto entry_point_str = std::string(desc->entry_point.data, desc->entry_point.length);
        const auto entry_point = utils::string_to_wstring(entry_point_str);

        std::vector<LPCWSTR> args;
        args.push_back(L"Zpc");
        args.push_back(L"-HV");
        args.push_back(L"2021");
        args.push_back(L"-T");
        args.push_back(target_from_stage(desc->shader_stage));
        args.push_back(L"-E");
        args.push_back(entry_point.data());

        IDxcResult* dxc_result;
        auto result = _compiler->Compile(&src_buffer, args.data(), static_cast<uint32_t>(args.size()),
            nullptr, IID_PPV_ARGS(&dxc_result));

        if (FAILED(result)) {
            dxc_result->Release();
            return RHI_SHADER_COMPILER_ERROR;
        }

        IDxcBlob* blob;
        result = dxc_result->GetResult(&blob);
        if (FAILED(result)) {
            dxc_result->Release();
            return RHI_SHADER_COMPILER_ERROR;
        }

        out.resize(blob->GetBufferSize());
        memcpy(out.data(), blob->GetBufferPointer(), blob->GetBufferSize());

        blob->Release();
        dxc_result->Release();

        return RHI_SUCCESS;
    }
}
