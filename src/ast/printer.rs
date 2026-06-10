/// AST Pretty-Printer for debugging.
///
/// Spec §5 / COMPILER.md §5
///
/// Prints a human-readable, indented representation of the full AST.

use super::nodes::*;
use crate::lexer::token::FStringPart;
use std::fmt::Write;

pub struct AstPrinter {
    buf:    String,
    indent: usize,
}

impl AstPrinter {
    pub fn new() -> Self {
        Self { buf: String::new(), indent: 0 }
    }

    pub fn print_file(&mut self, file: &File) -> &str {
        self.line("File");
        self.indent();
        // Module
        let mod_path: Vec<&str> = file.module.path.iter().map(|i| i.name.as_str()).collect();
        self.line(&format!("module: {}", mod_path.join(".")));
        // Directives
        for d in &file.directives {
            if let Some(arg) = &d.arg {
                self.line(&format!("directive: #!{} {}", d.name, arg));
            } else {
                self.line(&format!("directive: #!{}", d.name));
            }
        }
        // Imports
        for imp in &file.imports {
            self.print_import(imp);
        }
        // Items
        for item in &file.items {
            self.print_item(item);
        }
        self.dedent();
        &self.buf
    }

    fn print_import(&mut self, imp: &Import) {
        match &imp.kind {
            ImportKind::Simple { alias: None } =>
                self.line(&format!("import \"{}\"", imp.path)),
            ImportKind::Simple { alias: Some(a) } =>
                self.line(&format!("import \"{}\" as {}", imp.path, a.name)),
            ImportKind::Selective { names } => {
                let ns: Vec<&str> = names.iter().map(|i| i.name.as_str()).collect();
                self.line(&format!("import \"{}\": {{{}}}", imp.path, ns.join(", ")));
            }
            ImportKind::GlobUse { ty_path } => {
                let p: Vec<&str> = ty_path.iter().map(|i| i.name.as_str()).collect();
                self.line(&format!("use {}.*", p.join(".")));
            }
        }
    }

    fn print_item(&mut self, item: &Item) {
        for attr in &item.attrs {
            self.print_attr(attr);
        }
        match &item.kind {
            ItemKind::Fn(f)         => self.print_fn(f),
            ItemKind::Struct(s)     => self.print_struct(s),
            ItemKind::Enum(e)       => self.print_enum(e),
            ItemKind::Union(u)      => self.print_union(u),
            ItemKind::Interface(i)  => self.print_interface(i),
            ItemKind::ImplBlock(i)  => self.print_impl_block(i),
            ItemKind::ImplFor(i)    => self.print_impl_for(i),
            ItemKind::TypeAlias(t)  => {
                self.line(&format!("{}type {} = {}", vis(t.is_pub), t.name.name, self.type_str(&t.ty)));
            }
            ItemKind::Newtype(n)    => {
                self.line(&format!("{}newtype {} = {}", vis(n.is_pub), n.name.name, self.type_str(&n.ty)));
            }
            ItemKind::Const(c)      => {
                self.line(&format!("const {}: {}", c.name.name, self.type_str(&c.ty)));
                self.indent();
                self.print_expr(&c.value);
                self.dedent();
            }
            ItemKind::Var(v)        => self.print_var_decl(v),
            ItemKind::Extern(e)     => self.print_extern(e),
        }
    }

    fn print_attr(&mut self, attr: &Attribute) {
        self.line(&format!("@{}", attr.name.name));
    }

    fn print_fn(&mut self, f: &FnDecl) {
        let name = match &f.name {
            FnName::Simple(i) => i.name.clone(),
            FnName::Method { ty_name, method, .. } =>
                format!("{}.{}", ty_name.name, method.name),
        };
        let mut sig = format!("{}{}fn {}", vis(f.is_pub),
            if f.is_async { "async " } else { "" }, name);
        if !f.generics.is_empty() {
            let gs: Vec<String> = f.generics.iter().map(|g| g.name.name.clone()).collect();
            sig.push_str(&format!("[{}]", gs.join(", ")));
        }
        let params: Vec<String> = f.params.iter().map(|p| self.param_str(p)).collect();
        sig.push_str(&format!("({})", params.join(", ")));
        if let Some(ret) = &f.ret_ty {
            sig.push_str(&format!(" -> {}", self.type_str(ret)));
        }
        self.line(&sig);
        if let Some(body) = &f.body {
            self.indent();
            self.print_block(body);
            self.dedent();
        }
    }

