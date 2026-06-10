/// AST Visitor trait for traversing the G AST.
///
/// Spec §5 / COMPILER.md §5
///
/// The visitor pattern allows each compiler pass (name resolution, type checking,
/// linting, etc.) to implement only the visit methods it cares about.
/// All default implementations recurse into child nodes.

use super::nodes::*;

// ── Visitor Trait ─────────────────────────────────────────────────────────────

/// A trait for walking the AST tree top-down.
///
/// Each `visit_*` method has a default implementation that recurses into
/// child nodes. Override only the nodes you want to inspect or transform.
pub trait Visitor: Sized {
    // ── Top-level ─────────────────────────────────────────────────────────

    fn visit_file(&mut self, file: &File) {
        walk_file(self, file);
    }

    fn visit_directive(&mut self, _dir: &Directive) {}

    fn visit_module_decl(&mut self, _decl: &ModuleDecl) {}

    fn visit_import(&mut self, _import: &Import) {}

    fn visit_item(&mut self, item: &Item) {
        walk_item(self, item);
    }

    fn visit_attribute(&mut self, _attr: &Attribute) {}

    // ── Items ─────────────────────────────────────────────────────────────

    fn visit_fn_decl(&mut self, decl: &FnDecl) {
        walk_fn_decl(self, decl);
    }

    fn visit_struct_decl(&mut self, decl: &StructDecl) {
        walk_struct_decl(self, decl);
    }

    fn visit_enum_decl(&mut self, decl: &EnumDecl) {
        walk_enum_decl(self, decl);
    }

    fn visit_union_decl(&mut self, _decl: &UnionDecl) {}

    fn visit_interface_decl(&mut self, decl: &InterfaceDecl) {
        walk_interface_decl(self, decl);
    }

    fn visit_impl_block(&mut self, block: &ImplBlock) {
        walk_impl_block(self, block);
    }

    fn visit_impl_for(&mut self, imp: &ImplFor) {
        walk_impl_for(self, imp);
    }

    fn visit_type_alias(&mut self, _alias: &TypeAlias) {}

    fn visit_newtype_decl(&mut self, _decl: &NewtypeDecl) {}

    fn visit_const_decl(&mut self, decl: &ConstDecl) {
        self.visit_expr(&decl.value);
    }

    fn visit_var_decl(&mut self, decl: &VarDecl) {
        walk_var_decl(self, decl);
    }

    fn visit_extern_decl(&mut self, _decl: &ExternDecl) {}

    // ── Types ─────────────────────────────────────────────────────────────

    fn visit_type(&mut self, _ty: &Type) {}

    fn visit_generic_param(&mut self, _param: &GenericParam) {}

    // ── Statements ────────────────────────────────────────────────────────

    fn visit_block(&mut self, block: &Block) {
        walk_block(self, block);
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        walk_stmt(self, stmt);
    }

    // ── Expressions ───────────────────────────────────────────────────────

    fn visit_expr(&mut self, expr: &Expr) {
        walk_expr(self, expr);
    }

    // ── Patterns ──────────────────────────────────────────────────────────

    fn visit_pat(&mut self, _pat: &Pat) {}

    // ── Params ────────────────────────────────────────────────────────────

    fn visit_param(&mut self, _param: &Param) {}
}

// ── Walk functions ────────────────────────────────────────────────────────────
// These are the default traversal implementations. They recurse into all child
// nodes so visitor implementations can focus on the nodes they care about.

pub fn walk_file<V: Visitor>(v: &mut V, file: &File) {
    for dir in &file.directives {
        v.visit_directive(dir);
    }
    v.visit_module_decl(&file.module);
    for import in &file.imports {
        v.visit_import(import);
    }
    for item in &file.items {
        v.visit_item(item);
    }
}

pub fn walk_item<V: Visitor>(v: &mut V, item: &Item) {
    for attr in &item.attrs {
        v.visit_attribute(attr);
    }
    match &item.kind {
        ItemKind::Fn(decl)         => v.visit_fn_decl(decl),
        ItemKind::Struct(decl)     => v.visit_struct_decl(decl),
        ItemKind::Enum(decl)       => v.visit_enum_decl(decl),
        ItemKind::Union(decl)      => v.visit_union_decl(decl),
        ItemKind::Interface(decl)  => v.visit_interface_decl(decl),
        ItemKind::ImplBlock(block) => v.visit_impl_block(block),
        ItemKind::ImplFor(imp)     => v.visit_impl_for(imp),
        ItemKind::TypeAlias(alias) => v.visit_type_alias(alias),
        ItemKind::Newtype(decl)    => v.visit_newtype_decl(decl),
        ItemKind::Const(decl)      => v.visit_const_decl(decl),
        ItemKind::Var(decl)        => v.visit_var_decl(decl),
        ItemKind::Extern(decl)     => v.visit_extern_decl(decl),
    }
}

