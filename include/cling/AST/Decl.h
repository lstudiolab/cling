#pragma once
#include "cling/AST/ASTNode.h"
#include "cling/AST/Stmt.h"
#include <memory>
#include <string>
#include <utility>
#include <vector>
namespace cling {
class Decl : public ASTNode {
public:
 explicit Decl(ASTNodeKind kind, SourceRange range = {}) noexcept : ASTNode(kind, range) {}
};
class ParameterDecl final : public Decl {
public:
 ParameterDecl(std::string name, std::string typeName, SourceRange range = {})
  : Decl(ASTNodeKind::ParameterDecl, range), name(std::move(name)), typeName(std::move(typeName)) {}
 std::string name;
 std::string typeName;
};
class VariableDecl final : public Decl {
public:
 VariableDecl(std::string name, bool mutableBinding, std::unique_ptr<Expr> initializer,
              std::string typeName = {}, SourceRange range = {})
  : Decl(ASTNodeKind::VariableDecl, range), name(std::move(name)), mutableBinding(mutableBinding),
    initializer(std::move(initializer)), typeName(std::move(typeName)) {}
 std::string name;
 bool mutableBinding;
 std::unique_ptr<Expr> initializer;
 std::string typeName;
};
class FunctionDecl final : public Decl {
public:
 FunctionDecl(std::string name, std::vector<ParameterDecl> parameters, std::string returnType,
              std::unique_ptr<BlockStmt> body, SourceRange range = {})
  : Decl(ASTNodeKind::FunctionDecl, range), name(std::move(name)), parameters(std::move(parameters)),
    returnType(std::move(returnType)), body(std::move(body)) {}
 std::string name;
 std::vector<ParameterDecl> parameters;
 std::string returnType;
 std::unique_ptr<BlockStmt> body;
};
} // namespace cling
