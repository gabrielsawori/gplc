#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Register(pub usize);

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Reg(Register),
    ImmInt(i64),
    ImmString(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Memory Allocation & Access (Crucial for low-level memory control)
    Alloca { dest: Register, ty: String },
    Store { ptr: Register, value: Operand },
    Load { dest: Register, ptr: Register },

    // Arithmetic Logic Unit (ALU) Operations
    Add { dest: Register, left: Operand, right: Operand },
    Sub { dest: Register, left: Operand, right: Operand },
    Mul { dest: Register, left: Operand, right: Operand },
    Div { dest: Register, left: Operand, right: Operand },
    // Comparisons
    CmpLt { dest: Register, left: Operand, right: Operand },
    CmpGt { dest: Register, left: Operand, right: Operand },
    CmpLtEq { dest: Register, left: Operand, right: Operand },
    CmpGtEq { dest: Register, left: Operand, right: Operand },
    CmpEq { dest: Register, left: Operand, right: Operand },
    CmpNotEq { dest: Register, left: Operand, right: Operand },

    // Control Flow
    Return { value: Option<Operand> },
    Call { dest: Option<Register>, func: String, args: Vec<Operand> },
    Br { cond: Operand, if_true: usize, if_false: usize },
    Jmp { dest: usize },
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: usize,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct FunctionIR {
    pub name: String,
    pub params: usize, // Store number of parameters
    pub blocks: Vec<BasicBlock>,
}

#[derive(Debug, Clone)]
pub struct ModuleIR {
    pub name: String,
    pub functions: Vec<FunctionIR>,
}
