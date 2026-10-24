use std::fmt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self { start, end, line, column }
    }

    pub fn join(self, other: Self) -> Self {
        Self { start: self.start.min(other.start), end: self.end.max(other.end), line: self.line, column: self.column }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub message: String,
    pub span: Option<Span>,
    pub note: Option<String>,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>, span: Span) -> Self {
        Self { message: message.into(), span: Some(span), note: None }
    }

    pub fn global(message: impl Into<String>) -> Self {
        Self { message: message.into(), span: None, note: None }
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    pub fn render(&self, filename: &str) -> String {
        match self.span {
            Some(span) => format!("{filename}:{}:{}: error: {}{}", span.line, span.column, self.message,
                self.note.as_ref().map(|n| format!("\n  note: {n}")).unwrap_or_default()),
            None => format!("{filename}: error: {}{}", self.message,
                self.note.as_ref().map(|n| format!("\n  note: {n}")).unwrap_or_default()),
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(span) = self.span {
            write!(f, "{}:{}: error: {}", span.line, span.column, self.message)?;
        } else {
            write!(f, "error: {}", self.message)?;
        }
        if let Some(note) = &self.note { write!(f, "\n  note: {note}")?; }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spans_join_to_cover_both_ranges() {
        let a = Span::new(3, 5, 1, 4);
        let b = Span::new(8, 10, 1, 9);
        assert_eq!(a.join(b), Span::new(3, 10, 1, 4));
    }
}
