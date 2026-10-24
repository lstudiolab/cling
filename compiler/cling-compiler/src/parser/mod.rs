mod declarations;
mod expressions;
mod statements;

use crate::ast::Program;
use crate::diagnostic::Diagnostic;
use crate::lexer::{Token, TokenKind};

pub fn parse(tokens: Vec<Token>) -> Result<Program, Vec<Diagnostic>> {
    Parser::new(tokens).program()
}

pub(super) struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<Diagnostic>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self { Self { tokens, current: 0, errors: Vec::new() } }

    fn program(mut self) -> Result<Program, Vec<Diagnostic>> {
        let first = self.peek().span;
        let mut functions = Vec::new();
        while !self.at_end() {
            if !self.check(&TokenKind::Func) {
                self.error_at_current("expected a function declaration beginning with 'func'");
                self.synchronize();
                continue;
            }
            match self.function() { Some(function) => functions.push(function), None => self.synchronize() }
        }
        let end = self.peek().span;
        if self.errors.is_empty() { Ok(Program { functions, span: first.join(end) }) } else { Err(self.errors) }
    }

    fn consume(&mut self, expected: &TokenKind, message: &str) -> Option<&Token> {
        if self.check(expected) { Some(self.advance()) } else { self.error_at_current(message); None }
    }
    fn check(&self, expected: &TokenKind) -> bool { std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(expected) }
    fn matches(&mut self, expected: &TokenKind) -> bool { if self.check(expected) { self.advance(); true } else { false } }
    fn at_end(&self) -> bool { matches!(self.peek().kind, TokenKind::Eof) }
    fn peek(&self) -> &Token { &self.tokens[self.current.min(self.tokens.len() - 1)] }
    fn peek_next(&self) -> &Token { &self.tokens[(self.current + 1).min(self.tokens.len() - 1)] }
    fn previous(&self) -> &Token { &self.tokens[self.current.saturating_sub(1)] }
    fn advance(&mut self) -> &Token { if !self.at_end() { self.current += 1; } self.previous() }
    fn error_at_current(&mut self, message: &str) { self.errors.push(Diagnostic::error(message, self.peek().span)); }
    fn synchronize(&mut self) {
        if !self.at_end() { self.advance(); }
        while !self.at_end() {
            if matches!(self.previous().kind, TokenKind::Semicolon) { return; }
            if matches!(self.peek().kind, TokenKind::Func | TokenKind::Let | TokenKind::Var | TokenKind::If | TokenKind::While | TokenKind::Return) { return; }
            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOp, ExprKind, Stmt};
    use crate::lexer::lex;

    #[test]
    fn parses_hello_world_function() {
        let program = parse(lex("func main() { print(\"Hello, World!\"); }").unwrap()).unwrap();
        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "main");
        assert_eq!(program.functions[0].body.len(), 1);
    }

    #[test]
    fn parses_arithmetic_precedence() {
        let program = parse(lex("func main() { let x = 2 + 3 * 4; }").unwrap()).unwrap();
        let Stmt::Let { initializer, .. } = &program.functions[0].body[0] else { panic!("expected let") };
        let ExprKind::Binary { op: BinaryOp::Add, right, .. } = &initializer.kind else { panic!("expected addition at root") };
        assert!(matches!(right.kind, ExprKind::Binary { op: BinaryOp::Multiply, .. }));
    }
}
