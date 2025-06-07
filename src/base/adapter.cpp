#include "adapter.hpp"
#include "utils.hpp"

namespace sob {
    RhiResult Adapter::adapter_get_properties(RhiAdapterProperties* properties) const {
        properties->name = utils::string_view_from_string(_name);

        return RHI_SUCCESS;
    }
}