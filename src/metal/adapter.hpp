#pragma once

#include "../base/adapter.hpp"

#include <Metal/Metal.hpp>

namespace sob {
    class MetalAdapter : public Adapter {
    public:
        MetalAdapter(MTL::Device* device);
    private:
        MTL::Device* _device;
    };
}