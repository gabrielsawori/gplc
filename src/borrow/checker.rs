use std::collections::HashMap;
use crate::ast::nodes::{Expression, Statement};
use crate::borrow::lifetime::{BorrowState, Lifetime};

pub struct BorrowChecker {
    // A stack of environments to track variable lifetimes across scopes
    environments: Vec<HashMap<String, Lifetime>>,
}

impl BorrowChecker {
    pub fn new() -> Self {
        Self {
            environments: vec![HashMap::new()], // Global scope
        }
    }

    pub fn check_program(&mut self, program: &[Statement]) -> Result<(), String> {
        for stmt in program {
            self.check_statement(stmt)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::LetStatement { name, is_mut, value } => {
                // Check the expression first (e.g., if it moves another variable)
                self.check_expression(value)?;
                
                // Track the new variable in the current environment
                let lifetime = Lifetime::new(name.clone(), *is_mut);
                if let Some(env) = self.environments.last_mut() {
                    env.insert(name.clone(), lifetime);
                }
                Ok(())
            }
            Statement::ExpressionStatement(expr) | Statement::ReturnStatement(expr) => {
                self.check_expression(expr)?;
                Ok(())
            }
            Statement::FunctionDeclaration { name: _, return_type: _, body } => {
                // Enter new scope for function body
                self.environments.push(HashMap::new());
                for b_stmt in body {
                    self.check_statement(b_stmt)?;
                }
                // Exit scope, dropping all local lifetimes
                self.environments.pop();
                Ok(())
            }
        }
    }

    fn check_expression(&mut self, expr: &Expression) -> Result<(), String> {
        match expr {
            Expression::Identifier(name) => {
                // A simple read of an identifier. For a strict Move semantics simulation,
                // reading a non-Copy struct would move it. We simulate checking if it's moved here.
                self.verify_access(name)?;
                Ok(())
            }
            Expression::InfixOp(left, _op, right) => {
                self.check_expression(left)?;
                self.check_expression(right)?;
                Ok(())
            }
            Expression::IntLiteral(_) | Expression::StringLiteral(_) => {
                // Primitives do not violate borrow rules on instantiation
                Ok(())
            }
        }
    }

    fn verify_access(&mut self, name: &str) -> Result<(), String> {
        for env in self.environments.iter_mut().rev() {
            if let Some(lifetime) = env.get_mut(name) {
                if lifetime.state == BorrowState::Moved {
                    return Err(format!(
                        "BorrowError [E0200]: use_after_move. Value '{}' used after being moved.",
                        name
                    ));
                }
                return Ok(());
            }
        }
        // If not found, it's either an unresolved name (caught by Phase 4) or primitive.
        Ok(())
    }
}