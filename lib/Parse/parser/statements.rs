use super::Parser;
use crate::ast::Stmt;
use crate::lexer::TokenKind;

impl Parser {
    pub(super) fn block(&mut self) -> Option<Vec<Stmt>> {
        self.consume(&TokenKind::LeftBrace, "expected '{' to begin block")?;
        let mut statements = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.at_end() {
            let before = self.current;
            if let Some(stmt) = self.statement() { statements.push(stmt); }
            if before == self.current { self.advance(); }
        }
        self.consume(&TokenKind::RightBrace, "expected '}' after block")?;
        Some(statements)
    }

    fn statement(&mut self) -> Option<Stmt> {
        if self.matches(&TokenKind::Let) { return self.variable(false); }
        if self.matches(&TokenKind::Var) { return self.variable(true); }
        if self.matches(&TokenKind::Return) { return self.return_statement(); }
        if self.matches(&TokenKind::If) { return self.if_statement(); }
        if self.matches(&TokenKind::While) { return self.while_statement(); }
        if self.matches(&TokenKind::Pass) {
            let span = self.previous().span;
            self.consume(&TokenKind::Semicolon, "expected ';' after 'pass'")?;
            return Some(Stmt::Pass { span: span.join(self.previous().span) });
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
                return Some(Stmt::Assign { name, span: start.join(self.previous().span), value });
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
        let annotation = if self.matches(&TokenKind::Colon) { Some(self.type_name(false)?) } else { None };
        self.consume(&TokenKind::Equal, "expected '=' before variable initializer")?;
        let initializer = self.expression()?;
        self.consume(&TokenKind::Semicolon, "expected ';' after variable declaration")?;
        Some(Stmt::Let { name, mutable, annotation, initializer, span: start.join(self.previous().span) })
    }

    fn return_statement(&mut self) -> Option<Stmt> {
        let start = self.previous().span;
        let value = if self.check(&TokenKind::Semicolon) { None } else { Some(self.expression()?) };
        self.consume(&TokenKind::Semicolon, "expected ';' after return")?;
        Some(Stmt::Return { value, span: start.join(self.previous().span) })
    }

    fn if_statement(&mut self) -> Option<Stmt> {
        let start = self.previous().span;
        let condition = if self.matches(&TokenKind::LeftParen) {
            let expr = self.expression()?;
            self.consume(&TokenKind::RightParen, "expected ')' after if condition")?;
            expr
        } else { self.expression()? };
        let then_body = self.block()?;
        let else_body = if self.matches(&TokenKind::Else) {
            if self.matches(&TokenKind::If) {
                let nested = self.if_statement()?;
                vec![nested]
            } else { self.block()? }
        } else { Vec::new() };
        Some(Stmt::If { condition, then_body, else_body, span: start.join(self.previous().span) })
    }

    fn while_statement(&mut self) -> Option<Stmt> {
        let start = self.previous().span;
        let condition = if self.matches(&TokenKind::LeftParen) {
            let expr = self.expression()?;
            self.consume(&TokenKind::RightParen, "expected ')' after while condition")?;
            expr
        } else { self.expression()? };
        let body = self.block()?;
        Some(Stmt::While { condition, body, span: start.join(self.previous().span) })
    }


}