pub fn walk_fn_decl<V: Visitor>(v: &mut V, decl: &FnDecl) {
    for gp in &decl.generics {
        v.visit_generic_param(gp);
    }
    for param in &decl.params {
        v.visit_param(param);
    }
    if let Some(ret) = &decl.ret_ty {
        v.visit_type(ret);
    }
    if let Some(body) = &decl.body {
        v.visit_block(body);
    }
}

pub fn walk_struct_decl<V: Visitor>(v: &mut V, decl: &StructDecl) {
    for gp in &decl.generics {
        v.visit_generic_param(gp);
    }
    for field in &decl.fields {
        v.visit_type(&field.ty);
        if let Some(def) = &field.default {
            v.visit_expr(def);
        }
    }
}

pub fn walk_enum_decl<V: Visitor>(v: &mut V, decl: &EnumDecl) {
    for gp in &decl.generics {
        v.visit_generic_param(gp);
    }
    for variant in &decl.variants {
        match &variant.kind {
            VariantKind::Unit => {}
            VariantKind::Tuple(types) => {
                for ty in types {
                    v.visit_type(ty);
                }
            }
            VariantKind::Struct(fields) => {
                for field in fields {
                    v.visit_type(&field.ty);
                }
            }
            VariantKind::Named(pairs) => {
                for (_, ty) in pairs {
                    v.visit_type(ty);
                }
            }
        }
        if let Some(val) = &variant.value {
            v.visit_expr(val);
        }
    }
}

pub fn walk_interface_decl<V: Visitor>(v: &mut V, decl: &InterfaceDecl) {
    for gp in &decl.generics {
        v.visit_generic_param(gp);
    }
    for item in &decl.items {
        match item {
            InterfaceItem::Method(fn_decl) => v.visit_fn_decl(fn_decl),
            InterfaceItem::AssocType { .. } => {}
        }
    }
}

pub fn walk_impl_block<V: Visitor>(v: &mut V, block: &ImplBlock) {
    v.visit_type(&block.ty);
    for method in &block.methods {
        v.visit_fn_decl(method);
    }
}

pub fn walk_impl_for<V: Visitor>(v: &mut V, imp: &ImplFor) {
    for gp in &imp.generics {
        v.visit_generic_param(gp);
    }
    v.visit_type(&imp.interface);
    v.visit_type(&imp.for_ty);
    for method in &imp.methods {
        v.visit_fn_decl(method);
    }
}

pub fn walk_var_decl<V: Visitor>(v: &mut V, decl: &VarDecl) {
    if let Some(ty) = &decl.ty {
        v.visit_type(ty);
    }
    if let Some(values) = &decl.value {
        for val in values {
            v.visit_expr(val);
        }
    }
}

pub fn walk_block<V: Visitor>(v: &mut V, block: &Block) {
    for stmt in &block.stmts {
        v.visit_stmt(stmt);
    }
}

pub fn walk_stmt<V: Visitor>(v: &mut V, stmt: &Stmt) {
    match &stmt.kind {
        StmtKind::Var(decl) => v.visit_var_decl(decl),
        StmtKind::Assign { target, value, .. } => {
            v.visit_expr(target);
            v.visit_expr(value);
        }
        StmtKind::Expr(expr) => v.visit_expr(expr),
        StmtKind::Return(Some(expr)) => v.visit_expr(expr),
        StmtKind::Return(None) => {}
        StmtKind::Break(_) | StmtKind::Continue(_) => {}
        StmtKind::Defer(inner) => v.visit_stmt(inner),
        StmtKind::If(if_stmt) => {
            v.visit_expr(&if_stmt.cond);
            v.visit_block(&if_stmt.then);
            for (cond, block) in &if_stmt.elifs {
                v.visit_expr(cond);
                v.visit_block(block);
            }
            if let Some(else_block) = &if_stmt.else_ {
                v.visit_block(else_block);
            }
        }
        StmtKind::While { cond, body, .. } => {
            v.visit_expr(cond);
            v.visit_block(body);
        }
        StmtKind::For { iter, body, step, .. } => {
            v.visit_expr(iter);
            if let Some(s) = step {
                v.visit_expr(s);
            }
            v.visit_block(body);
        }
        StmtKind::Loop { body, .. } => {
            v.visit_block(body);
        }
        StmtKind::Match(ms) => {
            v.visit_expr(&ms.scrutinee);
            for arm in &ms.arms {
                for pat in &arm.pats {
                    v.visit_pat(pat);
                }
                if let Some(guard) = &arm.guard {
                    v.visit_expr(guard);
                }
                match &arm.body {
                    MatchBody::Expr(e) => v.visit_expr(e),
                    MatchBody::Block(b) => v.visit_block(b),
                }
            }
        }
        StmtKind::Try(ts) => {
            v.visit_block(&ts.body);
            for catch in &ts.catches {
                v.visit_type(&catch.ty);
                v.visit_block(&catch.body);
            }
            if let Some(fin) = &ts.finally {
                v.visit_block(fin);
            }
        }
        StmtKind::Unsafe(block) => v.visit_block(block),
        StmtKind::Asm(_) => {}
        StmtKind::Comptime(inner) => v.visit_stmt(inner),
    }
}

