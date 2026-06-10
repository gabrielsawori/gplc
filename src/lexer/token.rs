/// Span — byte range in a source file.
/// Used by every token and AST node for error reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: u32,
    pub end:   u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn dummy() -> Self {
        Self { start: 0, end: 0 }
    }

    /// Merge two spans into one that covers both.
    pub fn to(&self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end:   self.end.max(other.end),
        }
    }
}

// ── Integer suffix ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntSuffix {
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
    Isize, Usize,
}

impl IntSuffix {
    pub fn as_str(self) -> &'static str {
        match self {
            IntSuffix::I8    => "i8",
            IntSuffix::I16   => "i16",
            IntSuffix::I32   => "i32",
            IntSuffix::I64   => "i64",
            IntSuffix::I128  => "i128",
            IntSuffix::U8    => "u8",
            IntSuffix::U16   => "u16",
            IntSuffix::U32   => "u32",
            IntSuffix::U64   => "u64",
            IntSuffix::U128  => "u128",
            IntSuffix::Isize => "isize",
            IntSuffix::Usize => "usize",
        }
    }
}

// ── Float suffix ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatSuffix {
    F32,
    F64,
}

// ── Keywords ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    // Declarations
    Module, Import, As, Use,
    Var, Let, Const, Comptime,
    Fn, Return, Async, Await, Yield,
    Struct, Enum, Union, Interface, Impl,
    Embed, Pub,
    Type, Newtype,
    // Control flow
    If, Elif, Else,
    For, While, Loop, In, Step,
    Match, Break, Continue, Defer,
    // Operators (word-form)
    And, Or, Not, Is,
    // Values
    True, False, Null,
    // Safety
    Unsafe, Asm,
    // Error handling
    Try, Catch, Finally,
    Panic, Assert,
    // Concurrency
    Select, After, Default,
    // Types
    Never, Void, Any,
    // Special
    SelfValue,   // self
    SelfType,    // Self
    Extern,
    Move,
}

impl Keyword {
    /// Try to parse a string as a keyword.
    pub fn from_str(s: &str) -> Option<Keyword> {
        match s {
            "module"    => Some(Keyword::Module),
            "import"    => Some(Keyword::Import),
            "as"        => Some(Keyword::As),
            "use"       => Some(Keyword::Use),
            "var"       => Some(Keyword::Var),
            "let"       => Some(Keyword::Let),
            "const"     => Some(Keyword::Const),
            "comptime"  => Some(Keyword::Comptime),
            "fn"        => Some(Keyword::Fn),
            "return"    => Some(Keyword::Return),
            "async"     => Some(Keyword::Async),
            "await"     => Some(Keyword::Await),
            "yield"     => Some(Keyword::Yield),
            "struct"    => Some(Keyword::Struct),
            "enum"      => Some(Keyword::Enum),
            "union"     => Some(Keyword::Union),
            "interface" => Some(Keyword::Interface),
            "impl"      => Some(Keyword::Impl),
            "embed"     => Some(Keyword::Embed),
            "pub"       => Some(Keyword::Pub),
            "type"      => Some(Keyword::Type),
            "newtype"   => Some(Keyword::Newtype),
            "if"        => Some(Keyword::If),
            "elif"      => Some(Keyword::Elif),
            "else"      => Some(Keyword::Else),
            "for"       => Some(Keyword::For),
            "while"     => Some(Keyword::While),
            "loop"      => Some(Keyword::Loop),
            "in"        => Some(Keyword::In),
            "step"      => Some(Keyword::Step),
            "match"     => Some(Keyword::Match),
            "break"     => Some(Keyword::Break),
            "continue"  => Some(Keyword::Continue),
            "defer"     => Some(Keyword::Defer),
            "and"       => Some(Keyword::And),
            "or"        => Some(Keyword::Or),
            "not"       => Some(Keyword::Not),
            "is"        => Some(Keyword::Is),
            "true"      => Some(Keyword::True),
            "false"     => Some(Keyword::False),
            "null"      => Some(Keyword::Null),
            "unsafe"    => Some(Keyword::Unsafe),
            "asm"       => Some(Keyword::Asm),
            "try"       => Some(Keyword::Try),
            "catch"     => Some(Keyword::Catch),
            "finally"   => Some(Keyword::Finally),
            "panic"     => Some(Keyword::Panic),
            "assert"    => Some(Keyword::Assert),
            "select"    => Some(Keyword::Select),
            "after"     => Some(Keyword::After),
            "default"   => Some(Keyword::Default),
            "never"     => Some(Keyword::Never),
            "void"      => Some(Keyword::Void),
            "any"       => Some(Keyword::Any),
            "self"      => Some(Keyword::SelfValue),
            "Self"      => Some(Keyword::SelfType),
            "extern"    => Some(Keyword::Extern),
            "move"      => Some(Keyword::Move),
            _           => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Keyword::Module    => "module",
            Keyword::Import    => "import",
            Keyword::As        => "as",
            Keyword::Use       => "use",
            Keyword::Var       => "var",
            Keyword::Let       => "let",
            Keyword::Const     => "const",
            Keyword::Comptime  => "comptime",
            Keyword::Fn        => "fn",
            Keyword::Return    => "return",
            Keyword::Async     => "async",
            Keyword::Await     => "await",
            Keyword::Yield     => "yield",
            Keyword::Struct    => "struct",
            Keyword::Enum      => "enum",
            Keyword::Union     => "union",
            Keyword::Interface => "interface",
            Keyword::Impl      => "impl",
            Keyword::Embed     => "embed",
            Keyword::Pub       => "pub",
            Keyword::Type      => "type",
            Keyword::Newtype   => "newtype",
            Keyword::If        => "if",
            Keyword::Elif      => "elif",
            Keyword::Else      => "else",
            Keyword::For       => "for",
            Keyword::While     => "while",
            Keyword::Loop      => "loop",
            Keyword::In        => "in",
            Keyword::Step      => "step",
            Keyword::Match     => "match",
            Keyword::Break     => "break",
            Keyword::Continue  => "continue",
            Keyword::Defer     => "defer",
            Keyword::And       => "and",
            Keyword::Or        => "or",
            Keyword::Not       => "not",
            Keyword::Is        => "is",
            Keyword::True      => "true",
            Keyword::False     => "false",
            Keyword::Null      => "null",
            Keyword::Unsafe    => "unsafe",
            Keyword::Asm       => "asm",
            Keyword::Try       => "try",
            Keyword::Catch     => "catch",
            Keyword::Finally   => "finally",
            Keyword::Panic     => "panic",
            Keyword::Assert    => "assert",
            Keyword::Select    => "select",
            Keyword::After     => "after",
            Keyword::Default   => "default",
            Keyword::Never     => "never",
            Keyword::Void      => "void",
            Keyword::Any       => "any",
            Keyword::SelfValue => "self",
            Keyword::SelfType  => "Self",
            Keyword::Extern    => "extern",
            Keyword::Move      => "move",
        }
    }
}

