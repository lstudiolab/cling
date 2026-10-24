use super::Parser;
use crate::ast::{Function, Parameter, TypeName};
use crate::diagnostic::{Diagnostic, Span};
use crate::lexer::TokenKind;

impl Parser {
    pub(super) fn function(&mut self) -> Option<Function> {
        let start = self.consume(&TokenKind::Func, "expected 'func'")?.span;
        let (name, _) = self.identifier("expected function name")?;
        self.consume(&TokenKind::LeftParen, "expected '(' after function name")?;
        let mut params = Vec::new();
        if !self.check(&TokenKind::RightParen) {
            loop {
                let (param_name, param_span) = self.identifier("expected parameter name")?;
                self.consume(&TokenKind::Colon, "expected ':' after parameter name")?;
                let ty = self.type_name(false)?;
                params.push(Parameter { name: param_name, ty, span: param_span.join(self.previous().span) });
                if !self.matches(&TokenKind::Comma) { break; }
                if self.check(&TokenKind::RightParen) { break; }
            }
        }
        self.consume(&TokenKind::RightParen, "expected ')' after parameter list")?;
        let return_type = if self.matches(&TokenKind::Arrow) { self.type_name(true)? } else { TypeName::Void };
        let body = self.block()?;
        let end = self.previous().span;
        Some(Function { name, params, return_type, body, span: start.join(end) })
    }

    pub(super) fn type_name(&mut self, allow_void: bool) -> Option<TypeName> {
        let token = self.advance().clone();
        let ty = match token.kind {
            TokenKind::Identifier(ref name) if name == "Int" => Some(TypeName::Int),
            TokenKind::Identifier(ref name) if name == "Bool" => Some(TypeName::Bool),
            TokenKind::Identifier(ref name) if name == "String" => Some(TypeName::String),
            TokenKind::Identifier(ref name) if name == "Void" && allow_void => Some(TypeName::Void),
            _ => None,
        };
        if ty.is_none() { self.errors.push(Diagnostic::error(if allow_void { "expected type Int, Bool, String, or Void" } else { "expected type Int, Bool, or String" }, token.span)); }
        ty
    }

    pub(super) fn identifier(&mut self, message: &str) -> Option<(String, Span)> {
        let token = self.advance().clone();
        match token.kind { TokenKind::Identifier(name) => Some((name, token.span)), _ => { self.errors.push(Diagnostic::error(message, token.span)); None } }
    }


}
