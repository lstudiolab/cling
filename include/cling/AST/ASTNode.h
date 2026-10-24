#pragma once
#include "cling/Basic/SourceLocation.h"
#include <cstddef>
namespace cling {
enum class ASTNodeKind {
 Program, FunctionDecl, ParameterDecl, VariableDecl, BlockStmt, IfStmt, WhileStmt,
 ReturnStmt, ExpressionStmt, IntegerExpr, StringExpr, BooleanExpr, NameExpr, CallExpr,
 UnaryExpr, BinaryExpr, AssignmentExpr
};
class ASTNode {
public:
 explicit ASTNode(ASTNodeKind kind, SourceRange range = {}) noexcept : kind_(kind), range_(range) {}
 virtual ~ASTNode() = default;
 ASTNode(const ASTNode&) = delete;
 ASTNode& operator=(const ASTNode&) = delete;
 [[nodiscard]] ASTNodeKind kind() const noexcept { return kind_; }
 [[nodiscard]] SourceRange range() const noexcept { return range_; }
 void setRange(SourceRange range) noexcept { range_ = range; }
private:
 ASTNodeKind kind_;
 SourceRange range_;
};
} // namespace cling
