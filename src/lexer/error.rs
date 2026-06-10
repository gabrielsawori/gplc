use super::token::Span;
use std::fmt;

/// A lexer diagnostic — error or warning with location.
#[derive(Debug, Clone)]
pub struct LexError {
    pub code:    &'static str,   // e.g. "E0003"
    pub message: String,
    pub span:    Span,
}

impl LexError {
    pub fn new(code: &'static str, message: impl Into<String>, span: Span) -> Self {
        Self { code, message: message.into(), span }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

/// Helper constructors for every lex error code.
pub fn e0003_bad_indent(span: Span) -> LexError {
    LexError::new("E0003", "indentation is not a multiple of 4 spaces", span)
}

pub fn e0004_mixed_indent(span: Span) -> LexError {
    LexError::new("E0004", "tabs and spaces cannot be mixed in indentation; use 4 spaces", span)
}

pub fn e0006_invalid_escape(ch: char, span: Span) -> LexError {
    LexError::new("E0006", format!("invalid escape sequence `\\{}`", ch), span)
}

pub fn e0007_unterminated_string(span: Span) -> LexError {
    LexError::new("E0007", "unterminated string literal", span)
}

pub fn e0008_invalid_unicode(span: Span) -> LexError {
    LexError::new("E0008", "invalid unicode escape: value out of range or malformed", span)
}

pub fn e0009_invalid_number(msg: impl Into<String>, span: Span) -> LexError {
    LexError::new("E0009", format!("invalid number literal: {}", msg.into()), span)
}

pub fn e0001_unexpected_char(ch: char, span: Span) -> LexError {
    LexError::new("E0001", format!("unexpected character `{}`", ch), span)
}
