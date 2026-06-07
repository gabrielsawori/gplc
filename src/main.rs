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
use crate::ir::gir::{FunctionIR, ModuleIR};
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
            
            // For MVP, we wrap the global statements inside an implicit "main" function
            let main_func_ir = ir_builder.build_function("main", &ast);
            let gir_module = ModuleIR {
                name: file.clone(),
                functions: vec![main_func_ir],
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
            }
        }
    }
}