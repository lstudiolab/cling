#pragma once

#include <cstddef>
#include <string>
#include <string_view>

namespace cling {

struct SourceLocation {
    std::size_t offset = 0;
    std::size_t line = 1;
    std::size_t column = 1;

    [[nodiscard]] std::string toString() const;
};

struct SourceRange {
    SourceLocation begin;
    SourceLocation end;
};

class SourceBuffer {
public:
    SourceBuffer(std::string name, std::string contents);

    [[nodiscard]] const std::string& name() const noexcept;
    [[nodiscard]] std::string_view contents() const noexcept;
    [[nodiscard]] std::size_t size() const noexcept;
    [[nodiscard]] char at(std::size_t offset) const noexcept;
    [[nodiscard]] bool isEOF(std::size_t offset) const noexcept;
    [[nodiscard]] SourceLocation locationAt(std::size_t offset) const noexcept;

private:
    std::string name_;
    std::string contents_;
};

} // namespace cling
