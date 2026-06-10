use crate::lexer::token::{Span, IntSuffix, FloatSuffix, FStringPart};

// ── Identifiers ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl Ident {
    pub fn new(name: impl Into<String>, span: Span) -> Self {
        Self { name: name.into(), span }
    }
}

// ── Labels (@outer) ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub name: String,
    pub span: Span,
}

// ── Lifetime ('a) ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Lifetime {
    pub name: String, // "a", "static", etc. (without the ')
    pub span: Span,
}

// ─────────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Primitive: i32, f64, bool, str, etc.
    Primitive(PrimType, Span),
    /// Named type, possibly generic: MyStruct, List[i32], Map[str, i32]
    Path {
        segments: Vec<Ident>,
        generics: Vec<Type>,
        span:     Span,
    },
    /// *T or *const T
    Pointer {
        is_const: bool,
        inner:    Box<Type>,
        span:     Span,
    },
    /// ?T
    Optional(Box<Type>, Span),
    /// []T
    Slice(Box<Type>, Span),
    /// [N]T
    Array {
        size: Box<Expr>,
        elem: Box<Type>,
        span: Span,
    },
    /// (T, U, V)
    Tuple(Vec<Type>, Span),
    /// fn(T, U) -> R
    FnPtr {
        params:  Vec<Type>,
        ret:     Box<Type>,
        is_async: bool,
        span:    Span,
    },
    /// map[K]V
    Map {
        key:  Box<Type>,
        val:  Box<Type>,
        span: Span,
    },
    /// set[T]
    Set(Box<Type>, Span),
    /// &T or &mut T or &'a T
    Ref {
        lifetime: Option<Lifetime>,
        mutable:  bool,
        inner:    Box<Type>,
        span:     Span,
    },
    /// never
    Never(Span),
    /// void
    Void(Span),
    /// any
    Any(Span),
    /// _ (infer)
    Infer(Span),
}

