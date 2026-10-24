use std::collections::HashMap;

use super::{Analyzer, VariableInfo};
use crate::ast::{Stmt, TypeName};
use crate::diagnostic::Diagnostic;

impl Analyzer {
    pub(super) fn check_statement(&mut self, stmt: &Stmt) -> bool {
        match stmt {
            Stmt::Let { name, mutable, annotation, initializer, span } => {
                let actual = self.check_expr(initializer);
                if actual == TypeName::Void {
                    self.errors.push(Diagnostic::error("a Void expression cannot initialize a variable", initializer.span));
                }
                let ty = annotation.unwrap_or(actual);
                if let Some(expected) = annotation { self.require_type(actual, *expected, initializer.span, "variable initializer"); }
                let scope = self.scopes.last_mut().expect("function scope always exists");
                if scope.contains_key(name) {
                    self.errors.push(Diagnostic::error(format!("'{}' is already declared in this scope", name), *span));
                } else {
                    scope.insert(name.clone(), VariableInfo { ty, mutable: *mutable, span: *span });
                }
                false
            }
            Stmt::Assign { name, value, span } => {
                let value_type = self.check_expr(value);
                if let Some(info) = self.lookup(name).cloned() {
                    if !info.mutable {
                        self.errors.push(Diagnostic::error(format!("cannot assign to immutable binding '{}'", name), *span)
                            .with_note(format!("binding declared at {}:{}; use 'var' for a mutable binding", info.span.line, info.span.column)));
                    }
                    self.require_type(value_type, info.ty, value.span, "assignment");
                } else {
                    self.errors.push(Diagnostic::error(format!("use of undeclared variable '{}'", name), *span));
                }
                false
            }
            Stmt::Expr { expr, .. } => { self.check_expr(expr); false }
            Stmt::Return { value, span } => {
                match (self.current_return, value) {
                    (TypeName::Void, None) => {},
                    (TypeName::Void, Some(expr)) => {
                        self.check_expr(expr);
                        self.errors.push(Diagnostic::error("a Void function cannot return a value", expr.span));
                    }
                    (expected, Some(expr)) => { let actual = self.check_expr(expr); self.require_type(actual, expected, expr.span, "return value"); }
                    (_, None) => self.errors.push(Diagnostic::error("this function must return a value", *span)),
                }
                true
            }
            Stmt::If { condition, then_body, else_body, .. } => {
                let ty = self.check_expr(condition);
                self.require_type(ty, TypeName::Bool, condition.span, "if condition");
                self.scopes.push(HashMap::new());
                let then_returns = self.check_statements(then_body);
                self.scopes.pop();
                self.scopes.push(HashMap::new());
                let else_returns = self.check_statements(else_body);
                self.scopes.pop();
                !else_body.is_empty() && then_returns && else_returns
            }
            Stmt::While { condition, body, .. } => {
                let ty = self.check_expr(condition);
                self.require_type(ty, TypeName::Bool, condition.span, "while condition");
                self.scopes.push(HashMap::new());
                self.check_statements(body);
                self.scopes.pop();
                false
            }
            Stmt::Block { body, .. } => {
                self.scopes.push(HashMap::new());
                let returns = self.check_statements(body);
                self.scopes.pop();
                returns
            }
            Stmt::Pass { .. } => false,
        }
    }


}
