#pragma once

#include <star_over_bethlehem.h>

#include <codecvt>
#include <locale>

namespace sob::utils {
    [[nodiscard]] inline RhiStringView string_view_from_string(const std::string& str) {
        RhiStringView result;
        result.data = str.data();
        result.length = str.size();

        return result;
    }

    [[nodiscard]] inline std::wstring string_to_wstring(const std::string& str) {
        std::wstring_convert<std::codecvt_utf8_utf16<wchar_t>> converter;
        return converter.from_bytes(str);
    }
}
