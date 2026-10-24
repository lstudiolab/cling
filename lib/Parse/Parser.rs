// Cling parser: declarations, statements, expressions, and parser tests.
// Shared token navigation and recovery primitives live in ParserSupport.rs.
use crate::ast::{BinaryOp, Expr, ExprKind, Function, Parameter, Program, Stmt, TypeName, UnaryOp};
use crate::diagnostic::{Diagnostic, Span};
use crate::lexer::{Token, TokenKind};

#[path = "ParserSupport.rs"]
mod support;
use support::TokenCursor;

pub fn parse(tokens: Vec<Token>) -> Result<Program, Vec<Diagnostic>> {
    Parser::new(tokens).program()
}

struct Parser {
    cursor: TokenCursor,
    errors: Vec<Diagnostic>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { cursor: TokenCursor::new(tokens), errors: Vec::new() }
    }

    fn program(mut self) -> Result<Program, Vec<Diagnostic>> {
        let first = self.peek().span;
        let mut functions = Vec::new();

        while !self.at_end() {
            if !self.check(&TokenKind::Func) {
                self.error_at_current("expected a function declaration beginning with 'func'");
                self.synchronize();
                continue;
            }

            if let Some(function) = self.function() {
                functions.push(function);
            } else {
                self.synchronize();
            }
        }

        let end = self.peek().span;
        if self.errors.is_empty() {
            Ok(Program { functions, span: first.join(end) })
        } else {
            Err(self.errors)
        }
    }

    fn consume(&mut self, expected: &TokenKind, message: &str) -> Option<Token> {
        if self.check(expected) {
            Some(self.advance().clone())
        } else {
            self.error_at_current(message);
            None
        }
    }

    fn check(&self, expected: &TokenKind) -> bool {
        self.cursor.check(expected)
    }

    fn matches(&mut self, expected: &TokenKind) -> bool {
        self.cursor.matches(expected)
    }

    fn at_end(&self) -> bool {
        self.cursor.at_end()
    }

    fn peek(&self) -> &Token {
        self.cursor.peek()
    }

    fn peek_next(&self) -> &Token {
        self.cursor.peek_next()
    }

    fn previous(&self) -> &Token {
        self.cursor.previous()
    }

    fn advance(&mut self) -> &Token {
        self.cursor.advance()
    }

    fn error_at_current(&mut self, message: &str) {
        self.errors.push(Diagnostic::error(message, self.peek().span));
    }

    fn synchronize(&mut self) {
        if !self.at_end() {
            self.advance();
        }

        while !self.at_end() {
            if matches!(self.previous().kind, TokenKind::Semicolon) {
                return;
            }

            if matches!(
                self.peek().kind,
                TokenKind::Func
                    | TokenKind::Let
                    | TokenKind::Var
                    | TokenKind::If
                    | TokenKind::While
                    | TokenKind::Return
            ) {
                return;
            }

            self.advance();
        }
    }

    fn function(&mut self) -> Option<Function> {
        let start = self.consume(&TokenKind::Func, "expected 'func'")?.span;
        let (name, _) = self.identifier("expected function name")?;
        self.consume(&TokenKind::LeftParen, "expected '(' after function name")?;

        let mut params = Vec::new();
        if !self.check(&TokenKind::RightParen) {
            loop {
                let (param_name, param_span) = self.identifier("expected parameter name")?;
                self.consume(&TokenKind::Colon, "expected ':' after parameter name")?;
                let ty = self.type_name(false)?;
                params.push(Parameter {
                    name: param_name,
                    ty,
                    span: param_span.join(self.previous().span),
                });

                if !self.matches(&TokenKind::Comma) || self.check(&TokenKind::RightParen) {
                    break;
                }
            }
        }

        self.consume(&TokenKind::RightParen, "expected ')' after parameter list")?;
        let return_type = if self.matches(&TokenKind::Arrow) {
            self.type_name(true)?
        } else {
            TypeName::Void
        };

        let body = self.block()?;
        let end = self.previous().span;
        Some(Function { name, params, return_type, body, span: start.join(end) })
    }

    fn type_name(&mut self, allow_void: bool) -> Option<TypeName> {
        let token = self.advance().clone();
        let ty = match &token.kind {
            TokenKind::Identifier(name) if name == "Int" => Some(TypeName::Int),
            TokenKind::Identifier(name) if name == "Bool" => Some(TypeName::Bool),
            TokenKind::Identifier(name) if name == "String" => Some(TypeName::String),
            TokenKind::Identifier(name) if name == "Void" && allow_void => Some(TypeName::Void),
            _ => None,
        };

        if ty.is_none() {
            let message = if allow_void {
                "expected type Int, Bool, String, or Void"
            } else {
                "expected type Int, Bool, or String"
            };
            self.errors.push(Diagnostic::error(message, token.span));
        }
        ty
    }

    fn identifier(&mut self, message: &str) -> Option<(String, Span)> {
        let token = self.advance().clone();
        match token.kind {
            TokenKind::Identifier(name) => Some((name, token.span)),
            _ => {
                self.errors.push(Diagnostic::error(message, token.span));
                None
            }
        }
    }

    fn block(&mut self) -> Option<Vec<Stmt>> {
        self.consume(&TokenKind::LeftBrace, "expected '{' to begin block")?;
        let mut statements = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.at_end() {
            let before = self.cursor.position();
            if let Some(statement) = self.statement() {
                statements.push(statement);
            }
            if before == self.cursor.position() {
                self.advance();
            }
        }

        self.consume(&TokenKind::RightBrace, "expected '}' after block")?;
        Some(statements)
    }

    fn statement(&mut self) -> Option<Stmt> {
        if self.matches(&TokenKind::Let) {
            return self.variable(false);
        }
        if self.matches(&TokenKind::Var) {
            return self.variable(true);
        }
        if self.matches(&TokenKind::Return) {
            return self.return_statement();
        }
        if self.matches(&TokenKind::If) {
            return self.if_statement();
        }
        if self.matches(&TokenKind::While) {
            return self.while_statement();
        }
        if self.matches(&TokenKind::Pass) {
            let start = self.previous().span;
            self.consume(&TokenKind::Semicolon, "expected ';' after 'pass'")?;
            return Some(Stmt::Pass { span: start.join(self.previous().span) });
        }
        if self.check(&TokenKind::LeftBrace) {
            let start = self.peek().span;
            let body = self.block()?;
            return Some(Stmt::Block { body, span: start.join(self.previous().span) });
        }

        if let TokenKind::Identifier(name) = &self.peek().kind {
            if matches!(self.peek_next().kind, TokenKind::Equal) {
                let name = name.clone();
                let start = self.advance().span;
                self.advance();
                let value = self.expression()?;
                self.consume(&TokenKind::Semicolon, "expected ';' after assignment")?;
                return Some(Stmt::Assign {
                    name,
                    span: start.join(self.previous().span),
                    value,
                });
            }
        }

        let start = self.peek().span;
        let expr = self.expression()?;
        self.consume(&TokenKind::Semicolon, "expected ';' after expression")?;
        Some(Stmt::Expr { span: start.join(self.previous().span), expr })
    }

    fn variable(&mut self, mutable: bool) -> Option<Stmt> {
        let start = self.previous().span;
        let (name, _) = self.identifier("expected variable name")?;
        let annotation = if self.matches(&TokenKind::Colon) {
            Some(self.type_name(false)?)
        } else {
            None
        };

        self.consume(&TokenKind::Equal, "expected '=' before variable initializer")?;
        let initializer = self.expression()?;
        self.consume(&TokenKind::Semicolon, "expected ';' after variable declaration")?;
        Some(Stmt::Let {
            name,
            mutable,
            annotation,
            initializer,
            span: start.join(self.previous().span),
        })
    }

    fn return_statement(&mut self) -> Option<Stmt> {
        let start = self.previous().span;
        let value = if self.check(&TokenKind::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(&TokenKind::Semicolon, "expected ';' after return")?;
        Some(Stmt::Return { value, span: start.join(self.previous().span) })
    }

    fn if_statement(&mut self) -> Option<Stmt> {
        let start = self.previous().span;
        let condition = self.condition()?;
        let then_body = self.block()?;
        let else_body = if self.matches(&TokenKind::Else) {
            if self.matches(&TokenKind::If) {
                vec![self.if_statement()?]
            } else {
                self.block()?
            }
        } else {
            Vec::new()
        };

        Some(Stmt::If {
            condition,
            then_body,
            else_body,
            span: start.join(self.previous().span),
        })
    }

    fn while_statement(&mut self) -> Option<Stmt> {
        let start = self.previous().span;
        let condition = self.condition()?;
        let body = self.block()?;
        Some(Stmt::While { condition, body, span: start.join(self.previous().span) })
    }

    fn condition(&mut self) -> Option<Expr> {
        if self.matches(&TokenKind::LeftParen) {
            let expression = self.expression()?;
            self.consume(&TokenKind::RightParen, "expected ')' after condition")?;
            Some(expression)
        } else {
            self.expression()
        }
    }

    fn expression(&mut self) -> Option<Expr> {
        self.or()
    }

    fn or(&mut self) -> Option<Expr> {
        self.binary_chain(Self::and, &[(TokenKind::OrOr, BinaryOp::Or)])
    }

    fn and(&mut self) -> Option<Expr> {
        self.binary_chain(Self::equality, &[(TokenKind::AndAnd, BinaryOp::And)])
    }

    fn equality(&mut self) -> Option<Expr> {
        self.binary_chain(
            Self::comparison,
            &[(TokenKind::EqualEqual, BinaryOp::Equal), (TokenKind::BangEqual, BinaryOp::NotEqual)],
        )
    }

    fn comparison(&mut self) -> Option<Expr> {
        self.binary_chain(
            Self::term,
            &[
                (TokenKind::Less, BinaryOp::Less),
                (TokenKind::LessEqual, BinaryOp::LessEqual),
                (TokenKind::Greater, BinaryOp::Greater),
                (TokenKind::GreaterEqual, BinaryOp::GreaterEqual),
            ],
        )
    }

    fn term(&mut self) -> Option<Expr> {
        self.binary_chain(
            Self::factor,
            &[(TokenKind::Plus, BinaryOp::Add), (TokenKind::Minus, BinaryOp::Subtract)],
        )
    }

    fn factor(&mut self) -> Option<Expr> {
        self.binary_chain(
            Self::unary,
            &[
                (TokenKind::Star, BinaryOp::Multiply),
                (TokenKind::Slash, BinaryOp::Divide),
                (TokenKind::Percent, BinaryOp::Remainder),
            ],
        )
    }

    fn binary_chain(
        &mut self,
        next: fn(&mut Self) -> Option<Expr>,
        operators: &[(TokenKind, BinaryOp)],
    ) -> Option<Expr> {
        let mut expression = next(self)?;

        loop {
            let operator = operators
                .iter()
                .find(|(kind, _)| self.check(kind))
                .map(|(_, operator)| *operator);
            let Some(operator) = operator else { break };

            self.advance();
            let right = next(self)?;
            let span = expression.span.join(right.span);
            expression = Expr {
                kind: ExprKind::Binary {
                    op: operator,
                    left: Box::new(expression),
                    right: Box::new(right),
                },
                span,
            };
        }

        Some(expression)
    }

    fn unary(&mut self) -> Option<Expr> {
        if self.matches(&TokenKind::Bang) || self.matches(&TokenKind::Minus) {
            let token = self.previous().clone();
            let operator = if matches!(token.kind, TokenKind::Bang) {
                UnaryOp::Not
            } else {
                UnaryOp::Negate
            };
            let operand = self.unary()?;
            let span = token.span.join(operand.span);
            return Some(Expr {
                kind: ExprKind::Unary { op: operator, operand: Box::new(operand) },
                span,
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Option<Expr> {
        let token = self.advance().clone();
        match token.kind {
            TokenKind::Integer(value) => Some(Expr { kind: ExprKind::Int(value), span: token.span }),
            TokenKind::String(value) => Some(Expr { kind: ExprKind::String(value), span: token.span }),
            TokenKind::True => Some(Expr { kind: ExprKind::Bool(true), span: token.span }),
            TokenKind::False => Some(Expr { kind: ExprKind::Bool(false), span: token.span }),
            TokenKind::Identifier(name) => {
                if self.matches(&TokenKind::LeftParen) {
                    let mut arguments = Vec::new();
                    if !self.check(&TokenKind::RightParen) {
                        loop {
                            arguments.push(self.expression()?);
                            if !self.matches(&TokenKind::Comma) || self.check(&TokenKind::RightParen) {
                                break;
                            }
                        }
                    }

                    let end = self.consume(&TokenKind::RightParen, "expected ')' after arguments")?.span;
                    Some(Expr {
                        kind: ExprKind::Call { callee: name, arguments },
                        span: token.span.join(end),
                    })
                } else {
                    Some(Expr { kind: ExprKind::Variable(name), span: token.span })
                }
            }
            TokenKind::LeftParen => {
                let mut expression = self.expression()?;
                let end = self.consume(&TokenKind::RightParen, "expected ')' after expression")?.span;
                expression.span = token.span.join(end);
                Some(expression)
            }
            _ => {
                self.errors.push(Diagnostic::error("expected an expression", token.span));
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOp, ExprKind};
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
        let Stmt::Let { initializer, .. } = &program.functions[0].body[0] else {
            panic!("expected let");
        };
        let ExprKind::Binary { op: BinaryOp::Add, right, .. } = &initializer.kind else {
            panic!("expected addition at root");
        };
        assert!(matches!(right.kind, ExprKind::Binary { op: BinaryOp::Multiply, .. }));
    }

    #[test]
    fn reports_a_missing_closing_delimiter_without_panicking() {
        let tokens = lex("func main() { print(\"hello\";").unwrap();
        assert!(parse(tokens).is_err());
    }

    #[test]
    fn empty_token_stream_is_safe() {
        assert!(parse(Vec::new()).is_err());
    }
}
