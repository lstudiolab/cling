use crate::diagnostic::{Diagnostic, Span};
use super::{Token, TokenKind};

pub fn lex(source: &str) -> Result<Vec<Token>, Vec<Diagnostic>> {
    Lexer::new(source).run()
}

struct Lexer { chars: Vec<char>, current: usize, line: usize, column: usize, start: usize, start_line: usize, start_column: usize, tokens: Vec<Token>, errors: Vec<Diagnostic> }

impl Lexer {
    fn new(source: &str) -> Self {
        Self { chars: source.chars().collect(), current: 0, line: 1, column: 1, start: 0, start_line: 1, start_column: 1, tokens: Vec::new(), errors: Vec::new() }
    }

    fn run(mut self) -> Result<Vec<Token>, Vec<Diagnostic>> {
        while !self.at_end() {
            self.start = self.current; self.start_line = self.line; self.start_column = self.column;
            self.scan_token();
        }
        self.tokens.push(Token { kind: TokenKind::Eof, span: Span::new(self.current, self.current, self.line, self.column) });
        if self.errors.is_empty() { Ok(self.tokens) } else { Err(self.errors) }
    }

    fn at_end(&self) -> bool { self.current >= self.chars.len() }
    fn peek(&self) -> char { self.chars.get(self.current).copied().unwrap_or('\0') }
    fn peek_next(&self) -> char { self.chars.get(self.current + 1).copied().unwrap_or('\0') }
    fn advance(&mut self) -> char { let c = self.peek(); if !self.at_end() { self.current += 1; if c == '\n' { self.line += 1; self.column = 1; } else { self.column += 1; } } c }
    fn matches(&mut self, expected: char) -> bool { if self.peek() != expected { return false; } self.advance(); true }
    fn span(&self) -> Span { Span::new(self.start, self.current, self.start_line, self.start_column) }
    fn push(&mut self, kind: TokenKind) { self.tokens.push(Token { kind, span: self.span() }); }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            ' ' | '\r' | '\t' | '\n' => {},
            '(' => self.push(TokenKind::LeftParen), ')' => self.push(TokenKind::RightParen),
            '{' => self.push(TokenKind::LeftBrace), '}' => self.push(TokenKind::RightBrace),
            ':' => self.push(TokenKind::Colon), ',' => self.push(TokenKind::Comma), ';' => self.push(TokenKind::Semicolon),
            '+' => self.push(TokenKind::Plus), '*' => self.push(TokenKind::Star), '/' => {
                if self.matches('/') { while self.peek() != '\n' && !self.at_end() { self.advance(); } } else { self.push(TokenKind::Slash); }
            },
            '%' => self.push(TokenKind::Percent), '-' => { if self.matches('>') { self.push(TokenKind::Arrow); } else { self.push(TokenKind::Minus); } },
            '!' => { if self.matches('=') { self.push(TokenKind::BangEqual); } else { self.push(TokenKind::Bang); } },
            '=' => { if self.matches('=') { self.push(TokenKind::EqualEqual); } else { self.push(TokenKind::Equal); } },
            '<' => { if self.matches('=') { self.push(TokenKind::LessEqual); } else { self.push(TokenKind::Less); } },
            '>' => { if self.matches('=') { self.push(TokenKind::GreaterEqual); } else { self.push(TokenKind::Greater); } },
            '&' => { if self.matches('&') { self.push(TokenKind::AndAnd); } else { self.errors.push(Diagnostic::error("expected '&' after '&'", self.span())); } },
            '|' => { if self.matches('|') { self.push(TokenKind::OrOr); } else { self.errors.push(Diagnostic::error("expected '|' after '|'", self.span())); } },
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if is_ident_start(c) => self.identifier(),
            _ => self.errors.push(Diagnostic::error(format!("unexpected character '{c}'"), self.span())),
        }
    }

    fn string(&mut self) {
        let mut value = String::new();
        while !self.at_end() && self.peek() != '"' {
            let c = self.advance();
            if c == '\n' { self.errors.push(Diagnostic::error("newline in string literal", self.span())); return; }
            if c != '\\' { value.push(c); continue; }
            if self.at_end() { self.errors.push(Diagnostic::error("unterminated escape sequence", self.span())); return; }
            match self.advance() {
                'n' => value.push('\n'), 'r' => value.push('\r'), 't' => value.push('\t'), '0' => value.push('\0'),
                '"' => value.push('"'), '\\' => value.push('\\'),
                other => { self.errors.push(Diagnostic::error(format!("unsupported escape sequence '\\{other}'"), self.span())); value.push(other); }
            }
        }
        if self.at_end() { self.errors.push(Diagnostic::error("unterminated string literal", self.span())); return; }
        self.advance();
        self.push(TokenKind::String(value));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() { self.advance(); }
        let text: String = self.chars[self.start..self.current].iter().collect();
        match text.parse::<i64>() { Ok(value) => self.push(TokenKind::Integer(value)), Err(_) => self.errors.push(Diagnostic::error("integer literal is outside the signed 64-bit range", self.span())) }
    }

    fn identifier(&mut self) {
        while is_ident_continue(self.peek()) { self.advance(); }
        let text: String = self.chars[self.start..self.current].iter().collect();
        let kind = match text.as_str() {
            "func" => TokenKind::Func, "let" => TokenKind::Let, "var" => TokenKind::Var,
            "if" => TokenKind::If, "else" => TokenKind::Else, "while" => TokenKind::While,
            "return" => TokenKind::Return, "true" => TokenKind::True, "false" => TokenKind::False,
            "pass" => TokenKind::Pass, _ => TokenKind::Identifier(text),
        };
        self.push(kind);
    }
}

fn is_ident_start(c: char) -> bool { c == '_' || c.is_ascii_alphabetic() || !c.is_ascii() && c.is_alphabetic() }
fn is_ident_continue(c: char) -> bool { is_ident_start(c) || c.is_ascii_digit() }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recognizes_keywords_operators_and_escaped_strings() {
        let tokens = lex("func main() { print(\"hi\\n\"); let n = 4 + 2; }").unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::Func));
        assert!(tokens.iter().any(|t| matches!(&t.kind, TokenKind::String(s) if s == "hi\n")));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Plus)));
    }

    #[test]
    fn reports_unterminated_strings() {
        assert!(lex("func main() { print(\"oops); }").is_err());
    }
}