impl Type {
    pub fn span(&self) -> Span {
        match self {
            Type::Primitive(_, s)        => *s,
            Type::Path { span, .. }      => *span,
            Type::Pointer { span, .. }   => *span,
            Type::Optional(_, s)         => *s,
            Type::Slice(_, s)            => *s,
            Type::Array { span, .. }     => *span,
            Type::Tuple(_, s)            => *s,
            Type::FnPtr { span, .. }     => *span,
            Type::Map { span, .. }       => *span,
            Type::Set(_, s)              => *s,
            Type::Ref { span, .. }       => *span,
            Type::Never(s)               => *s,
            Type::Void(s)                => *s,
            Type::Any(s)                 => *s,
            Type::Infer(s)               => *s,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimType {
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
    F32, F64,
    Bool, Byte, Rune, Str,
    Usize, Isize,
}

// ─────────────────────────────────────────────────────────────────────────────
// Generic parameters and arguments
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct GenericParam {
    pub name:    Ident,
    pub bounds:  Vec<Type>,   // T: Comparable + Eq
    pub default: Option<Type>,
    pub span:    Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhereClause {
    pub predicates: Vec<WherePredicate>,
    pub span:       Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WherePredicate {
    pub ty:     Type,
    pub bounds: Vec<Type>,
    pub span:   Span,
}

// ─────────────────────────────────────────────────────────────────────────────
// Attributes  @inline  @packed  @section(".text")
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: Ident,
    pub args: Vec<AttrArg>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttrArg {
    Ident(Ident),
    Str(String, Span),
    Int(u128, Span),
    Bool(bool, Span),
    KeyValue { key: Ident, val: Box<AttrArg> },
}

// ─────────────────────────────────────────────────────────────────────────────
// Top-level file
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub directives: Vec<Directive>,
    pub module:     ModuleDecl,
    pub imports:    Vec<Import>,
    pub items:      Vec<Item>,
    pub span:       Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    pub name: String,  // "no_std", "no_runtime", "no_fp"
    pub arg:  Option<String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleDecl {
    pub path: Vec<Ident>,  // ["myapp", "utils", "math"]
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub path:     String,
    pub kind:     ImportKind,
    pub span:     Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportKind {
    /// import "std/io"  or  import "std/io" as io
    Simple { alias: Option<Ident> },
    /// import "std/io": {println, eprintln}
    Selective { names: Vec<Ident> },
    /// use Direction.*
    GlobUse { ty_path: Vec<Ident> },
}

// ─────────────────────────────────────────────────────────────────────────────
// Items (top-level declarations)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub kind:  ItemKind,
    pub attrs: Vec<Attribute>,
    pub span:  Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemKind {
    Fn(FnDecl),
    Struct(StructDecl),
    Enum(EnumDecl),
    Union(UnionDecl),
    Interface(InterfaceDecl),
    ImplBlock(ImplBlock),
    ImplFor(ImplFor),
    TypeAlias(TypeAlias),
    Newtype(NewtypeDecl),
    Const(ConstDecl),
    Var(VarDecl),
    Extern(ExternDecl),
}

// ─────────────────────────────────────────────────────────────────────────────
// Functions
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub name:     FnName,
    pub generics: Vec<GenericParam>,
    pub params:   Vec<Param>,
    pub ret_ty:   Option<Type>,
    pub body:     Option<Block>,
    pub is_pub:   bool,
    pub is_async: bool,
    pub is_const: bool,
    pub span:     Span,
}

/// Function name: either a simple ident or a method `Type.method`
#[derive(Debug, Clone, PartialEq)]
pub enum FnName {
    Simple(Ident),
    Method { ty_name: Ident, method: Ident, span: Span },
}

impl FnName {
    pub fn span(&self) -> Span {
        match self {
            FnName::Simple(i)         => i.span,
            FnName::Method { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub kind: ParamKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParamKind {
    /// Regular: name: Type  or  name: Type = default
    Regular {
        name:    Ident,
        ty:      Type,
        default: Option<Expr>,
    },
    /// Self receiver: self  or  self: *Foo  or  self: &Foo
    SelfParam {
        mutable:  bool,
        ty_annot: Option<Type>,
    },
    /// Variadic: ...name: ...Type
    Variadic {
        name: Ident,
        ty:   Type,
    },
}

// ─────────────────────────────────────────────────────────────────────────────
// Structs
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub name:     Ident,
    pub generics: Vec<GenericParam>,
    pub fields:   Vec<StructField>,
    pub is_pub:   bool,
    pub span:     Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub attrs:   Vec<Attribute>,
    pub is_pub:  bool,
    pub is_embed:bool,
    pub name:    Ident,
    pub ty:      Type,
    pub default: Option<Expr>,
    pub span:    Span,
}

// ─────────────────────────────────────────────────────────────────────────────
// Enums
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub name:     Ident,
    pub generics: Vec<GenericParam>,
    pub variants: Vec<EnumVariant>,
    pub is_pub:   bool,
    pub span:     Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub attrs:   Vec<Attribute>,
    pub name:    Ident,
    pub kind:    VariantKind,
    pub value:   Option<Expr>,   // explicit discriminant
    pub span:    Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariantKind {
    Unit,
    Tuple(Vec<Type>),
    Struct(Vec<StructField>),
    Named(Vec<(Ident, Type)>),  // Circle(radius: f64)
}

// ─────────────────────────────────────────────────────────────────────────────
// Unions
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct UnionDecl {
    pub name:   Ident,
    pub fields: Vec<UnionField>,
    pub is_pub: bool,
    pub span:   Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionField {
    pub name: Ident,
    pub ty:   Type,
    pub span: Span,
}

// ─────────────────────────────────────────────────────────────────────────────
// Interfaces
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceDecl {
    pub name:     Ident,
    pub generics: Vec<GenericParam>,
    pub supers:   Vec<Type>,
    pub items:    Vec<InterfaceItem>,
    pub is_pub:   bool,
    pub span:     Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterfaceItem {
    Method(FnDecl),
    AssocType {
        name:    Ident,
        default: Option<Type>,
        span:    Span,
    },
}

// ─────────────────────────────────────────────────────────────────────────────
// Impl blocks
// ─────────────────────────────────────────────────────────────────────────────

/// impl Struct:  (method grouping)
#[derive(Debug, Clone, PartialEq)]
pub struct ImplBlock {
    pub ty:      Type,
    pub methods: Vec<FnDecl>,
    pub span:    Span,
}

/// impl Interface for Type:
#[derive(Debug, Clone, PartialEq)]
pub struct ImplFor {
    pub generics:  Vec<GenericParam>,
    pub interface: Type,
    pub for_ty:    Type,
    pub where_:    Option<WhereClause>,
    pub methods:   Vec<FnDecl>,
    pub assoc_types: Vec<AssocTypeImpl>,
    pub span:      Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssocTypeImpl {
    pub name: Ident,
    pub ty:   Type,
    pub span: Span,
}

// ─────────────────────────────────────────────────────────────────────────────
// Other top-level items
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAlias {
    pub name:     Ident,
    pub generics: Vec<GenericParam>,
    pub ty:       Type,
    pub is_pub:   bool,
    pub span:     Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewtypeDecl {
    pub name:   Ident,
    pub ty:     Type,
    pub is_pub: bool,
    pub span:   Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstDecl {
    pub name:   Ident,
    pub ty:     Type,
    pub value:  Expr,
    pub is_pub: bool,
    pub span:   Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub names:    Vec<Ident>,
    pub ty:       Option<Type>,
    pub value:    Option<Vec<Expr>>,
    pub is_let:   bool,
    pub is_pub:   bool,
    pub span:     Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExternDecl {
    pub abi:  Option<String>,  // "C", "cdecl", etc.
    pub kind: ExternKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExternKind {
    Fn {
        name:   Ident,
        params: Vec<Type>,
        ret:    Type,
        variadic: bool,
    },
    Var {
        name: Ident,
        ty:   Type,
    },
}

// ─────────────────────────────────────────────────────────────────────────────
// Statements
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span:  Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
    /// var x: T = expr  or  var x := expr  or  let x: T = expr
    Var(VarDecl),
    /// x = expr  or  x += expr  etc.
    Assign {
        target: Expr,
        op:     AssignOp,
        value:  Expr,
    },
    /// Bare expression statement
    Expr(Expr),
    /// return  or  return expr
    Return(Option<Expr>),
    /// break  or  break @label
    Break(Option<Label>),
    /// continue  or  continue @label
    Continue(Option<Label>),
    /// defer expr  or  defer block
    Defer(Box<Stmt>),
    /// if ... elif ... else ...
    If(IfStmt),
    /// while cond: block
    While {
        label: Option<Label>,
        cond:  Expr,
        body:  Block,
    },
    /// for pat in expr: block
    For {
        label: Option<Label>,
        pat:   ForPat,
        iter:  Expr,
        step:  Option<Expr>,
        body:  Block,
    },
    /// loop: block
    Loop {
        label: Option<Label>,
        body:  Block,
    },
    /// match expr: arms
    Match(MatchStmt),
    /// try: ... catch e: T: ... finally: ...
    Try(TryStmt),
    /// unsafe: block
    Unsafe(Block),
    /// asm: ...
    Asm(AsmBlock),
    /// comptime var x := expr
    Comptime(Box<Stmt>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
    Eq,
    AddEq, SubEq, MulEq, DivEq, RemEq, PowEq,
    AndEq, OrEq, XorEq, ShlEq, ShrEq,
    AddWrapEq, SubWrapEq, MulWrapEq,
    AddSatEq,  SubSatEq,  MulSatEq,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForPat {
    pub kind: ForPatKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForPatKind {
    Single(Ident),
    IndexValue(Ident, Ident),  // i, item
    Discard,                    // _
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub cond:   Expr,
    pub then:   Block,
    pub elifs:  Vec<(Expr, Block)>,
    pub else_:  Option<Block>,
    pub span:   Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchStmt {
    pub scrutinee: Expr,
    pub arms:      Vec<MatchArm>,
    pub span:      Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pats:  Vec<Pat>,
    pub guard: Option<Expr>,
    pub body:  MatchBody,
    pub span:  Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchBody {
    Expr(Expr),
    Block(Block),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TryStmt {
    pub body:    Block,
    pub catches: Vec<CatchClause>,
    pub finally: Option<Block>,
    pub span:    Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause {
    pub name: Ident,
    pub ty:   Type,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AsmBlock {
    pub template:  Vec<String>,
    pub outputs:   Vec<AsmOperand>,
    pub inputs:    Vec<AsmOperand>,
    pub clobbers:  Vec<String>,
    pub span:      Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AsmOperand {
    pub kind:    AsmOpKind,
    pub expr:    Expr,
    pub span:    Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsmOpKind { In, Out, InOut }

// ─────────────────────────────────────────────────────────────────────────────
// Patterns (for match arms)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Pat {
    pub kind: PatKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatKind {
    Wildcard,
    Null,
    Bool(bool),
    Int(u128, Option<IntSuffix>),
    Float(f64, Option<FloatSuffix>),
    Str(String),
    Char(char),
    Ident(Ident),                             // binding pattern
    Path(Vec<Ident>),                         // Enum.Variant
    Tuple(Vec<Pat>),                          // (a, b, c)
    Struct(Vec<Ident>, Vec<(Ident, Pat)>),    // Struct { field: pat }
    TupleStruct(Vec<Ident>, Vec<Pat>),        // Enum.Variant(a, b)
    Or(Vec<Pat>),                             // pat1 | pat2
    Range { lo: Box<Pat>, hi: Box<Pat>, inclusive: bool },
}

// ─────────────────────────────────────────────────────────────────────────────
// Expressions
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    // ── Literals ──────────────────────────────────────────────────────────────
    Int(u128, Option<IntSuffix>),
    Float(f64, Option<FloatSuffix>),
    Str(String),
    RawStr(String),
    FStr(Vec<FStringPart>),
    Char(char),
    Byte(u8),
    Bool(bool),
    Null,

    // ── Name ──────────────────────────────────────────────────────────────────
    Ident(Ident),
    /// Path: Foo.Bar.Baz
    Path(Vec<Ident>),

    // ── Operators ─────────────────────────────────────────────────────────────
    BinOp {
        op:  BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    UnaryOp {
        op:      UnaryOp,
        operand: Box<Expr>,
    },
    /// x as T  or  x as! T  or  x as? T
    Cast {
        expr: Box<Expr>,
        ty:   Box<Type>,
        kind: CastKind,
    },
    /// x is T
    Is {
        expr: Box<Expr>,
        ty:   Box<Type>,
    },

    // ── Postfix ───────────────────────────────────────────────────────────────
    /// expr.field
    Field { base: Box<Expr>, field: Ident },
    /// expr->field
    PtrField { base: Box<Expr>, field: Ident },
    /// expr?.field
    OptField { base: Box<Expr>, field: Ident },
    /// expr[idx]
    Index { base: Box<Expr>, index: Box<Expr> },
    /// expr[lo:hi]
    Slice {
        base: Box<Expr>,
        lo:   Option<Box<Expr>>,
        hi:   Option<Box<Expr>>,
    },
    /// f(args)
    Call { callee: Box<Expr>, args: Vec<CallArg> },
    /// expr?.method(args)
    OptCall { base: Box<Expr>, method: Ident, args: Vec<CallArg> },
    /// expr? — error propagation
    Propagate(Box<Expr>),
    /// expr! — unwrap
    Unwrap(Box<Expr>),

    // ── Collections ───────────────────────────────────────────────────────────
    Array(Vec<Expr>),
    Tuple(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Set(Vec<Expr>),

    // ── Struct literal ────────────────────────────────────────────────────────
    StructLit {
        ty:     Vec<Ident>,             // type path
        fields: Vec<FieldInit>,
        rest:   Option<Box<Expr>>,      // ..other_struct
    },

    // ── Ranges ────────────────────────────────────────────────────────────────
    Range {
        lo:        Box<Expr>,
        hi:        Box<Expr>,
        inclusive: bool,
    },

    // ── Lambda / closure ──────────────────────────────────────────────────────
    Lambda {
        is_move:  bool,
        params:   Vec<Param>,
        ret_ty:   Option<Box<Type>>,
        body:     LambdaBody,
        is_async: bool,
    },

    // ── Control flow expressions ──────────────────────────────────────────────
    If {
        cond:  Box<Expr>,
        then:  Box<Expr>,
        elifs: Vec<(Expr, Expr)>,
        else_: Box<Expr>,
    },
    Match {
        scrutinee: Box<Expr>,
        arms:      Vec<MatchArm>,
    },
    Block(Block),

    // ── Built-in macros ───────────────────────────────────────────────────────
    Builtin {
        name:   String,
        ty_arg: Option<Box<Type>>,
        args:   Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum LambdaBody {
    Expr(Box<Expr>),
    Block(Block),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldInit {
    pub name:  Ident,
    pub value: Option<Expr>,   // None = shorthand  {x}  means  {x: x}
    pub span:  Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg {
    pub label: Option<Ident>,  // named argument: func(port: 8080)
    pub expr:  Expr,
    pub spread: bool,          // func(args...)
    pub span:  Span,
}

// ── Binary operators ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    // Arithmetic
    Add, Sub, Mul, Div, Rem, Pow,
    // Overflow variants
    AddWrap, SubWrap, MulWrap,
    AddSat,  SubSat,  MulSat,
    AddChk,  SubChk,  MulChk,
    // Bitwise
    BitAnd, BitOr, BitXor, Shl, Shr,
    // Comparison
    Eq, Ne, Lt, Le, Gt, Ge,
    // Logical
    And, Or,
    // Other
    Range,          // ..
    RangeInclusive, // ..=
    Pipe,           // |>
    Coalesce,       // ??
}

// ── Unary operators ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,        // -x
    Not,        // not x
    BitNot,     // ~x
    Deref,      // *x
    AddrOf,     // &x
    AddrOfMut,  // &mut x
}

// ── Cast kinds ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastKind {
    Safe,    // as
    Assert,  // as!
    Try,     // as?
}
