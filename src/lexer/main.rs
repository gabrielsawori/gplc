mod lexer;

use lexer::{Lexer, TokenKind};
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("G Language Compiler v0.1.0 (lexer stage)");
        eprintln!("");
        eprintln!("Usage:");
        eprintln!("  gplc lex <file.gpl>      Tokenize a file and print tokens");
        eprintln!("  gplc check <file.gpl>    Check for lex errors only");
        process::exit(1);
    }

    match args[1].as_str() {
        "lex" => {
            let path = args.get(2).unwrap_or_else(|| {
                eprintln!("error: missing file argument");
                process::exit(1);
            });
            cmd_lex(path);
        }
        "check" => {
            let path = args.get(2).unwrap_or_else(|| {
                eprintln!("error: missing file argument");
                process::exit(1);
            });
            cmd_check(path);
        }
        cmd => {
            eprintln!("error: unknown command `{}`", cmd);
            process::exit(1);
        }
    }
}

/// Tokenize a file and print every token.
fn cmd_lex(path: &str) {
    let src = read_file(path);
    let mut lex = Lexer::new(&src);
    let tokens = lex.tokenize();

    // Print errors first
    for err in &lex.errors {
        let (line, col) = offset_to_line_col(&src, err.span.start as usize);
        eprintln!("{}:{}:{}: {} {}", path, line, col, err.code, err.message);
    }

    // Print tokens
    println!("{:<6} {:<12} {:<8} {}", "LINE", "KIND", "SPAN", "VALUE");
    println!("{}", "-".repeat(60));

    for tok in &tokens {
        // Skip comments for cleaner output unless explicitly requested
        if matches!(tok.kind, TokenKind::LineComment(_) | TokenKind::DocComment(_)) {
            continue;
        }
        let (line, col) = offset_to_line_col(&src, tok.span.start as usize);
        let span_str = format!("{}:{}", line, col);
        let kind_str = token_kind_name(&tok.kind);
        let val_str  = token_value(&tok.kind);
        println!("{:<6} {:<12} {:<8} {}", line, kind_str, span_str, val_str);
    }

    if !lex.errors.is_empty() {
        process::exit(1);
    }
}

