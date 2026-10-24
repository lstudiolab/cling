#include "cling/AST/ASTNode.h"
#include <string_view>
namespace cling {
std::string_view astNodeKindName(ASTNodeKind kind) noexcept {
    switch (kind) {
    case ASTNodeKind::Program: return "Program";
    case ASTNodeKind::FunctionDecl: return "FunctionDecl";
    case ASTNodeKind::ParameterDecl: return "ParameterDecl";
    case ASTNodeKind::VariableDecl: return "VariableDecl";
    case ASTNodeKind::BlockStmt: return "BlockStmt";
    case ASTNodeKind::IfStmt: return "IfStmt";
    case ASTNodeKind::WhileStmt: return "WhileStmt";
    case ASTNodeKind::ReturnStmt: return "ReturnStmt";
    case ASTNodeKind::ExpressionStmt: return "ExpressionStmt";
    case ASTNodeKind::IntegerExpr: return "IntegerExpr";
    case ASTNodeKind::StringExpr: return "StringExpr";
    case ASTNodeKind::BooleanExpr: return "BooleanExpr";
    case ASTNodeKind::NameExpr: return "NameExpr";
    case ASTNodeKind::CallExpr: return "CallExpr";
    case ASTNodeKind::UnaryExpr: return "UnaryExpr";
    case ASTNodeKind::BinaryExpr: return "BinaryExpr";
    case ASTNodeKind::AssignmentExpr: return "AssignmentExpr";
    }
    return "UnknownASTNode";
}
} // namespace cling
