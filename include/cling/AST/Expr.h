#pragma once
#include "cling/AST/ASTNode.h"
#include <memory>
#include <string>
#include <utility>
#include <vector>
namespace cling {
class Expr : public ASTNode {
public:
 explicit Expr(ASTNodeKind nodeKind, SourceRange range = {}) noexcept : ASTNode(nodeKind, range) {}
};
class IntegerExpr final : public Expr {
public:
 IntegerExpr(long long initialValue, SourceRange range = {}) : Expr(ASTNodeKind::IntegerExpr, range), value(initialValue) {}
 long long value;
};
class StringExpr final : public Expr {
public:
 StringExpr(std::string initialValue, SourceRange range = {}) : Expr(ASTNodeKind::StringExpr, range), value(std::move(initialValue)) {}
 std::string value;
};
class BooleanExpr final : public Expr {
public:
 BooleanExpr(bool initialValue, SourceRange range = {}) : Expr(ASTNodeKind::BooleanExpr, range), value(initialValue) {}
 bool value;
};
class NameExpr final : public Expr {
public:
 NameExpr(std::string identifier, SourceRange range = {}) : Expr(ASTNodeKind::NameExpr, range), name(std::move(identifier)) {}
 std::string name;
};
class CallExpr final : public Expr {
public:
 CallExpr(std::unique_ptr<Expr> calleeExpr, std::vector<std::unique_ptr<Expr>> callArguments, SourceRange range = {})
  : Expr(ASTNodeKind::CallExpr, range), callee(std::move(calleeExpr)), arguments(std::move(callArguments)) {}
 std::unique_ptr<Expr> callee;
 std::vector<std::unique_ptr<Expr>> arguments;
};
class BinaryExpr final : public Expr {
public:
 BinaryExpr(std::string operationName, std::unique_ptr<Expr> leftOperand, std::unique_ptr<Expr> rightOperand, SourceRange range = {})
  : Expr(ASTNodeKind::BinaryExpr, range), operation(std::move(operationName)),
    left(std::move(leftOperand)), right(std::move(rightOperand)) {}
 std::string operation;
 std::unique_ptr<Expr> left;
 std::unique_ptr<Expr> right;
};
} // namespace cling
