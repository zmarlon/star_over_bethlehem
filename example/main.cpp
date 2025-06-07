#include <star_over_bethlehem.h>

int main() {
    RhiInstanceDesc instance_desc;
    instance_desc.backend = RHI_BACKEND_METAL;
    instance_desc.validation = true;

    RhiInstance instance;
    auto result = sob_instance_create(&instance_desc, &instance);
    if (result != RHI_SUCCESS) {
        return 1;
    }

    sob_instance_destroy(instance);

    return 0;
}