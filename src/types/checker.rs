use std::collections::HashMap;
use crate::ast::nodes::{Expression, Statement};
use crate::types::ty::Type;

pub struct TypeChecker {
    // Simulates the environment tracking variable types.
    // In a full pass, this integrates with the Symbol Table from the Name Resolution phase.
    environment: HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
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
            Statement::LetStatement { name, is_mut: _, value } => {
                // Infer type from the right-hand side expression
                let val_ty = self.check_expression(value)?;
                self.environment.insert(name.clone(), val_ty);
                Ok(())
            }
            Statement::ReturnStatement(expr) => {
                self.check_expression(expr)?;
                // Future enhancement: Verify it matches the enclosing function's return type
                Ok(())
            }
            Statement::FunctionDeclaration { name, return_type, body } => {
                // Simple string-to-type mapping for the prototype
                let ret_ty = if return_type == "i32" { Type::I32 } else { Type::Void };
                self.environment.insert(name.clone(), Type::Function {
                    params: vec![],
                    return_type: Box::new(ret_ty),
                });

                for b_stmt in body {
                    self.check_statement(b_stmt)?;
                }
                Ok(())
            }
            Statement::ExpressionStatement(expr) => {
                self.check_expression(expr)?;
                Ok(())
            }
        }
    }

    fn check_expression(&mut self, expr: &Expression) -> Result<Type, String> {
        match expr {
            Expression::IntLiteral(_) => Ok(Type::I64), // Default to i64 for inference
            Expression::StringLiteral(_) => Ok(Type::Str),
            Expression::Identifier(name) => {
                self.environment.get(name).cloned().ok_or_else(|| {
                    format!("TypeError [E0102]: Undefined type or variable '{}'.", name)
                })
            }
            Expression::InfixOp(left, op, right) => {
                let left_ty = self.check_expression(left)?;
                let right_ty = self.check_expression(right)?;

                // For strict typing, operations normally require identical types
                if left_ty != right_ty {
                    return Err(format!(
                        "TypeError [E0100]: Type mismatch. Cannot apply operator '{}' to '{}' and '{}'.",
                        op, left_ty.to_string(), right_ty.to_string()
                    ));
                }

                Ok(left_ty)
            }
        }
    }
}