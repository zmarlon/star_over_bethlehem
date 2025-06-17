#pragma once

#include "../base/instance.hpp"

namespace sob {
    class MetalInstance : public Instance {
    public:
        MetalInstance() = default;

        [[nodiscard]] RhiResult init(const RhiInstanceDesc* desc);

        [[nodiscard]] RhiResult create_device(const RhiDeviceDesc* desc, RhiDevice* device) override;
    };
}