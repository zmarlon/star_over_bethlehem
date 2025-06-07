#pragma once

#include "object.hpp"
#include <vector>

namespace sob {
    class Adapter;
    class Instance : public Object<Instance, RhiInstance> {
    public:
        virtual ~Instance();

        static RhiResult init(const RhiInstanceDesc* desc, RhiInstance* instance);
        void enumerate_adapters(uint32_t* num_adapters, RhiAdapter* adapters) const;

        void clean_adapters();
    protected:
        std::vector<Adapter*> _adapters;
    };
}