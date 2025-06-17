#pragma once

#include "object.hpp"
#include "util/shader_compiler.hpp"

namespace sob {
    class Device : public Object<Device, RhiDevice> {
    protected:
        ShaderCompiler _shader_compiler;
    public:
        [[nodiscard]] virtual RhiResult init();
    };
}