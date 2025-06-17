#include "dxil_to_metal.hpp"
#include <string>

namespace sob {
    [[nodiscard]] static IRShaderStage ir_stage_from_stage(RhiShaderStage stage) {
        switch (stage) {
            case RHI_SHADER_STAGE_VERTEX:
                return IRShaderStageVertex;
            case RHI_SHADER_STAGE_AMPLIFICATION:
                return IRShaderStageAmplification;
            case RHI_SHADER_STAGE_MESH:
                return IRShaderStageMesh;
            case RHI_SHADER_STAGE_PIXEL:
                return IRShaderStageFragment;
            case RHI_SHADER_STAGE_COMPUTE:
                return IRShaderStageCompute;
        }
    }

    DxilToMetal::~DxilToMetal() {
        IRCompilerDestroy(_compiler);
    }

    RhiResult DxilToMetal::init() {
        _compiler = IRCompilerCreate();
        if (!_compiler) {
            return RHI_IR_COMPILER_CREATION_ERROR;
        }

        return RHI_SUCCESS;
    }

    RhiResult DxilToMetal::to_metal(const RhiShaderDesc* desc, const std::vector<uint8_t>& dxil, std::vector<uint8_t>& metal) {
        std::string entry_point(desc->entry_point.data, desc->entry_point.length);
        IRCompilerSetEntryPointName(_compiler, entry_point.c_str());

        const auto* object = IRObjectCreateFromDXIL(dxil.data(), dxil.size(), IRBytecodeOwnershipNone);

        IRError* error = nullptr;
        const auto* out_ir = IRCompilerAllocCompileAndLink(_compiler, entry_point.c_str(),  object, &error);
        if (!out_ir) {
            IRErrorDestroy(error);
            IRObjectDestroy(object);
            return RHI_IR_COMPILER_COMPILATION_ERROR;
        }

        IRMetalLibBinary* metal_lib = IRMetalLibBinaryCreate();
        IRObjectGetMetalLibBinary(out_ir, ir_stage_from_stage(desc->shader_stage), metal_lib);
        const auto metal_lib_size = IRMetalLibGetBytecodeSize(metal_lib);

        metal.resize(metal_lib_size);
        IRMetalLibGetBytecode(metal_lib, metal.data());

        IRMetalLibBinaryDestroy(metal_lib);
        IRObjectDestroy(object);
        IRObjectDestroy(out_ir);

        return RHI_SUCCESS;
    }
}