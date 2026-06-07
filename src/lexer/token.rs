#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Fn,
    Let,
    Mut,
    Return,

    // Identifiers and Literals
    Ident(String),
    Int(i64),
    String(String),

    // Operators
    Assign,      // =
    Plus,        // +
    Minus,       // -
    Asterisk,    // *
    Slash,       // /

    // Delimiters
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    Comma,       // ,
    Colon,       // :
    Arrow,       // ->

    // Control
    EOF,
    Illegal(char),
}

impl Token {
    pub fn lookup_ident(ident: &str) -> Token {
        match ident {
            "fn" => Token::Fn,
            "let" => Token::Let,
            "mut" => Token::Mut,
            "return" => Token::Return,
            _ => Token::Ident(ident.to_string()),
        }
    }
}