mod lexer;
mod ast;
mod parser;
mod error;
mod session;
mod resolve;
mod types;

use lexer::{Lexer, TokenKind};
use parser::Parser;
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("G Language Compiler v0.1.0");
        eprintln!();
        eprintln!("Usage:");
        eprintln!("  gplc lex       <file.gpl>   Tokenize and print tokens");
        eprintln!("  gplc parse     <file.gpl>   Parse and print AST");
        eprintln!("  gplc ast-print <file.gpl>   Parse and pretty-print AST tree");
        eprintln!("  gplc resolve   <file.gpl>   Run name resolution");
        eprintln!("  gplc typecheck <file.gpl>   Run type checking");
        eprintln!("  gplc check     <file.gpl>   Run full check pipeline");
        process::exit(1);
    }

    let cmd  = args[1].as_str();
    let path = args[2].as_str();
    let src  = read_file(path);

    match cmd {
        "lex"       => cmd_lex(path, &src),
        "parse"     => cmd_parse(path, &src),
        "ast-print" => cmd_ast_print(path, &src),
        "resolve"   => cmd_resolve(path, &src),
        "typecheck" => cmd_typecheck(path, &src),
        "check"     => cmd_check(path, &src),
        other       => { eprintln!("error: unknown command `{}`", other); process::exit(1); }
    }
}

fn cmd_lex(path: &str, src: &str) {
    let mut lex    = Lexer::new(src);
    let tokens = lex.tokenize();

    for err in &lex.errors {
        let (line, col) = offset_to_line_col(src, err.span.start as usize);
        eprintln!("{}:{}:{}: {} {}", path, line, col, err.code, err.message);
    }

    println!("{:<6} {:<14} {}", "LINE", "KIND", "VALUE");
    println!("{}", "-".repeat(50));
    for tok in &tokens {
        if matches!(tok.kind,
            TokenKind::LineComment(_) | TokenKind::DocComment(_)) { continue; }
        let (line, _) = offset_to_line_col(src, tok.span.start as usize);
        let kind = token_kind_name(&tok.kind);
        let val  = token_value(&tok.kind);
        println!("{:<6} {:<14} {}", line, kind, val);
    }
    if !lex.errors.is_empty() { process::exit(1); }
}

fn cmd_parse(path: &str, src: &str) {
    let mut lex    = Lexer::new(src);
    let tokens = lex.tokenize();

    for err in &lex.errors {
        let (line, col) = offset_to_line_col(src, err.span.start as usize);
        eprintln!("{}:{}:{}: {} {}", path, line, col, err.code, err.message);
    }

    let mut p    = Parser::new(tokens);
    let file = p.parse_file();

    for err in &p.errors {
        let (line, col) = offset_to_line_col(src, err.span.start as usize);
        eprintln!("{}:{}:{}: {} {}", path, line, col, err.code, err.message);
    }

    // Print a simple AST summary
    println!("module: {}", file.module.path.iter()
        .map(|i| i.name.as_str()).collect::<Vec<_>>().join("."));
    println!("imports: {}", file.imports.len());
    println!("items:   {}", file.items.len());
    for item in &file.items {
        println!("  - {}", item_summary(item));
    }

    if !lex.errors.is_empty() || !p.errors.is_empty() {
        process::exit(1);
    }
}

fn cmd_ast_print(path: &str, src: &str) {
    let mut lex = Lexer::new(src);
    let tokens  = lex.tokenize();
    emit_lex_errors(path, src, &lex.errors);

    let mut p = Parser::new(tokens);
    let file  = p.parse_file();
    emit_parse_errors(path, src, &p.errors);

    if lex.errors.is_empty() && p.errors.is_empty() {
        let output = ast::print_ast(&file);
        print!("{}", output);
    } else {
        process::exit(1);
    }
}

