use clap::{Parser, Subcommand};
use std::process;

// --- INKWELL LLVM ---
use inkwell::context::Context;

pub mod ast;
pub mod borrow;
pub mod codegen;
pub mod driver;
pub mod error;
pub mod ir;
pub mod lexer;
pub mod opt;
pub mod parser;
pub mod resolve;
pub mod session;
pub mod types;

use crate::driver::Driver;
use crate::ir::builder::IRBuilder;
use crate::ir::gir::{ModuleIR};
use crate::codegen::LLVMCodegen;

#[derive(Parser)]
#[command(name = "gpl", version = "1.0.0-draft", about = "Compiler for the G Programming Language")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build { file: String },
    Run { file: String },
    Check { file: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { file } | Commands::Check { file } | Commands::Run { file } => {
            let is_check_only = matches!(cli.command, Commands::Check { .. });
            println!("⚙️  Compiling: {}", file);

            // 1. Run Frontend (Lexer -> Parser -> Semantic -> Borrow Checker)
            let ast = match Driver::run_frontend(file) {
                Ok(ast) => ast,
                Err(e) => {
                    eprintln!("❌ {}", e);
                    process::exit(1);
                }
            };

            if is_check_only {
                println!("✅ No syntax, type, or borrow errors found.");
                return;
            }

            // 2. Run Middle-end (AST -> GIR)
            println!("🔍 Generating Intermediate Representation (GIR)...");
            let mut ir_builder = IRBuilder::new();
            
            // We need to collect all functions. For MVP, assume any FunctionDeclaration is a function, and the rest is in `main`.
            let mut functions = Vec::new();
            let mut main_body = Vec::new();
            for stmt in ast {
                match stmt {
                    crate::ast::nodes::Statement::FunctionDeclaration { name, params, return_type: _, body } => {
                        functions.push(ir_builder.build_function(&name, &body, &params));
                    },
                    _ => {
                        main_body.push(stmt);
                    }
                }
            }
            functions.push(ir_builder.build_function("main", &main_body, &[]));

            let gir_module = ModuleIR {
                name: file.clone(),
                functions,
            };

            // 3. Run Backend (GIR -> LLVM IR)
            println!("🚀 Executing LLVM Code Generation...");
            let llvm_context = Context::create();
            let mut llvm_codegen = LLVMCodegen::new(&llvm_context, "g_main_module");

            if let Err(e) = llvm_codegen.compile_module(&gir_module) {
                eprintln!("❌ {}", e);
                process::exit(1);
            }

            println!("✅ Build completed successfully.");
            
            // Debug: Print the raw LLVM IR assembly to the terminal
            println!("\n--- LLVM IR OUTPUT ---");
            llvm_codegen.dump_ir();
            println!("----------------------\n");

            if matches!(cli.command, Commands::Run { .. }) {
                println!("⚠️  JIT Execution (Run) feature will be integrated next!");

                // Let's actually JIT the function and run it
                let execution_engine = llvm_codegen.module.create_jit_execution_engine(inkwell::OptimizationLevel::None).unwrap();
                unsafe {
                    let main_func = execution_engine.get_function::<unsafe extern "C" fn() -> u64>("main").unwrap();
                    let res = main_func.call();
                    println!("🎉 Program execution returned: {}", res);
                }
            }
        }
    }
}
