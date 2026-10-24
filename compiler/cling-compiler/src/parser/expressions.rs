use super::Parser;
use crate::ast::{BinaryOp, Expr, ExprKind, UnaryOp};
use crate::diagnostic::Diagnostic;
use crate::lexer::TokenKind;

impl Parser {
    pub(super) fn expression(&mut self) -> Option<Expr> { self.or() }
    fn or(&mut self) -> Option<Expr> { self.binary_chain(Self::and, &[(TokenKind::OrOr, BinaryOp::Or)]) }
    fn and(&mut self) -> Option<Expr> { self.binary_chain(Self::equality, &[(TokenKind::AndAnd, BinaryOp::And)]) }
    fn equality(&mut self) -> Option<Expr> { self.binary_chain(Self::comparison, &[(TokenKind::EqualEqual, BinaryOp::Equal), (TokenKind::BangEqual, BinaryOp::NotEqual)]) }
    fn comparison(&mut self) -> Option<Expr> { self.binary_chain(Self::term, &[(TokenKind::Less, BinaryOp::Less), (TokenKind::LessEqual, BinaryOp::LessEqual), (TokenKind::Greater, BinaryOp::Greater), (TokenKind::GreaterEqual, BinaryOp::GreaterEqual)]) }
    fn term(&mut self) -> Option<Expr> { self.binary_chain(Self::factor, &[(TokenKind::Plus, BinaryOp::Add), (TokenKind::Minus, BinaryOp::Subtract)]) }
    fn factor(&mut self) -> Option<Expr> { self.binary_chain(Self::unary, &[(TokenKind::Star, BinaryOp::Multiply), (TokenKind::Slash, BinaryOp::Divide), (TokenKind::Percent, BinaryOp::Remainder)]) }

    fn binary_chain(&mut self, next: fn(&mut Self) -> Option<Expr>, operators: &[(TokenKind, BinaryOp)]) -> Option<Expr> {
        let mut expr = next(self)?;
        loop {
            let matched = operators.iter().find(|(kind, _)| self.check(kind)).map(|(_, op)| *op);
            let Some(op) = matched else { break; };
            self.advance();
            let right = next(self)?;
            let span = expr.span.join(right.span);
            expr = Expr { kind: ExprKind::Binary { op, left: Box::new(expr), right: Box::new(right) }, span };
        }
        Some(expr)
    }

    fn unary(&mut self) -> Option<Expr> {
        if self.matches(&TokenKind::Bang) || self.matches(&TokenKind::Minus) {
            let token = self.previous().clone();
            let op = if matches!(token.kind, TokenKind::Bang) { UnaryOp::Not } else { UnaryOp::Negate };
            let operand = self.unary()?;
            let span = token.span.join(operand.span);
            return Some(Expr { kind: ExprKind::Unary { op, operand: Box::new(operand) }, span });
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
                            if !self.matches(&TokenKind::Comma) { break; }
                            if self.check(&TokenKind::RightParen) { break; }
                        }
                    }
                    let end = self.consume(&TokenKind::RightParen, "expected ')' after arguments")?.span;
                    Some(Expr { kind: ExprKind::Call { callee: name, arguments }, span: token.span.join(end) })
                } else { Some(Expr { kind: ExprKind::Variable(name), span: token.span }) }
            }
            TokenKind::LeftParen => {
                let mut expr = self.expression()?;
                let end = self.consume(&TokenKind::RightParen, "expected ')' after expression")?.span;
                expr.span = token.span.join(end);
                Some(expr)
            }
            _ => { self.errors.push(Diagnostic::error("expected an expression", token.span)); None }
        }
    }


}
