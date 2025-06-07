#include "adapter.hpp"

namespace sob {
    MetalAdapter::MetalAdapter(MTL::Device* device) : _device(device) {
        _name = _device->name()->utf8String();
    }
}