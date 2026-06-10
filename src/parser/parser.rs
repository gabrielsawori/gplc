use crate::ast::*;
use crate::lexer::token::{Keyword, Span, Token, TokenKind, IntSuffix, FloatSuffix};
use super::error::*;

// ─────────────────────────────────────────────────────────────────────────────
// Parser struct
// ─────────────────────────────────────────────────────────────────────────────

pub struct Parser {
    tokens: Vec<Token>,
    pos:    usize,
    pub errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        // Filter out comments but keep structural tokens
        let tokens: Vec<Token> = tokens
            .into_iter()
            .filter(|t| !matches!(t.kind,
                TokenKind::LineComment(_) | TokenKind::DocComment(_)))
            .collect();
        Self { tokens, pos: 0, errors: Vec::new() }
    }

    // ── Token navigation ─────────────────────────────────────────────────────

    fn peek(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn peek2(&self) -> &Token {
        let idx = (self.pos + 1).min(self.tokens.len() - 1);
        &self.tokens[idx]
    }

    fn advance(&mut self) -> &Token {
        let t = &self.tokens[self.pos.min(self.tokens.len() - 1)];
        if self.pos < self.tokens.len() - 1 { self.pos += 1; }
        t
    }

    fn is_eof(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    fn span(&self) -> Span { self.peek().span }

    /// Consume a token of the given kind or push an error and return None.
    fn expect_kind(&mut self, kind: &TokenKind, desc: &str) -> Option<Token> {
        if std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind) {
            Some(self.advance().clone())
        } else {
            let sp = self.span();
            let got = self.peek().kind.clone();
            self.errors.push(e0001_unexpected_token(&got, desc, sp));
            None
        }
    }

    /// Consume keyword or push error.
    fn expect_kw(&mut self, kw: Keyword) -> bool {
        if self.peek().kind == TokenKind::Kw(kw) {
            self.advance();
            true
        } else {
            let sp = self.span();
            let got = self.peek().kind.clone();
            self.errors.push(e0001_unexpected_token(&got,
                &format!("`{}`", kw.as_str()), sp));
            false
        }
    }

    fn eat_kw(&mut self, kw: Keyword) -> bool {
        if self.peek().kind == TokenKind::Kw(kw) {
            self.advance();
            true
        } else { false }
    }

    fn eat(&mut self, kind: TokenKind) -> bool {
        if std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(&kind) {
            self.advance();
            true
        } else { false }
    }

    fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }

    fn check_kw(&self, kw: Keyword) -> bool {
        self.peek().kind == TokenKind::Kw(kw)
    }

    /// Skip newlines (used where newlines are not significant).
    fn skip_newlines(&mut self) {
        while matches!(self.peek().kind, TokenKind::Newline) {
            self.advance();
        }
    }

    /// Expect a newline (or EOF) at end of statement.
    fn expect_newline(&mut self) {
        match self.peek().kind {
            TokenKind::Newline | TokenKind::Eof => { self.advance(); }
            _ => {
                let sp = self.span();
                let got = self.peek().kind.clone();
                self.errors.push(e0001_unexpected_token(&got, "newline", sp));
                // Recover: skip to next newline
                while !matches!(self.peek().kind,
                    TokenKind::Newline | TokenKind::Eof | TokenKind::Dedent) {
                    self.advance();
                }
                if matches!(self.peek().kind, TokenKind::Newline) {
                    self.advance();
                }
            }
        }
    }

    /// Consume INDENT token or error.
    fn expect_indent(&mut self) -> bool {
        self.skip_newlines();
        if matches!(self.peek().kind, TokenKind::Indent) {
            self.advance();
            true
        } else {
            let sp = self.span();
            let got = self.peek().kind.clone();
            self.errors.push(e0005_missing_colon(sp));
            self.errors.push(e0001_unexpected_token(&got, "indented block", sp));
            false
        }
    }

    /// Consume DEDENT token.
    fn expect_dedent(&mut self) {
        self.skip_newlines();
        if matches!(self.peek().kind, TokenKind::Dedent) {
            self.advance();
        }
        // If no DEDENT, it's ok — might be EOF or already outdented
    }

    /// Parse a colon + INDENT (opening a block).
    fn expect_colon_indent(&mut self) -> bool {
        if !matches!(self.peek().kind, TokenKind::Colon) {
            let sp = self.span();
            self.errors.push(e0005_missing_colon(sp));
            return false;
        }
        self.advance(); // consume :
        self.expect_newline();
        self.expect_indent()
    }

    // ── Error recovery ────────────────────────────────────────────────────────

    /// Skip tokens until we find something we can restart parsing from.
    fn synchronize(&mut self) {
        loop {
            match &self.peek().kind {
                TokenKind::Eof => return,
                TokenKind::Kw(Keyword::Fn)
                | TokenKind::Kw(Keyword::Struct)
                | TokenKind::Kw(Keyword::Enum)
                | TokenKind::Kw(Keyword::Interface)
                | TokenKind::Kw(Keyword::Impl)
                | TokenKind::Kw(Keyword::Const)
                | TokenKind::Kw(Keyword::Var)
                | TokenKind::Kw(Keyword::Import)
                | TokenKind::Kw(Keyword::Module) => return,
                TokenKind::Dedent => { self.advance(); return; }
                _ => { self.advance(); }
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Top-level: file
    // ─────────────────────────────────────────────────────────────────────────

    pub fn parse_file(&mut self) -> File {
        let start = self.span();

        // Directives: #![no_std] etc.
        let mut directives = Vec::new();
        while matches!(self.peek().kind, TokenKind::HashBang) {
            directives.push(self.parse_directive());
            self.skip_newlines();
        }
        self.skip_newlines();

        // Module declaration (required)
        let module = if self.check_kw(Keyword::Module) {
            self.parse_module_decl()
        } else {
            let sp = self.span();
            self.errors.push(e0012_missing_module(sp));
            ModuleDecl { path: vec![], span: sp }
        };
        self.skip_newlines();

        // Imports
        let mut imports = Vec::new();
        while self.check_kw(Keyword::Import) || self.check_kw(Keyword::Use) {
            if let Some(imp) = self.parse_import() {
                imports.push(imp);
            }
            self.skip_newlines();
        }

        // Top-level items
        let mut items = Vec::new();
        while !self.is_eof() {
            self.skip_newlines();
            if self.is_eof() { break; }
            match self.parse_item() {
                Some(item) => items.push(item),
                None       => self.synchronize(),
            }
        }

        let end = self.span();
        File {
            directives,
            module,
            imports,
            items,
            span: start.to(end),
        }
    }

    fn parse_directive(&mut self) -> Directive {
        let start = self.span();
        self.advance(); // consume #![
        // The content was already captured by the lexer as HashBang token
        // For now, parse simple directives from the token stream
        // In practice the lexer captures the name inside #![ ]
        // We emit a placeholder directive
        Directive {
            name: "no_std".to_string(),
            arg:  None,
            span: start,
        }
    }

    fn parse_module_decl(&mut self) -> ModuleDecl {
        let start = self.span();
        self.advance(); // consume `module`
        let mut path = vec![self.parse_ident().unwrap_or_else(|| {
            Ident::new("?", self.span())
        })];
        while matches!(self.peek().kind, TokenKind::Dot) {
            self.advance();
            if let Some(seg) = self.parse_ident() {
                path.push(seg);
            }
        }
        self.expect_newline();
        ModuleDecl { path, span: start.to(self.span()) }
    }

    fn parse_import(&mut self) -> Option<Import> {
        let start = self.span();

        // use Direction.*
        if self.eat_kw(Keyword::Use) {
            let mut path_segs = vec![self.parse_ident()?];
            while matches!(self.peek().kind, TokenKind::Dot) {
                self.advance();
                if matches!(self.peek().kind, TokenKind::Star) {
                    self.advance();
                    break;
                }
                if let Some(seg) = self.parse_ident() {
                    path_segs.push(seg);
                }
            }
            self.expect_newline();
            return Some(Import {
                path: String::new(),
                kind: ImportKind::GlobUse { ty_path: path_segs },
                span: start.to(self.span()),
            });
        }

        // import "path" ...
        self.expect_kw(Keyword::Import);
        let path = match &self.peek().kind {
            TokenKind::Str(s) => { let s = s.clone(); self.advance(); s }
            _ => {
                let sp = self.span();
                let got = self.peek().kind.clone();
                self.errors.push(e0001_unexpected_token(&got, "string path", sp));
                return None;
            }
        };

        let kind = if self.eat_kw(Keyword::As) {
            let alias = self.parse_ident();
            self.expect_newline();
            ImportKind::Simple { alias }
        } else if matches!(self.peek().kind, TokenKind::Colon) {
            self.advance(); // consume :
            // {name, name, ...}
            let mut names = Vec::new();
            if matches!(self.peek().kind, TokenKind::LBrace) {
                self.advance();
                self.skip_newlines();
                while !matches!(self.peek().kind,
                    TokenKind::RBrace | TokenKind::Eof) {
                    if let Some(n) = self.parse_ident() { names.push(n); }
                    if !self.eat(TokenKind::Comma) { break; }
                    self.skip_newlines();
                }
                self.eat(TokenKind::RBrace);
            }
            self.expect_newline();
            ImportKind::Selective { names }
        } else {
            self.expect_newline();
            ImportKind::Simple { alias: None }
        };

        Some(Import { path, kind, span: start.to(self.span()) })
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Items
    // ─────────────────────────────────────────────────────────────────────────

    fn parse_item(&mut self) -> Option<Item> {
        let start   = self.span();
        let attrs   = self.parse_attributes();
        let is_pub  = self.eat_kw(Keyword::Pub);

        let kind = match &self.peek().kind {
            TokenKind::Kw(Keyword::Fn)        => {
                let f = self.parse_fn_decl(is_pub)?;
                ItemKind::Fn(f)
            }
            TokenKind::Kw(Keyword::Async)
            if matches!(self.peek2().kind, TokenKind::Kw(Keyword::Fn)) => {
                let f = self.parse_fn_decl(is_pub)?;
                ItemKind::Fn(f)
            }
            TokenKind::Kw(Keyword::Const)
            if matches!(self.peek2().kind, TokenKind::Kw(Keyword::Fn)) => {
                let f = self.parse_fn_decl(is_pub)?;
                ItemKind::Fn(f)
            }
            TokenKind::Kw(Keyword::Struct)    => ItemKind::Struct(self.parse_struct(is_pub)?),
            TokenKind::Kw(Keyword::Enum)      => ItemKind::Enum(self.parse_enum(is_pub)?),
            TokenKind::Kw(Keyword::Union)     => ItemKind::Union(self.parse_union(is_pub)?),
            TokenKind::Kw(Keyword::Interface) => ItemKind::Interface(self.parse_interface(is_pub)?),
            TokenKind::Kw(Keyword::Impl)      => self.parse_impl()?,
            TokenKind::Kw(Keyword::Type)      => ItemKind::TypeAlias(self.parse_type_alias(is_pub)?),
            TokenKind::Kw(Keyword::Newtype)   => ItemKind::Newtype(self.parse_newtype(is_pub)?),
            TokenKind::Kw(Keyword::Const)     => ItemKind::Const(self.parse_const(is_pub)?),
            TokenKind::Kw(Keyword::Var)
            | TokenKind::Kw(Keyword::Let)     => ItemKind::Var(self.parse_var_decl(is_pub)?),
            TokenKind::Kw(Keyword::Extern)    => ItemKind::Extern(self.parse_extern()?),
            _ => {
                let sp = self.span();
                let got = self.peek().kind.clone();
                self.errors.push(e0001_unexpected_token(&got, "item declaration", sp));
                return None;
            }
        };

        Some(Item { kind, attrs, span: start.to(self.span()) })
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Attributes
    // ─────────────────────────────────────────────────────────────────────────

    fn parse_attributes(&mut self) -> Vec<Attribute> {
        let mut attrs = Vec::new();
        while matches!(self.peek().kind, TokenKind::At) {
            attrs.push(self.parse_attribute());
            self.skip_newlines();
        }
        attrs
    }

    fn parse_attribute(&mut self) -> Attribute {
        let start = self.span();
        self.advance(); // consume @
        let name = self.parse_ident().unwrap_or_else(|| Ident::new("?", self.span()));
        let mut args = Vec::new();
        if matches!(self.peek().kind, TokenKind::LParen) {
            self.advance();
            while !matches!(self.peek().kind, TokenKind::RParen | TokenKind::Eof) {
                args.push(self.parse_attr_arg());
                if !self.eat(TokenKind::Comma) { break; }
            }
            self.eat(TokenKind::RParen);
        }
        Attribute { name, args, span: start.to(self.span()) }
    }

    fn parse_attr_arg(&mut self) -> AttrArg {
        let sp = self.span();
        match self.peek().kind.clone() {
            TokenKind::Ident(s) => {
                let ident = Ident::new(s, sp);
                self.advance();
                // key = value ?
                if matches!(self.peek().kind, TokenKind::Eq) {
                    self.advance();
                    let val = self.parse_attr_arg();
                    AttrArg::KeyValue { key: ident, val: Box::new(val) }
                } else {
                    AttrArg::Ident(ident)
                }
            }
            TokenKind::Kw(kw) => {
                let ident = Ident::new(kw.as_str(), sp);
                self.advance();
                AttrArg::Ident(ident)
            }
            TokenKind::Str(s) => { self.advance(); AttrArg::Str(s, sp) }
            TokenKind::Int(v, _) => { self.advance(); AttrArg::Int(v, sp) }
            TokenKind::Kw(Keyword::True) => { self.advance(); AttrArg::Bool(true, sp) }
            TokenKind::Kw(Keyword::False) => { self.advance(); AttrArg::Bool(false, sp) }
            _ => {
                self.advance();
                AttrArg::Ident(Ident::new("?", sp))
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Functions
    // ─────────────────────────────────────────────────────────────────────────

    fn parse_fn_decl(&mut self, is_pub: bool) -> Option<FnDecl> {
        let start    = self.span();
        let is_async = self.eat_kw(Keyword::Async);
        let is_const = self.eat_kw(Keyword::Const);
        self.expect_kw(Keyword::Fn);

        // fn Name  or  fn Type.method
        let name = self.parse_fn_name()?;
        let generics = self.parse_generic_params();

        // Parameters
        self.expect_kind(&TokenKind::LParen, "`(`")?;
        let params = self.parse_param_list();
        self.expect_kind(&TokenKind::RParen, "`)`")?;

        // Return type
        let ret_ty = if matches!(self.peek().kind, TokenKind::Arrow) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        // Body: either a block or just a newline (for extern/header declarations)
        let body = if matches!(self.peek().kind, TokenKind::Colon) {
            Some(self.parse_fn_body()?)
        } else {
            self.expect_newline();
            None
        };

        Some(FnDecl {
            name, generics, params, ret_ty, body,
            is_pub, is_async, is_const,
            span: start.to(self.span()),
        })
    }

    fn parse_fn_name(&mut self) -> Option<FnName> {
        let start = self.span();
        let first = self.parse_ident()?;

        // Check for Type.method pattern
        if matches!(self.peek().kind, TokenKind::Dot) {
            self.advance();
            let method = self.parse_ident()?;
            Some(FnName::Method {
                ty_name: first,
                method,
                span: start.to(self.span()),
            })
        } else {
            Some(FnName::Simple(first))
        }
    }

    fn parse_param_list(&mut self) -> Vec<Param> {
        let mut params = Vec::new();
        while !matches!(self.peek().kind, TokenKind::RParen | TokenKind::Eof) {
            if let Some(p) = self.parse_param() {
                params.push(p);
            }
            if !self.eat(TokenKind::Comma) { break; }
        }
        params
    }

    fn parse_param(&mut self) -> Option<Param> {
        let start = self.span();

        // Variadic: ...name: ...Type
        if matches!(self.peek().kind, TokenKind::DotDot) {
            self.advance(); // consume ..
            if matches!(self.peek().kind, TokenKind::Dot) { self.advance(); } // consume third .
            let name = self.parse_ident().unwrap_or_else(|| Ident::new("?", self.span()));
            self.expect_kind(&TokenKind::Colon, "`:`")?;
            // skip ... in type
            if matches!(self.peek().kind, TokenKind::DotDot) { self.advance(); }
            if matches!(self.peek().kind, TokenKind::Dot)    { self.advance(); }
            let ty = self.parse_type()?;
            return Some(Param {
                kind: ParamKind::Variadic { name, ty },
                span: start.to(self.span()),
            });
        }

        // Self param: self  or  self: *Foo  or  self: &Foo
        if self.check_kw(Keyword::SelfValue) {
            self.advance();
            let ty_annot = if matches!(self.peek().kind, TokenKind::Colon) {
                self.advance();
                self.parse_type()
            } else { None };
            return Some(Param {
                kind: ParamKind::SelfParam { mutable: false, ty_annot },
                span: start.to(self.span()),
            });
        }

        // Regular param: name: Type  or  name: Type = default
        let name = self.parse_ident()?;
        self.expect_kind(&TokenKind::Colon, "`:`")?;
        let ty = self.parse_type()?;
        let default = if matches!(self.peek().kind, TokenKind::Eq) {
            self.advance();
            self.parse_expr(0)
        } else { None };

        Some(Param {
            kind: ParamKind::Regular { name, ty, default },
            span: start.to(self.span()),
        })
    }

    fn parse_fn_body(&mut self) -> Option<Block> {
        self.advance(); // consume :

        // Single-expression shorthand: fn square(x: i32) -> i32: x * x
        if !matches!(self.peek().kind, TokenKind::Newline) {
            let start = self.span();
            let expr  = self.parse_expr(0)?;
            self.expect_newline();
            let span  = start.to(self.span());
            return Some(Block {
                stmts: vec![Stmt { kind: StmtKind::Expr(expr), span }],
                span,
            });
        }

        // Regular block
        self.expect_newline();
        self.parse_block()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Structs
    // ─────────────────────────────────────────────────────────────────────────

    fn parse_struct(&mut self, is_pub: bool) -> Option<StructDecl> {
        let start = self.span();
        self.advance(); // consume `struct`
        let name     = self.parse_ident()?;
        let generics = self.parse_generic_params();
        let fields   = self.parse_struct_body();
        Some(StructDecl { name, generics, fields, is_pub, span: start.to(self.span()) })
    }

    fn parse_struct_body(&mut self) -> Vec<StructField> {
        if !self.expect_colon_indent() { return vec![]; }
        let mut fields = Vec::new();
        while !matches!(self.peek().kind, TokenKind::Dedent | TokenKind::Eof) {
            self.skip_newlines();
            if matches!(self.peek().kind, TokenKind::Dedent | TokenKind::Eof) { break; }
            let f_start  = self.span();
            let f_attrs  = self.parse_attributes();
            let f_pub    = self.eat_kw(Keyword::Pub);
            let f_embed  = self.eat_kw(Keyword::Embed);
            if let Some(name) = self.parse_ident() {
                if matches!(self.peek().kind, TokenKind::Colon) {
                    self.advance();
                    if let Some(ty) = self.parse_type() {
                        let default = if matches!(self.peek().kind, TokenKind::Eq) {
                            self.advance();
                            self.parse_expr(0)
                        } else { None };
                        self.expect_newline();
                        fields.push(StructField {
                            attrs: f_attrs, is_pub: f_pub, is_embed: f_embed,
                            name, ty, default, span: f_start.to(self.span()),
                        });
                    }
                }
            } else {
                self.synchronize();
            }
        }
        self.expect_dedent();
        fields
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Enums
    // ─────────────────────────────────────────────────────────────────────────

    fn parse_enum(&mut self, is_pub: bool) -> Option<EnumDecl> {
        let start    = self.span();
        self.advance(); // consume `enum`
        let name     = self.parse_ident()?;
        let generics = self.parse_generic_params();
        if !self.expect_colon_indent() {
            return Some(EnumDecl { name, generics, variants: vec![], is_pub, span: start.to(self.span()) });
        }
        let mut variants = Vec::new();
        while !matches!(self.peek().kind, TokenKind::Dedent | TokenKind::Eof) {
            self.skip_newlines();
            if matches!(self.peek().kind, TokenKind::Dedent | TokenKind::Eof) { break; }
            let v_start = self.span();
            let v_attrs = self.parse_attributes();
            if let Some(vname) = self.parse_ident() {
                let kind = if matches!(self.peek().kind, TokenKind::LParen) {
                    self.advance();
                    let mut fields = Vec::new();
                    while !matches!(self.peek().kind, TokenKind::RParen | TokenKind::Eof) {
                        // Named field: name: Type
                        if matches!(self.peek().kind, TokenKind::Ident(_))
                            && matches!(self.peek2().kind, TokenKind::Colon) {
                            let n = self.parse_ident().unwrap();
                            self.advance(); // :
                            if let Some(ty) = self.parse_type() {
                                fields.push((n, ty));
                            }
                        } else if let Some(ty) = self.parse_type() {
                            // positional — use empty name
                            fields.push((Ident::new("", ty.span()), ty));
                        }
                        if !self.eat(TokenKind::Comma) { break; }
                    }
                    self.eat(TokenKind::RParen);
                    VariantKind::Named(fields)
                } else {
                    VariantKind::Unit
                };
                let value = if matches!(self.peek().kind, TokenKind::Eq) {
                    self.advance();
                    self.parse_expr(0)
                } else { None };
                self.expect_newline();
                variants.push(EnumVariant {
                    attrs: v_attrs, name: vname, kind, value,
                    span: v_start.to(self.span()),
                });
            } else {
                self.synchronize();
            }
        }
        self.expect_dedent();
        Some(EnumDecl { name, generics, variants, is_pub, span: start.to(self.span()) })
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Union
    // ─────────────────────────────────────────────────────────────────────────

    fn parse_union(&mut self, is_pub: bool) -> Option<UnionDecl> {
        let start = self.span();
        self.advance();
        let name = self.parse_ident()?;
        if !self.expect_colon_indent() {
            return Some(UnionDecl { name, fields: vec![], is_pub, span: start.to(self.span()) });
        }
        let mut fields = Vec::new();
        while !matches!(self.peek().kind, TokenKind::Dedent | TokenKind::Eof) {
            self.skip_newlines();
            if matches!(self.peek().kind, TokenKind::Dedent | TokenKind::Eof) { break; }
            let f_start = self.span();
            if let Some(fname) = self.parse_ident() {
                self.expect_kind(&TokenKind::Colon, "`:`");
                if let Some(ty) = self.parse_type() {
                    self.expect_newline();
                    fields.push(UnionField { name: fname, ty, span: f_start.to(self.span()) });
                }
            }
        }
        self.expect_dedent();
        Some(UnionDecl { name, fields, is_pub, span: start.to(self.span()) })
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Interface
    // ─────────────────────────────────────────────────────────────────────────

    fn parse_interface(&mut self, is_pub: bool) -> Option<InterfaceDecl> {
        let start    = self.span();
        self.advance();
        let name     = self.parse_ident()?;
        let generics = self.parse_generic_params();

        // Optional supers: interface Widget: Drawable, Serializable:
        let mut supers = Vec::new();
        if matches!(self.peek().kind, TokenKind::Colon) {
            // Peek ahead: if next line is indented, it's the block colon.
            // If on same line, it's supers.
            self.advance();
            if !matches!(self.peek().kind, TokenKind::Newline) {
                // Parse super types until :
                while !matches!(self.peek().kind,
                    TokenKind::Colon | TokenKind::Newline | TokenKind::Eof) {
                    if let Some(ty) = self.parse_type() { supers.push(ty); }
                    if !self.eat(TokenKind::Comma) { break; }
                }
            }
        }

        if !self.expect_colon_indent() {
            return Some(InterfaceDecl { name, generics, supers, items: vec![], is_pub, span: start.to(self.span()) });
        }

        let mut items = Vec::new();
        while !matches!(self.peek().kind, TokenKind::Dedent | TokenKind::Eof) {
            self.skip_newlines();
            if matches!(self.peek().kind, TokenKind::Dedent | TokenKind::Eof) { break; }

            let i_start = self.span();
            let i_attrs = self.parse_attributes();

            if self.check_kw(Keyword::Type) {
                self.advance();
                let tname = self.parse_ident().unwrap_or_else(|| Ident::new("?", self.span()));
                let default = if matches!(self.peek().kind, TokenKind::Eq) {
                    self.advance(); self.parse_type()
                } else { None };
                self.expect_newline();
                items.push(InterfaceItem::AssocType {
                    name: tname, default,
                    span: i_start.to(self.span()),
                });
            } else {
                let f = self.parse_fn_decl(false);
                if let Some(mut fd) = f {
                    fd.span = i_start.to(self.span());
                    items.push(InterfaceItem::Method(fd));
                }
            }
        }
        self.expect_dedent();
        Some(InterfaceDecl { name, generics, supers, items, is_pub, span: start.to(self.span()) })
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Impl
    // ─────────────────────────────────────────────────────────────────────────

    fn parse_impl(&mut self) -> Option<ItemKind> {
        let start    = self.span();
        self.advance(); // consume `impl`
        let generics = self.parse_generic_params();
        let first_ty = self.parse_type()?;

        // impl Interface for Type:
        if self.eat_kw(Keyword::For) {
            let for_ty = self.parse_type()?;
            let where_ = self.parse_where_clause();
            let (methods, assoc_types) = self.parse_impl_body();
            return Some(ItemKind::ImplFor(ImplFor {
                generics, interface: first_ty, for_ty, where_,
                methods, assoc_types,
                span: start.to(self.span()),
            }));
        }

        // impl Struct:
        let where_ = self.parse_where_clause();
        let (methods, _) = self.parse_impl_body();
        // Ignore where clause for plain impl blocks (future extension)
        let _ = where_;
        Some(ItemKind::ImplBlock(ImplBlock {
            ty: first_ty, methods,
            span: start.to(self.span()),
        }))
    }

    fn parse_impl_body(&mut self) -> (Vec<FnDecl>, Vec<AssocTypeImpl>) {
        if !self.expect_colon_indent() { return (vec![], vec![]); }
        let mut methods     = Vec::new();
        let mut assoc_types = Vec::new();
        while !matches!(self.peek().kind, TokenKind::Dedent | TokenKind::Eof) {
            self.skip_newlines();
            if matches!(self.peek().kind, TokenKind::Dedent | TokenKind::Eof) { break; }
            let _attrs = self.parse_attributes();
            if self.check_kw(Keyword::Type) {
                let i_start = self.span();
                self.advance();
                let name = self.parse_ident().unwrap_or_else(|| Ident::new("?", self.span()));
                self.expect_kind(&TokenKind::Eq, "`=`");
                if let Some(ty) = self.parse_type() {
                    self.expect_newline();
                    assoc_types.push(AssocTypeImpl { name, ty, span: i_start.to(self.span()) });
                }
            } else if let Some(f) = self.parse_fn_decl(false) {
                methods.push(f);
            } else {
                self.synchronize();
            }
        }
        self.expect_dedent();
        (methods, assoc_types)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Other top-level items
    // ─────────────────────────────────────────────────────────────────────────

    fn parse_type_alias(&mut self, is_pub: bool) -> Option<TypeAlias> {
        let start    = self.span();
        self.advance();
        let name     = self.parse_ident()?;
        let generics = self.parse_generic_params();
        self.expect_kind(&TokenKind::Eq, "`=`")?;
        let ty       = self.parse_type()?;
        self.expect_newline();
        Some(TypeAlias { name, generics, ty, is_pub, span: start.to(self.span()) })
    }

    fn parse_newtype(&mut self, is_pub: bool) -> Option<NewtypeDecl> {
        let start = self.span();
        self.advance();
        let name  = self.parse_ident()?;
        self.expect_kind(&TokenKind::Eq, "`=`")?;
        let ty    = self.parse_type()?;
        self.expect_newline();
        Some(NewtypeDecl { name, ty, is_pub, span: start.to(self.span()) })
    }

    fn parse_const(&mut self, is_pub: bool) -> Option<ConstDecl> {
        let start = self.span();
        self.advance();
        let name  = self.parse_ident()?;
        self.expect_kind(&TokenKind::Colon, "`:`")?;
        let ty    = self.parse_type()?;
        self.expect_kind(&TokenKind::Eq, "`=`")?;
        let value = self.parse_expr(0)?;
        self.expect_newline();
        Some(ConstDecl { name, ty, value, is_pub, span: start.to(self.span()) })
    }

    fn parse_var_decl(&mut self, is_pub: bool) -> Option<VarDecl> {
        let start  = self.span();
        let is_let = self.eat_kw(Keyword::Let);
        if !is_let { self.expect_kw(Keyword::Var); }

        let mut names = vec![self.parse_ident()?];
        while self.eat(TokenKind::Comma) {
            if let Some(n) = self.parse_ident() { names.push(n); }
        }

        // var x: T = val   or   var x := val   or   var x: T
        let (ty, value) = if matches!(self.peek().kind, TokenKind::ColonEq) {
            self.advance();
            let expr = self.parse_expr(0)?;
            (None, Some(vec![expr]))
        } else {
            self.expect_kind(&TokenKind::Colon, "`:`")?;
            let ty = self.parse_type();
            let val = if matches!(self.peek().kind, TokenKind::Eq) {
                self.advance();
                let mut vals = vec![self.parse_expr(0)?];
                while self.eat(TokenKind::Comma) {
                    if let Some(e) = self.parse_expr(0) { vals.push(e); }
                }
                Some(vals)
            } else { None };
            (ty, val)
        };

        self.expect_newline();
        Some(VarDecl { names, ty, value, is_let, is_pub, span: start.to(self.span()) })
    }

    fn parse_extern(&mut self) -> Option<ExternDecl> {
        let start = self.span();
        self.advance(); // consume `extern`
        // Optional ABI string
        let abi = if matches!(self.peek().kind, TokenKind::Str(_)) {
            if let TokenKind::Str(s) = self.advance().kind.clone() { Some(s) } else { None }
        } else { None };

        // fn or var
        let kind = if self.check_kw(Keyword::Fn) {
            self.advance();
            let name = self.parse_ident()?;
            self.expect_kind(&TokenKind::LParen, "`(`")?;
            let mut params  = Vec::new();
            let mut variadic = false;
            while !matches!(self.peek().kind, TokenKind::RParen | TokenKind::Eof) {
                if matches!(self.peek().kind, TokenKind::DotDot) {
                    self.advance();
                    if matches!(self.peek().kind, TokenKind::Dot) { self.advance(); }
                    variadic = true;
                    break;
                }
                if let Some(ty) = self.parse_type() { params.push(ty); }
                if !self.eat(TokenKind::Comma) { break; }
            }
            self.eat(TokenKind::RParen);
            let ret = if matches!(self.peek().kind, TokenKind::Arrow) {
                self.advance();
                self.parse_type().unwrap_or(Type::Void(self.span()))
            } else { Type::Void(self.span()) };
            self.expect_newline();
            ExternKind::Fn { name, params, ret, variadic }
        } else {
            self.expect_kw(Keyword::Var);
            let name = self.parse_ident()?;
            self.expect_kind(&TokenKind::Colon, "`:`")?;
            let ty = self.parse_type()?;
            self.expect_newline();
            ExternKind::Var { name, ty }
        };

        Some(ExternDecl { abi, kind, span: start.to(self.span()) })
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Blocks & Statements
    // ─────────────────────────────────────────────────────────────────────────

    fn parse_block(&mut self) -> Option<Block> {
        let start = self.span();
        if !self.expect_indent() { return None; }
        let mut stmts = Vec::new();
        while !matches!(self.peek().kind, TokenKind::Dedent | TokenKind::Eof) {
            self.skip_newlines();
            if matches!(self.peek().kind, TokenKind::Dedent | TokenKind::Eof) { break; }
            if let Some(s) = self.parse_stmt() {
                stmts.push(s);
            } else {
                self.synchronize();
            }
        }
        self.expect_dedent();
        Some(Block { stmts, span: start.to(self.span()) })
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        let start = self.span();

        let kind = match &self.peek().kind.clone() {
            TokenKind::Kw(Keyword::Var) | TokenKind::Kw(Keyword::Let) => {
                StmtKind::Var(self.parse_var_decl(false)?)
            }
            TokenKind::Kw(Keyword::Return) => {
                self.advance();
                let val = if !matches!(self.peek().kind,
                    TokenKind::Newline | TokenKind::Eof | TokenKind::Dedent) {
                    self.parse_expr(0)
                } else { None };
                self.expect_newline();
                StmtKind::Return(val)
            }
            TokenKind::Kw(Keyword::Break) => {
                self.advance();
                let label = self.parse_label_ref();
                self.expect_newline();
                StmtKind::Break(label)
            }
            TokenKind::Kw(Keyword::Continue) => {
                self.advance();
                let label = self.parse_label_ref();
                self.expect_newline();
                StmtKind::Continue(label)
            }
            TokenKind::Kw(Keyword::Defer) => {
                self.advance();
                let inner = Box::new(self.parse_stmt()?);
                StmtKind::Defer(inner)
            }
            TokenKind::Kw(Keyword::If) => {
                StmtKind::If(self.parse_if_stmt()?)
            }
            TokenKind::Kw(Keyword::While) => {
                self.parse_while_stmt(None)?
            }
            TokenKind::Kw(Keyword::For) => {
                self.parse_for_stmt(None)?
            }
            TokenKind::Kw(Keyword::Loop) => {
                self.parse_loop_stmt(None)?
            }
            TokenKind::Kw(Keyword::Match) => {
                StmtKind::Match(self.parse_match_stmt()?)
            }
            TokenKind::Kw(Keyword::Try) => {
                StmtKind::Try(self.parse_try_stmt()?)
            }
            TokenKind::Kw(Keyword::Unsafe) => {
                self.advance();
                self.advance(); // consume :
                self.expect_newline();
                let block = self.parse_block()?;
                StmtKind::Unsafe(block)
            }
            TokenKind::Kw(Keyword::Comptime) => {
                self.advance();
                let inner = Box::new(self.parse_stmt()?);
                StmtKind::Comptime(inner)
            }
            // Label: @outer while ...
            TokenKind::At => {
                self.advance();
                let label_name = self.parse_ident()?;
                let label = Some(Label { name: label_name.name.clone(), span: label_name.span });
                // Expect loop keyword
                match &self.peek().kind.clone() {
                    TokenKind::Kw(Keyword::While) => self.parse_while_stmt(label)?,
                    TokenKind::Kw(Keyword::For)   => self.parse_for_stmt(label)?,
                    TokenKind::Kw(Keyword::Loop)  => self.parse_loop_stmt(label)?,
                    _ => {
                        let sp = self.span();
                        let got = self.peek().kind.clone();
                        self.errors.push(e0001_unexpected_token(&got, "loop keyword after label", sp));
                        return None;
                    }
                }
            }
            // Underscore discard: _ := expr
            TokenKind::Underscore => {
                self.advance();
                self.expect_kind(&TokenKind::ColonEq, "`:=`")?;
                let val = self.parse_expr(0)?;
                self.expect_newline();
                StmtKind::Expr(val)
            }
            _ => {
                // Expression or assignment
                let expr = self.parse_expr(0)?;

                // Check for assignment
                if let Some(op) = self.peek_assign_op() {
                    self.advance();
                    let value = self.parse_expr(0)?;
                    self.expect_newline();
                    StmtKind::Assign { target: expr, op, value }
                } else {
                    self.expect_newline();
                    StmtKind::Expr(expr)
                }
            }
        };

        Some(Stmt { kind, span: start.to(self.span()) })
    }

    fn peek_assign_op(&self) -> Option<AssignOp> {
        match &self.peek().kind {
            TokenKind::Eq        => Some(AssignOp::Eq),
            TokenKind::PlusEq    => Some(AssignOp::AddEq),
            TokenKind::MinusEq   => Some(AssignOp::SubEq),
            TokenKind::StarEq    => Some(AssignOp::MulEq),
            TokenKind::SlashEq   => Some(AssignOp::DivEq),
            TokenKind::PercentEq => Some(AssignOp::RemEq),
            TokenKind::StarStarEq=> Some(AssignOp::PowEq),
            TokenKind::AmpEq     => Some(AssignOp::AndEq),
            TokenKind::BarEq     => Some(AssignOp::OrEq),
            TokenKind::CaretEq   => Some(AssignOp::XorEq),
            TokenKind::LtLtEq    => Some(AssignOp::ShlEq),
            TokenKind::GtGtEq    => Some(AssignOp::ShrEq),
            TokenKind::PlusPctEq => Some(AssignOp::AddWrapEq),
            TokenKind::MinusPctEq=> Some(AssignOp::SubWrapEq),
            TokenKind::StarPctEq => Some(AssignOp::MulWrapEq),
            TokenKind::PlusBarEq => Some(AssignOp::AddSatEq),
            TokenKind::MinusBarEq=> Some(AssignOp::SubSatEq),
            TokenKind::StarBarEq => Some(AssignOp::MulSatEq),
            _ => None,
        }
    }

    fn parse_if_stmt(&mut self) -> Option<IfStmt> {
        let start = self.span();
        self.advance(); // consume `if`
        let cond  = self.parse_expr(0)?;
        self.advance(); // consume `:`
        self.expect_newline();
        let then  = self.parse_block()?;

        let mut elifs = Vec::new();
        let mut else_ = None;

        loop {
            self.skip_newlines();
            if self.check_kw(Keyword::Elif) {
                self.advance();
                let ec = self.parse_expr(0)?;
                self.advance(); // :
                self.expect_newline();
                let eb = self.parse_block()?;
                elifs.push((ec, eb));
            } else if self.check_kw(Keyword::Else) {
                self.advance();
                self.advance(); // :
                self.expect_newline();
                else_ = self.parse_block();
                break;
            } else {
                break;
            }
        }

        Some(IfStmt { cond, then, elifs, else_, span: start.to(self.span()) })
    }

    fn parse_while_stmt(&mut self, label: Option<Label>) -> Option<StmtKind> {
        self.advance(); // consume `while`
        let cond = self.parse_expr(0)?;
        self.advance(); // :
        self.expect_newline();
        let body = self.parse_block()?;
        Some(StmtKind::While { label, cond, body })
    }

    fn parse_for_stmt(&mut self, label: Option<Label>) -> Option<StmtKind> {
        self.advance(); // consume `for`
        let pat   = self.parse_for_pat()?;
        self.expect_kw(Keyword::In);
        let iter  = self.parse_expr(0)?;
        let step  = if self.eat_kw(Keyword::Step) { self.parse_expr(0) } else { None };
        self.advance(); // :
        self.expect_newline();
        let body  = self.parse_block()?;
        Some(StmtKind::For { label, pat, iter, step, body })
    }

    fn parse_for_pat(&mut self) -> Option<ForPat> {
        let start = self.span();
        if matches!(self.peek().kind, TokenKind::Underscore) {
            self.advance();
            return Some(ForPat { kind: ForPatKind::Discard, span: start });
        }
        let first = self.parse_ident()?;
        if matches!(self.peek().kind, TokenKind::Comma) {
            self.advance();
            let second = self.parse_ident()?;
            Some(ForPat { kind: ForPatKind::IndexValue(first, second), span: start.to(self.span()) })
        } else {
            Some(ForPat { kind: ForPatKind::Single(first), span: start })
        }
    }

    fn parse_loop_stmt(&mut self, label: Option<Label>) -> Option<StmtKind> {
        self.advance(); // consume `loop`
        self.advance(); // :
        self.expect_newline();
        let body = self.parse_block()?;
        Some(StmtKind::Loop { label, body })
    }

    fn parse_match_stmt(&mut self) -> Option<MatchStmt> {
        let start = self.span();
        self.advance(); // consume `match`
        let scrutinee = self.parse_expr(0)?;
        self.advance(); // :
        self.expect_newline();
        if !self.expect_indent() { return None; }

        let mut arms = Vec::new();
        while !matches!(self.peek().kind, TokenKind::Dedent | TokenKind::Eof) {
            self.skip_newlines();
            if matches!(self.peek().kind, TokenKind::Dedent | TokenKind::Eof) { break; }
            if let Some(arm) = self.parse_match_arm() { arms.push(arm); }
        }
        self.expect_dedent();
        Some(MatchStmt { scrutinee, arms, span: start.to(self.span()) })
    }

    fn parse_match_arm(&mut self) -> Option<MatchArm> {
        let start = self.span();
        // Parse one or more patterns separated by |
        let mut pats = vec![self.parse_pat()?];
        while matches!(self.peek().kind, TokenKind::Bar) {
            self.advance();
            if let Some(p) = self.parse_pat() { pats.push(p); }
        }

        // Optional guard
        let guard = if self.check_kw(Keyword::If) {
            self.advance();
            self.parse_expr(0)
        } else { None };

        // =>
        self.expect_kind(&TokenKind::FatArrow, "`=>`")?;

        // Body: expr on same line, or block on next line
        let body = if matches!(self.peek().kind, TokenKind::Newline) {
            self.advance();
            let block = self.parse_block()?;
            MatchBody::Block(block)
        } else {
            let expr = self.parse_expr(0)?;
            self.expect_newline();
            MatchBody::Expr(expr)
        };

        Some(MatchArm { pats, guard, body, span: start.to(self.span()) })
    }

    fn parse_try_stmt(&mut self) -> Option<TryStmt> {
        let start = self.span();
        self.advance(); // consume `try`
        self.advance(); // :
        self.expect_newline();
        let body = self.parse_block()?;

        let mut catches = Vec::new();
        loop {
            self.skip_newlines();
            if !self.check_kw(Keyword::Catch) { break; }
            let c_start = self.span();
            self.advance();
            let name = self.parse_ident().unwrap_or_else(|| Ident::new("e", self.span()));
            self.expect_kind(&TokenKind::Colon, "`:`")?;
            let ty = self.parse_type()?;
            self.advance(); // :
            self.expect_newline();
            let cbody = self.parse_block()?;
            catches.push(CatchClause { name, ty, body: cbody, span: c_start.to(self.span()) });
        }

        let finally = if self.check_kw(Keyword::Finally) {
            self.advance();
            self.advance(); // :
            self.expect_newline();
            self.parse_block()
        } else { None };

        Some(TryStmt { body, catches, finally, span: start.to(self.span()) })
    }

    fn parse_label_ref(&mut self) -> Option<Label> {
        if matches!(self.peek().kind, TokenKind::At) {
            self.advance();
            let name = self.parse_ident()?;
            Some(Label { name: name.name, span: name.span })
        } else { None }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Patterns
    // ─────────────────────────────────────────────────────────────────────────

    fn parse_pat(&mut self) -> Option<Pat> {
        let start = self.span();
        let kind = match self.peek().kind.clone() {
            TokenKind::Underscore => { self.advance(); PatKind::Wildcard }
            TokenKind::Kw(Keyword::Null)  => { self.advance(); PatKind::Null }
            TokenKind::Kw(Keyword::True)  => { self.advance(); PatKind::Bool(true) }
            TokenKind::Kw(Keyword::False) => { self.advance(); PatKind::Bool(false) }
            TokenKind::Int(v, s)  => { self.advance(); PatKind::Int(v, s) }
            TokenKind::Float(v,s) => { self.advance(); PatKind::Float(v, s) }
            TokenKind::Str(s)     => { self.advance(); PatKind::Str(s) }
            TokenKind::Char(c)    => { self.advance(); PatKind::Char(c) }
            TokenKind::LParen     => {
                self.advance();
                let mut pats = Vec::new();
                while !matches!(self.peek().kind, TokenKind::RParen | TokenKind::Eof) {
                    if let Some(p) = self.parse_pat() { pats.push(p); }
                    if !self.eat(TokenKind::Comma) { break; }
                }
                self.eat(TokenKind::RParen);
                PatKind::Tuple(pats)
            }
            TokenKind::Ident(_) | TokenKind::Kw(_) => {
                // Could be:  Binding    MyEnum.Variant    MyEnum.Variant(fields)
                let mut segments = vec![self.parse_ident()?];
                while matches!(self.peek().kind, TokenKind::Dot) {
                    self.advance();
                    if let Some(seg) = self.parse_ident() { segments.push(seg); }
                }
                if matches!(self.peek().kind, TokenKind::LParen) {
                    self.advance();
                    let mut fields = Vec::new();
                    while !matches!(self.peek().kind, TokenKind::RParen | TokenKind::Eof) {
                        if let Some(p) = self.parse_pat() { fields.push(p); }
                        if !self.eat(TokenKind::Comma) { break; }
                    }
                    self.eat(TokenKind::RParen);
                    PatKind::TupleStruct(segments, fields)
                } else if segments.len() == 1 {
                    PatKind::Ident(segments.remove(0))
                } else {
                    PatKind::Path(segments)
                }
            }
            _ => {
                let sp = self.span();
                let got = self.peek().kind.clone();
                self.errors.push(e0001_unexpected_token(&got, "pattern", sp));
                return None;
            }
        };
        Some(Pat { kind, span: start.to(self.span()) })
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Expressions — Pratt parser
    // ─────────────────────────────────────────────────────────────────────────

    pub fn parse_expr(&mut self, min_bp: u8) -> Option<Expr> {
        let start = self.span();
        let mut lhs = self.parse_unary_or_primary()?;

        loop {
            // Postfix operators
            lhs = match &self.peek().kind {
                TokenKind::Dot => {
                    self.advance();
                    let field = self.parse_ident()?;
                    let sp = start.to(self.span());
                    // Is it a method call?
                    if matches!(self.peek().kind, TokenKind::LParen) {
                        self.advance();
                        let args = self.parse_call_args();
                        self.eat(TokenKind::RParen);
                        Expr::new(ExprKind::Call {
                            callee: Box::new(Expr::new(
                                ExprKind::Field { base: Box::new(lhs), field: field.clone() }, sp)),
                            args,
                        }, sp)
                    } else {
                        Expr::new(ExprKind::Field { base: Box::new(lhs), field }, sp)
                    }
                }
                TokenKind::Arrow => {
                    self.advance();
                    let field = self.parse_ident()?;
                    let sp = start.to(self.span());
                    Expr::new(ExprKind::PtrField { base: Box::new(lhs), field }, sp)
                }
                TokenKind::QDot => {
                    self.advance();
                    let field = self.parse_ident()?;
                    let sp = start.to(self.span());
                    if matches!(self.peek().kind, TokenKind::LParen) {
                        self.advance();
                        let args = self.parse_call_args();
                        self.eat(TokenKind::RParen);
                        Expr::new(ExprKind::OptCall { base: Box::new(lhs), method: field, args }, sp)
                    } else {
                        Expr::new(ExprKind::OptField { base: Box::new(lhs), field }, sp)
                    }
                }
                TokenKind::LParen => {
                    self.advance();
                    let args = self.parse_call_args();
                    self.eat(TokenKind::RParen);
                    let sp = start.to(self.span());
                    Expr::new(ExprKind::Call { callee: Box::new(lhs), args }, sp)
                }
                TokenKind::LBracket => {
                    self.advance();
                    let lo = if matches!(self.peek().kind, TokenKind::Colon) {
                        None
                    } else {
                        self.parse_expr(0)
                    };
                    if matches!(self.peek().kind, TokenKind::Colon) {
                        self.advance();
                        let hi = if matches!(self.peek().kind, TokenKind::RBracket) {
                            None
                        } else {
                            self.parse_expr(0)
                        };
                        self.eat(TokenKind::RBracket);
                        let sp = start.to(self.span());
                        Expr::new(ExprKind::Slice {
                            base: Box::new(lhs),
                            lo:   lo.map(Box::new),
                            hi:   hi.map(Box::new),
                        }, sp)
                    } else {
                        self.eat(TokenKind::RBracket);
                        let sp = start.to(self.span());
                        Expr::new(ExprKind::Index {
                            base:  Box::new(lhs),
                            index: Box::new(lo?),
                        }, sp)
                    }
                }
                TokenKind::Question => {
                    self.advance();
                    let sp = start.to(self.span());
                    Expr::new(ExprKind::Propagate(Box::new(lhs)), sp)
                }
                TokenKind::Bang => {
                    // Only if NOT followed by = (that would be !=)
                    if !matches!(self.peek2().kind, TokenKind::Eq) {
                        self.advance();
                        let sp = start.to(self.span());
                        Expr::new(ExprKind::Unwrap(Box::new(lhs)), sp)
                    } else {
                        break;
                    }
                }
                // `as`, `as!`, `as?`
                TokenKind::Kw(Keyword::As) => {
                    self.advance();
                    let kind = match self.peek().kind {
                        TokenKind::Bang     => { self.advance(); CastKind::Assert }
                        TokenKind::Question => { self.advance(); CastKind::Try    }
                        _                   =>                   CastKind::Safe,
                    };
                    let ty = self.parse_type()?;
                    let sp = start.to(self.span());
                    Expr::new(ExprKind::Cast { expr: Box::new(lhs), ty: Box::new(ty), kind }, sp)
                }
                // `is`
                TokenKind::Kw(Keyword::Is) => {
                    self.advance();
                    let ty = self.parse_type()?;
                    let sp = start.to(self.span());
                    Expr::new(ExprKind::Is { expr: Box::new(lhs), ty: Box::new(ty) }, sp)
                }
                _ => break,
            };

            // Infix operators (Pratt)
            let Some((left_bp, right_bp)) = self.peek_infix_bp() else { break };
            if left_bp < min_bp { break; }

            let op_tok = self.advance().kind.clone();
            let op = token_to_binop(&op_tok);
            let rhs = self.parse_expr(right_bp)?;
            let sp = start.to(self.span());
            lhs = Expr::new(ExprKind::BinOp { op, lhs: Box::new(lhs), rhs: Box::new(rhs) }, sp);
        }

        Some(lhs)
    }

    /// Returns (left_bp, right_bp) for infix operators, per spec §67.9.
    fn peek_infix_bp(&self) -> Option<(u8, u8)> {
        match &self.peek().kind {
            TokenKind::PipeGt           => Some((1, 2)),
            TokenKind::QQ               => Some((3, 4)),
            TokenKind::Kw(Keyword::Or)  => Some((5, 6)),
            TokenKind::Kw(Keyword::And) => Some((7, 8)),
            TokenKind::EqEq | TokenKind::BangEq
            | TokenKind::Lt  | TokenKind::Gt
            | TokenKind::LtEq| TokenKind::GtEq => Some((9, 0)),  // non-assoc
            TokenKind::Bar              => Some((11, 12)),
            TokenKind::Caret            => Some((13, 14)),
            TokenKind::Amp              => Some((15, 16)),
            TokenKind::LtLt | TokenKind::GtGt  => Some((17, 18)),
            TokenKind::Plus | TokenKind::Minus
            | TokenKind::PlusPct  | TokenKind::MinusPct
            | TokenKind::PlusBar  | TokenKind::MinusBar
            | TokenKind::PlusExcl | TokenKind::MinusExcl => Some((19, 20)),
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent
            | TokenKind::StarPct | TokenKind::StarBar | TokenKind::StarExcl => Some((21, 22)),
            TokenKind::DotDot | TokenKind::DotDotEq => Some((23, 0)),  // non-assoc
            TokenKind::StarStar => Some((25, 24)), // right-assoc
            _ => None,
        }
    }

    fn parse_unary_or_primary(&mut self) -> Option<Expr> {
        let start = self.span();
        match &self.peek().kind.clone() {
            // Prefix operators
            TokenKind::Minus => {
                self.advance();
                let operand = self.parse_expr(23)?;
                let sp = start.to(self.span());
                Some(Expr::new(ExprKind::UnaryOp { op: UnaryOp::Neg, operand: Box::new(operand) }, sp))
            }
            TokenKind::Tilde => {
                self.advance();
                let operand = self.parse_expr(23)?;
                let sp = start.to(self.span());
                Some(Expr::new(ExprKind::UnaryOp { op: UnaryOp::BitNot, operand: Box::new(operand) }, sp))
            }
            TokenKind::Kw(Keyword::Not) => {
                self.advance();
                let operand = self.parse_expr(23)?;
                let sp = start.to(self.span());
                Some(Expr::new(ExprKind::UnaryOp { op: UnaryOp::Not, operand: Box::new(operand) }, sp))
            }
            TokenKind::Star => {
                self.advance();
                let operand = self.parse_expr(23)?;
                let sp = start.to(self.span());
                Some(Expr::new(ExprKind::UnaryOp { op: UnaryOp::Deref, operand: Box::new(operand) }, sp))
            }
            TokenKind::Amp => {
                self.advance();
                let is_mut = self.eat_kw(Keyword::Move); // &mut
                let operand = self.parse_expr(23)?;
                let sp = start.to(self.span());
                let op = if is_mut { UnaryOp::AddrOfMut } else { UnaryOp::AddrOf };
                Some(Expr::new(ExprKind::UnaryOp { op, operand: Box::new(operand) }, sp))
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        let start = self.span();
        match self.peek().kind.clone() {
            // Literals
            TokenKind::Int(v, s)   => { self.advance(); Some(Expr::new(ExprKind::Int(v, s), start)) }
            TokenKind::Float(v, s) => { self.advance(); Some(Expr::new(ExprKind::Float(v, s), start)) }
            TokenKind::Str(s)      => { self.advance(); Some(Expr::new(ExprKind::Str(s), start)) }
            TokenKind::RawStr(s)   => { self.advance(); Some(Expr::new(ExprKind::RawStr(s), start)) }
            TokenKind::FStr(p)     => { self.advance(); Some(Expr::new(ExprKind::FStr(p), start)) }
            TokenKind::Char(c)     => { self.advance(); Some(Expr::new(ExprKind::Char(c), start)) }
            TokenKind::Byte(b)     => { self.advance(); Some(Expr::new(ExprKind::Byte(b), start)) }
            TokenKind::Kw(Keyword::True)  => { self.advance(); Some(Expr::new(ExprKind::Bool(true),  start)) }
            TokenKind::Kw(Keyword::False) => { self.advance(); Some(Expr::new(ExprKind::Bool(false), start)) }
            TokenKind::Kw(Keyword::Null)  => { self.advance(); Some(Expr::new(ExprKind::Null, start)) }

            // Identifier / path / struct literal
            TokenKind::Ident(_) | TokenKind::Kw(Keyword::SelfValue) | TokenKind::Kw(Keyword::SelfType) => {
                self.parse_ident_or_path_expr(start)
            }

            // Grouped or tuple: (expr) or (a, b, c)
            TokenKind::LParen => {
                self.advance();
                if matches!(self.peek().kind, TokenKind::RParen) {
                    self.advance();
                    return Some(Expr::new(ExprKind::Tuple(vec![]), start.to(self.span())));
                }
                let first = self.parse_expr(0)?;
                if matches!(self.peek().kind, TokenKind::Comma) {
                    let mut elems = vec![first];
                    while self.eat(TokenKind::Comma) {
                        if matches!(self.peek().kind, TokenKind::RParen) { break; }
                        if let Some(e) = self.parse_expr(0) { elems.push(e); }
                    }
                    self.eat(TokenKind::RParen);
                    Some(Expr::new(ExprKind::Tuple(elems), start.to(self.span())))
                } else {
                    self.eat(TokenKind::RParen);
                    Some(first)
                }
            }

            // Array: [1, 2, 3]
            TokenKind::LBracket => {
                self.advance();
                let mut elems = Vec::new();
                while !matches!(self.peek().kind, TokenKind::RBracket | TokenKind::Eof) {
                    if let Some(e) = self.parse_expr(0) { elems.push(e); }
                    if !self.eat(TokenKind::Comma) { break; }
                }
                self.eat(TokenKind::RBracket);
                Some(Expr::new(ExprKind::Array(elems), start.to(self.span())))
            }

            // Map: {"key": val, ...}
            TokenKind::LBrace => {
                self.advance();
                let mut entries = Vec::new();
                while !matches!(self.peek().kind, TokenKind::RBrace | TokenKind::Eof) {
                    let k = self.parse_expr(0)?;
                    self.expect_kind(&TokenKind::Colon, "`:`")?;
                    let v = self.parse_expr(0)?;
                    entries.push((k, v));
                    if !self.eat(TokenKind::Comma) { break; }
                }
                self.eat(TokenKind::RBrace);
                Some(Expr::new(ExprKind::Map(entries), start.to(self.span())))
            }

            // Lambda: fn(params) -> T: body
            TokenKind::Kw(Keyword::Fn) => self.parse_lambda(),

            // Move lambda: move fn(params) -> T: body
            TokenKind::Kw(Keyword::Move) => {
                self.advance();
                self.parse_lambda_with_move(true)
            }

            // If expression
            TokenKind::Kw(Keyword::If) => self.parse_if_expr(),

            // Match expression
            TokenKind::Kw(Keyword::Match) => self.parse_match_expr(),

            // Builtin: @size_of[T](...)
            TokenKind::At => {
                self.advance();
                let name = self.parse_ident()
                    .unwrap_or_else(|| Ident::new("?", self.span()))
                    .name;
                let ty_arg = if matches!(self.peek().kind, TokenKind::LBracket) {
                    self.advance();
                    let ty = self.parse_type();
                    self.eat(TokenKind::RBracket);
                    ty
                } else { None };
                let mut args = Vec::new();
                if matches!(self.peek().kind, TokenKind::LParen) {
                    self.advance();
                    while !matches!(self.peek().kind, TokenKind::RParen | TokenKind::Eof) {
                        if let Some(e) = self.parse_expr(0) { args.push(e); }
                        if !self.eat(TokenKind::Comma) { break; }
                    }
                    self.eat(TokenKind::RParen);
                }
                Some(Expr::new(ExprKind::Builtin {
                    name,
                    ty_arg: ty_arg.map(Box::new),
                    args,
                }, start.to(self.span())))
            }

            _ => {
                let sp = self.span();
                let got = self.peek().kind.clone();
                self.errors.push(e0001_unexpected_token(&got, "expression", sp));
                None
            }
        }
    }

    fn parse_ident_or_path_expr(&mut self, start: Span) -> Option<Expr> {
        let first = self.parse_ident()?;
        let mut segments = vec![first];

        // Build path: Foo.Bar.Baz
        while matches!(self.peek().kind, TokenKind::Dot) {
            // Peek ahead: if next is ident, it could be path segment or field access
            // At this stage (primary), build path; field access happens in postfix loop
            if matches!(self.peek2().kind, TokenKind::Ident(_) | TokenKind::Kw(_)) {
                // Could be field or path — we collect as path, postfix turns into field
                break;
            }
            break;
        }

        // Struct literal: Foo { field: val } or Foo.Bar { field: val }
        // Only if we see `{` on the same line (no newline between)
        if matches!(self.peek().kind, TokenKind::LBrace) {
            self.advance();
            let mut fields = Vec::new();
            let mut rest   = None;
            while !matches!(self.peek().kind, TokenKind::RBrace | TokenKind::Eof) {
                if matches!(self.peek().kind, TokenKind::DotDot) {
                    self.advance();
                    rest = self.parse_expr(0).map(Box::new);
                    break;
                }
                let f_start = self.span();
                if let Some(name) = self.parse_ident() {
                    let value = if matches!(self.peek().kind, TokenKind::Colon) {
                        self.advance();
                        self.parse_expr(0)
                    } else { None };
                    fields.push(FieldInit { name, value, span: f_start.to(self.span()) });
                }
                if !self.eat(TokenKind::Comma) { break; }
            }
            self.eat(TokenKind::RBrace);
            return Some(Expr::new(ExprKind::StructLit {
                ty: segments, fields, rest,
            }, start.to(self.span())));
        }

        if segments.len() == 1 {
            Some(Expr::new(ExprKind::Ident(segments.remove(0)), start))
        } else {
            Some(Expr::new(ExprKind::Path(segments), start.to(self.span())))
        }
    }

    fn parse_lambda(&mut self) -> Option<Expr> {
        self.parse_lambda_with_move(false)
    }

    fn parse_lambda_with_move(&mut self, is_move: bool) -> Option<Expr> {
        let start    = self.span();
        let is_async = self.eat_kw(Keyword::Async);
        self.expect_kw(Keyword::Fn);
        self.expect_kind(&TokenKind::LParen, "`(`")?;
        let params = self.parse_param_list();
        self.eat(TokenKind::RParen);

        let ret_ty = if matches!(self.peek().kind, TokenKind::Arrow) {
            self.advance();
            self.parse_type().map(Box::new)
        } else { None };

        self.expect_kind(&TokenKind::Colon, "`:`")?;

        let body = if matches!(self.peek().kind, TokenKind::Newline) {
            self.advance();
            let block = self.parse_block()?;
            LambdaBody::Block(block)
        } else {
            let expr = self.parse_expr(0)?;
            LambdaBody::Expr(Box::new(expr))
        };

        Some(Expr::new(ExprKind::Lambda {
            is_move, params, ret_ty, body, is_async,
        }, start.to(self.span())))
    }

    fn parse_if_expr(&mut self) -> Option<Expr> {
        let start = self.span();
        self.advance(); // consume `if`
        let cond  = self.parse_expr(0)?;
        self.eat(TokenKind::Colon);
        let then  = self.parse_expr(0)?;

        let mut elifs = Vec::new();
        while self.check_kw(Keyword::Elif) {
            self.advance();
            let ec = self.parse_expr(0)?;
            self.eat(TokenKind::Colon);
            let ev = self.parse_expr(0)?;
            elifs.push((ec, ev));
        }

        self.expect_kw(Keyword::Else);
        self.eat(TokenKind::Colon);
        let else_ = self.parse_expr(0)?;

        Some(Expr::new(ExprKind::If {
            cond: Box::new(cond),
            then: Box::new(then),
            elifs,
            else_: Box::new(else_),
        }, start.to(self.span())))
    }

    fn parse_match_expr(&mut self) -> Option<Expr> {
        let start = self.span();
        self.advance(); // consume `match`
        let scrutinee = self.parse_expr(0)?;
        self.advance(); // :
        self.expect_newline();
        if !self.expect_indent() { return None; }

        let mut arms = Vec::new();
        while !matches!(self.peek().kind, TokenKind::Dedent | TokenKind::Eof) {
            self.skip_newlines();
            if matches!(self.peek().kind, TokenKind::Dedent | TokenKind::Eof) { break; }
            if let Some(arm) = self.parse_match_arm() { arms.push(arm); }
        }
        self.expect_dedent();

        Some(Expr::new(ExprKind::Match {
            scrutinee: Box::new(scrutinee), arms,
        }, start.to(self.span())))
    }

    fn parse_call_args(&mut self) -> Vec<CallArg> {
        let mut args = Vec::new();
        while !matches!(self.peek().kind, TokenKind::RParen | TokenKind::Eof) {
            let start = self.span();
            // Named arg: name: expr
            let label = if matches!(self.peek().kind, TokenKind::Ident(_))
                && matches!(self.peek2().kind, TokenKind::Colon) {
                let n = self.parse_ident();
                self.advance(); // :
                n
            } else { None };

            if let Some(expr) = self.parse_expr(0) {
                // Spread: expr...
                let spread = matches!(self.peek().kind, TokenKind::DotDot)
                    && { self.advance(); matches!(self.peek().kind, TokenKind::Dot) && { self.advance(); true } };
                args.push(CallArg { label, expr, spread, span: start.to(self.span()) });
            }
            if !self.eat(TokenKind::Comma) { break; }
        }
        args
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Types
    // ─────────────────────────────────────────────────────────────────────────

    fn parse_type(&mut self) -> Option<Type> {
        let start = self.span();
        match self.peek().kind.clone() {
            // Pointer: *T or *const T or *mut T
            TokenKind::Star => {
                self.advance();
                let is_const = self.eat_kw(Keyword::Const);
                let inner    = self.parse_type()?;
                Some(Type::Pointer { is_const, inner: Box::new(inner), span: start.to(self.span()) })
            }
            // Optional: ?T
            TokenKind::Question => {
                self.advance();
                let inner = self.parse_type()?;
                Some(Type::Optional(Box::new(inner), start.to(self.span())))
            }
            // Slice: []T or Array: [N]T
            TokenKind::LBracket => {
                self.advance();
                if matches!(self.peek().kind, TokenKind::RBracket) {
                    self.advance();
                    let elem = self.parse_type()?;
                    Some(Type::Slice(Box::new(elem), start.to(self.span())))
                } else {
                    let size = self.parse_expr(0)?;
                    self.eat(TokenKind::RBracket);
                    let elem = self.parse_type()?;
                    Some(Type::Array { size: Box::new(size), elem: Box::new(elem), span: start.to(self.span()) })
                }
            }
            // Tuple: (T, U, V)
            TokenKind::LParen => {
                self.advance();
                let mut tys = Vec::new();
                while !matches!(self.peek().kind, TokenKind::RParen | TokenKind::Eof) {
                    if let Some(ty) = self.parse_type() { tys.push(ty); }
                    if !self.eat(TokenKind::Comma) { break; }
                }
                self.eat(TokenKind::RParen);
                Some(Type::Tuple(tys, start.to(self.span())))
            }
            // Borrow: &T or &mut T or &'a T
            TokenKind::Amp => {
                self.advance();
                let lifetime = if matches!(self.peek().kind, TokenKind::Lifetime(_)) {
                    if let TokenKind::Lifetime(n) = self.advance().kind.clone() {
                        Some(Lifetime { name: n, span: self.span() })
                    } else { None }
                } else { None };
                let mutable = self.eat_kw(Keyword::Move); // reuse Move for `mut` placeholder
                let inner   = self.parse_type()?;
                Some(Type::Ref { lifetime, mutable, inner: Box::new(inner), span: start.to(self.span()) })
            }
            // fn type: fn(T, U) -> R
            TokenKind::Kw(Keyword::Fn) => {
                self.advance();
                self.eat(TokenKind::LParen);
                let mut params = Vec::new();
                while !matches!(self.peek().kind, TokenKind::RParen | TokenKind::Eof) {
                    if let Some(ty) = self.parse_type() { params.push(ty); }
                    if !self.eat(TokenKind::Comma) { break; }
                }
                self.eat(TokenKind::RParen);
                let ret = if matches!(self.peek().kind, TokenKind::Arrow) {
                    self.advance();
                    Box::new(self.parse_type().unwrap_or(Type::Void(self.span())))
                } else {
                    Box::new(Type::Void(self.span()))
                };
                Some(Type::FnPtr { params, ret, is_async: false, span: start.to(self.span()) })
            }
            // map[K]V
            TokenKind::Ident(ref s) if s == "map" => {
                self.advance();
                self.eat(TokenKind::LBracket);
                let key = self.parse_type()?;
                self.eat(TokenKind::RBracket);
                let val = self.parse_type()?;
                Some(Type::Map { key: Box::new(key), val: Box::new(val), span: start.to(self.span()) })
            }
            // set[T]
            TokenKind::Ident(ref s) if s == "set" => {
                self.advance();
                self.eat(TokenKind::LBracket);
                let inner = self.parse_type()?;
                self.eat(TokenKind::RBracket);
                Some(Type::Set(Box::new(inner), start.to(self.span())))
            }
            // never
            TokenKind::Kw(Keyword::Never) => { self.advance(); Some(Type::Never(start)) }
            // void
            TokenKind::Kw(Keyword::Void)  => { self.advance(); Some(Type::Void(start)) }
            // any
            TokenKind::Kw(Keyword::Any)   => { self.advance(); Some(Type::Any(start)) }
            // _ (infer)
            TokenKind::Underscore         => { self.advance(); Some(Type::Infer(start)) }

            // Primitive types & named types
            TokenKind::Ident(_) | TokenKind::Kw(_) => {
                // Check for primitive
                if let Some(prim) = self.try_parse_primitive() {
                    return Some(Type::Primitive(prim, start));
                }
                // Named type with optional generics: MyType[T, U]
                let mut segments = vec![self.parse_ident()?];
                while matches!(self.peek().kind, TokenKind::Dot) {
                    self.advance();
                    if let Some(seg) = self.parse_ident() { segments.push(seg); }
                }
                let generics = if matches!(self.peek().kind, TokenKind::LBracket) {
                    self.advance();
                    let mut gs = Vec::new();
                    while !matches!(self.peek().kind, TokenKind::RBracket | TokenKind::Eof) {
                        if let Some(ty) = self.parse_type() { gs.push(ty); }
                        if !self.eat(TokenKind::Comma) { break; }
                    }
                    self.eat(TokenKind::RBracket);
                    gs
                } else { Vec::new() };
                Some(Type::Path { segments, generics, span: start.to(self.span()) })
            }
            _ => {
                let sp = self.span();
                let got = self.peek().kind.clone();
                self.errors.push(e0001_unexpected_token(&got, "type", sp));
                None
            }
        }
    }

    fn try_parse_primitive(&mut self) -> Option<PrimType> {
        let prim = match &self.peek().kind {
            TokenKind::Ident(s) => match s.as_str() {
                "i8"    => PrimType::I8,   "i16"  => PrimType::I16,
                "i32"   => PrimType::I32,  "i64"  => PrimType::I64,
                "i128"  => PrimType::I128, "u8"   => PrimType::U8,
                "u16"   => PrimType::U16,  "u32"  => PrimType::U32,
                "u64"   => PrimType::U64,  "u128" => PrimType::U128,
                "f32"   => PrimType::F32,  "f64"  => PrimType::F64,
                "bool"  => PrimType::Bool, "byte" => PrimType::Byte,
                "rune"  => PrimType::Rune, "str"  => PrimType::Str,
                "usize" => PrimType::Usize,"isize"=> PrimType::Isize,
                _ => return None,
            },
            _ => return None,
        };
        self.advance();
        Some(prim)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Generics
    // ─────────────────────────────────────────────────────────────────────────

    fn parse_generic_params(&mut self) -> Vec<GenericParam> {
        if !matches!(self.peek().kind, TokenKind::LBracket) { return vec![]; }
        self.advance();
        let mut params = Vec::new();
        while !matches!(self.peek().kind, TokenKind::RBracket | TokenKind::Eof) {
            let start = self.span();
            if let Some(name) = self.parse_ident() {
                let bounds = if matches!(self.peek().kind, TokenKind::Colon) {
                    self.advance();
                    let mut bs = Vec::new();
                    loop {
                        if let Some(ty) = self.parse_type() { bs.push(ty); }
                        if !matches!(self.peek().kind, TokenKind::Plus) { break; }
                        self.advance();
                    }
                    bs
                } else { vec![] };
                let default = if matches!(self.peek().kind, TokenKind::Eq) {
                    self.advance(); self.parse_type()
                } else { None };
                params.push(GenericParam { name, bounds, default, span: start.to(self.span()) });
            }
            if !self.eat(TokenKind::Comma) { break; }
        }
        self.eat(TokenKind::RBracket);
        params
    }

    fn parse_where_clause(&mut self) -> Option<WhereClause> {
        if !self.check_kw(Keyword::Extern) { return None; } // placeholder — real `where` keyword not in keyword list
        // TODO: add Where keyword and implement
        None
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Helpers
    // ─────────────────────────────────────────────────────────────────────────

    fn parse_ident(&mut self) -> Option<Ident> {
        let sp = self.span();
        match self.peek().kind.clone() {
            TokenKind::Ident(s) => { self.advance(); Some(Ident::new(s, sp)) }
            TokenKind::Kw(Keyword::SelfValue) => { self.advance(); Some(Ident::new("self", sp)) }
            TokenKind::Kw(Keyword::SelfType)  => { self.advance(); Some(Ident::new("Self", sp)) }
            // Allow some keywords as identifiers in certain positions
            TokenKind::Kw(kw) => {
                // We allow type-like idents (i32, f64, etc.) — handled elsewhere
                let sp2 = self.span();
                let got = self.peek().kind.clone();
                self.errors.push(e0001_unexpected_token(&got,
                    "identifier", sp2));
                None
            }
            _ => {
                let sp2 = self.span();
                let got = self.peek().kind.clone();
                self.errors.push(e0001_unexpected_token(&got, "identifier", sp2));
                None
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper: map token to BinOp
// ─────────────────────────────────────────────────────────────────────────────

fn token_to_binop(tok: &TokenKind) -> BinOp {
    match tok {
        TokenKind::Plus        => BinOp::Add,
        TokenKind::Minus       => BinOp::Sub,
        TokenKind::Star        => BinOp::Mul,
        TokenKind::Slash       => BinOp::Div,
        TokenKind::Percent     => BinOp::Rem,
        TokenKind::StarStar    => BinOp::Pow,
        TokenKind::PlusPct     => BinOp::AddWrap,
        TokenKind::MinusPct    => BinOp::SubWrap,
        TokenKind::StarPct     => BinOp::MulWrap,
        TokenKind::PlusBar     => BinOp::AddSat,
        TokenKind::MinusBar    => BinOp::SubSat,
        TokenKind::StarBar     => BinOp::MulSat,
        TokenKind::PlusExcl    => BinOp::AddChk,
        TokenKind::MinusExcl   => BinOp::SubChk,
        TokenKind::StarExcl    => BinOp::MulChk,
        TokenKind::Amp         => BinOp::BitAnd,
        TokenKind::Bar         => BinOp::BitOr,
        TokenKind::Caret       => BinOp::BitXor,
        TokenKind::LtLt        => BinOp::Shl,
        TokenKind::GtGt        => BinOp::Shr,
        TokenKind::EqEq        => BinOp::Eq,
        TokenKind::BangEq      => BinOp::Ne,
        TokenKind::Lt          => BinOp::Lt,
        TokenKind::LtEq        => BinOp::Le,
        TokenKind::Gt          => BinOp::Gt,
        TokenKind::GtEq        => BinOp::Ge,
        TokenKind::Kw(Keyword::And) => BinOp::And,
        TokenKind::Kw(Keyword::Or)  => BinOp::Or,
        TokenKind::DotDot      => BinOp::Range,
        TokenKind::DotDotEq    => BinOp::RangeInclusive,
        TokenKind::PipeGt      => BinOp::Pipe,
        TokenKind::QQ          => BinOp::Coalesce,
        _                      => BinOp::Add, // fallback (shouldn't reach)
    }
}
