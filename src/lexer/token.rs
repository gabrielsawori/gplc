#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Module,
    Import,
    Fn,
    Let,
    Mut,
    Return,
    If,
    Else,
    While,

    // Identifiers and Literals
    Ident(String),
    Int(i64),
    String(String),
    Bool(bool),

    // Operators
    Assign,      // =
    Plus,        // +
    Minus,       // -
    Asterisk,    // *
    Slash,       // /
    Lt,          // <
    Gt,          // >
    LtEq,        // <=
    GtEq,        // >=
    Eq,          // ==
    NotEq,       // !=

    // Delimiters
    LParen,      // (
    RParen,      // )
    LBrace,      // {  (Keep for now in case of dicts/sets later)
    RBrace,      // }
    Comma,       // ,
    Colon,       // :
    Arrow,       // ->

    // Indentation
    Indent,
    Dedent,
    Newline,

    // Control
    EOF,
    Illegal(char),
}

impl Token {
    pub fn lookup_ident(ident: &str) -> Token {
        match ident {
            "module" => Token::Module,
            "import" => Token::Import,
            "fn" => Token::Fn,
            "let" => Token::Let,
            "mut" => Token::Mut,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            _ => Token::Ident(ident.to_string()),
        }
    }
}