    fn print_struct(&mut self, s: &StructDecl) {
        self.line(&format!("{}struct {}", vis(s.is_pub), s.name.name));
        self.indent();
        for field in &s.fields {
            let def = if field.default.is_some() { " = ..." } else { "" };
            self.line(&format!("{}{}: {}{}", vis(field.is_pub), field.name.name,
                self.type_str(&field.ty), def));
        }
        self.dedent();
    }

    fn print_enum(&mut self, e: &EnumDecl) {
        self.line(&format!("{}enum {}", vis(e.is_pub), e.name.name));
        self.indent();
        for v in &e.variants {
            match &v.kind {
                VariantKind::Unit => self.line(&v.name.name),
                VariantKind::Tuple(types) => {
                    let ts: Vec<String> = types.iter().map(|t| self.type_str(t)).collect();
                    self.line(&format!("{}({})", v.name.name, ts.join(", ")));
                }
                VariantKind::Struct(fields) => {
                    self.line(&format!("{} {{...}}", v.name.name));
                    let _ = fields;
                }
                VariantKind::Named(pairs) => {
                    let ps: Vec<String> = pairs.iter()
                        .map(|(n, t)| format!("{}: {}", n.name, self.type_str(t)))
                        .collect();
                    self.line(&format!("{}({})", v.name.name, ps.join(", ")));
                }
            }
        }
        self.dedent();
    }

    fn print_union(&mut self, u: &UnionDecl) {
        self.line(&format!("{}union {}", vis(u.is_pub), u.name.name));
        self.indent();
        for f in &u.fields {
            self.line(&format!("{}: {}", f.name.name, self.type_str(&f.ty)));
        }
        self.dedent();
    }

    fn print_interface(&mut self, i: &InterfaceDecl) {
        self.line(&format!("{}interface {}", vis(i.is_pub), i.name.name));
        self.indent();
        for item in &i.items {
            match item {
                InterfaceItem::Method(f) => self.print_fn(f),
                InterfaceItem::AssocType { name, .. } =>
                    self.line(&format!("type {}", name.name)),
            }
        }
        self.dedent();
    }

    fn print_impl_block(&mut self, i: &ImplBlock) {
        self.line(&format!("impl {}", self.type_str(&i.ty)));
        self.indent();
        for m in &i.methods { self.print_fn(m); }
        self.dedent();
    }

    fn print_impl_for(&mut self, i: &ImplFor) {
        self.line(&format!("impl {} for {}", self.type_str(&i.interface), self.type_str(&i.for_ty)));
        self.indent();
        for m in &i.methods { self.print_fn(m); }
        self.dedent();
    }

    fn print_extern(&mut self, e: &ExternDecl) {
        let abi = e.abi.as_deref().unwrap_or("C");
        match &e.kind {
            ExternKind::Fn { name, .. } =>
                self.line(&format!("extern \"{}\" fn {}", abi, name.name)),
            ExternKind::Var { name, ty } =>
                self.line(&format!("extern \"{}\" var {}: {}", abi, name.name, self.type_str(ty))),
        }
    }

    fn print_block(&mut self, block: &Block) {
        for stmt in &block.stmts {
            self.print_stmt(stmt);
        }
    }

