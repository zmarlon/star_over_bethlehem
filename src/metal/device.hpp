#include "../base/device.hpp"

namespace sob {
    class MetalAdapter;
    class MetalDevice : public Device {
    public:
        MetalDevice(MetalAdapter* adapter) : _adapter(adapter) {}
    private:
        MetalAdapter* _adapter;
    };
}