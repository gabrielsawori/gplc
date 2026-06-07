use crate::ast::nodes::{Expression, Statement};
use crate::resolve::scope::ScopeStack;

pub struct Resolver {
    scope_stack: ScopeStack,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            scope_stack: ScopeStack::new(),
        }
    }

    pub fn resolve_program(&mut self, program: &[Statement]) -> Result<(), String> {
        for stmt in program {
            self.resolve_statement(stmt)?;
        }
        Ok(())
    }

    fn resolve_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::LetStatement { name, is_mut, value } => {
                // Rule: Resolve the right-hand side expression first
                self.resolve_expression(value)?;
                // Then define the variable in the current scope
                self.scope_stack.define(name.clone(), *is_mut)?;
                Ok(())
            }
            Statement::AssignStatement { name, value } => {
                self.resolve_expression(value)?;
                if self.scope_stack.lookup(name).is_none() {
                    return Err(format!("ReferenceError: Undefined variable '{}'.", name));
                }
                Ok(())
            }
            Statement::ReturnStatement(expr) => {
                self.resolve_expression(expr)?;
                Ok(())
            }
            Statement::FunctionDeclaration { name, return_type: _, body, params } => {
                // 1. Define the function in the current (outer) scope
                self.scope_stack.define(name.clone(), false)?;
                
                // 2. Enter a new scope for the function body
                self.scope_stack.enter_scope();
                for param in params {
                    self.scope_stack.define(param.clone(), false)?;
                }
                for b_stmt in body {
                    self.resolve_statement(b_stmt)?;
                }
                self.scope_stack.exit_scope();
                
                Ok(())
            }
            Statement::ExpressionStatement(expr) => {
                self.resolve_expression(expr)?;
                Ok(())
            }
            Statement::IfStatement { condition, body, else_body } => {
                self.resolve_expression(condition)?;
                self.scope_stack.enter_scope();
                for b_stmt in body {
                    self.resolve_statement(b_stmt)?;
                }
                self.scope_stack.exit_scope();

                if let Some(e_body) = else_body {
                    self.scope_stack.enter_scope();
                    for b_stmt in e_body {
                        self.resolve_statement(b_stmt)?;
                    }
                    self.scope_stack.exit_scope();
                }
                Ok(())
            }
            Statement::WhileStatement { condition, body } => {
                self.resolve_expression(condition)?;
                self.scope_stack.enter_scope();
                for b_stmt in body {
                    self.resolve_statement(b_stmt)?;
                }
                self.scope_stack.exit_scope();
                Ok(())
            }
            Statement::ModuleDecl(_) | Statement::ImportDecl(_) => Ok(()),
        }
    }

    fn resolve_expression(&mut self, expr: &Expression) -> Result<(), String> {
        match expr {
            Expression::Identifier(name) => {
                if self.scope_stack.lookup(name).is_none() {
                    // E0300: Undefined variable
                    return Err(format!("ReferenceError: Undefined variable '{}'.", name));
                }
                Ok(())
            }
            Expression::InfixOp(left, _op, right) => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
                Ok(())
            }
            Expression::Call(func, args) => {
                self.resolve_expression(func)?;
                for arg in args {
                    self.resolve_expression(arg)?;
                }
                Ok(())
            }
            Expression::IntLiteral(_) | Expression::StringLiteral(_) | Expression::BoolLiteral(_) => {
                // Primitives do not need name resolution
                Ok(())
            }
        }
    }
}
