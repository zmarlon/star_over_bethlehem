#pragma once

#include "object.hpp"
#include <string>

namespace sob {
    class Adapter : public Object<Adapter, RhiAdapter> {
    public:
        virtual RhiResult adapter_get_properties(RhiAdapterProperties* properties) const;
    protected:
        std::string _name;
    };
}