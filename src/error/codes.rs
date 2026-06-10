/// All error / warning / note codes from spec §78.
/// Grouped by phase that emits them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    // ── Lexer errors (E00xx) ──────────────────────────────────────────────────
    E0001, // unexpected character
    E0003, // bad indentation
    E0004, // mixed tabs and spaces
    E0006, // invalid escape sequence
    E0007, // unterminated string literal
    E0008, // invalid unicode escape
    E0009, // invalid number literal

    // ── Parser errors (E01xx) ─────────────────────────────────────────────────
    E0100, // unexpected token
    E0101, // unexpected EOF
    E0102, // expected `:` to open block
    E0103, // duplicate module declaration
    E0104, // missing module declaration

    // ── Name resolution errors (E03xx) ─────────────────────────────────────────
    E0300, // undefined variable
    E0301, // undefined function
    E0302, // undefined type
    E0303, // duplicate definition
    E0304, // import not found
    E0305, // ambiguous name
    E0306, // private access

    // ── Type errors (E04xx) ───────────────────────────────────────────────────
    E0400, // type mismatch
    E0401, // cannot infer type
    E0402, // wrong number of arguments
    E0403, // wrong number of generic arguments
    E0404, // field not found
    E0405, // method not found
    E0406, // not callable
    E0407, // missing return type
    E0408, // return type mismatch
    E0409, // incompatible types in binary operation
    E0410, // cannot apply unary operator
    E0411, // cannot cast
    E0412, // missing interface method
    E0413, // duplicate method in impl
    E0414, // non-exhaustive match

    // ── Borrow errors (E02xx) ─────────────────────────────────────────────────
    E0200, // use of moved value
    E0201, // cannot move out of borrowed reference
    E0202, // cannot borrow as mutable: already borrowed
    E0203, // borrow does not live long enough
    E0204, // cannot assign to immutable variable

    // ── Warnings (W0xxx) ──────────────────────────────────────────────────────
    W0001, // unused variable
    W0002, // unused import
    W0003, // unused function
    W0004, // unreachable code
    W0005, // unnecessary cast
    W0006, // variable shadowing
}

impl ErrorCode {
    /// Human-readable code string, e.g. "E0400".
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorCode::E0001 => "E0001",
            ErrorCode::E0003 => "E0003",
            ErrorCode::E0004 => "E0004",
            ErrorCode::E0006 => "E0006",
            ErrorCode::E0007 => "E0007",
            ErrorCode::E0008 => "E0008",
            ErrorCode::E0009 => "E0009",
            ErrorCode::E0100 => "E0100",
            ErrorCode::E0101 => "E0101",
            ErrorCode::E0102 => "E0102",
            ErrorCode::E0103 => "E0103",
            ErrorCode::E0104 => "E0104",
            ErrorCode::E0300 => "E0300",
            ErrorCode::E0301 => "E0301",
            ErrorCode::E0302 => "E0302",
            ErrorCode::E0303 => "E0303",
            ErrorCode::E0304 => "E0304",
            ErrorCode::E0305 => "E0305",
            ErrorCode::E0306 => "E0306",
            ErrorCode::E0400 => "E0400",
            ErrorCode::E0401 => "E0401",
            ErrorCode::E0402 => "E0402",
            ErrorCode::E0403 => "E0403",
            ErrorCode::E0404 => "E0404",
            ErrorCode::E0405 => "E0405",
            ErrorCode::E0406 => "E0406",
            ErrorCode::E0407 => "E0407",
            ErrorCode::E0408 => "E0408",
            ErrorCode::E0409 => "E0409",
            ErrorCode::E0410 => "E0410",
            ErrorCode::E0411 => "E0411",
            ErrorCode::E0412 => "E0412",
            ErrorCode::E0413 => "E0413",
            ErrorCode::E0414 => "E0414",
            ErrorCode::E0200 => "E0200",
            ErrorCode::E0201 => "E0201",
            ErrorCode::E0202 => "E0202",
            ErrorCode::E0203 => "E0203",
            ErrorCode::E0204 => "E0204",
            ErrorCode::W0001 => "W0001",
            ErrorCode::W0002 => "W0002",
            ErrorCode::W0003 => "W0003",
            ErrorCode::W0004 => "W0004",
            ErrorCode::W0005 => "W0005",
            ErrorCode::W0006 => "W0006",
        }
    }

    pub fn is_warning(self) -> bool {
        matches!(self,
            ErrorCode::W0001 | ErrorCode::W0002 | ErrorCode::W0003
            | ErrorCode::W0004 | ErrorCode::W0005 | ErrorCode::W0006)
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