fn cmd_resolve(path: &str, src: &str) {
    let mut lex = Lexer::new(src);
    let tokens  = lex.tokenize();
    emit_lex_errors(path, src, &lex.errors);

    let mut p = Parser::new(tokens);
    let file  = p.parse_file();
    emit_parse_errors(path, src, &p.errors);

    if !lex.errors.is_empty() || !p.errors.is_empty() {
        process::exit(1);
    }

    let mut resolver = resolve::Resolver::new();
    resolver.resolve(&file);

    let reporter = error::Reporter::new(path, src);
    for diag in &resolver.diagnostics {
        let (line, col) = offset_to_line_col(src, diag.primary.span.start as usize);
        let sev = diag.severity.as_str();
        eprintln!("{}:{}:{}: {}[{}]: {}", path, line, col, sev, diag.code, diag.message);
    }

    let sym_count = resolver.table.symbols.len();
    println!("{}: resolve ok — {} symbols defined", path, sym_count);

    if resolver.has_errors() {
        process::exit(1);
    }
}

fn cmd_typecheck(path: &str, src: &str) {
    let mut lex = Lexer::new(src);
    let tokens  = lex.tokenize();
    emit_lex_errors(path, src, &lex.errors);

    let mut p = Parser::new(tokens);
    let file  = p.parse_file();
    emit_parse_errors(path, src, &p.errors);

    if !lex.errors.is_empty() || !p.errors.is_empty() {
        process::exit(1);
    }

    // Name resolution first
    let mut resolver = resolve::Resolver::new();
    resolver.resolve(&file);
    for diag in &resolver.diagnostics {
        let (line, col) = offset_to_line_col(src, diag.primary.span.start as usize);
        let sev = diag.severity.as_str();
        eprintln!("{}:{}:{}: {}[{}]: {}", path, line, col, sev, diag.code, diag.message);
    }

    // Type checking
    let mut checker = types::TypeChecker::new();
    checker.check(&file);
    for diag in &checker.diagnostics {
        let (line, col) = offset_to_line_col(src, diag.primary.span.start as usize);
        let sev = diag.severity.as_str();
        eprintln!("{}:{}:{}: {}[{}]: {}", path, line, col, sev, diag.code, diag.message);
    }

    let total_errors = resolver.diagnostics.iter()
        .chain(checker.diagnostics.iter())
        .filter(|d| d.severity == error::Severity::Error)
        .count();

    if total_errors == 0 {
        println!("{}: typecheck ok — {} types interned", path, checker.arena.len());
    } else {
        eprintln!("{} error(s) found", total_errors);
        process::exit(1);
    }
}

fn cmd_check(path: &str, src: &str) {
    let mut lex = Lexer::new(src);
    let tokens  = lex.tokenize();
    emit_lex_errors(path, src, &lex.errors);

    let mut p = Parser::new(tokens);
    let file  = p.parse_file();
    emit_parse_errors(path, src, &p.errors);

    if !lex.errors.is_empty() || !p.errors.is_empty() {
        let total = lex.errors.len() + p.errors.len();
        eprintln!("{} error(s) found", total);
        process::exit(1);
    }

    // Name resolution
    let mut resolver = resolve::Resolver::new();
    resolver.resolve(&file);

    // Type checking
    let mut checker = types::TypeChecker::new();
    checker.check(&file);

    // Collect all diagnostics
    let mut all_diags: Vec<&error::Diagnostic> = Vec::new();
    all_diags.extend(resolver.diagnostics.iter());
    all_diags.extend(checker.diagnostics.iter());

    for diag in &all_diags {
        let (line, col) = offset_to_line_col(src, diag.primary.span.start as usize);
        let sev = diag.severity.as_str();
        eprintln!("{}:{}:{}: {}[{}]: {}", path, line, col, sev, diag.code, diag.message);
    }

    let errors = all_diags.iter().filter(|d| d.severity == error::Severity::Error).count();
    let warnings = all_diags.iter().filter(|d| d.severity == error::Severity::Warning).count();

    if errors == 0 {
        println!("{}: ok ({} warnings)", path, warnings);
    } else {
        eprintln!("{} error(s), {} warning(s)", errors, warnings);
        process::exit(1);
    }
}

fn emit_lex_errors(path: &str, src: &str, errors: &[lexer::error::LexError]) {
    for err in errors {
        let (line, col) = offset_to_line_col(src, err.span.start as usize);
        eprintln!("{}:{}:{}: {} {}", path, line, col, err.code, err.message);
    }
}

