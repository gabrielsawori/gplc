use std::collections::HashMap;
use crate::ast::nodes::{Expression, Statement};
use crate::types::ty::Type;

pub struct TypeChecker {
    environment: HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
        }
    }

    pub fn check_program(&mut self, program: &[Statement]) -> Result<(), String> {
        // Pre-declare all functions so they can call each other / themselves
        for stmt in program {
            if let Statement::FunctionDeclaration { name, return_type, params, .. } = stmt {
                let ret_ty = if return_type == "i32" { Type::I64 } else { Type::Void }; // Use i64 everywhere for MVP
                self.environment.insert(name.clone(), Type::Function {
                    params: params.iter().map(|_| Type::I64).collect(),
                    return_type: Box::new(ret_ty),
                });
            }
        }

        for stmt in program {
            self.check_statement(stmt)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::LetStatement { name, is_mut: _, value } => {
                let val_ty = self.check_expression(value)?;
                self.environment.insert(name.clone(), val_ty);
                Ok(())
            }
            Statement::AssignStatement { name: _, value } => {
                self.check_expression(value)?;
                Ok(())
            }
            Statement::ReturnStatement(expr) => {
                self.check_expression(expr)?;
                Ok(())
            }
            Statement::FunctionDeclaration { name: _, return_type: _, body, params } => {
                let backup_env = self.environment.clone();

                for param in params {
                    self.environment.insert(param.clone(), Type::I64); // Assume all params are i64 for now
                }

                for b_stmt in body {
                    self.check_statement(b_stmt)?;
                }

                self.environment = backup_env;
                Ok(())
            }
            Statement::ExpressionStatement(expr) => {
                self.check_expression(expr)?;
                Ok(())
            }
            Statement::IfStatement { condition, body, else_body } => {
                let _cond_ty = self.check_expression(condition)?;
                for b_stmt in body {
                    self.check_statement(b_stmt)?;
                }
                if let Some(e_body) = else_body {
                    for b_stmt in e_body {
                        self.check_statement(b_stmt)?;
                    }
                }
                Ok(())
            }
            Statement::WhileStatement { condition, body } => {
                let _cond_ty = self.check_expression(condition)?;
                for b_stmt in body {
                    self.check_statement(b_stmt)?;
                }
                Ok(())
            }
            Statement::ModuleDecl(_) | Statement::ImportDecl(_) => Ok(()),
        }
    }

    fn check_expression(&mut self, expr: &Expression) -> Result<Type, String> {
        match expr {
            Expression::IntLiteral(_) => Ok(Type::I64),
            Expression::StringLiteral(_) => Ok(Type::Str),
            Expression::BoolLiteral(_) => Ok(Type::Bool),
            Expression::Identifier(name) => {
                self.environment.get(name).cloned().ok_or_else(|| {
                    format!("TypeError [E0102]: Undefined type or variable '{}'.", name)
                })
            }
            Expression::InfixOp(left, _op, right) => {
                let _left_ty = self.check_expression(left)?;
                let _right_ty = self.check_expression(right)?;

                // Just return I64 for MVP, avoiding strict type check issues for Call
                Ok(Type::I64)
            }
            Expression::Call(func, args) => {
                let func_ty = self.check_expression(func)?;
                match func_ty {
                    Type::Function { params, return_type } => {
                        if args.len() != params.len() {
                            return Err(format!("TypeError: Expected {} arguments, found {}", params.len(), args.len()));
                        }
                        for (arg, _param_ty) in args.iter().zip(params.iter()) {
                            let _arg_ty = self.check_expression(arg)?;
                        }
                        Ok(*return_type)
                    }
                    _ => Err(format!("TypeError: Cannot call non-function type {:?}", func_ty)),
                }
            }
        }
    }
}
