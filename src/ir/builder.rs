use std::collections::HashMap;
use crate::ast::nodes::{Expression, Statement};
use crate::ir::gir::*;

pub struct IRBuilder {
    next_reg: usize,
    next_block_id: usize,
    current_block: BasicBlock,
    blocks: Vec<BasicBlock>,
    // Maps variable names to their allocated memory pointer (Register)
    var_environment: HashMap<String, Register>,
}

impl IRBuilder {
    pub fn new() -> Self {
        Self {
            next_reg: 0,
            next_block_id: 1,
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

    fn new_block(&mut self) -> BasicBlock {
        let b = BasicBlock { id: self.next_block_id, instructions: Vec::new() };
        self.next_block_id += 1;
        b
    }

    pub fn build_function(&mut self, name: &str, body: &[Statement], params: &[String]) -> FunctionIR {
        // Reset states for the new function
        self.blocks.clear();
        self.next_block_id = 1;
        self.current_block = BasicBlock { id: 0, instructions: Vec::new() };
        self.var_environment.clear();
        self.next_reg = 0;

        // Create allocas for parameters
        for param in params {
            let ptr_reg = self.fresh_reg();
            self.current_block.instructions.push(Instruction::Alloca {
                dest: ptr_reg.clone(),
                ty: "i64".to_string(), // Defaulting to 64-bit integer for MVP
            });
            self.var_environment.insert(param.clone(), ptr_reg);
        }

        for stmt in body {
            self.build_statement(stmt);
        }

        // Push the final block
        self.blocks.push(self.current_block.clone());

        FunctionIR {
            name: name.to_string(),
            params: params.len(),
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
            Statement::AssignStatement { name, value } => {
                let val_op = self.build_expression(value);
                let ptr_reg = self.var_environment.get(name).unwrap().clone();
                self.current_block.instructions.push(Instruction::Store {
                    ptr: ptr_reg,
                    value: val_op,
                });
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
            Statement::IfStatement { condition, body, else_body } => {
                let cond_op = self.build_expression(condition);

                let true_block = self.new_block();
                let false_block = self.new_block();
                let merge_block = self.new_block();

                self.current_block.instructions.push(Instruction::Br {
                    cond: cond_op,
                    if_true: true_block.id,
                    if_false: false_block.id,
                });

                self.blocks.push(self.current_block.clone());

                self.current_block = true_block;

                for b_stmt in body {
                    self.build_statement(b_stmt);
                }

                self.current_block.instructions.push(Instruction::Jmp { dest: merge_block.id });
                self.blocks.push(self.current_block.clone());

                self.current_block = false_block;
                if let Some(e_body) = else_body {
                    for b_stmt in e_body {
                        self.build_statement(b_stmt);
                    }
                }
                self.current_block.instructions.push(Instruction::Jmp { dest: merge_block.id });
                self.blocks.push(self.current_block.clone());

                self.current_block = merge_block;
            }
            Statement::WhileStatement { condition, body } => {
                let cond_block = self.new_block();
                let body_block = self.new_block();
                let merge_block = self.new_block();

                self.current_block.instructions.push(Instruction::Jmp { dest: cond_block.id });
                self.blocks.push(self.current_block.clone());

                self.current_block = cond_block.clone();
                let cond_op = self.build_expression(condition);
                self.current_block.instructions.push(Instruction::Br {
                    cond: cond_op,
                    if_true: body_block.id,
                    if_false: merge_block.id,
                });
                self.blocks.push(self.current_block.clone());

                self.current_block = body_block;
                for b_stmt in body {
                    self.build_statement(b_stmt);
                }

                let len = self.current_block.instructions.len();
                if len > 0 {
                   match self.current_block.instructions[len - 1] {
                       Instruction::Return { .. } => {},
                       _ => self.current_block.instructions.push(Instruction::Jmp { dest: cond_block.id }),
                   }
                } else {
                   self.current_block.instructions.push(Instruction::Jmp { dest: cond_block.id });
                }

                self.blocks.push(self.current_block.clone());

                self.current_block = merge_block;
            }
        }
    }

    fn build_expression(&mut self, expr: &Expression) -> Operand {
        match expr {
            Expression::IntLiteral(val) => Operand::ImmInt(*val),
            Expression::StringLiteral(val) => Operand::ImmString(val.clone()),
            Expression::BoolLiteral(val) => Operand::ImmInt(if *val { 1 } else { 0 }),
            Expression::Identifier(name) => {
                let ptr_reg = self.var_environment.get(name).cloned();
                if let Some(ptr_reg) = ptr_reg {
                    let load_reg = self.fresh_reg();
                    self.current_block.instructions.push(Instruction::Load {
                        dest: load_reg.clone(),
                        ptr: ptr_reg,
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
                    "<" => Instruction::CmpLt { dest: dest_reg.clone(), left: left_op, right: right_op },
                    ">" => Instruction::CmpGt { dest: dest_reg.clone(), left: left_op, right: right_op },
                    "<=" => Instruction::CmpLtEq { dest: dest_reg.clone(), left: left_op, right: right_op },
                    ">=" => Instruction::CmpGtEq { dest: dest_reg.clone(), left: left_op, right: right_op },
                    "==" => Instruction::CmpEq { dest: dest_reg.clone(), left: left_op, right: right_op },
                    "!=" => Instruction::CmpNotEq { dest: dest_reg.clone(), left: left_op, right: right_op },
                    _ => panic!("Compiler Bug: Unsupported operator '{}' in IR builder", op),
                };

                self.current_block.instructions.push(inst);
                Operand::Reg(dest_reg)
            }
            Expression::Call(func, args) => {
                let func_name = match &**func {
                    Expression::Identifier(name) => name.clone(),
                    _ => panic!("Compiler Bug: Expected identifier for function call"),
                };
                let mut arg_ops = Vec::new();
                for arg in args {
                    arg_ops.push(self.build_expression(arg));
                }
                let dest_reg = self.fresh_reg();
                self.current_block.instructions.push(Instruction::Call {
                    dest: Some(dest_reg.clone()),
                    func: func_name,
                    args: arg_ops,
                });
                Operand::Reg(dest_reg)
            }
        }
    }
}
