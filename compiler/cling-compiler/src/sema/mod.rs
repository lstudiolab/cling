mod expressions;
mod statements;

use std::collections::HashMap;
use crate::ast::{Function, Program, Stmt, TypeName};
use crate::diagnostic::{Diagnostic, Span};

#[derive(Clone, Debug)]
struct FunctionSignature {
    parameters: Vec<TypeName>,
    return_type: TypeName,
    span: Span,
}

#[derive(Clone, Debug)]
struct VariableInfo {
    ty: TypeName,
    mutable: bool,
    span: Span,
}

pub fn check(program: &Program) -> Result<(), Vec<Diagnostic>> {
    Analyzer::new(program).run(program)
}

struct Analyzer {
    functions: HashMap<String, FunctionSignature>,
    errors: Vec<Diagnostic>,
    scopes: Vec<HashMap<String, VariableInfo>>,
    current_return: TypeName,
}

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
