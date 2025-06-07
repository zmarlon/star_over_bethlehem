#define NS_PRIVATE_IMPLEMENTATION
#define CA_PRIVATE_IMPLEMENTATION
#define MTL_PRIVATE_IMPLEMENTATION
#include <Foundation/Foundation.hpp>
#include <Metal/Metal.hpp>
#include <QuartzCore/QuartzCore.hpp>

#include "instance.hpp"
#include "adapter.hpp"

namespace sob {
    static bool supports_features(MTL::Device* device) {
        return device->supportsRaytracing() && device->supportsFamily(MTL::GPUFamilyApple7);
    }

    RhiResult MetalInstance::init(const RhiInstanceDesc* desc) {
        const auto devices = MTL::CopyAllDevices();
        for (NS::UInteger i = 0; i < devices->count(); i++) {
            const auto device = reinterpret_cast<MTL::Device*>(devices->object(i));
            if (!supports_features(device)) {
                _adapters.push_back(new MetalAdapter(device));
            }
        }

        return RHI_SUCCESS;
    }
}
