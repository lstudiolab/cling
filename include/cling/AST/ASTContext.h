#pragma once

#include <memory>
#include <vector>

namespace cling {

class ASTNode;

// Owns AST nodes for the lifetime of a single compilation unit. The context is intentionally
// separate from parser state so later passes can retain stable node addresses.
class ASTContext {
public:
    ASTContext() = default;
    ASTContext(const ASTContext&) = delete;
    ASTContext& operator=(const ASTContext&) = delete;

    template <typename T, typename... Args>
    T* make(Args&&... args) {
        auto node = std::make_unique<T>(static_cast<Args&&>(args)...);
        T* raw = node.get();
        storage_.push_back(std::move(node));
        return raw;
    }

    [[nodiscard]] std::size_t nodeCount() const noexcept { return storage_.size(); }

private:
    std::vector<std::unique_ptr<ASTNode>> storage_;
};

} // namespace cling
