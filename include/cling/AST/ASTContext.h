#pragma once

#include "cling/AST/ASTNode.h"

#include <cstddef>
#include <memory>
#include <utility>
#include <vector>

namespace cling {

// Owns AST nodes for one compilation unit. It is separate from parser state so later passes
// can retain stable node addresses without depending on the parser's lifetime.
class ASTContext {
public:
    ASTContext() = default;
    ASTContext(const ASTContext&) = delete;
    ASTContext& operator=(const ASTContext&) = delete;

    template <typename T, typename... Args>
    T* make(Args&&... args) {
        static_assert(std::is_base_of_v<ASTNode, T>, "ASTContext only owns AST nodes");
        auto node = std::make_unique<T>(std::forward<Args>(args)...);
        T* raw = node.get();
        storage_.push_back(std::move(node));
        return raw;
    }

    [[nodiscard]] std::size_t nodeCount() const noexcept { return storage_.size(); }

private:
    std::vector<std::unique_ptr<ASTNode>> storage_;
};

} // namespace cling
