#pragma once
#include <star_over_bethlehem.h>
#include <metal_irconverter/metal_irconverter.h>
#include <vector>

namespace sob {
    class DxilToMetal {
        IRCompiler* _compiler;
    public:
        DxilToMetal() = default;
        ~DxilToMetal();

        [[nodiscard]] RhiResult init();

        [[nodiscard]] RhiResult to_metal(const RhiShaderDesc* desc, const std::vector<uint8_t>& dxil, std::vector<uint8_t>& metal);
    };
}