fn emit_parse_errors(path: &str, src: &str, errors: &[parser::error::ParseError]) {
    for err in errors {
        let (line, col) = offset_to_line_col(src, err.span.start as usize);
        eprintln!("{}:{}:{}: {} {}", path, line, col, err.code, err.message);
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("error: cannot read `{}`: {}", path, e);
        process::exit(1);
    })
}

fn offset_to_line_col(src: &str, offset: usize) -> (usize, usize) {
    let before = &src[..offset.min(src.len())];
    let line = before.bytes().filter(|&b| b == b'\n').count() + 1;
    let col  = before.rfind('\n').map(|i| offset - i - 1).unwrap_or(offset) + 1;
    (line, col)
}

fn item_summary(item: &ast::Item) -> String {
    match &item.kind {
        ast::ItemKind::Fn(f) => format!("fn {}", fn_name_str(&f.name)),
        ast::ItemKind::Struct(s)    => format!("struct {}", s.name.name),
        ast::ItemKind::Enum(e)      => format!("enum {}", e.name.name),
        ast::ItemKind::Union(u)     => format!("union {}", u.name.name),
        ast::ItemKind::Interface(i) => format!("interface {}", i.name.name),
        ast::ItemKind::ImplBlock(i) => format!("impl (block)"),
        ast::ItemKind::ImplFor(i)   => format!("impl for"),
        ast::ItemKind::TypeAlias(t) => format!("type {}", t.name.name),
        ast::ItemKind::Newtype(n)   => format!("newtype {}", n.name.name),
        ast::ItemKind::Const(c)     => format!("const {}", c.name.name),
        ast::ItemKind::Var(v)       => format!("var {}",
            v.names.iter().map(|i| i.name.as_str()).collect::<Vec<_>>().join(", ")),
        ast::ItemKind::Extern(_)    => "extern".to_string(),
    }
}

fn fn_name_str(name: &ast::FnName) -> String {
    match name {
        ast::FnName::Simple(i)              => i.name.clone(),
        ast::FnName::Method { ty_name, method, .. } =>
            format!("{}.{}", ty_name.name, method.name),
    }
}

fn token_kind_name(kind: &TokenKind) -> &'static str {
    match kind {
        TokenKind::Int(..)     => "Int",
        TokenKind::Float(..)   => "Float",
        TokenKind::Str(_)      => "Str",
        TokenKind::RawStr(_)   => "RawStr",
        TokenKind::FStr(_)     => "FStr",
        TokenKind::Char(_)     => "Char",
        TokenKind::Byte(_)     => "Byte",
        TokenKind::Ident(_)    => "Ident",
        TokenKind::Kw(_)       => "Keyword",
        TokenKind::Lifetime(_) => "Lifetime",
        TokenKind::Newline     => "Newline",
        TokenKind::Indent      => "Indent",
        TokenKind::Dedent      => "Dedent",
        TokenKind::Eof         => "Eof",
        _                      => "Op/Punct",
    }
}

