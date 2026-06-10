use crate::lexer::token::{Span, TokenKind};
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub code:    &'static str,
    pub message: String,
    pub span:    Span,
    pub hint:    Option<String>,
}

impl ParseError {
    pub fn new(code: &'static str, msg: impl Into<String>, span: Span) -> Self {
        Self { code, message: msg.into(), span, hint: None }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

// ── Constructor helpers ───────────────────────────────────────────────────────

pub fn e0001_unexpected_token(got: &TokenKind, expected: &str, span: Span) -> ParseError {
    ParseError::new(
        "E0001",
        format!("unexpected {}, expected {}", got.display(), expected),
        span,
    )
}

pub fn e0002_unexpected_eof(expected: &str, span: Span) -> ParseError {
    ParseError::new(
        "E0002",
        format!("unexpected end of file, expected {}", expected),
        span,
    )
}

pub fn e0005_missing_colon(span: Span) -> ParseError {
    ParseError::new("E0005", "expected `:` to open block", span)
        .with_hint("add `:` at the end of this line")
}

pub fn e0011_duplicate_module(span: Span) -> ParseError {
    ParseError::new("E0011", "more than one `module` declaration in file", span)
}

pub fn e0012_missing_module(span: Span) -> ParseError {
    ParseError::new("E0012", "file has no `module` declaration", span)
        .with_hint("add `module my_module` as the first line")
}
