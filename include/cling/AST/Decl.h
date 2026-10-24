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
 explicit Decl(ASTNodeKind nodeKind, SourceRange range = {}) noexcept : ASTNode(nodeKind, range) {}
};
class ParameterDecl final : public Decl {
public:
 ParameterDecl(std::string parameterName, std::string declaredType, SourceRange range = {})
  : Decl(ASTNodeKind::ParameterDecl, range), name(std::move(parameterName)), typeName(std::move(declaredType)) {}
 std::string name;
 std::string typeName;
};
class VariableDecl final : public Decl {
public:
 VariableDecl(std::string variableName, bool isMutable, std::unique_ptr<Expr> initialValue,
              std::string typeAnnotation = {}, SourceRange range = {})
  : Decl(ASTNodeKind::VariableDecl, range), name(std::move(variableName)), mutableBinding(isMutable),
    initializer(std::move(initialValue)), typeName(std::move(typeAnnotation)) {}
 std::string name;
 bool mutableBinding;
 std::unique_ptr<Expr> initializer;
 std::string typeName;
};
class FunctionDecl final : public Decl {
public:
 FunctionDecl(std::string functionName, std::vector<ParameterDecl> functionParameters,
              std::string resultType, std::unique_ptr<BlockStmt> functionBody, SourceRange range = {})
  : Decl(ASTNodeKind::FunctionDecl, range), name(std::move(functionName)),
    parameters(std::move(functionParameters)), returnType(std::move(resultType)),
    body(std::move(functionBody)) {}
 std::string name;
 std::vector<ParameterDecl> parameters;
 std::string returnType;
 std::unique_ptr<BlockStmt> body;
};
} // namespace cling
