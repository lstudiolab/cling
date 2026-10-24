// Parser support: safe token-stream navigation shared by all parser routines.
use crate::diagnostic::Span;
use crate::lexer::{Token, TokenKind};

pub(super) struct TokenCursor {
    tokens: Vec<Token>,
    current: usize,
}

impl TokenCursor {
    pub(super) fn new(mut tokens: Vec<Token>) -> Self {
        if tokens.is_empty() {
            tokens.push(Token {
                kind: TokenKind::Eof,
                span: Span::new(0, 0, 1, 1),
            });
        } else if !matches!(tokens.last().map(|token| &token.kind), Some(TokenKind::Eof)) {
            let end = tokens.last().expect("non-empty token list").span;
            tokens.push(Token {
                kind: TokenKind::Eof,
                span: Span::new(end.end, end.end, end.line, end.column),
            });
        }

        Self { tokens, current: 0 }
    }

    pub(super) fn position(&self) -> usize {
        self.current
    }

    pub(super) fn check(&self, expected: &TokenKind) -> bool {
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(expected)
    }

    pub(super) fn matches(&mut self, expected: &TokenKind) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub(super) fn at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    pub(super) fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub(super) fn peek_next(&self) -> &Token {
        &self.tokens[(self.current + 1).min(self.tokens.len() - 1)]
    }

    pub(super) fn previous(&self) -> &Token {
        &self.tokens[self.current.saturating_sub(1)]
    }

    pub(super) fn advance(&mut self) -> &Token {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;

    #[test]
    fn appends_eof_when_missing() {
        let tokens = lex("func main() {}").unwrap();
        let without_eof = tokens[..tokens.len() - 1].to_vec();
        let cursor = TokenCursor::new(without_eof);
        assert!(matches!(cursor.peek_next().kind, TokenKind::Identifier(_)));
        assert!(matches!(cursor.peek().kind, TokenKind::Func));
    }

    #[test]
    fn empty_input_has_a_safe_eof_token() {
        let cursor = TokenCursor::new(Vec::new());
        assert!(cursor.at_end());
        assert!(matches!(cursor.peek().kind, TokenKind::Eof));
    }
}
