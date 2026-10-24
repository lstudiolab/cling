#pragma once
#include "cling/AST/ASTNode.h"
#include "cling/AST/Expr.h"
#include <memory>
#include <utility>
#include <vector>
namespace cling {
class Stmt : public ASTNode {
public:
 explicit Stmt(ASTNodeKind kind, SourceRange range = {}) noexcept : ASTNode(kind, range) {}
};
class BlockStmt final : public Stmt {
public:
 explicit BlockStmt(std::vector<std::unique_ptr<Stmt>> statements = {}, SourceRange range = {})
  : Stmt(ASTNodeKind::BlockStmt, range), statements(std::move(statements)) {}
 std::vector<std::unique_ptr<Stmt>> statements;
};
class ReturnStmt final : public Stmt {
public:
 explicit ReturnStmt(std::unique_ptr<Expr> value = {}, SourceRange range = {})
  : Stmt(ASTNodeKind::ReturnStmt, range), value(std::move(value)) {}
 std::unique_ptr<Expr> value;
};
class ExpressionStmt final : public Stmt {
public:
 explicit ExpressionStmt(std::unique_ptr<Expr> expression, SourceRange range = {})
  : Stmt(ASTNodeKind::ExpressionStmt, range), expression(std::move(expression)) {}
 std::unique_ptr<Expr> expression;
};
class IfStmt final : public Stmt {
public:
 IfStmt(std::unique_ptr<Expr> condition, std::unique_ptr<Stmt> thenBranch,
        std::unique_ptr<Stmt> elseBranch = {}, SourceRange range = {})
  : Stmt(ASTNodeKind::IfStmt, range), condition(std::move(condition)),
    thenBranch(std::move(thenBranch)), elseBranch(std::move(elseBranch)) {}
 std::unique_ptr<Expr> condition;
 std::unique_ptr<Stmt> thenBranch;
 std::unique_ptr<Stmt> elseBranch;
};
} // namespace cling
