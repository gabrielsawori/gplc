use std::fs;

// Importing the phases we have built so far
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::resolve::resolver::Resolver;
use crate::types::checker::TypeChecker;
use crate::borrow::checker::BorrowChecker;
use crate::ast::nodes::Statement;

pub struct Driver;

impl Driver {
    /// Runs the frontend pipeline of the G-Lang compiler (Phases 1 to 6).
    /// Returns the validated Typed-AST if successful, or an error message.
    pub fn run_frontend(file_path: &str) -> Result<Vec<Statement>, String> {
        // Read the source file
        let source_code = fs::read_to_string(file_path).map_err(|err| {
            format!("IOError: Failed to read file '{}': {}", file_path, err)
        })?;

        // Phase 1: Lexical Analysis
        // Note: Assumes Lexer is implemented to take a string slice and return tokens
        let mut lexer = Lexer::new(&source_code);
        let tokens = lexer.tokenize();

        // Phase 2 & 3: Syntax Parsing and AST Construction
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program()?;

        // Phase 4: Name Resolution (Scope Checking)
        let mut resolver = Resolver::new();
        resolver.resolve_program(&ast)?;

        // Phase 5: Semantic Analysis (Strict Static Typing)
        let mut type_checker = TypeChecker::new();
        type_checker.check_program(&ast)?;

        // Phase 6: Borrow Checking (Memory Safety rules)
        let mut borrow_checker = BorrowChecker::new();
        borrow_checker.check_program(&ast)?;

        Ok(ast)
    }
}