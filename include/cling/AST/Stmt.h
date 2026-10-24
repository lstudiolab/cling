#pragma once
#include "cling/AST/ASTNode.h"
#include "cling/AST/Expr.h"
#include <memory>
#include <utility>
#include <vector>
namespace cling {
class Stmt : public ASTNode {
public:
 explicit Stmt(ASTNodeKind nodeKind, SourceRange range = {}) noexcept : ASTNode(nodeKind, range) {}
};
class BlockStmt final : public Stmt {
public:
 explicit BlockStmt(std::vector<std::unique_ptr<Stmt>> bodyStatements = {}, SourceRange range = {})
  : Stmt(ASTNodeKind::BlockStmt, range), statements(std::move(bodyStatements)) {}
 std::vector<std::unique_ptr<Stmt>> statements;
};
class ReturnStmt final : public Stmt {
public:
 explicit ReturnStmt(std::unique_ptr<Expr> returnValue = {}, SourceRange range = {})
  : Stmt(ASTNodeKind::ReturnStmt, range), value(std::move(returnValue)) {}
 std::unique_ptr<Expr> value;
};
class ExpressionStmt final : public Stmt {
public:
 explicit ExpressionStmt(std::unique_ptr<Expr> expressionValue, SourceRange range = {})
  : Stmt(ASTNodeKind::ExpressionStmt, range), expression(std::move(expressionValue)) {}
 std::unique_ptr<Expr> expression;
};
class IfStmt final : public Stmt {
public:
 IfStmt(std::unique_ptr<Expr> testCondition, std::unique_ptr<Stmt> thenStatement,
        std::unique_ptr<Stmt> elseStatement = {}, SourceRange range = {})
  : Stmt(ASTNodeKind::IfStmt, range), condition(std::move(testCondition)),
    thenBranch(std::move(thenStatement)), elseBranch(std::move(elseStatement)) {}
 std::unique_ptr<Expr> condition;
 std::unique_ptr<Stmt> thenBranch;
 std::unique_ptr<Stmt> elseBranch;
};
} // namespace cling
