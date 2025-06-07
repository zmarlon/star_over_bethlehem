#pragma once

#include "../base/instance.hpp"

namespace sob {
    class MetalInstance : public Instance {
    public:
        MetalInstance() = default;

        RhiResult init(const RhiInstanceDesc* desc);
    };
}