#pragma once

#include "cling/Basic/SourceLocation.h"
#include "cling/Lex/TokenKind.h"

#include <string>
#include <string_view>

namespace cling {

struct Token {
    TokenKind kind = TokenKind::Unknown;
    SourceRange range;
    std::string text;

    [[nodiscard]] std::string_view spelling() const noexcept { return text; }
};

} // namespace cling
