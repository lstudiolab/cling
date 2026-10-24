#pragma once
#include "cling/AST/Decl.h"
#include <memory>
#include <utility>
#include <vector>
namespace cling {
class Program final : public ASTNode {
public:
 explicit Program(std::vector<std::unique_ptr<FunctionDecl>> functionDeclarations = {}, SourceRange range = {})
  : ASTNode(ASTNodeKind::Program, range), functions(std::move(functionDeclarations)) {}
 std::vector<std::unique_ptr<FunctionDecl>> functions;
};
} // namespace cling
