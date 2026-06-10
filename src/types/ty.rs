/// Type representation for the G type system.
///
/// COMPILER.md §7.1
///
/// All types are stored in a `TypeArena` and referenced by `TyId`.
/// This allows efficient comparison (by id) and avoids deep cloning.

use std::fmt;

/// Index into TypeArena.
pub type TyId = u32;

/// Unique definition ID (used for user-defined types).
pub type DefId = u32;

/// Inference variable ID (used by the unifier).
pub type InferVarId = u32;

/// Lifetime region ID.
pub type LifetimeId = u32;

/// The internal type representation. Matches COMPILER.md §7.1 exactly.
#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
    // ── Primitives ────────────────────────────────────────────────────────
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
    F32, F64,
    Bool, Byte, Rune, Usize, Isize,
    Str,
    Void,
    Never,
    Any,

    // ── Compound ──────────────────────────────────────────────────────────
    Pointer   { mutable: bool, inner: TyId },
    Optional  (TyId),
    Slice     (TyId),
    Array     { size: u64, elem: TyId },
    Tuple     (Vec<TyId>),
    FnPtr     { params: Vec<TyId>, ret: TyId, is_async: bool },
    Map       { key: TyId, val: TyId },
    Set       (TyId),
    Ref       { lifetime: LifetimeId, mutable: bool, inner: TyId },

    // ── User-defined ──────────────────────────────────────────────────────
    Struct    { id: DefId, generics: Vec<TyId> },
    Enum      { id: DefId, generics: Vec<TyId> },
    Union     { id: DefId },
    Interface { id: DefId, generics: Vec<TyId> },
    Newtype   { id: DefId, inner: TyId },

    // ── Special ───────────────────────────────────────────────────────────
    /// Unresolved type variable — filled in by unification.
    Infer(InferVarId),
    /// Error type — suppresses cascade errors after a type mismatch.
    Error,
}

impl Ty {
    /// True if this is a numeric type (integer or float).
    pub fn is_numeric(&self) -> bool {
        matches!(self,
            Ty::I8 | Ty::I16 | Ty::I32 | Ty::I64 | Ty::I128
            | Ty::U8 | Ty::U16 | Ty::U32 | Ty::U64 | Ty::U128
            | Ty::F32 | Ty::F64 | Ty::Usize | Ty::Isize | Ty::Byte)
    }

    /// True if this is an integer type.
    pub fn is_integer(&self) -> bool {
        matches!(self,
            Ty::I8 | Ty::I16 | Ty::I32 | Ty::I64 | Ty::I128
            | Ty::U8 | Ty::U16 | Ty::U32 | Ty::U64 | Ty::U128
            | Ty::Usize | Ty::Isize | Ty::Byte)
    }

    /// True if this is a floating-point type.
    pub fn is_float(&self) -> bool {
        matches!(self, Ty::F32 | Ty::F64)
    }

    /// True if this is a signed integer type.
    pub fn is_signed(&self) -> bool {
        matches!(self, Ty::I8 | Ty::I16 | Ty::I32 | Ty::I64 | Ty::I128 | Ty::Isize)
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::I8    => write!(f, "i8"),
            Ty::I16   => write!(f, "i16"),
            Ty::I32   => write!(f, "i32"),
            Ty::I64   => write!(f, "i64"),
            Ty::I128  => write!(f, "i128"),
            Ty::U8    => write!(f, "u8"),
            Ty::U16   => write!(f, "u16"),
            Ty::U32   => write!(f, "u32"),
            Ty::U64   => write!(f, "u64"),
            Ty::U128  => write!(f, "u128"),
            Ty::F32   => write!(f, "f32"),
            Ty::F64   => write!(f, "f64"),
            Ty::Bool  => write!(f, "bool"),
            Ty::Byte  => write!(f, "byte"),
            Ty::Rune  => write!(f, "rune"),
            Ty::Str   => write!(f, "str"),
            Ty::Void  => write!(f, "void"),
            Ty::Never => write!(f, "never"),
            Ty::Any   => write!(f, "any"),
            Ty::Usize => write!(f, "usize"),
            Ty::Isize => write!(f, "isize"),
            Ty::Pointer { mutable, inner } =>
                write!(f, "*{}{}", if *mutable { "mut " } else { "" }, inner),
            Ty::Optional(inner)  => write!(f, "?{}", inner),
            Ty::Slice(inner)     => write!(f, "[]{}", inner),
            Ty::Array { size, elem } => write!(f, "[{}]{}", size, elem),
            Ty::Tuple(elems) => {
                write!(f, "(")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            Ty::FnPtr { params, ret, .. } => {
                write!(f, "fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            Ty::Infer(id) => write!(f, "?T{}", id),
            Ty::Error     => write!(f, "<error>"),
            _ => write!(f, "<type>"),
        }
    }
}

// ── Type Arena ────────────────────────────────────────────────────────────────

/// Arena-based type storage. All types are interned here and referenced
/// by `TyId` for efficient comparison and sharing.
#[derive(Debug)]
pub struct TypeArena {
    types: Vec<Ty>,
}

impl TypeArena {
    pub fn new() -> Self {
        Self { types: Vec::new() }
    }

    /// Intern a type and return its ID.
    pub fn intern(&mut self, ty: Ty) -> TyId {
        // Check if this exact type already exists (deduplication)
        for (i, existing) in self.types.iter().enumerate() {
            if existing == &ty {
                return i as TyId;
            }
        }
        let id = self.types.len() as TyId;
        self.types.push(ty);
        id
    }

    /// Get a type by its ID.
    pub fn get(&self, id: TyId) -> &Ty {
        &self.types[id as usize]
    }

    /// Create a fresh inference variable.
    pub fn fresh_infer(&mut self) -> TyId {
        let var_id = self.types.len() as InferVarId;
        self.intern(Ty::Infer(var_id))
    }

    /// Number of interned types.
    pub fn len(&self) -> usize {
        self.types.len()
    }

    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }
}

impl Default for TypeArena {
    fn default() -> Self {
        Self::new()
    }
}
