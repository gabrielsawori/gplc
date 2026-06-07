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
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            Token::Ident(_) => {
                if *self.peek_token() == Token::Assign {
                    self.parse_assign_statement()
                } else {
                    self.parse_expression_statement()
                }
            },
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_assign_statement(&mut self) -> Result<Statement, String> {
        let name = match self.current_token() {
            Token::Ident(ident) => ident.clone(),
            _ => return Err(format!("SyntaxError: Expected identifier, found {:?}", self.current_token())),
        };
        self.advance();

        if *self.current_token() != Token::Assign {
            return Err(format!("SyntaxError: Expected '=' after identifier '{}'", name));
        }
        self.advance();

        let value = self.parse_expression()?;

        Ok(Statement::AssignStatement { name, value })
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

        if *self.current_token() != Token::LParen { return Err("SyntaxError: Expected '('".to_string()); }
        self.advance();

        let mut params = Vec::new();
        while *self.current_token() != Token::RParen && *self.current_token() != Token::EOF {
            if let Token::Ident(param_name) = self.current_token() {
                params.push(param_name.clone());
                self.advance();
            } else {
                return Err("SyntaxError: Expected identifier in parameter list".to_string());
            }
            if *self.current_token() == Token::Comma {
                self.advance();
            }
        }

        if *self.current_token() != Token::RParen { return Err("SyntaxError: Expected ')'".to_string()); }
        self.advance();

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

        if *self.current_token() != Token::LBrace { return Err("SyntaxError: Expected '{' before function body".to_string()); }
        self.advance();

        let mut body = Vec::new();
        while *self.current_token() != Token::RBrace && *self.current_token() != Token::EOF {
            body.push(self.parse_statement()?);
        }

        if *self.current_token() != Token::RBrace { return Err("SyntaxError: Expected '}' after function body".to_string()); }
        self.advance();

        Ok(Statement::FunctionDeclaration { name, params, return_type, body })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'if'

        let condition = self.parse_expression()?;

        if *self.current_token() != Token::LBrace { return Err("SyntaxError: Expected '{' after if condition".to_string()); }
        self.advance();

        let mut body = Vec::new();
        while *self.current_token() != Token::RBrace && *self.current_token() != Token::EOF {
            body.push(self.parse_statement()?);
        }

        if *self.current_token() != Token::RBrace { return Err("SyntaxError: Expected '}' after if body".to_string()); }
        self.advance();

        let mut else_body = None;
        if *self.current_token() == Token::Else {
            self.advance();
            if *self.current_token() != Token::LBrace { return Err("SyntaxError: Expected '{' after else".to_string()); }
            self.advance();
            let mut e_body = Vec::new();
            while *self.current_token() != Token::RBrace && *self.current_token() != Token::EOF {
                e_body.push(self.parse_statement()?);
            }
            if *self.current_token() != Token::RBrace { return Err("SyntaxError: Expected '}' after else body".to_string()); }
            self.advance();
            else_body = Some(e_body);
        }

        Ok(Statement::IfStatement { condition, body, else_body })
    }

    fn parse_while_statement(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'while'

        let condition = self.parse_expression()?;

        if *self.current_token() != Token::LBrace { return Err("SyntaxError: Expected '{' after while condition".to_string()); }
        self.advance();

        let mut body = Vec::new();
        while *self.current_token() != Token::RBrace && *self.current_token() != Token::EOF {
            body.push(self.parse_statement()?);
        }

        if *self.current_token() != Token::RBrace { return Err("SyntaxError: Expected '}' after while body".to_string()); }
        self.advance();

        Ok(Statement::WhileStatement { condition, body })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, String> {
        let expr = self.parse_expression()?;
        Ok(Statement::ExpressionStatement(expr))
    }

    // A simple precedence parser
    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_expression_precedence(0)
    }

    fn get_precedence(token: &Token) -> u8 {
        match token {
            Token::Eq | Token::NotEq => 1,
            Token::Lt | Token::Gt | Token::LtEq | Token::GtEq => 2,
            Token::Plus | Token::Minus => 3,
            Token::Asterisk | Token::Slash => 4,
            _ => 0,
        }
    }

    fn parse_expression_precedence(&mut self, precedence: u8) -> Result<Expression, String> {
        let mut left = self.parse_expression_primary()?;

        loop {
            let next_prec = Self::get_precedence(self.current_token());
            if next_prec == 0 || next_prec <= precedence {
                break;
            }

            let op = match self.current_token() {
                Token::Plus => "+", Token::Minus => "-", Token::Asterisk => "*", Token::Slash => "/",
                Token::Lt => "<", Token::Gt => ">", Token::LtEq => "<=", Token::GtEq => ">=",
                Token::Eq => "==", Token::NotEq => "!=",
                _ => unreachable!()
            }.to_string();
            self.advance();

            let right = self.parse_expression_precedence(next_prec)?;
            left = Expression::InfixOp(Box::new(left), op, Box::new(right));
        }

        Ok(left)
    }

    fn parse_expression_primary(&mut self) -> Result<Expression, String> {
         let mut primary = match self.current_token() {
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
            Token::Bool(val) => {
                let expr = Expression::BoolLiteral(*val);
                self.advance();
                expr
            }
            Token::Ident(val) => {
                let expr = Expression::Identifier(val.clone());
                self.advance();
                expr
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                if *self.current_token() != Token::RParen {
                    return Err("SyntaxError: Expected ')'".to_string());
                }
                self.advance();
                expr
            }
            _ => return Err(format!("SyntaxError: Unexpected token in expression primary: {:?}", self.current_token())),
        };

        loop {
            match self.current_token() {
                Token::LParen => {
                    self.advance();
                    let mut args = Vec::new();
                    while *self.current_token() != Token::RParen && *self.current_token() != Token::EOF {
                        args.push(self.parse_expression()?);
                        if *self.current_token() == Token::Comma {
                            self.advance();
                        }
                    }
                    if *self.current_token() != Token::RParen {
                        return Err("SyntaxError: Expected ')' in function call".to_string());
                    }
                    self.advance();
                    primary = Expression::Call(Box::new(primary), args);
                }
                _ => break,
            }
        }

        Ok(primary)
    }
}
