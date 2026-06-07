use crate::lexer::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,      // Current position in input (points to current char)
    read_position: usize, // Current reading position (after current char)
    ch: char,             // Current char under examination
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'; // EOF
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => Token::Assign,
            '+' => Token::Plus,
            '-' => {
                if self.peek_char() == '>' {
                    self.read_char();
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ',' => Token::Comma,
            ':' => Token::Colon,
            '"' => {
                let string_literal = self.read_string();
                Token::String(string_literal)
            }
            '\0' => Token::EOF,
            _ => {
                if self.is_letter(self.ch) {
                    let ident = self.read_identifier();
                    return Token::lookup_ident(&ident);
                } else if self.ch.is_ascii_digit() {
                    let num = self.read_number();
                    return Token::Int(num);
                } else {
                    Token::Illegal(self.ch)
                }
            }
        };

        self.read_char();
        tok
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if tok == Token::EOF {
                tokens.push(tok);
                break;
            }
            tokens.push(tok);
        }
        tokens
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        while self.is_letter(self.ch) || self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[start_pos..self.position].iter().collect()
    }

    fn read_number(&mut self) -> i64 {
        let start_pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        let num_str: String = self.input[start_pos..self.position].iter().collect();
        num_str.parse().unwrap_or(0)
    }

    fn read_string(&mut self) -> String {
        self.read_char(); // Skip opening quote
        let start_pos = self.position;
        while self.ch != '"' && self.ch != '\0' {
            self.read_char();
        }
        let str_val: String = self.input[start_pos..self.position].iter().collect();
        self.read_char(); // Skip closing quote
        str_val
    }

    fn is_letter(&self, ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}