fn token_value(kind: &TokenKind) -> String {
    match kind {
        TokenKind::Int(v, None)    => format!("{}", v),
        TokenKind::Int(v, Some(s)) => format!("{}{}", v, s.as_str()),
        TokenKind::Float(v, _)     => format!("{}", v),
        TokenKind::Str(s)          => format!("\"{}\"", s),
        TokenKind::RawStr(s)       => format!("r\"{}\"", s),
        TokenKind::FStr(p)         => format!("f\"<{} parts>\"", p.len()),
        TokenKind::Char(c)         => format!("'{}'", c),
        TokenKind::Byte(b)         => format!("b'{}'", *b as char),
        TokenKind::Ident(s)        => s.clone(),
        TokenKind::Kw(kw)          => kw.as_str().to_string(),
        TokenKind::Lifetime(s)     => format!("'{}", s),
        _                          => String::new(),
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::Lexer;
    use parser::Parser;

    fn parse_ok(src: &str) {
        let mut lex = Lexer::new(src);
        let tokens  = lex.tokenize();
        assert!(lex.errors.is_empty(),
            "lex errors: {:?}", lex.errors);
        let mut p = Parser::new(tokens);
        p.parse_file();
        assert!(p.errors.is_empty(),
            "parse errors in:\n{}\nerrors: {:?}", src, p.errors);
    }

    fn parse_err(src: &str) {
        let mut lex = Lexer::new(src);
        let tokens  = lex.tokenize();
        let mut p   = Parser::new(tokens);
        p.parse_file();
        assert!(!p.errors.is_empty(),
            "expected parse errors but got none in:\n{}", src);
    }

    // ── Module & imports ─────────────────────────────────────────────────────

    #[test]
    fn test_module_decl() {
        parse_ok("module main\n");
    }

    #[test]
    fn test_module_path() {
        parse_ok("module myapp.utils.math\n");
    }

    #[test]
    fn test_import_simple() {
        parse_ok("module main\nimport \"std/io\"\n");
    }

    #[test]
    fn test_import_alias() {
        parse_ok("module main\nimport \"std/io\" as io\n");
    }

    #[test]
    fn test_import_selective() {
        parse_ok("module main\nimport \"std/io\": {println, eprintln}\n");
    }

    // ── Functions ─────────────────────────────────────────────────────────────

    #[test]
    fn test_fn_simple() {
        parse_ok(r#"
module main
fn add(a: i32, b: i32) -> i32:
    return a + b
"#);
    }

    #[test]
    fn test_fn_no_return() {
        parse_ok(r#"
module main
fn greet(name: str) -> void:
    return
"#);
    }

    #[test]
    fn test_fn_method() {
        parse_ok(r#"
module main
struct Point:
    x: f64
    y: f64

fn Point.distance(self, other: Point) -> f64:
    var dx := self.x - other.x
    var dy := self.y - other.y
    return dx * dx + dy * dy
"#);
    }

    #[test]
    fn test_fn_single_expr_body() {
        parse_ok(r#"
module main
fn square(x: i32) -> i32: x * x
"#);
    }

    #[test]
    fn test_fn_default_param() {
        parse_ok(r#"
module main
fn connect(host: str, port: i32 = 8080) -> void:
    return
"#);
    }

    #[test]
    fn test_fn_variadic() {
        parse_ok(r#"
module main
fn sum(...nums: ...i32) -> i32:
    return 0
"#);
    }

    #[test]
    fn test_fn_async() {
        parse_ok(r#"
module main
async fn fetch(url: str) -> str:
    return url
"#);
    }

    // ── Structs ───────────────────────────────────────────────────────────────

    #[test]
    fn test_struct_basic() {
        parse_ok(r#"
module main
struct Point:
    x: f64
    y: f64
"#);
    }

    #[test]
    fn test_struct_with_default() {
        parse_ok(r#"
module main
struct Config:
    host: str = "localhost"
    port: i32 = 8080
"#);
    }

    #[test]
    fn test_struct_pub_fields() {
        parse_ok(r#"
module main
pub struct Player:
    pub name: str
    pub hp:   i32
    score:    i32
"#);
    }

    // ── Enums ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_enum_basic() {
        parse_ok(r#"
module main
enum Color:
    Red
    Green
    Blue
"#);
    }

    #[test]
    fn test_enum_with_data() {
        parse_ok(r#"
module main
enum Shape:
    Circle(radius: f64)
    Rectangle(width: f64, height: f64)
"#);
    }

    #[test]
    fn test_enum_with_values() {
        parse_ok(r#"
module main
enum StatusCode:
    Ok = 200
    NotFound = 404
"#);
    }

    // ── Interfaces ────────────────────────────────────────────────────────────

    #[test]
    fn test_interface_basic() {
        parse_ok(r#"
module main
interface Drawable:
    fn draw(self) -> void
    fn bounds(self) -> (f64, f64, f64, f64)
"#);
    }

    // ── Impl ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_impl_block() {
        parse_ok(r#"
module main
struct Circle:
    radius: f64

impl Circle:
    fn new(r: f64) -> Circle:
        return Circle{radius: r}

    fn area(self) -> f64:
        return 3.14 * self.radius * self.radius
"#);
    }

    #[test]
    fn test_impl_for() {
        parse_ok(r#"
module main
interface Display:
    fn to_string(self) -> str

struct Point:
    x: f64
    y: f64

impl Display for Point:
    fn to_string(self) -> str:
        return "point"
"#);
    }

    // ── Statements ────────────────────────────────────────────────────────────

    #[test]
    fn test_var_decl() {
        parse_ok(r#"
module main
fn f() -> void:
    var x: i32 = 10
    var y := 3.14
    let name: str = "Alice"
"#);
    }

    #[test]
    fn test_assign_ops() {
        parse_ok(r#"
module main
fn f() -> void:
    var x: i32 = 0
    x += 1
    x -= 1
    x *= 2
    x /= 2
    x %= 3
"#);
    }

    #[test]
    fn test_if_elif_else() {
        parse_ok(r#"
module main
fn f(x: i32) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"
"#);
    }

    #[test]
    fn test_for_range() {
        parse_ok(r#"
module main
fn f() -> void:
    for i in 0..10:
        var x := i
"#);
    }

    #[test]
    fn test_for_index_value() {
        parse_ok(r#"
module main
fn f(arr: []i32) -> void:
    for i, item in arr:
        var x := item
"#);
    }

    #[test]
    fn test_while_loop() {
        parse_ok(r#"
module main
fn f() -> void:
    var i := 0
    while i < 10:
        i += 1
"#);
    }

    #[test]
    fn test_loop_break() {
        parse_ok(r#"
module main
fn f() -> void:
    loop:
        break
"#);
    }

    #[test]
    fn test_match_basic() {
        parse_ok(r#"
module main
fn f(x: i32) -> void:
    match x:
        0 => var a := 1
        1 => var b := 2
        _ => var c := 3
"#);
    }

    #[test]
    fn test_defer() {
        parse_ok(r#"
module main
fn f() -> void:
    defer var x := 1
    return
"#);
    }

    #[test]
    fn test_try_catch() {
        parse_ok(r#"
module main
fn f() -> void:
    try:
        var x := 1
    catch e: str:
        var y := 2
    finally:
        var z := 3
"#);
    }

    // ── Expressions ───────────────────────────────────────────────────────────

    #[test]
    fn test_binary_ops() {
        parse_ok(r#"
module main
fn f() -> i32:
    return 1 + 2 * 3 - 4 / 2
"#);
    }

    #[test]
    fn test_overflow_ops() {
        parse_ok(r#"
module main
fn f(a: u8, b: u8) -> u8:
    return a +% b
"#);
    }

    #[test]
    fn test_comparison() {
        parse_ok(r#"
module main
fn f(a: i32, b: i32) -> bool:
    return a == b and b != 0
"#);
    }

    #[test]
    fn test_call_expr() {
        parse_ok(r#"
module main
fn add(a: i32, b: i32) -> i32:
    return a + b

fn main() -> i32:
    return add(1, 2)
"#);
    }

    #[test]
    fn test_method_call() {
        parse_ok(r#"
module main
fn main() -> void:
    var s := "hello"
    var n := s.len
"#);
    }

    #[test]
    fn test_field_access() {
        parse_ok(r#"
module main
struct Point:
    x: f64
    y: f64

fn main() -> void:
    var p := Point{x: 1.0, y: 2.0}
    var x := p.x
"#);
    }

    #[test]
    fn test_index_expr() {
        parse_ok(r#"
module main
fn main() -> void:
    var arr: []i32 = [1, 2, 3]
    var x := arr[0]
"#);
    }

    #[test]
    fn test_slice_expr() {
        parse_ok(r#"
module main
fn main() -> void:
    var arr: []i32 = [1, 2, 3, 4, 5]
    var sl := arr[1:3]
"#);
    }

    #[test]
    fn test_lambda() {
        parse_ok(r#"
module main
fn main() -> void:
    var double := fn(x: i32) -> i32: x * 2
    var result := double(5)
"#);
    }

    #[test]
    fn test_if_expr() {
        parse_ok(r#"
module main
fn abs(x: i32) -> i32:
    return if x < 0: -x else: x
"#);
    }

    #[test]
    fn test_range_expr() {
        parse_ok(r#"
module main
fn main() -> void:
    for i in 0..10:
        var x := i
    for i in 0..=10:
        var y := i
"#);
    }

    #[test]
    fn test_struct_literal() {
        parse_ok(r#"
module main
struct Point:
    x: f64
    y: f64

fn main() -> void:
    var p := Point{x: 1.0, y: 2.0}
"#);
    }

    #[test]
    fn test_array_literal() {
        parse_ok(r#"
module main
fn main() -> void:
    var arr := [1, 2, 3, 4, 5]
"#);
    }

    #[test]
    fn test_propagate_op() {
        parse_ok(r#"
module main
fn risky() -> i32:
    return 42

fn safe() -> i32:
    var x := risky()
    return x
"#);
    }

    #[test]
    fn test_cast_expr() {
        parse_ok(r#"
module main
fn main() -> void:
    var x: i64 = 100
    var y: i32 = x as i32
"#);
    }

    #[test]
    fn test_unary_ops() {
        parse_ok(r#"
module main
fn main() -> void:
    var x: i32 = 5
    var neg := -x
    var deref_ex: i32 = 0
    var addr := &deref_ex
"#);
    }

    #[test]
    fn test_pipe_op() {
        parse_ok(r#"
module main
fn double(x: i32) -> i32: x * 2

fn main() -> void:
    var result := 5 |> double
"#);
    }

    #[test]
    fn test_coalesce_op() {
        parse_ok(r#"
module main
fn main() -> void:
    var x: ?i32 = null
    var y := x ?? 42
"#);
    }

    // ── Types ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_pointer_type() {
        parse_ok(r#"
module main
fn f(p: *i32) -> void:
    return
"#);
    }

    #[test]
    fn test_optional_type() {
        parse_ok(r#"
module main
fn f(x: ?i32) -> void:
    return
"#);
    }

    #[test]
    fn test_slice_type() {
        parse_ok(r#"
module main
fn f(arr: []i32) -> void:
    return
"#);
    }

    #[test]
    fn test_array_type() {
        parse_ok(r#"
module main
fn f(arr: [4]i32) -> void:
    return
"#);
    }

    #[test]
    fn test_tuple_type() {
        parse_ok(r#"
module main
fn f() -> (i32, str, bool):
    return 1, "hi", true
"#);
    }

    #[test]
    fn test_fn_type() {
        parse_ok(r#"
module main
fn apply(f: fn(i32) -> i32, x: i32) -> i32:
    return f(x)
"#);
    }

    #[test]
    fn test_generic_type() {
        parse_ok(r#"
module main
fn first[T](arr: []T) -> T:
    return arr[0]
"#);
    }

    // ── Full programs ─────────────────────────────────────────────────────────

    #[test]
    fn test_hello_world() {
        parse_ok(r#"
module main

import "std/io"

fn main() -> i32:
    io.println("Hello, World!")
    return 0
"#);
    }

    #[test]
    fn test_fibonacci() {
        parse_ok(r#"
module main

fn fibonacci(n: i32) -> i32:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

fn main() -> i32:
    var result := fibonacci(10)
    return 0
"#);
    }

    #[test]
    fn test_enum_with_match() {
        parse_ok(r#"
module main

enum Direction:
    North
    South
    East
    West

fn move_player(dir: Direction, x: i32, y: i32) -> (i32, i32):
    match dir:
        Direction.North => return x, y - 1
        Direction.South => return x, y + 1
        Direction.East  => return x + 1, y
        Direction.West  => return x - 1, y
"#);
    }

    #[test]
    fn test_struct_with_impl() {
        parse_ok(r#"
module main

struct Stack:
    data: []i32
    size: i32

impl Stack:
    fn new() -> Stack:
        return Stack{data: [], size: 0}

    fn push(self: *Stack, val: i32) -> void:
        self.size += 1

    fn pop(self: *Stack) -> ?i32:
        if self.size == 0:
            return null
        self.size -= 1
        return null
"#);
    }

    #[test]
    fn test_closures_in_function() {
        parse_ok(r#"
module main

fn make_adder(n: i32) -> fn(i32) -> i32:
    return fn(x: i32) -> i32: x + n
"#);
    }

    #[test]
    fn test_const_and_type_alias() {
        parse_ok(r#"
module main

const MAX_SIZE: usize = 4096
const PI: f64 = 3.14159

type Seconds = f64
type Buffer  = []u8
"#);
    }

    #[test]
    fn test_newtype() {
        parse_ok(r#"
module main

newtype UserId = i32
newtype Email  = str
"#);
    }
}