    fn print_stmt(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Var(v) => self.print_var_decl(v),
            StmtKind::Assign { target, op, value } => {
                self.line(&format!("assign {:?}", op));
                self.indent();
                self.print_expr(target);
                self.print_expr(value);
                self.dedent();
            }
            StmtKind::Expr(e) => self.print_expr(e),
            StmtKind::Return(None) => self.line("return"),
            StmtKind::Return(Some(e)) => {
                self.line("return");
                self.indent(); self.print_expr(e); self.dedent();
            }
            StmtKind::Break(label) => {
                let l = label.as_ref().map(|l| format!(" @{}", l.name)).unwrap_or_default();
                self.line(&format!("break{}", l));
            }
            StmtKind::Continue(label) => {
                let l = label.as_ref().map(|l| format!(" @{}", l.name)).unwrap_or_default();
                self.line(&format!("continue{}", l));
            }
            StmtKind::Defer(inner) => {
                self.line("defer");
                self.indent(); self.print_stmt(inner); self.dedent();
            }
            StmtKind::If(ifs) => {
                self.line("if");
                self.indent();
                self.print_expr(&ifs.cond);
                self.line("then:");
                self.indent(); self.print_block(&ifs.then); self.dedent();
                for (c, b) in &ifs.elifs {
                    self.line("elif");
                    self.indent(); self.print_expr(c); self.dedent();
                    self.line("then:");
                    self.indent(); self.print_block(b); self.dedent();
                }
                if let Some(e) = &ifs.else_ {
                    self.line("else:");
                    self.indent(); self.print_block(e); self.dedent();
                }
                self.dedent();
            }
            StmtKind::While { cond, body, .. } => {
                self.line("while");
                self.indent(); self.print_expr(cond); self.dedent();
                self.indent(); self.print_block(body); self.dedent();
            }
            StmtKind::For { pat, iter, body, .. } => {
                self.line(&format!("for {:?}", pat.kind));
                self.indent(); self.print_expr(iter); self.dedent();
                self.indent(); self.print_block(body); self.dedent();
            }
            StmtKind::Loop { body, .. } => {
                self.line("loop");
                self.indent(); self.print_block(body); self.dedent();
            }
            StmtKind::Match(m) => {
                self.line("match");
                self.indent();
                self.print_expr(&m.scrutinee);
                for arm in &m.arms {
                    self.line(&format!("arm ({} patterns)", arm.pats.len()));
                    self.indent();
                    match &arm.body {
                        MatchBody::Expr(e) => self.print_expr(e),
                        MatchBody::Block(b) => self.print_block(b),
                    }
                    self.dedent();
                }
                self.dedent();
            }
            StmtKind::Try(t) => {
                self.line("try");
                self.indent(); self.print_block(&t.body); self.dedent();
                for c in &t.catches {
                    self.line(&format!("catch {}: {}", c.name.name, self.type_str(&c.ty)));
                    self.indent(); self.print_block(&c.body); self.dedent();
                }
                if let Some(f) = &t.finally {
                    self.line("finally");
                    self.indent(); self.print_block(f); self.dedent();
                }
            }
            StmtKind::Unsafe(b) => {
                self.line("unsafe");
                self.indent(); self.print_block(b); self.dedent();
            }
            StmtKind::Asm(_) => self.line("asm { ... }"),
            StmtKind::Comptime(inner) => {
                self.line("comptime");
                self.indent(); self.print_stmt(inner); self.dedent();
            }
        }
    }

    fn print_var_decl(&mut self, v: &VarDecl) {
        let kw = if v.is_let { "let" } else { "var" };
        let names: Vec<&str> = v.names.iter().map(|i| i.name.as_str()).collect();
        let ty = v.ty.as_ref().map(|t| format!(": {}", self.type_str(t))).unwrap_or_default();
        self.line(&format!("{}{} {}{}", vis(v.is_pub), kw, names.join(", "), ty));
        if let Some(vals) = &v.value {
            self.indent();
            for val in vals { self.print_expr(val); }
            self.dedent();
        }
    }

    fn print_expr(&mut self, expr: &Expr) {
        match &expr.kind {
            ExprKind::Int(v, suf) => {
                let s = suf.map(|s| s.as_str()).unwrap_or("");
                self.line(&format!("int {}{}", v, s));
            }
            ExprKind::Float(v, _) => self.line(&format!("float {}", v)),
            ExprKind::Str(s) => self.line(&format!("str \"{}\"", s)),
            ExprKind::RawStr(s) => self.line(&format!("raw_str \"{}\"", s)),
            ExprKind::FStr(parts) => self.line(&format!("fstr ({} parts)", parts.len())),
            ExprKind::Char(c) => self.line(&format!("char '{}'", c)),
            ExprKind::Byte(b) => self.line(&format!("byte {}", b)),
            ExprKind::Bool(b) => self.line(&format!("bool {}", b)),
            ExprKind::Null => self.line("null"),
            ExprKind::Ident(i) => self.line(&format!("ident {}", i.name)),
            ExprKind::Path(segs) => {
                let p: Vec<&str> = segs.iter().map(|i| i.name.as_str()).collect();
                self.line(&format!("path {}", p.join(".")));
            }
            ExprKind::BinOp { op, lhs, rhs } => {
                self.line(&format!("binop {:?}", op));
                self.indent(); self.print_expr(lhs); self.print_expr(rhs); self.dedent();
            }
            ExprKind::UnaryOp { op, operand } => {
                self.line(&format!("unary {:?}", op));
                self.indent(); self.print_expr(operand); self.dedent();
            }
            ExprKind::Call { callee, args } => {
                self.line(&format!("call ({} args)", args.len()));
                self.indent();
                self.print_expr(callee);
                for a in args { self.print_expr(&a.expr); }
                self.dedent();
            }
            ExprKind::Field { base, field } => {
                self.line(&format!("field .{}", field.name));
                self.indent(); self.print_expr(base); self.dedent();
            }
            ExprKind::Index { base, index } => {
                self.line("index");
                self.indent(); self.print_expr(base); self.print_expr(index); self.dedent();
            }
            ExprKind::StructLit { ty, fields, .. } => {
                let p: Vec<&str> = ty.iter().map(|i| i.name.as_str()).collect();
                self.line(&format!("struct_lit {} ({} fields)", p.join("."), fields.len()));
            }
            ExprKind::Array(elems) => self.line(&format!("array [{} elems]", elems.len())),
            ExprKind::Tuple(elems) => self.line(&format!("tuple ({} elems)", elems.len())),
            ExprKind::Range { inclusive, .. } => {
                self.line(&format!("range {}", if *inclusive { "..=" } else { ".." }));
            }
            ExprKind::Cast { expr: inner, ty, kind } => {
                self.line(&format!("cast {:?} -> {}", kind, self.type_str(ty)));
                self.indent(); self.print_expr(inner); self.dedent();
            }
            ExprKind::Lambda { params, .. } => {
                self.line(&format!("lambda ({} params)", params.len()));
            }
            ExprKind::Propagate(inner) => {
                self.line("propagate ?");
                self.indent(); self.print_expr(inner); self.dedent();
            }
            ExprKind::Unwrap(inner) => {
                self.line("unwrap !");
                self.indent(); self.print_expr(inner); self.dedent();
            }
            ExprKind::Block(b) => {
                self.line("block_expr");
                self.indent(); self.print_block(b); self.dedent();
            }
            _ => self.line(&format!("expr {:?}", std::mem::discriminant(&expr.kind))),
        }
    }

    fn type_str(&self, ty: &Type) -> String {
        match ty {
            Type::Primitive(p, _) => format!("{:?}", p).to_lowercase(),
            Type::Path { segments, generics, .. } => {
                let p: Vec<&str> = segments.iter().map(|i| i.name.as_str()).collect();
                if generics.is_empty() {
                    p.join(".")
                } else {
                    let gs: Vec<String> = generics.iter().map(|t| self.type_str(t)).collect();
                    format!("{}[{}]", p.join("."), gs.join(", "))
                }
            }
            Type::Pointer { is_const, inner, .. } =>
                format!("*{}{}", if *is_const { "const " } else { "" }, self.type_str(inner)),
            Type::Optional(inner, _) => format!("?{}", self.type_str(inner)),
            Type::Slice(inner, _) => format!("[]{}", self.type_str(inner)),
            Type::Array { size: _, elem, .. } => format!("[N]{}", self.type_str(elem)),
            Type::Tuple(types, _) => {
                let ts: Vec<String> = types.iter().map(|t| self.type_str(t)).collect();
                format!("({})", ts.join(", "))
            }
            Type::FnPtr { params, ret, .. } => {
                let ps: Vec<String> = params.iter().map(|t| self.type_str(t)).collect();
                format!("fn({}) -> {}", ps.join(", "), self.type_str(ret))
            }
            Type::Map { key, val, .. } => format!("map[{}]{}", self.type_str(key), self.type_str(val)),
            Type::Set(inner, _) => format!("set[{}]", self.type_str(inner)),
            Type::Ref { mutable, inner, .. } =>
                format!("&{}{}", if *mutable { "mut " } else { "" }, self.type_str(inner)),
            Type::Never(_) => "never".to_string(),
            Type::Void(_) => "void".to_string(),
            Type::Any(_) => "any".to_string(),
            Type::Infer(_) => "_".to_string(),
        }
    }

    fn param_str(&self, param: &Param) -> String {
        match &param.kind {
            ParamKind::Regular { name, ty, .. } =>
                format!("{}: {}", name.name, self.type_str(ty)),
            ParamKind::SelfParam { mutable, .. } =>
                if *mutable { "mut self".into() } else { "self".into() },
            ParamKind::Variadic { name, ty } =>
                format!("...{}: ...{}", name.name, self.type_str(ty)),
        }
    }

    // ── Output helpers ────────────────────────────────────────────────────

    fn line(&mut self, text: &str) {
        let pad = "  ".repeat(self.indent);
        let _ = writeln!(self.buf, "{}{}", pad, text);
    }

    fn indent(&mut self) { self.indent += 1; }
    fn dedent(&mut self) { self.indent = self.indent.saturating_sub(1); }
}

fn vis(is_pub: bool) -> &'static str {
    if is_pub { "pub " } else { "" }
}

/// Convenience function: pretty-print an AST file and return the string.
pub fn print_ast(file: &File) -> String {
    let mut printer = AstPrinter::new();
    printer.print_file(file);
    printer.buf
}