// ── F-string parts ────────────────────────────────────────────────────────────

/// A segment of an f-string, e.g. f"Hello {name:.2f}!"
#[derive(Debug, Clone, PartialEq)]
pub enum FStringPart {
    /// Literal text between interpolations
    Text(String),
    /// An interpolated expression: {expr}
    Expr(Vec<Token>),
    /// An interpolated expression with format spec: {expr:.2f}
    ExprFmt { tokens: Vec<Token>, fmt: String },
}

// ── Token kind ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // ── Literals ──────────────────────────────────────────────────────────────
    /// Integer literal, e.g.  42  0xFF  0b1010  1_000u32
    Int(u128, Option<IntSuffix>),
    /// Float literal, e.g.  3.14  1.5e-3  0.5f32
    Float(f64, Option<FloatSuffix>),
    /// Processed string literal (escapes resolved), e.g. "hello\n"
    Str(String),
    /// Raw string literal (no processing), e.g. r"C:\path"
    RawStr(String),
    /// Interpolated string, e.g. f"Hello {name}!"
    FStr(Vec<FStringPart>),
    /// Character literal, e.g. 'A'  '\n'  '\u{1F600}'
    Char(char),
    /// Byte literal, e.g. b'A'
    Byte(u8),

    // ── Identifiers & keywords ────────────────────────────────────────────────
    Ident(String),
    Kw(Keyword),

    // ── Lifetime ──────────────────────────────────────────────────────────────
    /// e.g.  'a  'static
    Lifetime(String),

    // ── Arithmetic ────────────────────────────────────────────────────────────
    Plus,          // +
    Minus,         // -
    Star,          // *
    Slash,         // /
    Percent,       // %
    StarStar,      // **
    // Overflow variants
    PlusPct,       // +%
    MinusPct,      // -%
    StarPct,       // *%
    PlusBar,       // +|
    MinusBar,      // -|
    StarBar,       // *|
    PlusExcl,      // +!
    MinusExcl,     // -!
    StarExcl,      // *!

    // ── Bitwise ───────────────────────────────────────────────────────────────
    Amp,           // &
    Bar,           // |
    Caret,         // ^
    Tilde,         // ~
    LtLt,          // <<
    GtGt,          // >>

    // ── Comparison ────────────────────────────────────────────────────────────
    Eq,            // =
    EqEq,          // ==
    BangEq,        // !=
    Lt,            // <
    Gt,            // >
    LtEq,          // <=
    GtEq,          // >=

    // ── Assignment ────────────────────────────────────────────────────────────
    PlusEq,        // +=
    MinusEq,       // -=
    StarEq,        // *=
    SlashEq,       // /=
    PercentEq,     // %=
    StarStarEq,    // **=
    AmpEq,         // &=
    BarEq,         // |=
    CaretEq,       // ^=
    LtLtEq,        // <<=
    GtGtEq,        // >>=
    PlusPctEq,     // +%=
    MinusPctEq,    // -%=
    StarPctEq,     // *%=
    PlusBarEq,     // +|=
    MinusBarEq,    // -|=
    StarBarEq,     // *|=
    ColonEq,       // :=

    // ── Other operators ───────────────────────────────────────────────────────
    Arrow,         // ->
    FatArrow,      // =>
    DotDot,        // ..
    DotDotEq,      // ..=
    PipeGt,        // |>
    QQ,            // ??
    QDot,          // ?.
    Question,      // ?
    Bang,          // !

    // ── Punctuation ───────────────────────────────────────────────────────────
    Dot,           // .
    Comma,         // ,
    Colon,         // :
    Semicolon,     // ;
    At,            // @
    Hash,          // #
    HashBang,      // #!
    Underscore,    // _ (wildcard)

    // ── Delimiters ────────────────────────────────────────────────────────────
    LParen,        // (
    RParen,        // )
    LBracket,      // [
    RBracket,      // ]
    LBrace,        // {
    RBrace,        // }

    // ── Comments ─────────────────────────────────────────────────────────────
    LineComment(String),
    DocComment(String),

    // ── Virtual indentation tokens ────────────────────────────────────────────
    Newline,
    Indent,
    Dedent,

    // ── End of file ───────────────────────────────────────────────────────────
    Eof,
}

