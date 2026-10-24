use std::collections::HashMap;

use crate::ast::{BinaryOp, Expr, ExprKind, Function, Program, Stmt, TypeName, UnaryOp};
use crate::diagnostic::{Diagnostic, Span};

#[derive(Clone, Debug)]
struct FunctionSignature { parameters: Vec<TypeName>, return_type: TypeName, span: Span }
#[derive(Clone, Debug)]
struct VariableInfo { ty: TypeName, mutable: bool, span: Span }

pub fn check(program: &Program) -> Result<(), Vec<Diagnostic>> {
    Analyzer::new(program).run(program)
}

struct Analyzer { functions: HashMap<String, FunctionSignature>, errors: Vec<Diagnostic>, scopes: Vec<HashMap<String, VariableInfo>>, current_return: TypeName }

impl Analyzer {
    fn new(program: &Program) -> Self {
        let mut functions = HashMap::new();
        let mut errors = Vec::new();
        for function in &program.functions {
            let signature = FunctionSignature { parameters: function.params.iter().map(|p| p.ty).collect(), return_type: function.return_type, span: function.span };
            if let Some(previous) = functions.get(&function.name) {
                let previous: &FunctionSignature = previous;
                errors.push(Diagnostic::error(format!("function '{}' is declared more than once", function.name), function.span)
                    .with_note(format!("previous declaration is at {}:{}", previous.span.line, previous.span.column)));
            } else { functions.insert(function.name.clone(), signature); }
        }
        Self { functions, errors, scopes: Vec::new(), current_return: TypeName::Void }
    }

    fn run(mut self, program: &Program) -> Result<(), Vec<Diagnostic>> {
        if program.functions.is_empty() { self.errors.push(Diagnostic::global("a Cling source file must declare at least one function")); }
        for function in &program.functions { self.check_function(function); }
        if self.errors.is_empty() { Ok(()) } else { Err(self.errors) }
    }

    fn check_function(&mut self, function: &Function) {
        self.current_return = function.return_type;
        self.scopes.clear();
        self.scopes.push(HashMap::new());
        for parameter in &function.params {
            if self.scopes[0].contains_key(&parameter.name) {
                self.errors.push(Diagnostic::error(format!("duplicate parameter '{}'", parameter.name), parameter.span));
            } else {
                self.scopes[0].insert(parameter.name.clone(), VariableInfo { ty: parameter.ty, mutable: false, span: parameter.span });
            }
        }
        let returns = self.check_statements(&function.body);
        if function.return_type != TypeName::Void && !returns {
            self.errors.push(Diagnostic::error(format!("function '{}' must return {} on every path", function.name, function.return_type), function.span));
        }
    }

    fn check_statements(&mut self, statements: &[Stmt]) -> bool {
        let mut always_returns = false;
        for stmt in statements {
            let returns = self.check_statement(stmt);
            if returns { always_returns = true; }
        }
        always_returns
    }

    fn check_statement(&mut self, stmt: &Stmt) -> bool {
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

    fn check_expr(&mut self, expr: &Expr) -> TypeName {
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

    fn lookup(&self, name: &str) -> Option<&VariableInfo> {
        self.scopes.iter().rev().find_map(|scope| scope.get(name))
    }

    fn require_type(&mut self, actual: TypeName, expected: TypeName, span: Span, context: &str) {
        if actual != expected {
            self.errors.push(Diagnostic::error(format!("{context} has type {actual}, expected {expected}"), span));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer, parser};
    use super::*;

    fn check_source(source: &str) -> Result<(), Vec<Diagnostic>> {
        let tokens = lexer::lex(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        check(&program)
    }

    #[test]
    fn accepts_typed_mutable_variables() {
        assert!(check_source("func main() { var count: Int = 1; count = count + 1; print(count); }").is_ok());
    }

    #[test]
    fn rejects_mutation_of_let() {
        let errors = check_source("func main() { let count = 1; count = 2; }").unwrap_err();
        assert!(errors.iter().any(|e| e.message.contains("immutable")));
    }

    #[test]
    fn checks_return_type() {
        let errors = check_source("func answer() -> Int { return true; }").unwrap_err();
        assert!(errors.iter().any(|e| e.message.contains("return value")));
    }

    #[test]
    fn rejects_void_variable_initializers() {
        let errors = check_source("func main() { let result = print(1); }").unwrap_err();
        assert!(errors.iter().any(|e| e.message.contains("Void expression cannot initialize")));
    }
}
