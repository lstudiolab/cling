#pragma once
#include "cling/AST/ASTNode.h"
#include <memory>
#include <string>
#include <utility>
#include <vector>
namespace cling {
class Expr : public ASTNode {
public:
 explicit Expr(ASTNodeKind kind, SourceRange range = {}) noexcept : ASTNode(kind, range) {}
};
class IntegerExpr final : public Expr {
public:
 IntegerExpr(long long value, SourceRange range = {}) : Expr(ASTNodeKind::IntegerExpr, range), value(value) {}
 long long value;
};
class StringExpr final : public Expr {
public:
 StringExpr(std::string value, SourceRange range = {}) : Expr(ASTNodeKind::StringExpr, range), value(std::move(value)) {}
 std::string value;
};
class BooleanExpr final : public Expr {
public:
 BooleanExpr(bool value, SourceRange range = {}) : Expr(ASTNodeKind::BooleanExpr, range), value(value) {}
 bool value;
};
class NameExpr final : public Expr {
public:
 NameExpr(std::string name, SourceRange range = {}) : Expr(ASTNodeKind::NameExpr, range), name(std::move(name)) {}
 std::string name;
};
class CallExpr final : public Expr {
public:
 CallExpr(std::unique_ptr<Expr> callee, std::vector<std::unique_ptr<Expr>> arguments, SourceRange range = {})
  : Expr(ASTNodeKind::CallExpr, range), callee(std::move(callee)), arguments(std::move(arguments)) {}
 std::unique_ptr<Expr> callee;
 std::vector<std::unique_ptr<Expr>> arguments;
};
class BinaryExpr final : public Expr {
public:
 BinaryExpr(std::string operation, std::unique_ptr<Expr> left, std::unique_ptr<Expr> right, SourceRange range = {})
  : Expr(ASTNodeKind::BinaryExpr, range), operation(std::move(operation)), left(std::move(left)), right(std::move(right)) {}
 std::string operation;
 std::unique_ptr<Expr> left;
 std::unique_ptr<Expr> right;
};
} // namespace cling
