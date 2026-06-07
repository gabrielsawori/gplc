# G Language Compiler — Implementation Guide

**Document version:** 1.0.0  
**Target:** Contributors who want to implement the G compiler  
**Prerequisite knowledge:** Basic understanding of compilers, data structures, and systems programming

---

## Table of Contents

1. [Architecture Overview](#1-architecture-overview)
2. [Repository Structure](#2-repository-structure)
3. [Phase 1 — Lexer](#3-phase-1--lexer)
4. [Phase 2 — Parser](#4-phase-2--parser)
5. [Phase 3 — AST](#5-phase-3--ast)
6. [Phase 4 — Name Resolution](#6-phase-4--name-resolution)
7. [Phase 5 — Type Checker](#7-phase-5--type-checker)
8. [Phase 6 — Borrow Checker](#8-phase-6--borrow-checker)
9. [Phase 7 — IR (Intermediate Representation)](#9-phase-7--ir-intermediate-representation)
10. [Phase 8 — Optimizations](#10-phase-8--optimizations)
11. [Phase 9 — Code Generation](#11-phase-9--code-generation)
12. [Phase 10 — Linker Integration](#12-phase-10--linker-integration)
13. [Standard Library Implementation](#13-standard-library-implementation)
14. [Error Reporting](#14-error-reporting)
15. [Testing the Compiler](#15-testing-the-compiler)
16. [Bootstrap Strategy](#16-bootstrap-strategy)
17. [Recommended Starting Point](#17-recommended-starting-point)

---

## 1. Architecture Overview

The G compiler (`gplc`) is a traditional **multi-pass compiler** with the following pipeline:

```
Source (.gpl)
     │
     ▼
┌─────────────┐
│   Lexer     │  Source text → Token stream
└──────┬──────┘
       │  Token stream
       ▼
┌─────────────┐
│   Parser    │  Token stream → AST (Abstract Syntax Tree)
└──────┬──────┘
       │  AST
       ▼
┌──────────────────┐
│ Name Resolution  │  Resolve all identifiers, build symbol table
└──────┬───────────┘
       │  AST + symbol table
       ▼
┌──────────────┐
│ Type Checker │  Infer & verify all types, check constraints
└──────┬───────┘
       │  Typed AST
       ▼
┌───────────────┐
│ Borrow Checker│  Verify ownership, lifetimes, aliasing rules
└──────┬────────┘
       │  Verified typed AST
       ▼
┌─────────────┐
│  IR Lowering│  AST → GIR (G Intermediate Representation)
└──────┬──────┘
       │  GIR
       ▼
┌──────────────┐
│ Optimizations│  Constant folding, DCE, inlining, loop opts
└──────┬───────┘
       │  Optimized GIR
       ▼
┌──────────────┐
│  Code Gen    │  GIR → LLVM IR  (or native assembly)
└──────┬───────┘
       │  LLVM IR / .s
       ▼
┌──────────────┐
│    LLVM      │  LLVM IR → object file (.o)
└──────┬───────┘
       │  .o
       ▼
┌──────────────┐
│    Linker    │  .o + deps → executable / library
└──────────────┘
       │
       ▼
  Binary output
```

### Design Principles for the Compiler

1. **Single-pass where possible** — minimize memory usage for large codebases
2. **Parallel compilation** — each `.gpl` file is an independent compilation unit
3. **Incremental compilation** — cache object files, only recompile changed files
4. **Good error messages first** — never output a vague error; always include location, hint, and error code
5. **Spec-driven** — every behavior must trace back to a section in the spec

---

## 2. Repository Structure

```
gplc/                          # compiler root
├── Cargo.toml                 # if using Rust for bootstrap
├── src/
│   ├── main.rs                # CLI entry point (gpl command)
│   ├── driver.rs              # orchestrates all passes
│   │
│   ├── lexer/
│   │   ├── mod.rs
│   │   ├── token.rs           # Token enum
│   │   ├── lexer.rs           # Lexer struct + impl
│   │   └── tests.rs
│   │
│   ├── parser/
│   │   ├── mod.rs
│   │   ├── parser.rs          # recursive descent parser
│   │   ├── precedence.rs      # Pratt parser for expressions
│   │   └── tests.rs
│   │
│   ├── ast/
│   │   ├── mod.rs
│   │   ├── nodes.rs           # all AST node types
│   │   ├── visitor.rs         # visitor trait for AST traversal
│   │   └── printer.rs         # debug AST printer
│   │
│   ├── resolve/
│   │   ├── mod.rs
│   │   ├── scope.rs           # Scope / symbol table
│   │   └── resolver.rs        # name resolution pass
│   │
│   ├── types/
│   │   ├── mod.rs
│   │   ├── ty.rs              # Type enum (all G types)
│   │   ├── infer.rs           # Hindley-Milner type inference
│   │   ├── checker.rs         # type checking pass
│   │   └── unify.rs           # unification algorithm
│   │
│   ├── borrow/
│   │   ├── mod.rs
│   │   ├── lifetime.rs        # lifetime regions
│   │   ├── checker.rs         # borrow checking pass
│   │   └── flow.rs            # control flow graph for borrows
│   │
│   ├── ir/
│   │   ├── mod.rs
│   │   ├── gir.rs             # GIR instruction set
│   │   ├── builder.rs         # GIR builder (from typed AST)
│   │   ├── cfg.rs             # control flow graph
│   │   └── printer.rs         # GIR text printer
│   │
│   ├── opt/
│   │   ├── mod.rs
│   │   ├── const_fold.rs      # constant folding & propagation
│   │   ├── dce.rs             # dead code elimination
│   │   ├── inline.rs          # function inlining
│   │   └── mem2reg.rs         # promote stack allocations to registers
│   │
│   ├── codegen/
│   │   ├── mod.rs
│   │   ├── llvm.rs            # LLVM IR emission
│   │   ├── abi.rs             # calling convention / ABI
│   │   └── mangle.rs          # name mangling
│   │
│   ├── error/
│   │   ├── mod.rs
│   │   ├── codes.rs           # all E/W/P codes from spec §78
│   │   └── reporter.rs        # diagnostic formatting
│   │
│   └── session/
│       ├── mod.rs
│       └── session.rs         # compilation session (config, file map)
│
├── tests/
│   ├── lexer/                 # lexer unit tests
│   ├── parser/                # parser unit tests
│   ├── typecheck/             # type checking tests
│   ├── borrow/                # borrow checker tests
│   ├── codegen/               # output verification tests
│   └── programs/              # full end-to-end programs
│
└── stdlib/                    # standard library source
    ├── core/
    │   ├── mem.gpl
    │   ├── atomic.gpl
    │   └── volatile.gpl
    └── std/
        ├── io.gpl
        ├── fs.gpl
        └── ...
```

---

## 3. Phase 1 — Lexer

The lexer converts raw source text into a flat stream of tokens.

### 3.1 Token Types

```rust
// token.rs
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    IntLit(u128, IntSuffix),      // value + suffix (i32, u64, etc.)
    FloatLit(f64, FloatSuffix),   // value + suffix (f32, f64)
    StringLit(String),            // processed string (escapes resolved)
    RawString(String),            // raw string (no processing)
    FString(Vec<FStringPart>),    // interpolated string parts
    CharLit(char),
    ByteLit(u8),
    BoolLit(bool),

    // Identifiers & keywords
    Ident(String),
    Keyword(Keyword),

    // Operators (each operator is its own variant for speed)
    Plus, Minus, Star, Slash, Percent, StarStar,
    PlusPercent, MinusPercent, StarPercent,
    PlusBar, MinusBar, StarBar,
    PlusExcl, MinusExcl, StarExcl,
    Ampersand, Bar, Caret, Tilde,
    LtLt, GtGt,
    Eq, EqEq, BangEq,
    Lt, Gt, LtEq, GtEq,
    DotDot, DotDotEq,
    PipeGt,         // |>
    QuestionQuestion, // ??
    QuestionDot,    // ?.
    Question,
    Bang,
    Arrow,          // ->
    FatArrow,       // =>
    Dot, Comma, Colon, ColonEq, Semicolon,
    At, Hash, HashBang,
    Ampersand2,     // &&  (not used — G uses 'and')

    // Delimiters
    LParen, RParen,
    LBracket, RBracket,
    LBrace, RBrace,

    // Assignment operators
    PlusEq, MinusEq, StarEq, SlashEq, PercentEq, StarStarEq,
    AmpersandEq, BarEq, CaretEq, LtLtEq, GtGtEq,
    PlusPercentEq, MinusPercentEq, StarPercentEq,
    PlusBarEq, MinusBarEq, StarBarEq,

    // Indentation (virtual tokens)
    Indent,
    Dedent,
    Newline,

    // Lifetime
    Lifetime(String),   // 'name

    // Comments (usually discarded, kept for doc generation)
    LineComment(String),
    DocComment(String),

    // Special
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,         // byte range in source file
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub file_id: u32,
    pub start:   u32,       // byte offset in file
    pub end:     u32,
}
```

### 3.2 Indentation Handling

The trickiest part of lexing G is producing `Indent`/`Dedent` tokens correctly.

```
Algorithm:
1. Maintain a stack of indentation levels: [0]
2. At the start of each non-empty, non-comment line:
   a. Count leading spaces (tabs = error E0004)
   b. If count > stack.top():
      - Push count onto stack
      - Emit INDENT token
   c. If count == stack.top():
      - Emit NEWLINE (continue same block)
   d. If count < stack.top():
      - While stack.top() > count:
          Pop stack
          Emit DEDENT token
      - If stack.top() != count:
          Emit error E0003 (indent not matching any outer level)
3. At EOF:
   - While stack.top() > 0:
       Pop stack
       Emit DEDENT token
   - Emit EOF
```

```rust
// lexer.rs (pseudocode)
pub struct Lexer<'src> {
    source:      &'src str,
    pos:         usize,
    file_id:     u32,
    indent_stack: Vec<usize>,
    pending_dedents: usize,
    at_line_start: bool,
}

impl<'src> Lexer<'src> {
    pub fn next_token(&mut self) -> Token {
        // handle pending dedents first
        if self.pending_dedents > 0 {
            self.pending_dedents -= 1;
            return self.make_token(TokenKind::Dedent);
        }

        // skip blank lines and comments
        self.skip_whitespace_and_comments();

        if self.at_line_start {
            self.at_line_start = false;
            return self.handle_indent();
        }

        // lex next token normally
        match self.peek_char() {
            '\0' => self.handle_eof(),
            '\n' => { self.at_line_start = true; self.advance(); self.make_token(TokenKind::Newline) }
            '"'  => self.lex_string(),
            '\'' => self.lex_char(),
            '0'..='9' => self.lex_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.lex_ident_or_keyword(),
            '+' => self.lex_plus(),
            // ... etc
            c    => self.error(E0001, format!("unexpected character: {:?}", c)),
        }
    }
}
```

### 3.3 F-String Lexing

F-strings contain mixed text and `{expr}` interpolations. Lex them as a sequence of parts:

```rust
enum FStringPart {
    Text(String),           // literal text segment
    Expr(Vec<Token>),       // tokens inside { ... }
    ExprWithFormat {        // tokens inside { ... : format_spec }
        tokens: Vec<Token>,
        format: String,
    },
}
```

When lexing `f"Hello, {name}! Score: {score:.2f}"`:
1. Emit `Text("Hello, ")`
2. Recursively lex tokens inside `{name}` → `Expr([Ident("name")])`
3. Emit `Text("! Score: ")`
4. Lex `{score:.2f}` → `ExprWithFormat { tokens: [Ident("score")], format: ".2f" }`

---

## 4. Phase 2 — Parser

The parser converts the token stream into an AST using **recursive descent** for statements  
and a **Pratt parser (top-down operator precedence)** for expressions.

### 4.1 Parser Structure

```rust
pub struct Parser<'a> {
    tokens:  &'a [Token],
    pos:     usize,
    errors:  Vec<Diagnostic>,
    session: &'a Session,
}

impl<'a> Parser<'a> {
    // Peek at current token without consuming
    fn peek(&self) -> &Token { &self.tokens[self.pos] }

    // Consume current token and advance
    fn advance(&mut self) -> &Token { ... }

    // Consume and assert token kind; emit error if mismatch
    fn expect(&mut self, kind: TokenKind) -> Result<&Token, Diagnostic> { ... }

    // Try to consume; return None if no match (no error)
    fn eat(&mut self, kind: TokenKind) -> Option<&Token> { ... }

    // Check without consuming
    fn check(&self, kind: TokenKind) -> bool { ... }

    // Entry points
    pub fn parse_file(&mut self) -> AstFile { ... }
    fn parse_item(&mut self) -> AstItem { ... }
    fn parse_stmt(&mut self) -> AstStmt { ... }
    fn parse_expr(&mut self, min_prec: u8) -> AstExpr { ... }  // Pratt
    fn parse_type(&mut self) -> AstType { ... }
    fn parse_block(&mut self) -> AstBlock { ... }
}
```

### 4.2 Pratt Parser for Expressions

The Pratt parser handles operator precedence elegantly using **binding powers** (from the spec §67.9):

```rust
// precedence.rs
fn infix_binding_power(op: &TokenKind) -> Option<(u8, u8)> {
    // (left_bp, right_bp)
    // right_bp > left_bp = right-associative
    // right_bp < left_bp = left-associative
    match op {
        // precedence 0 — right-assoc
        TokenKind::PipeGt           => Some((1, 0)),
        TokenKind::QuestionQuestion => Some((1, 0)),

        // precedence 1 — left-assoc
        TokenKind::Keyword(Keyword::Or) => Some((2, 3)),

        // precedence 2
        TokenKind::Keyword(Keyword::And) => Some((4, 5)),

        // precedence 3 — non-assoc (no right bp)
        TokenKind::EqEq | TokenKind::BangEq
        | TokenKind::Lt  | TokenKind::Gt
        | TokenKind::LtEq| TokenKind::GtEq
        | TokenKind::Keyword(Keyword::Is) => Some((6, 0)),

        // ... fill in all operators per spec §67.9 ...

        // precedence 11 — left-assoc
        TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Some((22, 23)),

        // precedence 12 — right-assoc
        TokenKind::StarStar => Some((24, 25)),

        _ => None,
    }
}

fn parse_expr(parser: &mut Parser, min_bp: u8) -> AstExpr {
    // parse left-hand side (prefix / atom)
    let mut lhs = parse_unary_or_primary(parser);

    loop {
        let op = parser.peek();
        let Some((left_bp, right_bp)) = infix_binding_power(&op.kind) else { break };

        if left_bp < min_bp { break }

        parser.advance();                          // consume operator
        let rhs = parse_expr(parser, right_bp);   // recursively parse rhs
        lhs = AstExpr::BinOp { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
    }

    lhs
}
```

### 4.3 Error Recovery

The parser should continue after errors to report as many issues as possible.

**Synchronization points** (where to restart after an error):
- Top-level: `fn`, `struct`, `enum`, `interface`, `const`, `var`, `import`
- Statement level: next `NEWLINE` at current indent level
- Expression level: `,`, `)`, `]`, `}`, `NEWLINE`

```rust
fn synchronize_to_statement(&mut self) {
    loop {
        match self.peek().kind {
            TokenKind::Keyword(Keyword::Fn)
            | TokenKind::Keyword(Keyword::Struct)
            | TokenKind::Keyword(Keyword::Var)
            | TokenKind::Eof => return,
            TokenKind::Newline => { self.advance(); return }
            _ => { self.advance(); }
        }
    }
}
```

---

## 5. Phase 3 — AST

The AST represents the syntactic structure of the program. Every node carries a `Span` for error reporting.

### 5.1 Key Node Types

```rust
// nodes.rs

// Top-level file
pub struct AstFile {
    pub module:  AstModuleDecl,
    pub imports: Vec<AstImport>,
    pub items:   Vec<AstItem>,
    pub span:    Span,
}

// Items (top-level declarations)
pub enum AstItem {
    Fn(AstFnDecl),
    Struct(AstStructDecl),
    Enum(AstEnumDecl),
    Union(AstUnionDecl),
    Interface(AstInterfaceDecl),
    Impl(AstImplDecl),
    TypeAlias(AstTypeAlias),
    Newtype(AstNewtype),
    Const(AstConst),
    Var(AstVar),
    Extern(AstExtern),
}

// Function declaration
pub struct AstFnDecl {
    pub name:       AstIdent,
    pub generics:   Vec<AstGenericParam>,
    pub params:     Vec<AstParam>,
    pub return_ty:  Option<AstType>,
    pub body:       Option<AstBlock>,   // None = extern/header
    pub attrs:      Vec<AstAttribute>,
    pub is_pub:     bool,
    pub is_async:   bool,
    pub is_const:   bool,
    pub span:       Span,
}

// Statements
pub enum AstStmt {
    Var(AstVarDecl),
    Assign(AstAssign),
    Expr(AstExpr),
    Return(Option<AstExpr>, Span),
    Break(Option<AstLabel>, Span),
    Continue(Option<AstLabel>, Span),
    Defer(Box<AstStmt>, Span),
    If(AstIf),
    While(AstWhile),
    For(AstFor),
    Loop(AstLoop),
    Match(AstMatch),
    Try(AstTry),
    Unsafe(AstBlock, Span),
    Asm(AstAsm),
}

// Expressions
pub enum AstExpr {
    Int(u128, Option<IntSuffix>, Span),
    Float(f64, Option<FloatSuffix>, Span),
    String(String, Span),
    FString(Vec<FStringPart>, Span),
    Bool(bool, Span),
    Null(Span),
    Ident(AstIdent),
    BinOp  { op: BinOp,  lhs: Box<AstExpr>, rhs: Box<AstExpr>, span: Span },
    UnaryOp{ op: UnaryOp, operand: Box<AstExpr>, span: Span },
    Call   { callee: Box<AstExpr>, args: Vec<AstArg>, span: Span },
    Index  { base: Box<AstExpr>, index: Box<AstExpr>, span: Span },
    Slice  { base: Box<AstExpr>, lo: Option<Box<AstExpr>>, hi: Option<Box<AstExpr>>, span: Span },
    Field  { base: Box<AstExpr>, field: AstIdent, span: Span },
    MethodCall { base: Box<AstExpr>, method: AstIdent, args: Vec<AstArg>, span: Span },
    Cast   { expr: Box<AstExpr>, ty: AstType, kind: CastKind, span: Span },
    If     { cond: Box<AstExpr>, then: Box<AstExpr>, elifs: Vec<(AstExpr, AstExpr)>, else_: Box<AstExpr>, span: Span },
    Match  (AstMatchExpr),
    Lambda (AstLambda),
    StructLit { ty: AstType, fields: Vec<AstFieldInit>, update: Option<Box<AstExpr>>, span: Span },
    ArrayLit  { elements: Vec<AstExpr>, span: Span },
    TupleLit  { elements: Vec<AstExpr>, span: Span },
    MapLit    { entries: Vec<(AstExpr, AstExpr)>, span: Span },
    Range  { lo: Box<AstExpr>, hi: Box<AstExpr>, inclusive: bool, span: Span },
    Propagate { expr: Box<AstExpr>, span: Span },   // ? operator
    Builtin { name: String, ty_arg: Option<AstType>, args: Vec<AstExpr>, span: Span },
}

// Types
pub enum AstType {
    Primitive(PrimType, Span),
    Path(Vec<String>, Option<Vec<AstType>>, Span),  // name + generic args
    Pointer { mutable: bool, inner: Box<AstType>, span: Span },
    Optional(Box<AstType>, Span),
    Slice(Box<AstType>, Span),
    Array { size: Box<AstExpr>, elem: Box<AstType>, span: Span },
    Tuple(Vec<AstType>, Span),
    FnType { params: Vec<AstType>, ret: Box<AstType>, span: Span },
    Map { key: Box<AstType>, val: Box<AstType>, span: Span },
    Ref { lifetime: Option<String>, mutable: bool, inner: Box<AstType>, span: Span },
    Never(Span),
    Void(Span),
    Any(Span),
    Infer(Span),    // _ : to be inferred
}
```

---

## 6. Phase 4 — Name Resolution

Name resolution walks the AST and resolves every identifier to a definition.

### 6.1 Scope Stack

```rust
pub struct Scope {
    pub kind:    ScopeKind,   // Module | Fn | Block | Loop | Struct | etc.
    pub symbols: HashMap<String, SymbolId>,
}

pub struct SymbolTable {
    pub scopes:  Vec<Scope>,
    pub symbols: Vec<Symbol>,   // indexed by SymbolId
}

pub struct Symbol {
    pub id:        SymbolId,
    pub name:      String,
    pub kind:      SymbolKind,  // Var | Fn | Type | Const | Module | ...
    pub ty:        Option<TypeId>,   // filled in by type checker
    pub span:      Span,
    pub is_pub:    bool,
    pub module:    ModuleId,
}
```

### 6.2 Resolution Order

Name resolution happens in **two passes** to handle forward references:

```
Pass 1 — Collect declarations:
  For each file in the module:
    - Register all top-level fn, struct, enum, interface, type, const, var
    - Build import map (resolve module paths → ModuleId)
    - Do NOT resolve function bodies yet

Pass 2 — Resolve bodies:
  For each file:
    For each function/const/var body:
      - Walk the AST top-down
      - For each identifier:
          1. Search current scope stack (innermost first)
          2. Search module scope
          3. Search imported modules
          4. If not found: emit E0300 / E0301 / E0302
      - For each new binding (var, let, param):
          - Add to current scope
          - Check for shadowing → emit W0006
```

### 6.3 Import Resolution

```
import "std/io"           → find module at stdlib_path/io.gpl
import "../utils/math"    → find module at relative_path/utils/math.gpl
import <vendor/raylib>    → find module at vendor_path/raylib.gpl
import "std/io": {println} → import only println into current scope
use Direction.*           → import all variants of Direction enum
```

---

## 7. Phase 5 — Type Checker

The type checker annotates every AST node with its type and verifies correctness.

### 7.1 Type Representation

```rust
pub enum Ty {
    // Primitives
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
    F32, F64,
    Bool, Byte, Rune, Usize, Isize,
    Str,            // fat pointer: (*u8, usize)
    Void,
    Never,
    Any,

    // Compound
    Pointer   { mutable: bool, inner: TyId },
    Optional  (TyId),
    Slice     (TyId),
    Array     { size: u64, elem: TyId },
    Tuple     (Vec<TyId>),
    FnPtr     { params: Vec<TyId>, ret: TyId, is_async: bool },
    Map       { key: TyId, val: TyId },
    Ref       { lifetime: LifetimeId, mutable: bool, inner: TyId },

    // User-defined
    Struct    { id: DefId, generics: Vec<TyId> },
    Enum      { id: DefId, generics: Vec<TyId> },
    Union     { id: DefId },
    Interface { id: DefId, generics: Vec<TyId> },
    Newtype   { id: DefId, inner: TyId },

    // Special
    Infer(InferVarId),    // unresolved type variable (filled in by inference)
    Error,                // propagated from earlier errors (suppress cascades)
}
```

### 7.2 Type Inference (Hindley-Milner)

G uses a variant of **Algorithm W** (Hindley-Milner) for type inference.

```
Key operations:
1. fresh()    → create a new unification variable (Ty::Infer(id))
2. unify(a, b) → make a and b the same type
   - If both are concrete: check structural equality
   - If one is Infer(id): bind id to the other type
   - If both are Infer: merge equivalence classes (union-find)
3. apply(ty)  → substitute all resolved Infer variables in ty
4. generalize(ty) → turn unresolved Infer vars into generic params
5. instantiate(scheme) → replace generic params with fresh Infer vars
```

Key inference rules:

```
Var rule:
  If x : T is in scope, then x has type T.

Fn rule:
  fn f(x: T) -> R: body
  Infer body with x:T in scope.
  Unify inferred body type with R.

Call rule:
  f(arg1, arg2)
  If f : fn(T1, T2) -> R, unify arg1:T1, arg2:T2.
  Expression has type R.

Let rule:
  var x := expr
  Infer type of expr → T.
  Bind x : T in scope.

If-else expression:
  if cond: then_expr else: else_expr
  Unify type(then_expr) with type(else_expr) → T.
  Expression has type T.

Match expression:
  All arms must have the same type (unified).
```

### 7.3 Generic Instantiation

When a generic function is called, the compiler **monomorphizes** it:

```
fn max[T: Comparable](a: T, b: T) -> T

max(3, 7)         → max_i32(a: i32, b: i32) -> i32
max(3.14, 2.71)   → max_f64(a: f64, b: f64) -> f64
max("a", "b")     → max_str(a: str, b: str) -> str
```

Monomorphization is **lazy** — only generate concrete instances that are actually called.

### 7.4 Interface Checking

When `impl Display for MyType`, verify:
1. All required methods are present
2. Method signatures match the interface definition exactly
3. Associated types are specified (if any)
4. No extra methods (warn W0003 if unexpectedly unused)

---

## 8. Phase 6 — Borrow Checker

The borrow checker verifies ownership, moves, and borrows.
It operates on a **Control Flow Graph (CFG)** of the function body.

### 8.1 Overview: NLL (Non-Lexical Lifetimes)

G uses **NLL** — lifetimes end at the last use of a borrow, not at the end of the syntactic scope.

```
Algorithm (simplified):
1. Build CFG for the function body.
2. For each variable, compute "live range" using liveness analysis.
3. For each borrow, compute "borrow region" using region inference.
4. Check borrow rules:
   a. No two mutable borrows of same place overlap in liveness.
   b. No mutable borrow overlaps with any other borrow of same place.
   c. No use of moved variable after the move point.
   d. No borrow of a value that has been moved.
   e. No borrow outlives the borrowed value.
```

### 8.2 Places

A **place** is an lvalue — something that can be borrowed or moved:

```rust
pub enum Place {
    Local(LocalId),                    // local variable
    Field(Box<Place>, FieldId),        // place.field
    Index(Box<Place>, Box<Operand>),   // place[index]
    Deref(Box<Place>),                 // *place
}
```

### 8.3 Move Tracking

Track "definitely initialized" and "definitely moved" sets per CFG node:

```
For each basic block:
  in_set  = ∩ out_set of predecessors  (intersection for definite init)
  out_set = (in_set - killed) ∪ gen

killed = set of places moved/dropped in this block
gen    = set of places initialized/assigned in this block

Error E0200: if a place is used but is in the "moved" set.
Error E0201: if a place is moved but is currently borrowed.
```

### 8.4 Borrow Regions

Each borrow `&x` gets a **region** — the set of CFG points where the borrow is live.

```
Constraint generation:
- At the point of borrow `r = &x`, add constraint: r ⊇ {current point}
- At each use of r, add constraint: r ⊇ {use point}
- At the end of borrow's lifetime, end the region.

Region inference: solve constraints using a worklist algorithm.
  - Start with each region = empty
  - For each constraint r ⊇ S, add all points in S to r
  - Propagate through CFG edges
  - Repeat until fixpoint
```

---

## 9. Phase 7 — IR (Intermediate Representation)

GIR (G Intermediate Representation) is a **3-address SSA** (Static Single Assignment) form.

### 9.1 GIR Structure

```rust
pub struct GirModule {
    pub name:      String,
    pub functions: Vec<GirFn>,
    pub globals:   Vec<GirGlobal>,
    pub types:     TypeArena,
}

pub struct GirFn {
    pub name:     String,           // mangled name
    pub params:   Vec<GirLocal>,
    pub ret_ty:   TyId,
    pub locals:   Vec<GirLocal>,    // all local variables (SSA values)
    pub blocks:   Vec<GirBlock>,    // basic blocks
    pub entry:    BlockId,
}

pub struct GirBlock {
    pub id:    BlockId,
    pub instrs: Vec<GirInstr>,
    pub term:   GirTerminator,      // how the block ends
}

pub enum GirInstr {
    // Assignments
    Assign   { dst: Local, src: Operand },
    BinOp    { dst: Local, op: BinOp, lhs: Operand, rhs: Operand },
    UnaryOp  { dst: Local, op: UnaryOp, operand: Operand },
    Call     { dst: Option<Local>, func: Operand, args: Vec<Operand> },
    // Memory
    Alloca   { dst: Local, ty: TyId },          // stack allocation
    Load     { dst: Local, ptr: Operand },
    Store    { ptr: Operand, val: Operand },
    GetField { dst: Local, base: Operand, field: FieldId },
    SetField { base: Operand, field: FieldId, val: Operand },
    GetIndex { dst: Local, base: Operand, idx: Operand },
    // Casts
    Cast     { dst: Local, src: Operand, to: TyId, kind: CastKind },
    // Volatile (MMIO)
    VolatileLoad  { dst: Local, ptr: Operand, ty: TyId },
    VolatileStore { ptr: Operand, val: Operand, ty: TyId },
    // Atomics
    AtomicLoad  { dst: Local, ptr: Operand, order: Ordering },
    AtomicStore { ptr: Operand, val: Operand, order: Ordering },
    AtomicRmw   { dst: Local, op: AtomicOp, ptr: Operand, val: Operand, order: Ordering },
    AtomicCas   { dst: Local, ptr: Operand, expected: Operand, new: Operand, succ: Ordering, fail: Ordering },
    // Misc
    Phi      { dst: Local, incoming: Vec<(Operand, BlockId)> },  // SSA φ-function
    Nop,
    Asm      { template: String, outputs: Vec<AsmOperand>, inputs: Vec<AsmOperand>, clobbers: Vec<String> },
}

pub enum GirTerminator {
    Return(Option<Operand>),
    Jump(BlockId),
    Branch { cond: Operand, then_: BlockId, else_: BlockId },
    Switch { val: Operand, cases: Vec<(u64, BlockId)>, default: BlockId },
    Unreachable,
    Panic { msg: String },     // for debug bounds checks etc.
}
```

### 9.2 Lowering from Typed AST to GIR

Key lowerings:

```
match expr → switch + phi nodes
for loop   → while loop with iterator.next() call
?  operator → check Result/Option, branch on Err/None, extract value
defer      → collect all defers, emit at all exit points of the scope
drop       → insert Drop::drop() calls at end of each value's live range
closures   → lower to struct (captures) + fn pointer
async fn   → lower to state machine (coroutine transform)
```

---

## 10. Phase 8 — Optimizations

Optimizations run on GIR before code generation.

### 10.1 Mandatory Passes (always run)

```
1. mem2reg
   Promote `Alloca` + `Load`/`Store` pairs into SSA values with Phi nodes.
   This is the key pass that enables all other optimizations.

2. Constant Folding
   Replace operations on constants with their results at compile time.
   Example: BinOp(Add, Const(2), Const(3)) → Const(5)

3. Dead Code Elimination (DCE)
   Remove instructions whose results are never used.
   Remove unreachable basic blocks.

4. Copy Propagation
   Replace uses of a variable with its definition if it's a simple copy.
   Example: x = y; z = x + 1 → z = y + 1
```

### 10.2 Optional Passes (release builds)

```
5. Inlining
   Replace call sites with the function body for small functions.
   Threshold: functions with <= 30 GIR instructions (configurable).

6. Loop-Invariant Code Motion (LICM)
   Move loop-invariant computations out of loops.

7. Scalar Replacement of Aggregates (SROA)
   Split struct values into individual scalar fields to enable
   more register allocation opportunities.

8. Common Subexpression Elimination (CSE)
   Detect repeated computations and reuse the result.

9. Tail Call Optimization
   Convert tail-recursive calls into jumps.

10. Devirtualization
    If an interface call can be statically resolved, convert to direct call.
```

---

## 11. Phase 9 — Code Generation

### 11.1 LLVM Backend (Recommended)

Use **LLVM** via `llvm-sys` (Rust bindings) or `inkwell` (safe Rust wrapper).

```rust
// codegen/llvm.rs (pseudocode)
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::Module;

pub struct LlvmCodegen<'ctx> {
    ctx:     &'ctx Context,
    module:  Module<'ctx>,
    builder: Builder<'ctx>,
    // maps GIR locals to LLVM values
    locals:  HashMap<Local, BasicValueEnum<'ctx>>,
}

impl<'ctx> LlvmCodegen<'ctx> {
    pub fn emit_module(&mut self, gir: &GirModule) {
        for func in &gir.functions {
            self.emit_fn(func);
        }
    }

    fn emit_fn(&mut self, func: &GirFn) {
        // declare LLVM function
        let llvm_fn = self.module.add_function(
            &func.name,
            self.fn_type(&func),
            None,
        );
        // emit each basic block
        for block in &func.blocks {
            self.emit_block(llvm_fn, block);
        }
    }

    fn emit_instr(&mut self, instr: &GirInstr) -> BasicValueEnum {
        match instr {
            GirInstr::BinOp { op: BinOp::Add, lhs, rhs, .. } =>
                self.builder.build_int_add(
                    self.operand(lhs).into_int_value(),
                    self.operand(rhs).into_int_value(),
                    "add",
                ).into(),
            // ... all other instructions
        }
    }
}
```

### 11.2 ABI Implementation

Follow §69 of the spec exactly:

```rust
// abi.rs
pub fn classify_argument(ty: &Ty, target: &Target) -> ArgLocation {
    match ty {
        // integers and pointers → integer registers
        Ty::I8 | Ty::I16 | Ty::I32 | Ty::I64
        | Ty::Pointer { .. } | Ty::Usize => ArgLocation::Register(IntReg),

        // floats → float registers
        Ty::F32 | Ty::F64 => ArgLocation::Register(FloatReg),

        // small structs (≤ 16 bytes) → up to 2 registers
        Ty::Struct { .. } if size_of(ty, target) <= 16 => ArgLocation::RegisterPair,

        // large structs → pass by pointer (hidden parameter)
        Ty::Struct { .. } => ArgLocation::ByPointer,

        // ...
    }
}
```

### 11.3 Name Mangling

Implement exactly per §69.6:

```rust
// mangle.rs
pub fn mangle(module: &str, name: &str, sig: &FnSignature) -> String {
    let module_mangled = module.replace('.', "__");
    let type_hash = compute_type_hash(sig);
    format!("_G_{}_{}_{:08x}", module_mangled, name, type_hash)
}

fn compute_type_hash(sig: &FnSignature) -> u32 {
    let mut hasher = FnvHasher::default();
    // hash each parameter type and return type
    for param in &sig.params {
        hash_type(&mut hasher, &param.ty);
    }
    hash_type(&mut hasher, &sig.ret);
    hasher.finish() as u32
}
```

---

## 12. Phase 10 — Linker Integration

### 12.1 Linking

After code generation, invoke the system linker:

```rust
// For Linux:
Command::new("ld")
    .args(["-o", &output_path])
    .args(object_files)
    .args(["-lc", "-lm"])         // always link libc, libm
    .args(user_link_flags)        // from @link() attributes
    .arg("--gc-sections")         // remove unused sections
    .spawn()?;

// Or via clang/gcc as linker driver (handles crtbegin/crtend etc.):
Command::new("cc")
    .args(["-o", &output_path])
    .args(object_files)
    .arg("-lgplrt")               // G runtime library
    .args(user_link_flags)
    .spawn()?;
```

### 12.2 For Kernel / No-std Targets

```rust
// Use ld directly with linker script
Command::new("ld")
    .args(["-T", &linker_script])
    .args(["-o", &output_path])
    .args(object_files)
    .args(["--gc-sections", "-n"])  // -n: no page alignment of sections
    .spawn()?;
```

---

## 13. Standard Library Implementation

### 13.1 Layering

```
core/           ← no OS, no heap, no threading
  mem.gpl       ← memcpy, memset, memcmp (calls compiler intrinsics)
  atomic.gpl    ← atomic types (calls compiler intrinsics)
  volatile.gpl  ← volatile read/write (calls compiler intrinsics)
  fmt.gpl       ← formatting into fixed buffers

std/            ← builds on core + OS
  io.gpl        ← wraps OS file descriptors
  fs.gpl        ← wraps OS filesystem calls
  mem/          ← heap allocators (wraps malloc or kernel allocator)
    global.gpl
    arena.gpl
    pool.gpl
  thread.gpl    ← wraps pthreads / Win32 threads / OS threads
  ...
```

### 13.2 Compiler Intrinsics

Some `core` operations need compiler intrinsics (cannot be implemented in G itself):

```gpl
# These are handled specially by the compiler:
@intrinsic("memcpy")
fn __memcpy(dst: *mut u8, src: *const u8, n: usize) -> void

@intrinsic("memset")
fn __memset(dst: *mut u8, val: u8, n: usize) -> void

@intrinsic("atomic_load_seqcst")
fn __atomic_load_u64(ptr: *const u64) -> u64

@intrinsic("overflow_add_i32")
fn __overflow_add_i32(a: i32, b: i32) -> (i32, bool)

@intrinsic("llvm.sqrt.f64")
fn __sqrt_f64(x: f64) -> f64

@intrinsic("llvm.trap")
fn __trap() -> never
```

---

## 14. Error Reporting

Every diagnostic must follow this structure (per spec §37.13):

```rust
pub struct Diagnostic {
    pub code:     ErrorCode,     // E0042, W0003, etc.
    pub severity: Severity,      // Error | Warning | Note | Help
    pub message:  String,
    pub primary:  Label,         // main span + message
    pub secondary: Vec<Label>,   // additional context spans
    pub notes:    Vec<String>,   // = note: ... lines
    pub hints:    Vec<String>,   // = hint: ... lines
    pub see:      Option<String>,// = see: spec link
}

pub struct Label {
    pub span:    Span,
    pub message: String,
}

// Rendering:
// error[E0042]: type mismatch
//   --> src/main.gpl:15:12
//    |
// 14 |     var id: UserId = 42
//    |             ------   ^^ expected UserId, found i32
//    |             |
//    |             declared as UserId here
//    |
//    = hint: wrap with UserId(42) to construct a UserId
//    = see: https://gpl-lang.org/spec/§22.2
```

**Rules:**
- Never emit a diagnostic without a span.
- Always emit `E0000` level errors in a single pass — collect all errors, report all at once.
- After a type error, substitute `Ty::Error` to suppress cascade errors.
- Warnings are opt-out per code via `#[allow(W0001)]` or `--W none`.

---

## 15. Testing the Compiler

### 15.1 Unit Tests

Each phase has its own unit tests:

```rust
// tests/lexer/test_basic.rs
#[test]
fn test_lex_integer_literals() {
    assert_tokens("42",         &[Token::IntLit(42, None)]);
    assert_tokens("0xFF",       &[Token::IntLit(255, None)]);
    assert_tokens("0b1010",     &[Token::IntLit(10, None)]);
    assert_tokens("1_000_000",  &[Token::IntLit(1000000, None)]);
    assert_tokens("100u8",      &[Token::IntLit(100, Some(IntSuffix::U8))]);
}

#[test]
fn test_indent_dedent() {
    let src = "if x:\n    y\nz";
    assert_tokens(src, &[
        Keyword(Keyword::If), Ident("x"), Colon, Newline,
        Indent, Ident("y"), Newline, Dedent,
        Ident("z"), Eof,
    ]);
}
```

### 15.2 Compiler Test Suite (UI Tests)

For type errors, borrow errors, and behavior tests, use **UI tests** — `.gpl` files with
expected output in comments:

```gpl
# tests/ui/E0200_use_after_move.gpl

var a := Buffer.new(64)
var b := a
var c := a    # Error: use after move

# EXPECTED:
# error[E0200]: use of moved value 'a'
#   --> tests/ui/E0200_use_after_move.gpl:4:10
```

```gpl
# tests/ui/ok_simple_add.gpl

fn add(a: i32, b: i32) -> i32:
    return a + b

fn main() -> i32:
    return add(2, 3)

# EXPECTED_OUTPUT: (exit code 0)
```

### 15.3 Conformance Tests

The spec defines expected behavior for all edge cases. There should be a
**conformance test suite** with one test per spec section that can be run
against any G implementation to verify compliance.

```
tests/conformance/
├── 04_types/
│   ├── integer_overflow_debug.gpl
│   ├── integer_overflow_release.gpl
│   └── float_nan.gpl
├── 08_functions/
│   ├── multiple_return.gpl
│   ├── default_params.gpl
│   └── variadic.gpl
├── 68_behavior/
│   ├── eval_order.gpl
│   ├── drop_order.gpl
│   └── closure_capture.gpl
└── ...
```

---

## 16. Bootstrap Strategy

G's compiler will be written in G itself eventually. The bootstrap path:

```
Stage 0: Write gplc0 in Rust (or C)
  - Full compiler for a subset of G
  - Target: x86_64 Linux only
  - No std library (just core)
  - Goal: compile a minimal G program to a working binary

Stage 1: Port gplc0 to G (using the subset)
  - Write gplc1.gpl using only features supported by gplc0
  - Compile gplc1.gpl using gplc0
  - gplc1 should produce identical output to gplc0 on the same input

Stage 2: Full G compiler in G
  - Expand gplc1 to support all G features
  - Compile gplc2.gpl using gplc1
  - gplc2 compiled by gplc1 should produce the same binary as
    gplc2 compiled by gplc2 (self-hosting check)

Stage 3: Remove Rust/C dependency
  - Build system now uses gplc2 to compile gplc
  - Rust/C only needed to build Stage 0 from scratch (reproducible builds)
```

---

## 17. Recommended Starting Point

If you are starting from scratch, here is the recommended order:

```
Week 1-2:  Lexer
  - Implement all token types
  - Handle indentation (INDENT/DEDENT)
  - Test with all literal types
  - Test with all operators
  - Handle f-strings

Week 3-4:  Parser
  - Parse top-level items (fn, struct, enum)
  - Parse statements (var, if, while, for, match, return)
  - Implement Pratt parser for expressions
  - Produce a valid AST + pretty-print it

Week 5-6:  Name Resolution
  - Build scope stack
  - Resolve all identifiers
  - Handle imports
  - Report E03xx errors

Week 7-9:  Type Checker
  - Implement type representation
  - Implement basic inference (literals, arithmetic)
  - Implement function call type checking
  - Implement struct field access
  - Implement match exhaustiveness

Week 10-11: Code Generation (simple)
  - Lower basic functions to LLVM IR
  - Handle integer arithmetic
  - Handle function calls
  - Handle if/while/for
  - Get "Hello, World!" compiling and running

Week 12+:  Iterate
  - Add borrow checker
  - Add generics
  - Add standard library
  - Add remaining features
```

**First milestone — "Hello, World!" compiles and runs:**

```gpl
module main
import "std/io"
fn main() -> i32:
    io.println("Hello, World!")
    return 0
```

This requires: lexer, parser, name resolution, basic type checking, LLVM codegen,
and a minimal `std/io` that wraps the `write` syscall. Everything else can come later.

---

## Resources

- **G Language Spec:** [`docs/spec/G_Language_Spec_v1.0.0.md`](docs/spec/G_Language_Spec_v1.0.0.md)
- **LLVM Tutorial:** https://llvm.org/docs/tutorial/
- **inkwell (Rust LLVM bindings):** https://github.com/TheDan64/inkwell
- **Crafting Interpreters** (free book): https://craftinginterpreters.com
- **Engineering a Compiler** (book): Keith Cooper & Linda Torczon
- **Modern Compiler Implementation in ML** (book): Andrew Appel
- **Pratt Parsers:** https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
- **NLL (Non-Lexical Lifetimes):** https://rust-lang.github.io/rfcs/2094-nll.html
- **SSA Form:** https://en.wikipedia.org/wiki/Static_single-assignment_form
- **Hindley-Milner:** https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system

---

*G Compiler Implementation Guide v1.0.0*  
*Maintained by the G language team.*  
*Open an issue if anything is unclear or incorrect.*
