#include "cling/Basic/SourceLocation.h"

#include <algorithm>
#include <sstream>
#include <utility>

namespace cling {

std::string SourceLocation::toString() const {
    std::ostringstream stream;
    stream << line << ':' << column;
    return stream.str();
}

SourceBuffer::SourceBuffer(std::string name, std::string contents)
    : name_(std::move(name)), contents_(std::move(contents)) {}

const std::string& SourceBuffer::name() const noexcept { return name_; }

std::string_view SourceBuffer::contents() const noexcept { return contents_; }

std::size_t SourceBuffer::size() const noexcept { return contents_.size(); }

char SourceBuffer::at(std::size_t offset) const noexcept {
    return offset < contents_.size() ? contents_[offset] : '\0';
}

bool SourceBuffer::isEOF(std::size_t offset) const noexcept {
    return offset >= contents_.size();
}

SourceLocation SourceBuffer::locationAt(std::size_t offset) const noexcept {
    offset = std::min(offset, contents_.size());
    SourceLocation location{offset, 1, 1};
    for (std::size_t index = 0; index < offset; ++index) {
        if (contents_[index] == '\n') {
            ++location.line;
            location.column = 1;
        } else {
            ++location.column;
        }
    }
    return location;
}

} // namespace cling