pub fn walk_expr<V: Visitor>(v: &mut V, expr: &Expr) {
    match &expr.kind {
        // Literals — leaf nodes
        ExprKind::Int(..) | ExprKind::Float(..) | ExprKind::Str(..)
        | ExprKind::RawStr(..) | ExprKind::Char(..) | ExprKind::Byte(..)
        | ExprKind::Bool(..) | ExprKind::Null => {}

        ExprKind::FStr(parts) => {
            // FStringPart tokens are lexer tokens, not AST — skip
            let _ = parts;
        }

        ExprKind::Ident(_) | ExprKind::Path(_) => {}

        ExprKind::BinOp { lhs, rhs, .. } => {
            v.visit_expr(lhs);
            v.visit_expr(rhs);
        }
        ExprKind::UnaryOp { operand, .. } => {
            v.visit_expr(operand);
        }
        ExprKind::Cast { expr: inner, ty, .. } => {
            v.visit_expr(inner);
            v.visit_type(ty);
        }
        ExprKind::Is { expr: inner, ty } => {
            v.visit_expr(inner);
            v.visit_type(ty);
        }

        ExprKind::Field { base, .. }
        | ExprKind::PtrField { base, .. }
        | ExprKind::OptField { base, .. } => {
            v.visit_expr(base);
        }
        ExprKind::Index { base, index } => {
            v.visit_expr(base);
            v.visit_expr(index);
        }
        ExprKind::Slice { base, lo, hi } => {
            v.visit_expr(base);
            if let Some(lo) = lo { v.visit_expr(lo); }
            if let Some(hi) = hi { v.visit_expr(hi); }
        }
        ExprKind::Call { callee, args } => {
            v.visit_expr(callee);
            for arg in args {
                v.visit_expr(&arg.expr);
            }
        }
        ExprKind::OptCall { base, args, .. } => {
            v.visit_expr(base);
            for arg in args {
                v.visit_expr(&arg.expr);
            }
        }
        ExprKind::Propagate(inner) | ExprKind::Unwrap(inner) => {
            v.visit_expr(inner);
        }

        ExprKind::Array(elems) | ExprKind::Tuple(elems) | ExprKind::Set(elems) => {
            for e in elems { v.visit_expr(e); }
        }
        ExprKind::Map(entries) => {
            for (k, val) in entries {
                v.visit_expr(k);
                v.visit_expr(val);
            }
        }
        ExprKind::StructLit { fields, rest, .. } => {
            for f in fields {
                if let Some(val) = &f.value {
                    v.visit_expr(val);
                }
            }
            if let Some(r) = rest {
                v.visit_expr(r);
            }
        }
        ExprKind::Range { lo, hi, .. } => {
            v.visit_expr(lo);
            v.visit_expr(hi);
        }
        ExprKind::Lambda { params, ret_ty, body, .. } => {
            for p in params { v.visit_param(p); }
            if let Some(ty) = ret_ty { v.visit_type(ty); }
            match body {
                LambdaBody::Expr(e) => v.visit_expr(e),
                LambdaBody::Block(b) => v.visit_block(b),
            }
        }
        ExprKind::If { cond, then, elifs, else_ } => {
            v.visit_expr(cond);
            v.visit_expr(then);
            for (c, e) in elifs {
                v.visit_expr(c);
                v.visit_expr(e);
            }
            v.visit_expr(else_);
        }
        ExprKind::Match { scrutinee, arms } => {
            v.visit_expr(scrutinee);
            for arm in arms {
                for pat in &arm.pats { v.visit_pat(pat); }
                if let Some(g) = &arm.guard { v.visit_expr(g); }
                match &arm.body {
                    MatchBody::Expr(e) => v.visit_expr(e),
                    MatchBody::Block(b) => v.visit_block(b),
                }
            }
        }
        ExprKind::Block(block) => v.visit_block(block),
        ExprKind::Builtin { args, ty_arg, .. } => {
            if let Some(ty) = ty_arg { v.visit_type(ty); }
            for a in args { v.visit_expr(a); }
        }
    }
}