/// Check for lex errors only, no output on success.
fn cmd_check(path: &str) {
    let src = read_file(path);
    let mut lex = Lexer::new(&src);
    lex.tokenize();

    if lex.errors.is_empty() {
        println!("{}: ok", path);
    } else {
        for err in &lex.errors {
            let (line, col) = offset_to_line_col(&src, err.span.start as usize);
            eprintln!("{}:{}:{}: {} {}", path, line, col, err.code, err.message);
        }
        process::exit(1);
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

fn token_kind_name(kind: &TokenKind) -> &'static str {
    match kind {
        TokenKind::Int(..)         => "Int",
        TokenKind::Float(..)       => "Float",
        TokenKind::Str(_)          => "Str",
        TokenKind::RawStr(_)       => "RawStr",
        TokenKind::FStr(_)         => "FStr",
        TokenKind::Char(_)         => "Char",
        TokenKind::Byte(_)         => "Byte",
        TokenKind::Ident(_)        => "Ident",
        TokenKind::Kw(_)           => "Keyword",
        TokenKind::Lifetime(_)     => "Lifetime",
        TokenKind::Plus            => "Plus",
        TokenKind::Minus           => "Minus",
        TokenKind::Star            => "Star",
        TokenKind::Slash           => "Slash",
        TokenKind::Percent         => "Percent",
        TokenKind::StarStar        => "StarStar",
        TokenKind::Amp             => "Amp",
        TokenKind::Bar             => "Bar",
        TokenKind::Caret           => "Caret",
        TokenKind::Tilde           => "Tilde",
        TokenKind::LtLt            => "LtLt",
        TokenKind::GtGt            => "GtGt",
        TokenKind::Eq              => "Eq",
        TokenKind::EqEq            => "EqEq",
        TokenKind::BangEq          => "BangEq",
        TokenKind::Lt              => "Lt",
        TokenKind::Gt              => "Gt",
        TokenKind::LtEq            => "LtEq",
        TokenKind::GtEq            => "GtEq",
        TokenKind::Arrow           => "Arrow",
        TokenKind::FatArrow        => "FatArrow",
        TokenKind::DotDot          => "DotDot",
        TokenKind::DotDotEq        => "DotDotEq",
        TokenKind::PipeGt          => "PipeGt",
        TokenKind::QQ              => "QQ",
        TokenKind::QDot            => "QDot",
        TokenKind::Question        => "Question",
        TokenKind::Bang            => "Bang",
        TokenKind::ColonEq         => "ColonEq",
        TokenKind::Dot             => "Dot",
        TokenKind::Comma           => "Comma",
        TokenKind::Colon           => "Colon",
        TokenKind::At              => "At",
        TokenKind::Hash            => "Hash",
        TokenKind::Underscore      => "Underscore",
        TokenKind::LParen          => "LParen",
        TokenKind::RParen          => "RParen",
        TokenKind::LBracket        => "LBracket",
        TokenKind::RBracket        => "RBracket",
        TokenKind::LBrace          => "LBrace",
        TokenKind::RBrace          => "RBrace",
        TokenKind::Newline         => "Newline",
        TokenKind::Indent          => "Indent",
        TokenKind::Dedent          => "Dedent",
        TokenKind::Eof             => "Eof",
        TokenKind::LineComment(_)  => "Comment",
        TokenKind::DocComment(_)   => "DocComment",
        // assignment ops
        TokenKind::PlusEq          => "PlusEq",
        TokenKind::MinusEq         => "MinusEq",
        TokenKind::StarEq          => "StarEq",
        TokenKind::SlashEq         => "SlashEq",
        TokenKind::PercentEq       => "PercentEq",
        TokenKind::StarStarEq      => "StarStarEq",
        TokenKind::AmpEq           => "AmpEq",
        TokenKind::BarEq           => "BarEq",
        TokenKind::CaretEq         => "CaretEq",
        TokenKind::LtLtEq          => "LtLtEq",
        TokenKind::GtGtEq          => "GtGtEq",
        // overflow ops
        TokenKind::PlusPct         => "PlusPct",
        TokenKind::MinusPct        => "MinusPct",
        TokenKind::StarPct         => "StarPct",
        TokenKind::PlusBar         => "PlusBar",
        TokenKind::MinusBar        => "MinusBar",
        TokenKind::StarBar         => "StarBar",
        TokenKind::PlusExcl        => "PlusExcl",
        TokenKind::MinusExcl       => "MinusExcl",
        TokenKind::StarExcl        => "StarExcl",
        TokenKind::PlusPctEq       => "PlusPctEq",
        TokenKind::MinusPctEq      => "MinusPctEq",
        TokenKind::StarPctEq       => "StarPctEq",
        TokenKind::PlusBarEq       => "PlusBarEq",
        TokenKind::MinusBarEq      => "MinusBarEq",
        TokenKind::StarBarEq       => "StarBarEq",
        TokenKind::HashBang        => "HashBang",
        TokenKind::Semicolon       => "Semicolon",
    }
}

fn token_value(kind: &TokenKind) -> String {
    match kind {
        TokenKind::Int(v, s)    => match s {
            Some(sf) => format!("{}{}", v, sf.as_str()),
            None     => format!("{}", v),
        },
        TokenKind::Float(v, s)  => match s {
            Some(lexer::token::FloatSuffix::F32) => format!("{}f32", v),
            Some(lexer::token::FloatSuffix::F64) => format!("{}f64", v),
            None => format!("{}", v),
        },
        TokenKind::Str(s)        => format!("\"{}\"", s),
        TokenKind::RawStr(s)     => format!("r\"{}\"", s),
        TokenKind::FStr(parts)   => format!("f\"<{} parts>\"", parts.len()),
        TokenKind::Char(c)       => format!("'{}'", c),
        TokenKind::Byte(b)       => format!("b'{}'", *b as char),
        TokenKind::Ident(s)      => s.clone(),
        TokenKind::Kw(kw)        => kw.as_str().to_string(),
        TokenKind::Lifetime(s)   => format!("'{}", s),
        TokenKind::LineComment(s)=> format!("# {}", s),
        TokenKind::DocComment(s) => format!("## {}", s),
        _                        => String::new(),
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::{Lexer, TokenKind, Keyword, IntSuffix};

    fn lex(src: &str) -> Vec<TokenKind> {
        let mut l = Lexer::new(src);
        l.tokenize()
            .into_iter()
            .map(|t| t.kind)
            .filter(|k| !matches!(k,
                TokenKind::Newline
                | TokenKind::Indent
                | TokenKind::Dedent
                | TokenKind::LineComment(_)
                | TokenKind::DocComment(_)
            ))
            .collect()
    }

    fn lex_all(src: &str) -> Vec<TokenKind> {
        let mut l = Lexer::new(src);
        l.tokenize().into_iter().map(|t| t.kind).collect()
    }

    fn no_errors(src: &str) {
        let mut l = Lexer::new(src);
        l.tokenize();
        assert!(l.errors.is_empty(),
            "expected no errors, got: {:?}", l.errors);
    }

    fn has_error(src: &str, code: &str) {
        let mut l = Lexer::new(src);
        l.tokenize();
        assert!(
            l.errors.iter().any(|e| e.code == code),
            "expected error {} in {:?}", code, l.errors
        );
    }

    // ── Integer literals ──────────────────────────────────────────────────────

    #[test]
    fn test_int_decimal() {
        assert_eq!(lex("42"), vec![TokenKind::Int(42, None), TokenKind::Eof]);
    }

    #[test]
    fn test_int_with_underscores() {
        assert_eq!(lex("1_000_000"),
            vec![TokenKind::Int(1_000_000, None), TokenKind::Eof]);
    }

    #[test]
    fn test_int_hex() {
        assert_eq!(lex("0xFF"),  vec![TokenKind::Int(255, None), TokenKind::Eof]);
        assert_eq!(lex("0xDEAD_BEEF"), vec![TokenKind::Int(0xDEAD_BEEF, None), TokenKind::Eof]);
    }

    #[test]
    fn test_int_binary() {
        assert_eq!(lex("0b1010"), vec![TokenKind::Int(10, None), TokenKind::Eof]);
        assert_eq!(lex("0b1111_0000"), vec![TokenKind::Int(0b1111_0000, None), TokenKind::Eof]);
    }

    #[test]
    fn test_int_octal() {
        assert_eq!(lex("0o777"), vec![TokenKind::Int(0o777, None), TokenKind::Eof]);
    }

    #[test]
    fn test_int_suffix() {
        assert_eq!(lex("100u8"),   vec![TokenKind::Int(100, Some(IntSuffix::U8)),   TokenKind::Eof]);
        assert_eq!(lex("42i32"),   vec![TokenKind::Int(42,  Some(IntSuffix::I32)),  TokenKind::Eof]);
        assert_eq!(lex("0usize"),  vec![TokenKind::Int(0,   Some(IntSuffix::Usize)),TokenKind::Eof]);
        assert_eq!(lex("255u64"),  vec![TokenKind::Int(255, Some(IntSuffix::U64)),  TokenKind::Eof]);
    }

    // ── Float literals ────────────────────────────────────────────────────────

    #[test]
    fn test_float_basic() {
        assert_eq!(lex("3.14"), vec![TokenKind::Float(3.14, None), TokenKind::Eof]);
    }

    #[test]
    fn test_float_scientific() {
        assert_eq!(lex("1.5e10"),  vec![TokenKind::Float(1.5e10, None),  TokenKind::Eof]);
        assert_eq!(lex("1.5e-3"),  vec![TokenKind::Float(1.5e-3, None),  TokenKind::Eof]);
        assert_eq!(lex("2.0E+4"),  vec![TokenKind::Float(2.0e4,  None),  TokenKind::Eof]);
    }

    #[test]
    fn test_float_suffix() {
        if let TokenKind::Float(_, Some(lexer::token::FloatSuffix::F32)) = lex("3.14f32")[0] {}
        else { panic!("expected f32 suffix"); }
    }

    // ── String literals ───────────────────────────────────────────────────────

    #[test]
    fn test_string_basic() {
        assert_eq!(lex(r#""hello""#),
            vec![TokenKind::Str("hello".into()), TokenKind::Eof]);
    }

    #[test]
    fn test_string_escapes() {
        assert_eq!(lex(r#""\n\t\\\""`"#.trim_end_matches('`')),
            vec![TokenKind::Str("\n\t\\\"".into()), TokenKind::Eof]);
    }

    #[test]
    fn test_string_unicode_escape() {
        assert_eq!(lex(r#""\u{41}""#),
            vec![TokenKind::Str("A".into()), TokenKind::Eof]);
    }

    #[test]
    fn test_raw_string() {
        assert_eq!(lex(r#"r"C:\Users\name""#),
            vec![TokenKind::RawStr(r"C:\Users\name".into()), TokenKind::Eof]);
    }

    #[test]
    fn test_unterminated_string() {
        has_error(r#""hello"#, "E0007");
    }

    #[test]
    fn test_invalid_escape() {
        has_error(r#""\q""#, "E0006");
    }

    // ── Char literals ─────────────────────────────────────────────────────────

    #[test]
    fn test_char_basic() {
        assert_eq!(lex("'A'"), vec![TokenKind::Char('A'), TokenKind::Eof]);
    }

    #[test]
    fn test_char_escape() {
        assert_eq!(lex("'\\n'"), vec![TokenKind::Char('\n'), TokenKind::Eof]);
    }

    #[test]
    fn test_byte_lit() {
        assert_eq!(lex("b'A'"), vec![TokenKind::Byte(65), TokenKind::Eof]);
    }

    // ── Keywords ──────────────────────────────────────────────────────────────

    #[test]
    fn test_keywords() {
        assert_eq!(lex("fn"),     vec![TokenKind::Kw(Keyword::Fn),     TokenKind::Eof]);
        assert_eq!(lex("struct"), vec![TokenKind::Kw(Keyword::Struct), TokenKind::Eof]);
        assert_eq!(lex("return"), vec![TokenKind::Kw(Keyword::Return), TokenKind::Eof]);
        assert_eq!(lex("if"),     vec![TokenKind::Kw(Keyword::If),     TokenKind::Eof]);
        assert_eq!(lex("for"),    vec![TokenKind::Kw(Keyword::For),    TokenKind::Eof]);
        assert_eq!(lex("true"),   vec![TokenKind::Kw(Keyword::True),   TokenKind::Eof]);
        assert_eq!(lex("null"),   vec![TokenKind::Kw(Keyword::Null),   TokenKind::Eof]);
    }

    #[test]
    fn test_ident_vs_keyword() {
        assert_eq!(lex("fns"),   vec![TokenKind::Ident("fns".into()),   TokenKind::Eof]);
        assert_eq!(lex("iffy"),  vec![TokenKind::Ident("iffy".into()),  TokenKind::Eof]);
        assert_eq!(lex("_x"),    vec![TokenKind::Ident("_x".into()),    TokenKind::Eof]);
    }

    #[test]
    fn test_underscore_wildcard() {
        assert_eq!(lex("_"), vec![TokenKind::Underscore, TokenKind::Eof]);
    }

    // ── Operators ─────────────────────────────────────────────────────────────

    #[test]
    fn test_operators_basic() {
        assert_eq!(lex("+"),  vec![TokenKind::Plus,    TokenKind::Eof]);
        assert_eq!(lex("-"),  vec![TokenKind::Minus,   TokenKind::Eof]);
        assert_eq!(lex("*"),  vec![TokenKind::Star,    TokenKind::Eof]);
        assert_eq!(lex("/"),  vec![TokenKind::Slash,   TokenKind::Eof]);
        assert_eq!(lex("%"),  vec![TokenKind::Percent, TokenKind::Eof]);
        assert_eq!(lex("**"), vec![TokenKind::StarStar,TokenKind::Eof]);
    }

    #[test]
    fn test_overflow_operators() {
        assert_eq!(lex("+%"), vec![TokenKind::PlusPct,  TokenKind::Eof]);
        assert_eq!(lex("-|"), vec![TokenKind::MinusBar,  TokenKind::Eof]);
        assert_eq!(lex("*!"), vec![TokenKind::StarExcl,  TokenKind::Eof]);
        assert_eq!(lex("+|"), vec![TokenKind::PlusBar,   TokenKind::Eof]);
    }

    #[test]
    fn test_comparison_operators() {
        assert_eq!(lex("=="), vec![TokenKind::EqEq,   TokenKind::Eof]);
        assert_eq!(lex("!="), vec![TokenKind::BangEq, TokenKind::Eof]);
        assert_eq!(lex("<="), vec![TokenKind::LtEq,   TokenKind::Eof]);
        assert_eq!(lex(">="), vec![TokenKind::GtEq,   TokenKind::Eof]);
    }

    #[test]
    fn test_assignment_operators() {
        assert_eq!(lex(":="), vec![TokenKind::ColonEq, TokenKind::Eof]);
        assert_eq!(lex("+="), vec![TokenKind::PlusEq,  TokenKind::Eof]);
        assert_eq!(lex("-="), vec![TokenKind::MinusEq, TokenKind::Eof]);
        assert_eq!(lex("**="),vec![TokenKind::StarStarEq, TokenKind::Eof]);
        assert_eq!(lex("<<="),vec![TokenKind::LtLtEq,  TokenKind::Eof]);
        assert_eq!(lex(">>="),vec![TokenKind::GtGtEq,  TokenKind::Eof]);
    }

    #[test]
    fn test_special_operators() {
        assert_eq!(lex("->"),  vec![TokenKind::Arrow,   TokenKind::Eof]);
        assert_eq!(lex("=>"),  vec![TokenKind::FatArrow,TokenKind::Eof]);
        assert_eq!(lex(".."),  vec![TokenKind::DotDot,  TokenKind::Eof]);
        assert_eq!(lex("..="), vec![TokenKind::DotDotEq,TokenKind::Eof]);
        assert_eq!(lex("|>"),  vec![TokenKind::PipeGt,  TokenKind::Eof]);
        assert_eq!(lex("??"),  vec![TokenKind::QQ,       TokenKind::Eof]);
        assert_eq!(lex("?."),  vec![TokenKind::QDot,     TokenKind::Eof]);
    }

    // ── Indentation ───────────────────────────────────────────────────────────

    #[test]
    fn test_indent_dedent_basic() {
        let src = "fn foo():\n    return 1\n";
        let kinds = lex_all(src);
        // Should contain: Kw(Fn) Ident LParen RParen Colon Newline Indent Kw(Return) Int Newline Dedent Eof
        assert!(kinds.contains(&TokenKind::Indent),  "missing Indent");
        assert!(kinds.contains(&TokenKind::Dedent),  "missing Dedent");
        assert!(kinds.contains(&TokenKind::Newline), "missing Newline");
    }

    #[test]
    fn test_nested_indent() {
        let src = "if x:\n    if y:\n        z\n";
        let kinds = lex_all(src);
        let indent_count = kinds.iter().filter(|k| **k == TokenKind::Indent).count();
        let dedent_count = kinds.iter().filter(|k| **k == TokenKind::Dedent).count();
        assert_eq!(indent_count, 2, "expected 2 indents");
        assert_eq!(dedent_count, 2, "expected 2 dedents");
    }

    #[test]
    fn test_tab_indent_error() {
        has_error("\tfn foo():", "E0004");
    }

    // ── Comments ──────────────────────────────────────────────────────────────

    #[test]
    fn test_line_comment() {
        let mut l = Lexer::new("# this is a comment\nfn");
        let toks = l.tokenize();
        let kinds: Vec<_> = toks.iter().map(|t| &t.kind).collect();
        assert!(kinds.iter().any(|k| matches!(k, TokenKind::LineComment(_))));
        assert!(kinds.iter().any(|k| matches!(k, TokenKind::Kw(Keyword::Fn))));
    }

    #[test]
    fn test_doc_comment() {
        let mut l = Lexer::new("## This is a doc comment\nfn");
        let toks = l.tokenize();
        assert!(toks.iter().any(|t| matches!(t.kind, TokenKind::DocComment(_))));
    }

    // ── Lifetime ──────────────────────────────────────────────────────────────

    #[test]
    fn test_lifetime() {
        assert_eq!(lex("'a"),
            vec![TokenKind::Lifetime("a".into()), TokenKind::Eof]);
        assert_eq!(lex("'static"),
            vec![TokenKind::Lifetime("static".into()), TokenKind::Eof]);
    }

    // ── Full programs ─────────────────────────────────────────────────────────

    #[test]
    fn test_hello_world() {
        let src = r#"
module main
import "std/io"

fn main() -> i32:
    io.println("Hello, World!")
    return 0
"#;
        no_errors(src);
        let kinds = lex(src);
        assert!(kinds.contains(&TokenKind::Kw(Keyword::Module)));
        assert!(kinds.contains(&TokenKind::Kw(Keyword::Import)));
        assert!(kinds.contains(&TokenKind::Kw(Keyword::Fn)));
        assert!(kinds.contains(&TokenKind::Kw(Keyword::Return)));
        assert!(kinds.contains(&TokenKind::Int(0, None)));
    }

    #[test]
    fn test_struct_with_methods() {
        let src = r#"
struct Point:
    x: f64
    y: f64

fn Point.distance(self, other: Point) -> f64:
    var dx := self.x - other.x
    var dy := self.y - other.y
    return (dx * dx + dy * dy) ** 0.5
"#;
        no_errors(src);
    }

    #[test]
    fn test_match_expression() {
        let src = r#"
match status:
    200 => "ok"
    404 => "not found"
    _   => "unknown"
"#;
        no_errors(src);
    }

    #[test]
    fn test_fstring() {
        let mut l = Lexer::new(r#"f"Hello, {name}!""#);
        let toks = l.tokenize();
        assert!(l.errors.is_empty());
        assert!(toks.iter().any(|t| matches!(t.kind, TokenKind::FStr(_))));
    }

    #[test]
    fn test_all_overflow_assign_ops() {
        no_errors("+%= -%= *%= +|= -|= *|=");
    }

    #[test]
    fn test_span_correctness() {
        let src = "fn add";
        let mut l = Lexer::new(src);
        let toks = l.tokenize();
        // "fn" starts at 0, ends at 2
        assert_eq!(toks[0].span.start, 0);
        assert_eq!(toks[0].span.end,   2);
        // "add" starts at 3, ends at 6
        assert_eq!(toks[1].span.start, 3);
        assert_eq!(toks[1].span.end,   6);
    }
}
