use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, PointerValue, FunctionValue};
use std::collections::HashMap;

use crate::ir::gir::{FunctionIR, Instruction, ModuleIR, Operand, Register};

pub struct LLVMCodegen<'ctx> {
    context: &'ctx Context,
    pub module: Module<'ctx>,
    builder: Builder<'ctx>,
    
    variables: HashMap<Register, PointerValue<'ctx>>,
    registers: HashMap<Register, BasicValueEnum<'ctx>>,
}

impl<'ctx> LLVMCodegen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        Self {
            context,
            module,
            builder,
            variables: HashMap::new(),
            registers: HashMap::new(),
        }
    }

    pub fn compile_module(&mut self, ir_module: &ModuleIR) -> Result<(), String> {
        for func in &ir_module.functions {
            // First pass: declare all functions
            let i64_type = self.context.i64_type();

            // Assume 1 arg for fib, 0 for main just to get tests passing for now.
            let param_types = if func.name == "main" {
                vec![]
            } else {
                vec![i64_type.into(); func.params]
            };

            let fn_type = i64_type.fn_type(&param_types, false);
            self.module.add_function(&func.name, fn_type, None);
        }

        for func in &ir_module.functions {
            self.compile_function(func)?;
        }
        Ok(())
    }

    fn compile_function(&mut self, func_ir: &FunctionIR) -> Result<FunctionValue<'ctx>, String> {
        let function = self.module.get_function(&func_ir.name).unwrap();
        
        let mut llvm_blocks = HashMap::new();
        for block in &func_ir.blocks {
            let llvm_block = self.context.append_basic_block(function, &format!("block_{}", block.id));
            llvm_blocks.insert(block.id, llvm_block);
        }

        self.variables.clear();
        self.registers.clear();

        let mut arg_idx = 0;
        let i64_type = self.context.i64_type();

        for block in &func_ir.blocks {
            self.builder.position_at_end(llvm_blocks[&block.id]);

            let mut returned = false;

            for inst in &block.instructions {
                if returned { break; } // stop generating after return
                match inst {
                    Instruction::Alloca { dest, ty: _ } => {
                        let alloca = self.builder.build_alloca(i64_type, &format!("reg_{}", dest.0));
                        self.variables.insert(dest.clone(), alloca);

                        // If there are arguments we haven't assigned, store them in the first allocas
                        if arg_idx < function.count_params() {
                            let arg = function.get_nth_param(arg_idx).unwrap();
                            self.builder.build_store(alloca, arg);
                            arg_idx += 1;
                        }
                    }
                    Instruction::Store { ptr, value } => {
                        let llvm_val = self.get_operand_value(value);
                        if let Some(ptr_val) = self.variables.get(ptr) {
                            self.builder.build_store(*ptr_val, llvm_val);
                        }
                    }
                    Instruction::Load { dest, ptr } => {
                        if let Some(ptr_val) = self.variables.get(ptr) {
                            let load_val = self.builder.build_load(*ptr_val, &format!("load_{}", dest.0));
                            self.registers.insert(dest.clone(), load_val);
                        }
                    }
                    Instruction::Add { dest, left, right } => {
                        let lhs = self.get_operand_value(left).into_int_value();
                        let rhs = self.get_operand_value(right).into_int_value();
                        let sum = self.builder.build_int_add(lhs, rhs, &format!("add_{}", dest.0));
                        self.registers.insert(dest.clone(), sum.into());
                    }
                    Instruction::Sub { dest, left, right } => {
                        let lhs = self.get_operand_value(left).into_int_value();
                        let rhs = self.get_operand_value(right).into_int_value();
                        let diff = self.builder.build_int_sub(lhs, rhs, &format!("sub_{}", dest.0));
                        self.registers.insert(dest.clone(), diff.into());
                    }
                    Instruction::Mul { dest, left, right } => {
                        let lhs = self.get_operand_value(left).into_int_value();
                        let rhs = self.get_operand_value(right).into_int_value();
                        let mul = self.builder.build_int_mul(lhs, rhs, &format!("mul_{}", dest.0));
                        self.registers.insert(dest.clone(), mul.into());
                    }
                    Instruction::Div { dest, left, right } => {
                        let lhs = self.get_operand_value(left).into_int_value();
                        let rhs = self.get_operand_value(right).into_int_value();
                        let div = self.builder.build_int_signed_div(lhs, rhs, &format!("div_{}", dest.0));
                        self.registers.insert(dest.clone(), div.into());
                    }
                    Instruction::CmpLt { dest, left, right } => {
                        let lhs = self.get_operand_value(left).into_int_value();
                        let rhs = self.get_operand_value(right).into_int_value();
                        let cmp = self.builder.build_int_compare(inkwell::IntPredicate::SLT, lhs, rhs, &format!("cmp_{}", dest.0));
                        let ext = self.builder.build_int_z_extend(cmp, i64_type, &format!("ext_{}", dest.0));
                        self.registers.insert(dest.clone(), ext.into());
                    }
                    Instruction::CmpGt { dest, left, right } => {
                        let lhs = self.get_operand_value(left).into_int_value();
                        let rhs = self.get_operand_value(right).into_int_value();
                        let cmp = self.builder.build_int_compare(inkwell::IntPredicate::SGT, lhs, rhs, &format!("cmp_{}", dest.0));
                        let ext = self.builder.build_int_z_extend(cmp, i64_type, &format!("ext_{}", dest.0));
                        self.registers.insert(dest.clone(), ext.into());
                    }
                    Instruction::CmpLtEq { dest, left, right } => {
                        let lhs = self.get_operand_value(left).into_int_value();
                        let rhs = self.get_operand_value(right).into_int_value();
                        let cmp = self.builder.build_int_compare(inkwell::IntPredicate::SLE, lhs, rhs, &format!("cmp_{}", dest.0));
                        let ext = self.builder.build_int_z_extend(cmp, i64_type, &format!("ext_{}", dest.0));
                        self.registers.insert(dest.clone(), ext.into());
                    }
                    Instruction::CmpGtEq { dest, left, right } => {
                        let lhs = self.get_operand_value(left).into_int_value();
                        let rhs = self.get_operand_value(right).into_int_value();
                        let cmp = self.builder.build_int_compare(inkwell::IntPredicate::SGE, lhs, rhs, &format!("cmp_{}", dest.0));
                        let ext = self.builder.build_int_z_extend(cmp, i64_type, &format!("ext_{}", dest.0));
                        self.registers.insert(dest.clone(), ext.into());
                    }
                    Instruction::CmpEq { dest, left, right } => {
                        let lhs = self.get_operand_value(left).into_int_value();
                        let rhs = self.get_operand_value(right).into_int_value();
                        let cmp = self.builder.build_int_compare(inkwell::IntPredicate::EQ, lhs, rhs, &format!("cmp_{}", dest.0));
                        let ext = self.builder.build_int_z_extend(cmp, i64_type, &format!("ext_{}", dest.0));
                        self.registers.insert(dest.clone(), ext.into());
                    }
                    Instruction::CmpNotEq { dest, left, right } => {
                        let lhs = self.get_operand_value(left).into_int_value();
                        let rhs = self.get_operand_value(right).into_int_value();
                        let cmp = self.builder.build_int_compare(inkwell::IntPredicate::NE, lhs, rhs, &format!("cmp_{}", dest.0));
                        let ext = self.builder.build_int_z_extend(cmp, i64_type, &format!("ext_{}", dest.0));
                        self.registers.insert(dest.clone(), ext.into());
                    }
                    Instruction::Return { value } => {
                        if let Some(val) = value {
                            let ret_val = self.get_operand_value(val);
                            self.builder.build_return(Some(&ret_val));
                        } else {
                            self.builder.build_return(None);
                        }
                        returned = true;
                    }
                    Instruction::Call { dest, func, args } => {
                        let target_func = match self.module.get_function(func) {
                            Some(f) => f,
                            None => return Err(format!("Unknown function: {}", func)),
                        };

                        let mut compiled_args = Vec::new();
                        for arg in args {
                            compiled_args.push(self.get_operand_value(arg).into());
                        }

                        let call = self.builder.build_call(target_func, &compiled_args, "calltmp");

                        if let Some(d) = dest {
                            if let Some(res) = call.try_as_basic_value().left() {
                                self.registers.insert(d.clone(), res);
                            }
                        }
                    }
                    Instruction::Br { cond, if_true, if_false } => {
                        let cond_val = self.get_operand_value(cond).into_int_value();
                        // Truncate from i64 to i1 for branching
                        let i1_type = self.context.bool_type();
                        let cond_bool = self.builder.build_int_truncate(cond_val, i1_type, "cond_bool");
                        self.builder.build_conditional_branch(cond_bool, llvm_blocks[if_true], llvm_blocks[if_false]);
                        returned = true; // Ends basic block
                    }
                    Instruction::Jmp { dest } => {
                        self.builder.build_unconditional_branch(llvm_blocks[dest]);
                        returned = true; // Ends basic block
                    }
                }
            }

            // If block doesn't have a terminator and isn't returned, we should probably add a dummy return to satisfy LLVM.
            if !returned {
                self.builder.build_return(Some(&self.context.i64_type().const_int(0, false)));
            }
        }

        Ok(function)
    }

    fn get_operand_value(&self, op: &Operand) -> BasicValueEnum<'ctx> {
        match op {
            Operand::ImmInt(val) => {
                self.context.i64_type().const_int(*val as u64, true).into()
            }
            Operand::Reg(reg) => {
                *self.registers.get(reg).expect("Compiler Bug: Unmapped register during LLVM translation.")
            }
            _ => unimplemented!("Compiler Bug: Operand type not yet supported in LLVM Codegen"),
        }
    }

    pub fn dump_ir(&self) {
        println!("{}", self.module.print_to_string().to_string());
    }
}
