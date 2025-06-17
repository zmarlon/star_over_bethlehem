#include "device.hpp"

namespace sob {
    RhiResult Device::init() {
        const auto result = _shader_compiler.init();
        if (result != RHI_SUCCESS) {
            return result;
        }

        return RHI_SUCCESS;
    }
}