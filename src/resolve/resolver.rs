/// Name resolution pass — resolves all identifiers to their definitions.
///
/// COMPILER.md §6.2, §6.3
///
/// Two-pass approach:
///   Pass 1: Collect all top-level declarations (fn, struct, enum, etc.)
///   Pass 2: Walk function/const/var bodies and resolve identifiers

use crate::ast::nodes::*;
use crate::ast::visitor::{self, Visitor};
use crate::error::{Diagnostic, ErrorCode, Severity};
use crate::lexer::token::Span;
use super::scope::*;

/// The name resolver walks the AST twice:
///  1. Collect top-level declarations into the symbol table
///  2. Resolve all identifier references in bodies
pub struct Resolver {
    pub table:       SymbolTable,
    pub diagnostics: Vec<Diagnostic>,
    module_id:       ModuleId,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            table:       SymbolTable::new(),
            diagnostics: Vec::new(),
            module_id:   0,
        }
    }

    /// Run both passes on the AST file.
    pub fn resolve(&mut self, file: &File) {
        // Pass 1 — collect all top-level declarations
        self.collect_declarations(file);
        // Pass 2 — resolve bodies
        self.resolve_bodies(file);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == Severity::Error)
    }

    // ── Pass 1: Collect declarations ──────────────────────────────────────

    fn collect_declarations(&mut self, file: &File) {
        for item in &file.items {
            self.collect_item(item);
        }
    }

    fn collect_item(&mut self, item: &Item) {
        match &item.kind {
            ItemKind::Fn(f) => {
                let (name, span) = fn_name_and_span(&f.name);
                self.define_or_error(&name, SymbolKind::Fn, span, f.is_pub);
            }
            ItemKind::Struct(s) => {
                self.define_or_error(&s.name.name, SymbolKind::Struct, s.name.span, s.is_pub);
            }
            ItemKind::Enum(e) => {
                self.define_or_error(&e.name.name, SymbolKind::Enum, e.name.span, e.is_pub);
                // Also register each variant
                for v in &e.variants {
                    let variant_name = format!("{}.{}", e.name.name, v.name.name);
                    self.define_or_error(&variant_name, SymbolKind::EnumVariant, v.name.span, e.is_pub);
                }
            }
            ItemKind::Union(u) => {
                self.define_or_error(&u.name.name, SymbolKind::Union, u.name.span, u.is_pub);
            }
            ItemKind::Interface(i) => {
                self.define_or_error(&i.name.name, SymbolKind::Interface, i.name.span, i.is_pub);
            }
            ItemKind::TypeAlias(t) => {
                self.define_or_error(&t.name.name, SymbolKind::TypeAlias, t.name.span, t.is_pub);
            }
            ItemKind::Newtype(n) => {
                self.define_or_error(&n.name.name, SymbolKind::Newtype, n.name.span, n.is_pub);
            }
            ItemKind::Const(c) => {
                self.define_or_error(&c.name.name, SymbolKind::Const, c.name.span, c.is_pub);
            }
            ItemKind::Var(v) => {
                for name in &v.names {
                    self.define_or_error(&name.name, SymbolKind::Var, name.span, v.is_pub);
                }
            }
            ItemKind::ImplBlock(_) | ItemKind::ImplFor(_) | ItemKind::Extern(_) => {}
        }
    }

    // ── Pass 2: Resolve bodies ────────────────────────────────────────────

    fn resolve_bodies(&mut self, file: &File) {
        for item in &file.items {
            match &item.kind {
                ItemKind::Fn(f) => self.resolve_fn(f),
                ItemKind::Const(c) => self.resolve_expr(&c.value),
                ItemKind::Var(v) => {
                    if let Some(vals) = &v.value {
                        for val in vals { self.resolve_expr(val); }
                    }
                }
                ItemKind::ImplBlock(block) => {
                    for m in &block.methods { self.resolve_fn(m); }
                }
                ItemKind::ImplFor(imp) => {
                    for m in &imp.methods { self.resolve_fn(m); }
                }
                _ => {}
            }
        }

        // Emit warnings for unused symbols
        for sym in self.table.unused_symbols() {
            let code = match sym.kind {
                SymbolKind::Var | SymbolKind::Let | SymbolKind::Param =>
                    ErrorCode::W0001,
                SymbolKind::Fn => ErrorCode::W0003,
                _ => continue,
            };
            self.diagnostics.push(
                Diagnostic::warning(code, format!("unused {}: `{}`",
                    kind_label(sym.kind), sym.name), sym.span)
            );
        }
    }

    fn resolve_fn(&mut self, f: &FnDecl) {
        self.table.push_scope(ScopeKind::Fn);

        // Register parameters
        for param in &f.params {
            match &param.kind {
                ParamKind::Regular { name, .. } => {
                    self.define_or_error(&name.name, SymbolKind::Param, name.span, false);
                }
                ParamKind::SelfParam { .. } => {
                    let span = param.span;
                    self.define_or_error("self", SymbolKind::Param, span, false);
                }
                ParamKind::Variadic { name, .. } => {
                    self.define_or_error(&name.name, SymbolKind::Param, name.span, false);
                }
            }
        }

        // Resolve body
        if let Some(body) = &f.body {
            self.resolve_block(body);
        }

        self.table.pop_scope();
    }

    fn resolve_block(&mut self, block: &Block) {
        self.table.push_scope(ScopeKind::Block);
        for stmt in &block.stmts {
            self.resolve_stmt(stmt);
        }
        self.table.pop_scope();
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Var(v) => {
                // Resolve initializer first (before the binding is visible)
                if let Some(vals) = &v.value {
                    for val in vals { self.resolve_expr(val); }
                }
                // Then define the binding
                let kind = if v.is_let { SymbolKind::Let } else { SymbolKind::Var };
                for name in &v.names {
                    // Check for shadowing (different scope)
                    if let Some(existing) = self.table.lookup(&name.name) {
                        if self.table.lookup_current(&name.name).is_none() {
                            self.diagnostics.push(
                                Diagnostic::warning(ErrorCode::W0006,
                                    format!("variable `{}` shadows a previous binding", name.name),
                                    name.span)
                                .with_label(self.table.get(existing).span,
                                    "previously defined here".to_string())
                            );
                        }
                    }
                    self.define_or_error(&name.name, kind, name.span, v.is_pub);
                }
            }
            StmtKind::Assign { target, value, .. } => {
                self.resolve_expr(target);
                self.resolve_expr(value);
            }
            StmtKind::Expr(e) => self.resolve_expr(e),
            StmtKind::Return(Some(e)) => self.resolve_expr(e),
            StmtKind::Return(None) => {}
            StmtKind::Break(_) | StmtKind::Continue(_) => {}
            StmtKind::Defer(inner) => self.resolve_stmt(inner),
            StmtKind::If(ifs) => {
                self.resolve_expr(&ifs.cond);
                self.resolve_block(&ifs.then);
                for (c, b) in &ifs.elifs {
                    self.resolve_expr(c);
                    self.resolve_block(b);
                }
                if let Some(e) = &ifs.else_ {
                    self.resolve_block(e);
                }
            }
            StmtKind::While { cond, body, .. } => {
                self.resolve_expr(cond);
                self.resolve_block(body);
            }
            StmtKind::For { pat, iter, step, body, .. } => {
                self.resolve_expr(iter);
                if let Some(s) = step { self.resolve_expr(s); }
                self.table.push_scope(ScopeKind::Loop);
                // Define loop variable(s)
                match &pat.kind {
                    ForPatKind::Single(name) => {
                        self.define_or_error(&name.name, SymbolKind::Var, name.span, false);
                    }
                    ForPatKind::IndexValue(idx, val) => {
                        self.define_or_error(&idx.name, SymbolKind::Var, idx.span, false);
                        self.define_or_error(&val.name, SymbolKind::Var, val.span, false);
                    }
                    ForPatKind::Discard => {}
                }
                // Resolve body (using the for-scope that has the loop variable)
                for stmt in &body.stmts {
                    self.resolve_stmt(stmt);
                }
                self.table.pop_scope();
            }
            StmtKind::Loop { body, .. } => {
                self.resolve_block(body);
            }
            StmtKind::Match(m) => {
                self.resolve_expr(&m.scrutinee);
                for arm in &m.arms {
                    if let Some(g) = &arm.guard { self.resolve_expr(g); }
                    match &arm.body {
                        MatchBody::Expr(e) => self.resolve_expr(e),
                        MatchBody::Block(b) => self.resolve_block(b),
                    }
                }
            }
            StmtKind::Try(t) => {
                self.resolve_block(&t.body);
                for c in &t.catches {
                    self.table.push_scope(ScopeKind::Block);
                    self.define_or_error(&c.name.name, SymbolKind::Var, c.name.span, false);
                    self.resolve_block(&c.body);
                    self.table.pop_scope();
                }
                if let Some(f) = &t.finally { self.resolve_block(f); }
            }
            StmtKind::Unsafe(b) => self.resolve_block(b),
            StmtKind::Asm(_) => {}
            StmtKind::Comptime(inner) => self.resolve_stmt(inner),
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match &expr.kind {
            ExprKind::Ident(ident) => {
                if let Some(id) = self.table.lookup(&ident.name) {
                    self.table.mark_used(id);
                } else {
                    self.diagnostics.push(
                        Diagnostic::error(ErrorCode::E0300,
                            format!("undefined variable `{}`", ident.name),
                            ident.span)
                    );
                }
            }
            ExprKind::Path(segments) => {
                // Try the first segment
                if let Some(first) = segments.first() {
                    if let Some(id) = self.table.lookup(&first.name) {
                        self.table.mark_used(id);
                    }
                    // For dotted paths like Enum.Variant, try full path
                    if segments.len() == 2 {
                        let full = format!("{}.{}", segments[0].name, segments[1].name);
                        if let Some(id) = self.table.lookup(&full) {
                            self.table.mark_used(id);
                        }
                    }
                }
            }
            ExprKind::BinOp { lhs, rhs, .. } => {
                self.resolve_expr(lhs);
                self.resolve_expr(rhs);
            }
            ExprKind::UnaryOp { operand, .. } => self.resolve_expr(operand),
            ExprKind::Call { callee, args } => {
                self.resolve_expr(callee);
                for a in args { self.resolve_expr(&a.expr); }
            }
            ExprKind::Field { base, .. } | ExprKind::PtrField { base, .. }
            | ExprKind::OptField { base, .. } => {
                self.resolve_expr(base);
            }
            ExprKind::Index { base, index } => {
                self.resolve_expr(base);
                self.resolve_expr(index);
            }
            ExprKind::Slice { base, lo, hi } => {
                self.resolve_expr(base);
                if let Some(l) = lo { self.resolve_expr(l); }
                if let Some(h) = hi { self.resolve_expr(h); }
            }
            ExprKind::StructLit { fields, rest, .. } => {
                for f in fields {
                    if let Some(v) = &f.value { self.resolve_expr(v); }
                }
                if let Some(r) = rest { self.resolve_expr(r); }
            }
            ExprKind::Array(elems) | ExprKind::Tuple(elems) | ExprKind::Set(elems) => {
                for e in elems { self.resolve_expr(e); }
            }
            ExprKind::Map(entries) => {
                for (k, v) in entries { self.resolve_expr(k); self.resolve_expr(v); }
            }
            ExprKind::Range { lo, hi, .. } => {
                self.resolve_expr(lo); self.resolve_expr(hi);
            }
            ExprKind::Cast { expr: inner, .. } | ExprKind::Is { expr: inner, .. }
            | ExprKind::Propagate(inner) | ExprKind::Unwrap(inner) => {
                self.resolve_expr(inner);
            }
            ExprKind::OptCall { base, args, .. } => {
                self.resolve_expr(base);
                for a in args { self.resolve_expr(&a.expr); }
            }
            ExprKind::Lambda { body, .. } => {
                match body {
                    LambdaBody::Expr(e) => self.resolve_expr(e),
                    LambdaBody::Block(b) => self.resolve_block(b),
                }
            }
            ExprKind::If { cond, then, elifs, else_ } => {
                self.resolve_expr(cond);
                self.resolve_expr(then);
                for (c, e) in elifs { self.resolve_expr(c); self.resolve_expr(e); }
                self.resolve_expr(else_);
            }
            ExprKind::Match { scrutinee, arms } => {
                self.resolve_expr(scrutinee);
                for arm in arms {
                    if let Some(g) = &arm.guard { self.resolve_expr(g); }
                    match &arm.body {
                        MatchBody::Expr(e) => self.resolve_expr(e),
                        MatchBody::Block(b) => self.resolve_block(b),
                    }
                }
            }
            ExprKind::Block(b) => self.resolve_block(b),
            ExprKind::Builtin { args, .. } => {
                for a in args { self.resolve_expr(a); }
            }
            // Literals — nothing to resolve
            _ => {}
        }
    }

    // ── Helpers ───────────────────────────────────────────────────────────

    fn define_or_error(&mut self, name: &str, kind: SymbolKind, span: Span, is_pub: bool) {
        if let Err(existing_id) = self.table.define(name, kind, span, is_pub, self.module_id) {
            let existing = self.table.get(existing_id);
            self.diagnostics.push(
                Diagnostic::error(ErrorCode::E0303,
                    format!("duplicate definition of `{}`", name), span)
                .with_label(existing.span, "first defined here".to_string())
            );
        }
    }
}

fn fn_name_and_span(name: &FnName) -> (String, Span) {
    match name {
        FnName::Simple(i) => (i.name.clone(), i.span),
        FnName::Method { ty_name, method, span } =>
            (format!("{}.{}", ty_name.name, method.name), *span),
    }
}

fn kind_label(kind: SymbolKind) -> &'static str {
    match kind {
        SymbolKind::Var => "variable",
        SymbolKind::Let => "binding",
        SymbolKind::Fn => "function",
        SymbolKind::Param => "parameter",
        SymbolKind::Struct => "struct",
        SymbolKind::Enum => "enum",
        SymbolKind::Const => "constant",
        _ => "symbol",
    }
}
