use crate::ast::nodes::{Expression, Statement};
use crate::lexer::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::EOF)
    }

    fn peek_token(&self) -> &Token {
        self.tokens.get(self.position + 1).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    /// Entry point for the parser
    pub fn parse_program(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while *self.current_token() != Token::EOF {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token() {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            Token::Fn => self.parse_function_declaration(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'let'

        let is_mut = if *self.current_token() == Token::Mut {
            self.advance(); // Skip 'mut'
            true
        } else {
            false
        };

        // Expect Identifier
        let name = match self.current_token() {
            Token::Ident(ident) => ident.clone(),
            _ => return Err(format!("SyntaxError: Expected identifier after 'let', found {:?}", self.current_token())),
        };
        self.advance();

        // Expect '='
        if *self.current_token() != Token::Assign {
            return Err(format!("SyntaxError: Expected '=' after identifier '{}'", name));
        }
        self.advance();

        // Parse Expression
        let value = self.parse_expression()?;

        Ok(Statement::LetStatement { name, is_mut, value })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'return'
        
        let value = self.parse_expression()?;
        
        Ok(Statement::ReturnStatement(value))
    }

    fn parse_function_declaration(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'fn'

        let name = match self.current_token() {
            Token::Ident(ident) => ident.clone(),
            _ => return Err("SyntaxError: Expected function name after 'fn'".to_string()),
        };
        self.advance();

        // Expect '(' and ')' for now (no params implementation in this minimal pass)
        if *self.current_token() != Token::LParen { return Err("SyntaxError: Expected '('".to_string()); }
        self.advance();
        if *self.current_token() != Token::RParen { return Err("SyntaxError: Expected ')'".to_string()); }
        self.advance();

        // Parse return type (e.g., -> i32)
        let mut return_type = "void".to_string();
        if *self.current_token() == Token::Arrow {
            self.advance();
            if let Token::Ident(ty) = self.current_token() {
                return_type = ty.clone();
                self.advance();
            } else {
                return Err("SyntaxError: Expected type after '->'".to_string());
            }
        }

        // Expect '{'
        if *self.current_token() != Token::LBrace { return Err("SyntaxError: Expected '{' before function body".to_string()); }
        self.advance();

        let mut body = Vec::new();
        while *self.current_token() != Token::RBrace && *self.current_token() != Token::EOF {
            body.push(self.parse_statement()?);
        }

        // Expect '}'
        if *self.current_token() != Token::RBrace { return Err("SyntaxError: Expected '}' after function body".to_string()); }
        self.advance();

        Ok(Statement::FunctionDeclaration { name, return_type, body })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, String> {
        let expr = self.parse_expression()?;
        Ok(Statement::ExpressionStatement(expr))
    }

    /// Basic expression parser
    fn parse_expression(&mut self) -> Result<Expression, String> {
        // Very simplistic parser targeting our defined tokens
        let left = match self.current_token() {
            Token::Int(val) => {
                let expr = Expression::IntLiteral(*val);
                self.advance();
                expr
            }
            Token::String(val) => {
                let expr = Expression::StringLiteral(val.clone());
                self.advance();
                expr
            }
            Token::Ident(val) => {
                let expr = Expression::Identifier(val.clone());
                self.advance();
                expr
            }
            _ => return Err(format!("SyntaxError: Unexpected token in expression: {:?}", self.current_token())),
        };

        // Check for basic infix math operations (+, -, *, /)
        match self.current_token() {
            Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => {
                let op = match self.current_token() {
                    Token::Plus => "+", Token::Minus => "-", Token::Asterisk => "*", Token::Slash => "/", _ => unreachable!()
                }.to_string();
                self.advance();
                
                let right = self.parse_expression()?; // Recursively parse the right side
                return Ok(Expression::InfixOp(Box::new(left), op, Box::new(right)));
            }
            _ => {} // Not an infix operation
        }

        Ok(left)
    }
}