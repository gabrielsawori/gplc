#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    // Primitives
    I8, I16, I32, I64, I128, ISize,
    U8, U16, U32, U64, U128, USize,
    F32, F64,
    Bool,
    Byte,
    Rune,
    Str,
    Void,
    Never,
    Any,

    // Pointers and Arrays
    Pointer(Box<Type>),      // *mut T
    ConstPtr(Box<Type>),     // *const T
    Slice(Box<Type>),        // []T
    Array(Box<Type>, usize), // [N]T

    // Functions
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },

    // User-defined
    Struct(String),
    Enum(String),

    // Type checking artifacts
    Unknown, // For type inference
    Error,   // Used to suppress cascading errors after the first type error is found
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::I8 => "i8".to_string(),
            Type::I32 => "i32".to_string(),
            Type::I64 => "i64".to_string(),
            Type::F64 => "f64".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Str => "str".to_string(),
            Type::Pointer(inner) => format!("*mut {}", inner.to_string()),
            Type::ConstPtr(inner) => format!("*const {}", inner.to_string()),
            Type::Slice(inner) => format!("[]{}", inner.to_string()),
            Type::Array(inner, size) => format!("[{}]{}", size, inner.to_string()),
            Type::Unknown => "unknown".to_string(),
            Type::Error => "error".to_string(),
            _ => format!("{:?}", self).to_lowercase(),
        }
    }
}