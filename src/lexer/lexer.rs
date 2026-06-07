use crate::lexer::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
    indent_stack: Vec<usize>,
    pending_dedents: usize,
    at_line_start: bool,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
            indent_stack: vec![0],
            pending_dedents: 0,
            at_line_start: true,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
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

    fn handle_indent(&mut self) -> Token {
        let mut count = 0;
        while self.ch == ' ' {
            count += 1;
            self.read_char();
        }

        if self.ch == '\t' {
            return Token::Illegal('\t'); // E0004: Tabs not allowed
        }

        if self.ch == '\n' || self.ch == '\0' || self.ch == '#' {
            // Blank line or comment-only line, ignore indentation
            return self.next_token();
        }

        let top = *self.indent_stack.last().unwrap();

        if count > top {
            self.indent_stack.push(count);
            return Token::Indent;
        } else if count == top {
            // No indent/dedent, but we just started a line with code, we don't emit Newline here, we already emitted Newline previously.
            // Actually, if we emit Newline at \n, we just return the first token of the line here.
            return self.next_token();
        } else {
            let mut dedents = 0;
            while let Some(&t) = self.indent_stack.last() {
                if t > count {
                    self.indent_stack.pop();
                    dedents += 1;
                } else if t == count {
                    break;
                } else {
                    return Token::Illegal(' '); // E0003: Indent not matching any outer level
                }
            }
            if dedents > 0 {
                self.pending_dedents = dedents - 1;
                return Token::Dedent;
            } else {
                return self.next_token();
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        if self.pending_dedents > 0 {
            self.pending_dedents -= 1;
            return Token::Dedent;
        }

        if self.at_line_start {
            self.at_line_start = false;
            let tok = self.handle_indent();
            // If handle_indent returns something other than EOF or Illegal, and it's not a recursively called next_token (wait, handle_indent calls next_token, so it returns the actual token).
            // Wait, if it returns Indent or Dedent, that's fine.
            // Let's restructure to ensure we don't skip tokens.
            if tok != Token::EOF || self.ch == '\0' {
                // If it returned a token, we must evaluate if we should just return it.
                // Wait, handle_indent handles the spaces and then returns Indent, Dedent, or calls next_token().
                // If it returns a token, we just return it!
                return tok;
            }
        }

        self.skip_whitespace_except_newline();

        let tok = match self.ch {
            '\n' => {
                self.at_line_start = true;
                Token::Newline
            }
            '#' => {
                self.skip_comment();
                // After comment, we might be at newline or EOF
                return self.next_token();
            }
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Illegal(self.ch)
                }
            }
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
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::LtEq
                } else {
                    Token::Lt
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::GtEq
                } else {
                    Token::Gt
                }
            }
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
            '\0' => {
                if self.indent_stack.len() > 1 {
                    self.indent_stack.pop();
                    self.pending_dedents = self.indent_stack.len() - 1;
                    return Token::Dedent;
                }
                Token::EOF
            },
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

        // Remove trailing Newlines before EOF
        while let Some(Token::Newline) = tokens.last() {
            tokens.pop();
        }

        tokens.push(Token::EOF);
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

    fn skip_whitespace_except_newline(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn skip_comment(&mut self) {
        while self.ch != '\n' && self.ch != '\0' {
            self.read_char();
        }
    }
}
