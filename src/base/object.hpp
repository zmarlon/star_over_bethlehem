#pragma once

#include <star_over_bethlehem.h>

namespace sob {
    template<typename Impl, typename Handle>
    class Object {
    public:
        virtual ~Object() = default;

        [[nodiscard]] Handle into_handle() {
            return reinterpret_cast<Handle>(this);
        }

        [[nodiscard]] static Impl* from_handle(Handle handle) {
            return reinterpret_cast<Impl*>(handle);
        }
    };
}