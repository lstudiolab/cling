use super::Analyzer;
use crate::ast::{BinaryOp, Expr, ExprKind, TypeName, UnaryOp};
use crate::diagnostic::Diagnostic;

impl Analyzer {
    pub(super) fn check_expr(&mut self, expr: &Expr) -> TypeName {
        match &expr.kind {
            ExprKind::Int(_) => TypeName::Int,
            ExprKind::Bool(_) => TypeName::Bool,
            ExprKind::String(_) => TypeName::String,
            ExprKind::Variable(name) => self.lookup(name).map(|v| v.ty).unwrap_or_else(|| {
                self.errors.push(Diagnostic::error(format!("use of undeclared variable '{}'", name), expr.span));
                TypeName::Int
            }),
            ExprKind::Unary { op, operand } => {
                let ty = self.check_expr(operand);
                match op {
                    UnaryOp::Negate => { self.require_type(ty, TypeName::Int, operand.span, "unary '-' operand"); TypeName::Int }
                    UnaryOp::Not => { self.require_type(ty, TypeName::Bool, operand.span, "'!' operand"); TypeName::Bool }
                }
            }
            ExprKind::Binary { op, left, right } => {
                let left_ty = self.check_expr(left);
                let right_ty = self.check_expr(right);
                match op {
                    BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide | BinaryOp::Remainder => {
                        self.require_type(left_ty, TypeName::Int, left.span, "arithmetic operand");
                        self.require_type(right_ty, TypeName::Int, right.span, "arithmetic operand");
                        TypeName::Int
                    }
                    BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual => {
                        self.require_type(left_ty, TypeName::Int, left.span, "comparison operand");
                        self.require_type(right_ty, TypeName::Int, right.span, "comparison operand");
                        TypeName::Bool
                    }
                    BinaryOp::Equal | BinaryOp::NotEqual => {
                        if left_ty != right_ty || left_ty == TypeName::Void {
                            self.errors.push(Diagnostic::error("equality operands must have the same non-Void type", expr.span));
                        }
                        TypeName::Bool
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        self.require_type(left_ty, TypeName::Bool, left.span, "logical operand");
                        self.require_type(right_ty, TypeName::Bool, right.span, "logical operand");
                        TypeName::Bool
                    }
                }
            }
            ExprKind::Call { callee, arguments } => {
                if callee == "print" || callee == "log" {
                    if arguments.len() != 1 {
                        self.errors.push(Diagnostic::error(format!("{}() expects exactly one argument", callee), expr.span));
                    }
                    for argument in arguments {
                        let ty = self.check_expr(argument);
                        if !matches!(ty, TypeName::Int | TypeName::Bool | TypeName::String) {
                            self.errors.push(Diagnostic::error("print argument must be Int, Bool, or String", argument.span));
                        }
                    }
                    return TypeName::Void;
                }
                if let Some(signature) = self.functions.get(callee).cloned() {
                    if arguments.len() != signature.parameters.len() {
                        self.errors.push(Diagnostic::error(format!("function '{}' expects {} arguments but received {}", callee, signature.parameters.len(), arguments.len()), expr.span));
                    }
                    for (index, argument) in arguments.iter().enumerate() {
                        let actual = self.check_expr(argument);
                        if let Some(expected) = signature.parameters.get(index) {
                            self.require_type(actual, *expected, argument.span, "function argument");
                        }
                    }
                    signature.return_type
                } else {
                    self.errors.push(Diagnostic::error(format!("call to unknown function '{}'", callee), expr.span));
                    for argument in arguments { self.check_expr(argument); }
                    TypeName::Int
                }
            }
        }
    }


}
