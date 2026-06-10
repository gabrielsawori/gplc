/// Type checking pass.
///
/// COMPILER.md §7.2 – §7.4
///
/// Walks the resolved AST, infers types for all expressions, and checks
/// that all type constraints are satisfied.

use crate::ast::nodes::*;
use crate::error::{Diagnostic, ErrorCode, Severity};
use crate::lexer::token::Span;
use super::ty::*;
use super::unify::Unifier;

/// The type checker maintains a type arena, a unifier, and diagnostics.
pub struct TypeChecker {
    pub arena:       TypeArena,
    pub unifier:     Unifier,
    pub diagnostics: Vec<Diagnostic>,

    // Pre-interned common types
    ty_i32:   TyId,
    ty_i64:   TyId,
    ty_f64:   TyId,
    ty_bool:  TyId,
    ty_str:   TyId,
    ty_void:  TyId,
    ty_never: TyId,
    ty_error: TyId,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut arena = TypeArena::new();
        let ty_i32   = arena.intern(Ty::I32);
        let ty_i64   = arena.intern(Ty::I64);
        let ty_f64   = arena.intern(Ty::F64);
        let ty_bool  = arena.intern(Ty::Bool);
        let ty_str   = arena.intern(Ty::Str);
        let ty_void  = arena.intern(Ty::Void);
        let ty_never = arena.intern(Ty::Never);
        let ty_error = arena.intern(Ty::Error);

