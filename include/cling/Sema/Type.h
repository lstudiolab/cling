#pragma once

#include <string_view>

namespace cling {

enum class TypeKind { Error, Void, Bool, Int, String, Function, Unknown };

struct Type {
    TypeKind kind = TypeKind::Unknown;

    [[nodiscard]] constexpr bool isError() const noexcept { return kind == TypeKind::Error; }
    [[nodiscard]] constexpr bool isNumeric() const noexcept { return kind == TypeKind::Int; }
    [[nodiscard]] constexpr bool isBoolean() const noexcept { return kind == TypeKind::Bool; }
    [[nodiscard]] constexpr bool isString() const noexcept { return kind == TypeKind::String; }
};

[[nodiscard]] std::string_view typeName(Type type) noexcept;

} // namespace cling
