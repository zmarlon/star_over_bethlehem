#include "instance.hpp"
#include "adapter.hpp"

#ifdef SOB_METAL_SUPPORTED
#include "../metal/instance.hpp"
#endif

namespace sob {
    Instance::~Instance() {
        clean_adapters();
    }

    RhiResult Instance::init(const RhiInstanceDesc* desc, RhiInstance* instance) {
        switch (desc->backend) {
#ifdef SOB_D3D12_SUPPORTED
            case RHI_BACKEND_D3D12:
                return RHI_BACKEND_NOT_SUPPORTED;
#endif
#ifdef SOB_METAL_SUPPORTED
            case RHI_BACKEND_METAL: {
                auto* metal_instance = new MetalInstance;
                const auto result = metal_instance->init(desc);

                if (result != RHI_SUCCESS) {
                    delete metal_instance;
                    return result;
                }

                *instance = metal_instance->into_handle();

                return RHI_SUCCESS;
            }
#endif
#ifdef SOB_VULKAN_SUPPORTED
            case RHI_BACKEND_VULKAN:
                return RHI_BACKEND_NOT_SUPPORTED;
#endif
            default:
                return RHI_BACKEND_NOT_SUPPORTED;
        }
    }

    void Instance::enumerate_adapters(uint32_t* num_adapters, RhiAdapter* adapters) const {
        if (!adapters) {
            *num_adapters = static_cast<uint32_t>(_adapters.size());
            return;
        }

        memcpy(adapters, _adapters.data(), _adapters.size() * sizeof(RhiAdapter));
    }

    void Instance::clean_adapters() {
        for (const auto* adapter: _adapters) {
            delete adapter;
        }

        _adapters.clear();
    }
}