impl TokenKind {
    /// Human-readable name for use in error messages.
    pub fn display(&self) -> String {
        match self {
            TokenKind::Int(v, None)     => format!("integer `{}`", v),
            TokenKind::Int(v, Some(s))  => format!("integer `{}{}`", v, s.as_str()),
            TokenKind::Float(v, None)   => format!("float `{}`", v),
            TokenKind::Float(v, Some(FloatSuffix::F32)) => format!("float `{}f32`", v),
            TokenKind::Float(v, Some(FloatSuffix::F64)) => format!("float `{}f64`", v),
            TokenKind::Str(s)           => format!("string `\"{}\"`", s),
            TokenKind::RawStr(_)        => "raw string".to_string(),
            TokenKind::FStr(_)          => "f-string".to_string(),
            TokenKind::Char(c)          => format!("char `'{}'`", c),
            TokenKind::Byte(b)          => format!("byte `b'{}'`", *b as char),
            TokenKind::Ident(s)         => format!("`{}`", s),
            TokenKind::Kw(kw)           => format!("`{}`", kw.as_str()),
            TokenKind::Lifetime(s)      => format!("`'{}`", s),
            TokenKind::Plus             => "`+`".into(),
            TokenKind::Minus            => "`-`".into(),
            TokenKind::Star             => "`*`".into(),
            TokenKind::Slash            => "`/`".into(),
            TokenKind::Percent          => "`%`".into(),
            TokenKind::StarStar         => "`**`".into(),
            TokenKind::Amp              => "`&`".into(),
            TokenKind::Bar              => "`|`".into(),
            TokenKind::Caret            => "`^`".into(),
            TokenKind::Tilde            => "`~`".into(),
            TokenKind::LtLt             => "`<<`".into(),
            TokenKind::GtGt             => "`>>`".into(),
            TokenKind::Eq               => "`=`".into(),
            TokenKind::EqEq             => "`==`".into(),
            TokenKind::BangEq           => "`!=`".into(),
            TokenKind::Lt               => "`<`".into(),
            TokenKind::Gt               => "`>`".into(),
            TokenKind::LtEq             => "`<=`".into(),
            TokenKind::GtEq             => "`>=`".into(),
            TokenKind::Arrow            => "`->`".into(),
            TokenKind::FatArrow         => "`=>`".into(),
            TokenKind::DotDot           => "`..`".into(),
            TokenKind::DotDotEq         => "`..=`".into(),
            TokenKind::PipeGt           => "`|>`".into(),
            TokenKind::QQ               => "`??`".into(),
            TokenKind::QDot             => "`?.`".into(),
            TokenKind::Question         => "`?`".into(),
            TokenKind::Bang             => "`!`".into(),
            TokenKind::ColonEq          => "`:=`".into(),
            TokenKind::Dot              => "`.`".into(),
            TokenKind::Comma            => "`,`".into(),
            TokenKind::Colon            => "`:`".into(),
            TokenKind::At               => "`@`".into(),
            TokenKind::Hash             => "`#`".into(),
            TokenKind::Underscore       => "`_`".into(),
            TokenKind::LParen           => "`(`".into(),
            TokenKind::RParen           => "`)`".into(),
            TokenKind::LBracket         => "`[`".into(),
            TokenKind::RBracket         => "`]`".into(),
            TokenKind::LBrace           => "`{`".into(),
            TokenKind::RBrace           => "`}`".into(),
            TokenKind::Newline          => "newline".into(),
            TokenKind::Indent           => "indent".into(),
            TokenKind::Dedent           => "dedent".into(),
            TokenKind::Eof              => "end of file".into(),
            TokenKind::LineComment(_)   => "comment".into(),
            TokenKind::DocComment(_)    => "doc comment".into(),
            _                           => "token".into(),
        }
    }
}

// ── Token ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn is_eof(&self) -> bool {
        self.kind == TokenKind::Eof
    }
}
