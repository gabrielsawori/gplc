use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, PointerValue, FunctionValue};
use std::collections::HashMap;

use crate::ir::gir::{FunctionIR, Instruction, ModuleIR, Operand, Register};

pub struct LLVMCodegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    
    // Maps GIR pointers to LLVM PointerValues (for Alloca/Store/Load)
    variables: HashMap<Register, PointerValue<'ctx>>,
    
    // Maps GIR variables to LLVM BasicValueEnum (for intermediate math results)
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

    /// Compiles a fully constructed GIR Module into an LLVM Module
    pub fn compile_module(&mut self, ir_module: &ModuleIR) -> Result<(), String> {
        for func in &ir_module.functions {
            self.compile_function(func)?;
        }
        Ok(())
    }

    fn compile_function(&mut self, func_ir: &FunctionIR) -> Result<FunctionValue<'ctx>, String> {
        // Define function signature. For now, assuming () -> i64
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[], false);
        
        let function = self.module.add_function(&func_ir.name, fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        // Clear local environments
        self.variables.clear();
        self.registers.clear();

        for block in &func_ir.blocks {
            for inst in &block.instructions {
                match inst {
                    Instruction::Alloca { dest, ty: _ } => {
                        // Allocate memory on the stack
                        let alloca = self.builder.build_alloca(i64_type, &format!("reg_{}", dest.0))
                            .map_err(|e| format!("LLVMError: Failed to build Alloca: {:?}", e))?;
                        self.variables.insert(dest.clone(), alloca);
                    }
                    Instruction::Store { ptr, value } => {
                        let llvm_val = self.get_operand_value(value);
                        if let Some(ptr_val) = self.variables.get(ptr) {
                            self.builder.build_store(*ptr_val, llvm_val)
                                .map_err(|e| format!("LLVMError: Failed to build Store: {:?}", e))?;
                        }
                    }
                    Instruction::Load { dest, ptr } => {
                        if let Some(ptr_val) = self.variables.get(ptr) {
                            let load_val = self.builder.build_load(i64_type, *ptr_val, &format!("load_{}", dest.0))
                                .map_err(|e| format!("LLVMError: Failed to build Load: {:?}", e))?;
                            self.registers.insert(dest.clone(), load_val);
                        }
                    }
                    Instruction::Add { dest, left, right } => {
                        let lhs = self.get_operand_value(left).into_int_value();
                        let rhs = self.get_operand_value(right).into_int_value();
                        let sum = self.builder.build_int_add(lhs, rhs, &format!("add_{}", dest.0))
                            .map_err(|e| format!("LLVMError: Failed to build Add: {:?}", e))?;
                        self.registers.insert(dest.clone(), sum.into());
                    }
                    Instruction::Sub { dest, left, right } => {
                        let lhs = self.get_operand_value(left).into_int_value();
                        let rhs = self.get_operand_value(right).into_int_value();
                        let diff = self.builder.build_int_sub(lhs, rhs, &format!("sub_{}", dest.0))
                            .map_err(|e| format!("LLVMError: Failed to build Sub: {:?}", e))?;
                        self.registers.insert(dest.clone(), diff.into());
                    }
                    Instruction::Return { value } => {
                        if let Some(val) = value {
                            let ret_val = self.get_operand_value(val);
                            self.builder.build_return(Some(&ret_val))
                                .map_err(|e| format!("LLVMError: Failed to build Return: {:?}", e))?;
                        } else {
                            self.builder.build_return(None)
                                .map_err(|e| format!("LLVMError: Failed to build Return: {:?}", e))?;
                        }
                    }
                    _ => {
                        // Unimplemented instructions (Mul, Div, Call) can be added here later
                    }
                }
            }
        }

        Ok(function)
    }

    fn get_operand_value(&self, op: &Operand) -> BasicValueEnum<'ctx> {
        match op {
            Operand::ImmInt(val) => {
                // Compile literal numbers directly into LLVM constants
                self.context.i64_type().const_int(*val as u64, true).into()
            }
            Operand::Reg(reg) => {
                *self.registers.get(reg).expect("Compiler Bug: Unmapped register during LLVM translation.")
            }
            _ => unimplemented!("Compiler Bug: Operand type not yet supported in LLVM Codegen"),
        }
    }

    /// Prints the generated LLVM IR to stdout for debugging purposes
    pub fn dump_ir(&self) {
        println!("{}", self.module.print_to_string().to_string());
    }
}