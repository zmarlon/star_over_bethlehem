#pragma once

#include <star_over_bethlehem.h>

#include <string>

namespace sob::utils {
    [[nodiscard]] inline RhiStringView string_view_from_string(const std::string& str) {
        RhiStringView result;
        result.data = str.data();
        result.length = str.size();

        return result;
    }
}