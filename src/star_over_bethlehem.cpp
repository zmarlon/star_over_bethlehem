#include <star_over_bethlehem.h>

#include "base/instance.hpp"
#include "base/adapter.hpp"
#include "base/device.hpp"

extern "C" {
    RhiResult sob_instance_create(const RhiInstanceDesc* desc, RhiInstance* instance) {
        return sob::Instance::init(desc, instance);
    }

    void sob_instance_destroy(RhiInstance instance) {
        const auto* base_instance = sob::Instance::from_handle(instance);
        delete base_instance;
    }

    void sob_instance_enumerate_adapters(RhiInstance instance, uint32_t* num_adapters, RhiAdapter* adapters) {
        const auto* base_instance = sob::Instance::from_handle(instance);
        base_instance->enumerate_adapters(num_adapters, adapters);
    }

    RhiResult sob_instance_create_device(RhiInstance instance, const RhiDeviceDesc* desc, RhiDevice* device) {
        auto* base_instance = sob::Instance::from_handle(instance);
        return base_instance->create_device(desc, device);
    }

    void sob_destroy_device(RhiDevice device) {
        const auto* base_device = sob::Device::from_handle(device);
        delete base_device;
    }

    RhiResult sob_adapter_get_properties(RhiAdapter adapter, RhiAdapterDesc* desc) {
        const auto* base_adapter = sob::Adapter::from_handle(adapter);
        return base_adapter->adapter_get_properties(desc);
    }

    RhiResult sob_device_create_shader(RhiDevice device, const RhiShaderDesc* desc, RhiShader* shader) {
        return RHI_SUCCESS;
    }
}
