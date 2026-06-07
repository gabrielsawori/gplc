use std::collections::HashMap;
use crate::ast::nodes::{Expression, Statement};
use crate::ir::gir::*;

pub struct IRBuilder {
    next_reg: usize,
    current_block: BasicBlock,
    blocks: Vec<BasicBlock>,
    // Maps variable names to their allocated memory pointer (Register)
    var_environment: HashMap<String, Register>,
}

impl IRBuilder {
    pub fn new() -> Self {
        Self {
            next_reg: 0,
            current_block: BasicBlock { id: 0, instructions: Vec::new() },
            blocks: Vec::new(),
            var_environment: HashMap::new(),
        }
    }

    fn fresh_reg(&mut self) -> Register {
        let r = Register(self.next_reg);
        self.next_reg += 1;
        r
    }

    pub fn build_function(&mut self, name: &str, body: &[Statement]) -> FunctionIR {
        // Reset states for the new function
        self.blocks.clear();
        self.current_block = BasicBlock { id: 0, instructions: Vec::new() };
        self.var_environment.clear();
        self.next_reg = 0;

        for stmt in body {
            self.build_statement(stmt);
        }

        // Push the final block
        self.blocks.push(self.current_block.clone());

        FunctionIR {
            name: name.to_string(),
            blocks: self.blocks.clone(),
        }
    }

    fn build_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::LetStatement { name, is_mut: _, value } => {
                let val_op = self.build_expression(value);
                
                // 1. Allocate stack memory for the variable
                let ptr_reg = self.fresh_reg();
                self.current_block.instructions.push(Instruction::Alloca {
                    dest: ptr_reg.clone(),
                    ty: "i64".to_string(), // Defaulting to 64-bit integer for MVP
                });

                // 2. Store the evaluated expression into the memory
                self.current_block.instructions.push(Instruction::Store {
                    ptr: ptr_reg.clone(),
                    value: val_op,
                });

                // Track the pointer register
                self.var_environment.insert(name.clone(), ptr_reg);
            }
            Statement::ReturnStatement(expr) => {
                let val_op = self.build_expression(expr);
                self.current_block.instructions.push(Instruction::Return {
                    value: Some(val_op),
                });
            }
            Statement::ExpressionStatement(expr) => {
                self.build_expression(expr);
            }
            Statement::FunctionDeclaration { .. } => {
                // Handled at the module scope level in a full implementation
            }
        }
    }

    fn build_expression(&mut self, expr: &Expression) -> Operand {
        match expr {
            Expression::IntLiteral(val) => Operand::ImmInt(*val),
            Expression::StringLiteral(val) => Operand::ImmString(val.clone()),
            Expression::Identifier(name) => {
                if let Some(ptr_reg) = self.var_environment.get(name) {
                    let load_reg = self.fresh_reg();
                    self.current_block.instructions.push(Instruction::Load {
                        dest: load_reg.clone(),
                        ptr: ptr_reg.clone(),
                    });
                    Operand::Reg(load_reg)
                } else {
                    panic!("Compiler Bug: Unresolved variable '{}' reached IR Builder. Should have been caught by Phase 4.", name);
                }
            }
            Expression::InfixOp(left, op, right) => {
                let left_op = self.build_expression(left);
                let right_op = self.build_expression(right);
                let dest_reg = self.fresh_reg();

                let inst = match op.as_str() {
                    "+" => Instruction::Add { dest: dest_reg.clone(), left: left_op, right: right_op },
                    "-" => Instruction::Sub { dest: dest_reg.clone(), left: left_op, right: right_op },
                    "*" => Instruction::Mul { dest: dest_reg.clone(), left: left_op, right: right_op },
                    "/" => Instruction::Div { dest: dest_reg.clone(), left: left_op, right: right_op },
                    _ => panic!("Compiler Bug: Unsupported operator '{}' in IR builder", op),
                };

                self.current_block.instructions.push(inst);
                Operand::Reg(dest_reg)
            }
        }
    }
}