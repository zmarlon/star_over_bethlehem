#include <star_over_bethlehem.h>
#include <iostream>

int main() {
    RhiInstanceDesc instance_desc;
    instance_desc.backend = RHI_BACKEND_METAL;
    instance_desc.validation = true;

    RhiInstance instance;
    auto result = sob_instance_create(&instance_desc, &instance);
    if (result != RHI_SUCCESS) {
        std::cout << "Failed to create instance" << std::endl;
        return 1;
    }

    uint32_t num_adapters;
    sob_instance_enumerate_adapters(instance, &num_adapters, nullptr);

    auto* adapters = new RhiAdapter[num_adapters];
    sob_instance_enumerate_adapters(instance, &num_adapters, adapters);

    auto adapter = adapters[0];
    delete adapters;

    RhiAdapterDesc adapter_desc;
    result = sob_adapter_get_properties(adapter, &adapter_desc);
    if (result != RHI_SUCCESS) {
        std::cout << "Failed to get properties" << std::endl;
        return 1;
    }

    std::cout << adapter_desc.name.data << std::endl;

    RhiDeviceDesc device_desc;
    device_desc.adapter = adapter;

    RhiDevice device;

    result = sob_instance_create_device(instance, &device_desc, &device);
    if (result != RHI_SUCCESS) {
        std::cout << "Failed to create device" << std::endl;
        return 1;
    }

    sob_destroy_device(device);

    sob_instance_destroy(instance);

    return 0;
}