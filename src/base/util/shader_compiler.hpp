#pragma once

#include <star_over_bethlehem.h>
#include <dxc/dxcapi.h>
#include <vector>

namespace sob {
    class ShaderCompiler final {
        IDxcUtils* _utils;
        IDxcCompiler3* _compiler;
    public:
        ShaderCompiler() = default;

        [[nodiscard]] RhiResult init();

        [[nodiscard]] RhiResult compile(const RhiShaderDesc* desc, std::vector<uint8_t>& out) const;
    };
}