        Self {
            arena, unifier: Unifier::new(), diagnostics: Vec::new(),
            ty_i32, ty_i64, ty_f64, ty_bool, ty_str, ty_void, ty_never, ty_error,
        }
    }

    /// Run type checking on a parsed file.
    pub fn check(&mut self, file: &File) {
        for item in &file.items {
            self.check_item(item);
        }
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == Severity::Error)
    }

    // ── Items ─────────────────────────────────────────────────────────────

    fn check_item(&mut self, item: &Item) {
        match &item.kind {
            ItemKind::Fn(f)        => self.check_fn(f),
            ItemKind::Const(c)     => { self.infer_expr(&c.value); }
            ItemKind::Var(v)       => self.check_var_decl(v),
            ItemKind::ImplBlock(b) => {
                for m in &b.methods { self.check_fn(m); }
            }
            ItemKind::ImplFor(imp) => {
                for m in &imp.methods { self.check_fn(m); }
            }
            _ => {} // struct/enum/interface declarations don't need body checking
        }
    }

    fn check_fn(&mut self, f: &FnDecl) {
        let ret_ty = f.ret_ty.as_ref()
            .map(|t| self.lower_type(t))
            .unwrap_or(self.ty_void);

        if let Some(body) = &f.body {
            self.check_block(body, ret_ty);
        }
    }

    fn check_block(&mut self, block: &Block, expected_ret: TyId) {
        for stmt in &block.stmts {
            self.check_stmt(stmt, expected_ret);
        }
    }

    // ── Statements ────────────────────────────────────────────────────────

    fn check_stmt(&mut self, stmt: &Stmt, expected_ret: TyId) {
        match &stmt.kind {
            StmtKind::Var(v) => self.check_var_decl(v),
            StmtKind::Assign { target, value, .. } => {
                let lhs = self.infer_expr(target);
                let rhs = self.infer_expr(value);
                if self.unifier.unify(lhs, rhs, &self.arena).is_err() {
                    self.emit_mismatch(lhs, rhs, value.span);
                }
            }
            StmtKind::Expr(e) => { self.infer_expr(e); }
            StmtKind::Return(Some(e)) => {
                let ty = self.infer_expr(e);
                if self.unifier.unify(ty, expected_ret, &self.arena).is_err() {
                    self.diagnostics.push(
                        Diagnostic::error(ErrorCode::E0408,
                            format!("return type mismatch: expected `{}`, found `{}`",
                                self.arena.get(expected_ret), self.arena.get(ty)),
                            e.span)
                    );
                }
            }
            StmtKind::Return(None) => {
                if self.unifier.unify(self.ty_void, expected_ret, &self.arena).is_err() {
                    self.diagnostics.push(
                        Diagnostic::error(ErrorCode::E0408,
                            "missing return value for non-void function",
                            stmt.span)
                    );
                }
            }
            StmtKind::If(ifs) => {
                let cond_ty = self.infer_expr(&ifs.cond);
                if self.unifier.unify(cond_ty, self.ty_bool, &self.arena).is_err() {
                    self.diagnostics.push(
                        Diagnostic::error(ErrorCode::E0400,
                            "condition must be `bool`", ifs.cond.span)
                    );
                }
                self.check_block(&ifs.then, expected_ret);
                for (c, b) in &ifs.elifs {
                    let ct = self.infer_expr(c);
                    if self.unifier.unify(ct, self.ty_bool, &self.arena).is_err() {
                        self.diagnostics.push(
                            Diagnostic::error(ErrorCode::E0400,
                                "elif condition must be `bool`", c.span)
                        );
                    }
                    self.check_block(b, expected_ret);
                }
                if let Some(e) = &ifs.else_ {
                    self.check_block(e, expected_ret);
                }
            }
            StmtKind::While { cond, body, .. } => {
                let ct = self.infer_expr(cond);
                if self.unifier.unify(ct, self.ty_bool, &self.arena).is_err() {
                    self.diagnostics.push(
                        Diagnostic::error(ErrorCode::E0400,
                            "while condition must be `bool`", cond.span)
                    );
                }
                self.check_block(body, expected_ret);
            }
            StmtKind::For { iter, body, .. } => {
                self.infer_expr(iter);
                self.check_block(body, expected_ret);
            }
            StmtKind::Loop { body, .. } => {
                self.check_block(body, expected_ret);
            }
            StmtKind::Match(m) => {
                self.infer_expr(&m.scrutinee);
                for arm in &m.arms {
                    if let Some(g) = &arm.guard { self.infer_expr(g); }
                    match &arm.body {
                        MatchBody::Expr(e)  => { self.infer_expr(e); }
                        MatchBody::Block(b) => self.check_block(b, expected_ret),
                    }
                }
            }
            StmtKind::Try(t) => {
                self.check_block(&t.body, expected_ret);
                for c in &t.catches { self.check_block(&c.body, expected_ret); }
                if let Some(f) = &t.finally { self.check_block(f, expected_ret); }
            }
            StmtKind::Defer(inner)   => self.check_stmt(inner, expected_ret),
            StmtKind::Unsafe(b)      => self.check_block(b, expected_ret),
            StmtKind::Comptime(inner) => self.check_stmt(inner, expected_ret),
            StmtKind::Break(_) | StmtKind::Continue(_) | StmtKind::Asm(_) => {}
        }
    }

    fn check_var_decl(&mut self, v: &VarDecl) {
        let ann_ty = v.ty.as_ref().map(|t| self.lower_type(t));

        if let Some(vals) = &v.value {
            for val in vals {
                let inferred = self.infer_expr(val);
                if let Some(expected) = ann_ty {
                    if self.unifier.unify(inferred, expected, &self.arena).is_err() {
                        self.emit_mismatch(expected, inferred, val.span);
                    }
                }
            }
        }
    }

    // ── Expression inference ──────────────────────────────────────────────

    fn infer_expr(&mut self, expr: &Expr) -> TyId {
        match &expr.kind {
            ExprKind::Int(_, suffix) => {
                match suffix {
                    Some(s) => self.intern_int_suffix(*s),
                    None    => self.ty_i32, // default integer type
                }
            }
            ExprKind::Float(_, _)    => self.ty_f64,
            ExprKind::Str(_)
            | ExprKind::RawStr(_)
            | ExprKind::FStr(_)      => self.ty_str,
            ExprKind::Char(_)        => self.arena.intern(Ty::Rune),
            ExprKind::Byte(_)        => self.arena.intern(Ty::Byte),
            ExprKind::Bool(_)        => self.ty_bool,
            ExprKind::Null           => {
                let inner = self.arena.fresh_infer();
                self.arena.intern(Ty::Optional(inner))
            }
            ExprKind::Ident(_)       => self.arena.fresh_infer(),
            ExprKind::Path(_)        => self.arena.fresh_infer(),

            ExprKind::BinOp { op, lhs, rhs } => {
                let lt = self.infer_expr(lhs);
                let rt = self.infer_expr(rhs);
                match op {
                    BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le
                    | BinOp::Gt | BinOp::Ge | BinOp::And | BinOp::Or => {
                        self.ty_bool
                    }
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div
                    | BinOp::Rem | BinOp::Pow => {
                        if self.unifier.unify(lt, rt, &self.arena).is_err() {
                            self.diagnostics.push(
                                Diagnostic::error(ErrorCode::E0409,
                                    "incompatible types in binary operation",
                                    expr.span)
                            );
                            self.ty_error
                        } else {
                            self.unifier.resolve(lt, &self.arena)
                        }
                    }
                    BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor
                    | BinOp::Shl | BinOp::Shr => {
                        let _ = self.unifier.unify(lt, rt, &self.arena);
                        self.unifier.resolve(lt, &self.arena)
                    }
                    BinOp::Range | BinOp::RangeInclusive => {
                        self.arena.fresh_infer()
                    }
                    BinOp::Pipe => rt,
                    BinOp::Coalesce => lt,
                    _ => {
                        let _ = self.unifier.unify(lt, rt, &self.arena);
                        self.unifier.resolve(lt, &self.arena)
                    }
                }
            }
            ExprKind::UnaryOp { op, operand } => {
                let ot = self.infer_expr(operand);
                match op {
                    UnaryOp::Neg | UnaryOp::BitNot => ot,
                    UnaryOp::Not => self.ty_bool,
                    UnaryOp::Deref => self.arena.fresh_infer(),
                    UnaryOp::AddrOf => {
                        self.arena.intern(Ty::Pointer { mutable: false, inner: ot })
                    }
                    UnaryOp::AddrOfMut => {
                        self.arena.intern(Ty::Pointer { mutable: true, inner: ot })
                    }
                }
            }
            ExprKind::Call { callee, args } => {
                let _callee_ty = self.infer_expr(callee);
                for a in args { self.infer_expr(&a.expr); }
                self.arena.fresh_infer()
            }
            ExprKind::Field { base, .. }
            | ExprKind::PtrField { base, .. }
            | ExprKind::OptField { base, .. } => {
                self.infer_expr(base);
                self.arena.fresh_infer()
            }
            ExprKind::Index { base, index } => {
                self.infer_expr(base);
                self.infer_expr(index);
                self.arena.fresh_infer()
            }
            ExprKind::Slice { base, lo, hi } => {
                self.infer_expr(base);
                if let Some(l) = lo { self.infer_expr(l); }
                if let Some(h) = hi { self.infer_expr(h); }
                self.arena.fresh_infer()
            }
            ExprKind::Array(elems) => {
                let elem_ty = if elems.is_empty() {
                    self.arena.fresh_infer()
                } else {
                    let first = self.infer_expr(&elems[0]);
                    for e in &elems[1..] {
                        let et = self.infer_expr(e);
                        if self.unifier.unify(first, et, &self.arena).is_err() {
                            self.diagnostics.push(
                                Diagnostic::error(ErrorCode::E0400,
                                    "mismatched types in array literal", e.span)
                            );
                        }
                    }
                    first
                };
                self.arena.intern(Ty::Slice(elem_ty))
            }
            ExprKind::Tuple(elems) => {
                let tys: Vec<TyId> = elems.iter()
                    .map(|e| self.infer_expr(e)).collect();
                self.arena.intern(Ty::Tuple(tys))
            }
            ExprKind::StructLit { fields, rest, .. } => {
                for f in fields {
                    if let Some(v) = &f.value { self.infer_expr(v); }
                }
                if let Some(r) = rest { self.infer_expr(r); }
                self.arena.fresh_infer()
            }
            ExprKind::Lambda { body, .. } => {
                match body {
                    LambdaBody::Expr(e)  => { self.infer_expr(e); }
                    LambdaBody::Block(b) => {
                        let ret = self.arena.fresh_infer();
                        self.check_block(b, ret);
                    }
                }
                self.arena.fresh_infer()
            }
            ExprKind::Cast { expr: inner, ty, .. } => {
                self.infer_expr(inner);
                self.lower_type(ty)
            }
            ExprKind::Is { expr: inner, .. } => {
                self.infer_expr(inner);
                self.ty_bool
            }
            ExprKind::Propagate(inner) => {
                self.infer_expr(inner);
                self.arena.fresh_infer()
            }
            ExprKind::Unwrap(inner) => {
                self.infer_expr(inner);
                self.arena.fresh_infer()
            }
            ExprKind::If { cond, then, elifs, else_ } => {
                let ct = self.infer_expr(cond);
                if self.unifier.unify(ct, self.ty_bool, &self.arena).is_err() {
                    self.diagnostics.push(
                        Diagnostic::error(ErrorCode::E0400,
                            "if-expression condition must be `bool`", cond.span)
                    );
                }
                let then_ty = self.infer_expr(then);
                for (c, e) in elifs {
                    self.infer_expr(c);
                    let et = self.infer_expr(e);
                    let _ = self.unifier.unify(then_ty, et, &self.arena);
                }
                let else_ty = self.infer_expr(else_);
                let _ = self.unifier.unify(then_ty, else_ty, &self.arena);
                self.unifier.resolve(then_ty, &self.arena)
            }
            ExprKind::Match { scrutinee, arms } => {
                self.infer_expr(scrutinee);
                let result_ty = self.arena.fresh_infer();
                for arm in arms {
                    if let Some(g) = &arm.guard { self.infer_expr(g); }
                    let arm_ty = match &arm.body {
                        MatchBody::Expr(e)  => self.infer_expr(e),
                        MatchBody::Block(_) => self.arena.fresh_infer(),
                    };
                    let _ = self.unifier.unify(result_ty, arm_ty, &self.arena);
                }
                self.unifier.resolve(result_ty, &self.arena)
            }
            ExprKind::Block(b) => {
                let ret = self.arena.fresh_infer();
                self.check_block(b, ret);
                self.arena.fresh_infer()
            }
            ExprKind::Range { lo, hi, .. } => {
                self.infer_expr(lo);
                self.infer_expr(hi);
                self.arena.fresh_infer()
            }
            ExprKind::Map(entries) => {
                for (k, v) in entries {
                    self.infer_expr(k);
                    self.infer_expr(v);
                }
                self.arena.fresh_infer()
            }
            ExprKind::Set(elems) => {
                for e in elems { self.infer_expr(e); }
                self.arena.fresh_infer()
            }
            ExprKind::OptCall { base, args, .. } => {
                self.infer_expr(base);
                for a in args { self.infer_expr(&a.expr); }
                self.arena.fresh_infer()
            }
            ExprKind::Builtin { args, .. } => {
                for a in args { self.infer_expr(a); }
                self.arena.fresh_infer()
            }
        }
    }

    // ── AST Type → Ty lowering ────────────────────────────────────────────

    fn lower_type(&mut self, ty: &Type) -> TyId {
        match ty {
            Type::Primitive(p, _) => self.lower_prim(*p),
            Type::Void(_)   => self.ty_void,
            Type::Never(_)  => self.ty_never,
            Type::Any(_)    => self.arena.intern(Ty::Any),
            Type::Infer(_)  => self.arena.fresh_infer(),
            Type::Pointer { is_const, inner, .. } => {
                let inner_id = self.lower_type(inner);
                self.arena.intern(Ty::Pointer { mutable: !is_const, inner: inner_id })
            }
            Type::Optional(inner, _) => {
                let inner_id = self.lower_type(inner);
                self.arena.intern(Ty::Optional(inner_id))
            }
            Type::Slice(inner, _) => {
                let inner_id = self.lower_type(inner);
                self.arena.intern(Ty::Slice(inner_id))
            }
            Type::Tuple(types, _) => {
                let tys: Vec<TyId> = types.iter().map(|t| self.lower_type(t)).collect();
                self.arena.intern(Ty::Tuple(tys))
            }
            Type::FnPtr { params, ret, is_async, .. } => {
                let ps: Vec<TyId> = params.iter().map(|t| self.lower_type(t)).collect();
                let r = self.lower_type(ret);
                self.arena.intern(Ty::FnPtr { params: ps, ret: r, is_async: *is_async })
            }
            Type::Ref { mutable, inner, .. } => {
                let inner_id = self.lower_type(inner);
                self.arena.intern(Ty::Ref { lifetime: 0, mutable: *mutable, inner: inner_id })
            }
            Type::Map { key, val, .. } => {
                let k = self.lower_type(key);
                let v = self.lower_type(val);
                self.arena.intern(Ty::Map { key: k, val: v })
            }
            Type::Set(inner, _) => {
                let inner_id = self.lower_type(inner);
                self.arena.intern(Ty::Set(inner_id))
            }
            // Path types, Array types — use fresh infer for now
            _ => self.arena.fresh_infer(),
        }
    }

    fn lower_prim(&mut self, p: PrimType) -> TyId {
        let ty = match p {
            PrimType::I8    => Ty::I8,
            PrimType::I16   => Ty::I16,
            PrimType::I32   => Ty::I32,
            PrimType::I64   => Ty::I64,
            PrimType::I128  => Ty::I128,
            PrimType::U8    => Ty::U8,
            PrimType::U16   => Ty::U16,
            PrimType::U32   => Ty::U32,
            PrimType::U64   => Ty::U64,
            PrimType::U128  => Ty::U128,
            PrimType::F32   => Ty::F32,
            PrimType::F64   => Ty::F64,
            PrimType::Bool  => Ty::Bool,
            PrimType::Byte  => Ty::Byte,
            PrimType::Rune  => Ty::Rune,
            PrimType::Str   => Ty::Str,
            PrimType::Usize => Ty::Usize,
            PrimType::Isize => Ty::Isize,
        };
        self.arena.intern(ty)
    }

    fn intern_int_suffix(&mut self, s: crate::lexer::token::IntSuffix) -> TyId {
        use crate::lexer::token::IntSuffix;
        let ty = match s {
            IntSuffix::I8    => Ty::I8,
            IntSuffix::I16   => Ty::I16,
            IntSuffix::I32   => Ty::I32,
            IntSuffix::I64   => Ty::I64,
            IntSuffix::I128  => Ty::I128,
            IntSuffix::U8    => Ty::U8,
            IntSuffix::U16   => Ty::U16,
            IntSuffix::U32   => Ty::U32,
            IntSuffix::U64   => Ty::U64,
            IntSuffix::U128  => Ty::U128,
            IntSuffix::Isize => Ty::Isize,
            IntSuffix::Usize => Ty::Usize,
        };
        self.arena.intern(ty)
    }

    // ── Diagnostics helpers ───────────────────────────────────────────────

    fn emit_mismatch(&mut self, expected: TyId, found: TyId, span: Span) {
        self.diagnostics.push(
            Diagnostic::error(ErrorCode::E0400,
                format!("type mismatch: expected `{}`, found `{}`",
                    self.arena.get(expected), self.arena.get(found)),
                span)
        );
    }
}

impl Default for TypeChecker {
    fn default() -> Self { Self::new() }
}
