use super::token::{FloatSuffix, FStringPart, IntSuffix, Keyword, Span, Token, TokenKind};
use super::error::*;

// ── Lexer ─────────────────────────────────────────────────────────────────────

pub struct Lexer<'src> {
    /// Full source text as bytes (UTF-8).
    src:   &'src str,
    /// Current byte position in `src`.
    pos:   usize,

    /// Stack of indentation levels (in spaces). Always has [0] at bottom.
    indent_stack: Vec<usize>,
    /// How many DEDENT tokens are waiting to be emitted.
    pending_dedents: usize,
    /// Whether we are at the beginning of a new line
    /// (after a newline token was emitted, before content is seen).
    at_line_start: bool,
    /// Whether the very first line has been processed.
    first_line: bool,

    /// Accumulated diagnostics (non-fatal errors continue lexing).
    pub errors: Vec<LexError>,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            pos:             0,
            indent_stack:    vec![0],
            pending_dedents: 0,
            at_line_start:   true,
            first_line:      true,
            errors:          Vec::new(),
        }
    }

    // ── Tokenize entire source ────────────────────────────────────────────────

    /// Lex the entire source and return a flat `Vec<Token>`.
    /// Comments are included (for doc-generation). Filter them out if not needed.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let is_eof = tok.kind == TokenKind::Eof;
            tokens.push(tok);
            if is_eof { break; }
        }
        tokens
    }

    // ── Core: produce one token ───────────────────────────────────────────────

    pub fn next_token(&mut self) -> Token {
        // ── 1. Emit pending DEDENTs first ────────────────────────────────────
        if self.pending_dedents > 0 {
            self.pending_dedents -= 1;
            let sp = self.span_here();
            return Token::new(TokenKind::Dedent, sp);
        }

        // ── 2. Handle indentation at the start of a new logical line ─────────
        if self.at_line_start {
            self.at_line_start = false;
            if let Some(tok) = self.handle_indent() {
                return tok;
            }
        }

        // ── 3. Skip inline whitespace (spaces/tabs between tokens on a line) ─
        self.skip_spaces();

        // ── 4. EOF ────────────────────────────────────────────────────────────
        if self.pos >= self.src.len() {
            // Close all remaining indent levels
            if self.indent_stack.len() > 1 {
                self.indent_stack.pop();
                self.pending_dedents = self.indent_stack.len() - 1;
                // reset to 0 for next call
                self.indent_stack.truncate(1);
                let sp = self.span_here();
                return Token::new(TokenKind::Dedent, sp);
            }
            return Token::new(TokenKind::Eof, self.span_here());
        }

        let start = self.pos;
        let ch    = self.peek_char();

        match ch {
            // ── Newline ───────────────────────────────────────────────────────
            '\n' => {
                self.advance();
                self.at_line_start = true;
                Token::new(TokenKind::Newline, Span::new(start as u32, self.pos as u32))
            }
            '\r' => {
                self.advance();
                if self.peek_char() == '\n' { self.advance(); }
                self.at_line_start = true;
                Token::new(TokenKind::Newline, Span::new(start as u32, self.pos as u32))
            }

            // ── Comments & # ──────────────────────────────────────────────────
            '#' => {
                // Check for #![ (compiler directive)
                if self.starts_with("#![") {
                    self.advance(); self.advance(); self.advance(); // consume #![
                    let s = self.read_until(']');
                    self.advance(); // consume ]
                    Token::new(TokenKind::HashBang,
                        Span::new(start as u32, self.pos as u32))
                } else if self.starts_with("##") {
                    // Doc comment
                    self.advance(); self.advance(); // consume ##
                    let text = self.read_line();
                    Token::new(TokenKind::DocComment(text.trim().to_string()),
                        Span::new(start as u32, self.pos as u32))
                } else {
                    // Regular comment
                    self.advance(); // consume #
                    let text = self.read_line();
                    Token::new(TokenKind::LineComment(text.trim().to_string()),
                        Span::new(start as u32, self.pos as u32))
                }
            }

            // ── String literals ───────────────────────────────────────────────
            '"' => self.lex_string(start),
            'r' if self.peek_at(1) == '"' => {
                self.advance(); // consume r
                self.lex_raw_string(start)
            }
            'r' if self.peek_at(1) == '"' => self.lex_raw_string(start),
            'f' if self.peek_at(1) == '"' => {
                self.advance(); // consume f
                self.lex_fstring(start)
            }
            'b' if self.peek_at(1) == '\'' => {
                self.advance(); // consume b
                self.lex_byte_lit(start)
            }

            // ── Character literal ─────────────────────────────────────────────
            '\'' => self.lex_char_or_lifetime(start),

            // ── Numbers ───────────────────────────────────────────────────────
            '0'..='9' => self.lex_number(start),
            '.' if self.peek_at(1).is_ascii_digit() => self.lex_number(start),

            // ── Identifiers & keywords ────────────────────────────────────────
            c if is_ident_start(c) => self.lex_ident(start),

            // ── Operators & punctuation ───────────────────────────────────────
            '+' => self.lex_plus(start),
            '-' => self.lex_minus(start),
            '*' => self.lex_star(start),
            '/' => self.lex_slash(start),
            '%' => self.lex_percent(start),
            '&' => self.lex_amp(start),
            '|' => self.lex_bar(start),
            '^' => { self.advance(); self.maybe_eq(start, TokenKind::Caret, TokenKind::CaretEq) }
            '~' => { self.advance(); Token::new(TokenKind::Tilde,     self.span(start)) }
            '<' => self.lex_lt(start),
            '>' => self.lex_gt(start),
            '=' => self.lex_eq(start),
            '!' => self.lex_bang(start),
            '?' => self.lex_question(start),
            '.' => self.lex_dot(start),
            '@' => { self.advance(); Token::new(TokenKind::At,       self.span(start)) }
            ',' => { self.advance(); Token::new(TokenKind::Comma,    self.span(start)) }
            ':' => self.lex_colon(start),
            ';' => { self.advance(); Token::new(TokenKind::Semicolon,self.span(start)) }
            '(' => { self.advance(); Token::new(TokenKind::LParen,   self.span(start)) }
            ')' => { self.advance(); Token::new(TokenKind::RParen,   self.span(start)) }
            '[' => { self.advance(); Token::new(TokenKind::LBracket, self.span(start)) }
            ']' => { self.advance(); Token::new(TokenKind::RBracket, self.span(start)) }
            '{' => { self.advance(); Token::new(TokenKind::LBrace,   self.span(start)) }
            '}' => { self.advance(); Token::new(TokenKind::RBrace,   self.span(start)) }

            c => {
                let sp = Span::new(start as u32, (start + c.len_utf8()) as u32);
                self.errors.push(e0001_unexpected_char(c, sp));
                self.advance_char();
                // Return a dummy token and continue
                Token::new(TokenKind::Eof, sp)
            }
        }
    }

    // ── Indentation ───────────────────────────────────────────────────────────

    /// Called when `at_line_start` is true.
    /// Counts leading spaces, compares to indent stack, emits INDENT/DEDENT.
    /// Returns `Some(token)` if an indent/dedent token should be emitted now,
    /// or `None` if the indent level is unchanged (caller continues normally).
    fn handle_indent(&mut self) -> Option<Token> {
        // Skip entirely blank lines and comment-only lines
        let saved = self.pos;
        let spaces = self.count_leading_spaces();

        // Check if line is blank or comment
        let next = self.peek_char();
        if next == '\n' || next == '\r' || next == '\0' {
            // blank line — don't change indent state
            return None;
        }
        if next == '#' {
            // comment line — don't change indent state
            return None;
        }

        let sp = Span::new(saved as u32, self.pos as u32);
        let current = *self.indent_stack.last().unwrap();

        if spaces == current {
            // Same level — nothing to emit, just continue
            None
        } else if spaces > current {
            // Deeper — emit INDENT
            self.indent_stack.push(spaces);
            Some(Token::new(TokenKind::Indent, sp))
        } else {
            // Shallower — emit DEDENT(s)
            // Pop until we match or underflow
            while *self.indent_stack.last().unwrap() > spaces {
                self.indent_stack.pop();
            }
            if *self.indent_stack.last().unwrap() != spaces {
                self.errors.push(e0003_bad_indent(sp));
            }
            // We emit one DEDENT now, queue the rest
            let extra = self.indent_stack.len().saturating_sub(1);
            // Actually: we need one DEDENT per level closed
            let levels_closed = {
                // count how many were popped (we already popped them above)
                // approximate: compute from current stack size vs before
                // simpler: just emit 1 now, track pending
                let closed = (current - spaces) / 4;
                closed.max(1)
            };
            if levels_closed > 1 {
                self.pending_dedents = levels_closed - 1;
            }
            Some(Token::new(TokenKind::Dedent, sp))
        }
    }

    /// Count leading spaces on the current line.
    /// Errors on tabs (E0004). Returns the count.
    fn count_leading_spaces(&mut self) -> usize {
        let mut count = 0;
        loop {
            match self.peek_char() {
                ' ' => { self.advance(); count += 1; }
                '\t' => {
                    let sp = self.span_here();
                    self.errors.push(e0004_mixed_indent(sp));
                    self.advance();
                    count += 4; // treat tab as 4 for recovery
                }
                _ => break,
            }
        }
        count
    }

    // ── String lexing ─────────────────────────────────────────────────────────

    fn lex_string(&mut self, start: usize) -> Token {
        self.advance(); // consume opening "

        // Check for triple-quoted multiline string
        if self.starts_with("\"\"") {
            self.advance(); self.advance(); // consume ""
            return self.lex_multiline_string(start);
        }

        let mut s = String::new();
        loop {
            match self.peek_char() {
                '\0' | '\n' => {
                    self.errors.push(e0007_unterminated_string(self.span(start)));
                    break;
                }
                '"' => { self.advance(); break; }
                '\\' => {
                    self.advance();
                    match self.process_escape(start) {
                        Ok(ch) => s.push(ch),
                        Err(_) => {} // error already pushed
                    }
                }
                c => { s.push(c); self.advance_char(); }
            }
        }
        Token::new(TokenKind::Str(s), self.span(start))
    }

    fn lex_multiline_string(&mut self, start: usize) -> Token {
        let mut s = String::new();
        // Skip first newline if present
        if self.peek_char() == '\n' { self.advance(); }
        else if self.peek_char() == '\r' {
            self.advance();
            if self.peek_char() == '\n' { self.advance(); }
        }

        loop {
            if self.starts_with("\"\"\"") {
                self.advance(); self.advance(); self.advance();
                break;
            }
            if self.pos >= self.src.len() {
                self.errors.push(e0007_unterminated_string(self.span(start)));
                break;
            }
            let c = self.peek_char();
            s.push(c);
            self.advance_char();
        }
        // Strip common leading whitespace
        let stripped = strip_indent(&s);
        Token::new(TokenKind::Str(stripped), self.span(start))
    }

    fn lex_raw_string(&mut self, start: usize) -> Token {
        self.advance(); // consume opening "
        let mut s = String::new();
        loop {
            match self.peek_char() {
                '\0' => {
                    self.errors.push(e0007_unterminated_string(self.span(start)));
                    break;
                }
                '"' => { self.advance(); break; }
                c => { s.push(c); self.advance_char(); }
            }
        }
        Token::new(TokenKind::RawStr(s), self.span(start))
    }

    fn lex_fstring(&mut self, start: usize) -> Token {
        self.advance(); // consume opening "
        let mut parts: Vec<FStringPart> = Vec::new();
        let mut text = String::new();

        loop {
            match self.peek_char() {
                '\0' | '\n' => {
                    self.errors.push(e0007_unterminated_string(self.span(start)));
                    break;
                }
                '"' => { self.advance(); break; }
                '{' => {
                    // Check for {{ escape
                    if self.peek_at(1) == '{' {
                        text.push('{');
                        self.advance(); self.advance();
                        continue;
                    }
                    // Push accumulated text
                    if !text.is_empty() {
                        parts.push(FStringPart::Text(std::mem::take(&mut text)));
                    }
                    self.advance(); // consume {
                    // Collect tokens until }
                    let (tokens, fmt) = self.lex_fstring_expr();
                    if let Some(fmt_str) = fmt {
                        parts.push(FStringPart::ExprFmt { tokens, fmt: fmt_str });
                    } else {
                        parts.push(FStringPart::Expr(tokens));
                    }
                }
                '}' if self.peek_at(1) == '}' => {
                    text.push('}');
                    self.advance(); self.advance();
                }
                '\\' => {
                    self.advance();
                    match self.process_escape(start) {
                        Ok(ch) => text.push(ch),
                        Err(_) => {}
                    }
                }
                c => { text.push(c); self.advance_char(); }
            }
        }
        if !text.is_empty() {
            parts.push(FStringPart::Text(text));
        }
        Token::new(TokenKind::FStr(parts), self.span(start))
    }

    /// Lex the expression inside `{...}` in an f-string.
    /// Returns (tokens, Some(format_spec)) or (tokens, None).
    fn lex_fstring_expr(&mut self) -> (Vec<Token>, Option<String>) {
        let mut tokens = Vec::new();
        let mut depth  = 1usize; // track nested braces

        loop {
            self.skip_spaces();
            match self.peek_char() {
                '\0' => break,
                ':' if depth == 1 => {
                    // Format spec follows
                    self.advance();
                    let fmt = self.read_until('}');
                    self.advance(); // consume }
                    return (tokens, Some(fmt));
                }
                '}' => {
                    depth -= 1;
                    self.advance();
                    if depth == 0 { break; }
                }
                '{' => { depth += 1; self.advance(); }
                _ => {
                    let tok = self.next_token();
                    tokens.push(tok);
                }
            }
        }
        (tokens, None)
    }

    // ── Character & byte literals ─────────────────────────────────────────────

    fn lex_char_or_lifetime(&mut self, start: usize) -> Token {
        self.advance(); // consume '

        // Detect lifetime: 'ident (not followed by content + closing quote)
        let next = self.peek_char();
        if next.is_alphabetic() || next == '_' {
            // Could be lifetime 'a or char 'a'
            let name_start = self.pos;
            while is_ident_continue(self.peek_char()) { self.advance_char(); }
            let name = self.src[name_start..self.pos].to_string();

            if self.peek_char() == '\'' {
                // It's a char literal: 'x'
                self.advance(); // consume closing '
                let ch = name.chars().next().unwrap_or('\0');
                return Token::new(TokenKind::Char(ch), self.span(start));
            } else {
                // It's a lifetime: 'name
                return Token::new(TokenKind::Lifetime(name), self.span(start));
            }
        }

        // Regular char literal
        let ch = match self.peek_char() {
            '\\' => {
                self.advance();
                self.process_escape(start).unwrap_or('\0')
            }
            '\0' | '\'' => {
                self.errors.push(e0007_unterminated_string(self.span(start)));
                '\0'
            }
            c => { let r = c; self.advance_char(); r }
        };
        if self.peek_char() == '\'' {
            self.advance();
        } else {
            self.errors.push(e0007_unterminated_string(self.span(start)));
        }
        Token::new(TokenKind::Char(ch), self.span(start))
    }

    fn lex_byte_lit(&mut self, start: usize) -> Token {
        self.advance(); // consume '
        let b = match self.peek_char() {
            '\\' => {
                self.advance();
                self.process_escape(start).unwrap_or('\0') as u8
            }
            c if c.is_ascii() => { let b = c as u8; self.advance(); b }
            _ => {
                self.errors.push(e0001_unexpected_char(self.peek_char(), self.span_here()));
                0
            }
        };
        if self.peek_char() == '\'' { self.advance(); }
        Token::new(TokenKind::Byte(b), self.span(start))
    }

    // ── Escape sequence processing ────────────────────────────────────────────

    fn process_escape(&mut self, lit_start: usize) -> Result<char, ()> {
        let esc_start = self.pos - 1; // position of the backslash
        match self.peek_char() {
            'n'  => { self.advance(); Ok('\n') }
            'r'  => { self.advance(); Ok('\r') }
            't'  => { self.advance(); Ok('\t') }
            '\\' => { self.advance(); Ok('\\') }
            '"'  => { self.advance(); Ok('"')  }
            '\'' => { self.advance(); Ok('\'') }
            '0'  => { self.advance(); Ok('\0') }
            'a'  => { self.advance(); Ok('\x07') }
            'b'  => { self.advance(); Ok('\x08') }
            'f'  => { self.advance(); Ok('\x0C') }
            'v'  => { self.advance(); Ok('\x0B') }
            'x'  => {
                // \xHH
                self.advance();
                let h1 = self.advance_hex_digit()?;
                let h2 = self.advance_hex_digit()?;
                let byte = h1 * 16 + h2;
                Ok(byte as char)
            }
            'u' => {
                // \u{HHHH}
                self.advance();
                if self.peek_char() != '{' {
                    let sp = Span::new(esc_start as u32, self.pos as u32);
                    self.errors.push(e0008_invalid_unicode(sp));
                    return Err(());
                }
                self.advance(); // consume {
                let mut value: u32 = 0;
                let mut count = 0;
                loop {
                    match self.peek_char() {
                        '}' => { self.advance(); break; }
                        c if c.is_ascii_hexdigit() => {
                            value = value * 16 + c.to_digit(16).unwrap();
                            count += 1;
                            self.advance();
                            if count > 6 {
                                let sp = Span::new(esc_start as u32, self.pos as u32);
                                self.errors.push(e0008_invalid_unicode(sp));
                                return Err(());
                            }
                        }
                        _ => {
                            let sp = Span::new(esc_start as u32, self.pos as u32);
                            self.errors.push(e0008_invalid_unicode(sp));
                            return Err(());
                        }
                    }
                }
                char::from_u32(value).ok_or_else(|| {
                    let sp = Span::new(esc_start as u32, self.pos as u32);
                    self.errors.push(e0008_invalid_unicode(sp));
                })
            }
            c => {
                let sp = Span::new(esc_start as u32, self.pos as u32);
                self.errors.push(e0006_invalid_escape(c, sp));
                self.advance_char();
                Err(())
            }
        }
    }

    fn advance_hex_digit(&mut self) -> Result<u8, ()> {
        let c = self.peek_char();
        if c.is_ascii_hexdigit() {
            self.advance();
            Ok(c.to_digit(16).unwrap() as u8)
        } else {
            let sp = self.span_here();
            self.errors.push(e0008_invalid_unicode(sp));
            Err(())
        }
    }

    // ── Number lexing ─────────────────────────────────────────────────────────

    fn lex_number(&mut self, start: usize) -> Token {
        // Detect base
        if self.peek_char() == '0' {
            match self.peek_at(1) {
                'x' | 'X' => return self.lex_int_with_base(start, 16, "0x"),
                'b' | 'B' => return self.lex_int_with_base(start, 2,  "0b"),
                'o' | 'O' => return self.lex_int_with_base(start, 8,  "0o"),
                _ => {}
            }
        }

        // Decimal integer or float
        let int_part = self.read_digits(10);

        // Check for float: decimal point followed by digit
        if self.peek_char() == '.' && self.peek_at(1).is_ascii_digit() {
            self.advance(); // consume .
            let frac_part = self.read_digits(10);
            return self.finish_float(start, &format!("{}.{}", int_part, frac_part));
        }
        // Or float: leading dot
        if self.peek_char() == '.' && int_part.is_empty() {
            self.advance(); // consume .
            let frac_part = self.read_digits(10);
            return self.finish_float(start, &format!("0.{}", frac_part));
        }
        // Check for exponent on integer (makes it a float)
        if self.peek_char() == 'e' || self.peek_char() == 'E' {
            return self.finish_float(start, &int_part);
        }

        // Integer suffix
        let suffix = self.read_int_suffix();
        let clean: String = int_part.chars().filter(|&c| c != '_').collect();
        match u128::from_str_radix(&clean, 10) {
            Ok(v) => Token::new(TokenKind::Int(v, suffix), self.span(start)),
            Err(e) => {
                self.errors.push(e0009_invalid_number(e.to_string(), self.span(start)));
                Token::new(TokenKind::Int(0, suffix), self.span(start))
            }
        }
    }

    fn lex_int_with_base(&mut self, start: usize, base: u32, prefix: &str) -> Token {
        self.advance(); self.advance(); // consume prefix (0x / 0b / 0o)
        let digits = self.read_digits(base);
        if digits.is_empty() {
            self.errors.push(e0009_invalid_number(
                format!("expected digits after `{}`", prefix), self.span(start)));
            return Token::new(TokenKind::Int(0, None), self.span(start));
        }
        let suffix = self.read_int_suffix();
        let clean: String = digits.chars().filter(|&c| c != '_').collect();
        match u128::from_str_radix(&clean, base) {
            Ok(v) => Token::new(TokenKind::Int(v, suffix), self.span(start)),
            Err(e) => {
                self.errors.push(e0009_invalid_number(e.to_string(), self.span(start)));
                Token::new(TokenKind::Int(0, suffix), self.span(start))
            }
        }
    }

    fn finish_float(&mut self, start: usize, so_far: &str) -> Token {
        // Exponent part
        let mut s = so_far.to_string();
        if self.peek_char() == 'e' || self.peek_char() == 'E' {
            s.push('e');
            self.advance();
            if self.peek_char() == '+' || self.peek_char() == '-' {
                s.push(self.peek_char());
                self.advance();
            }
            s.push_str(&self.read_digits(10));
        }
        // Float suffix
        let suffix = if self.peek_char() == 'f' {
            match (self.peek_at(1), self.peek_at(2)) {
                ('3', '2') => { self.advance(); self.advance(); self.advance(); Some(FloatSuffix::F32) }
                ('6', '4') => { self.advance(); self.advance(); self.advance(); Some(FloatSuffix::F64) }
                _ => None,
            }
        } else { None };

        let clean: String = s.chars().filter(|&c| c != '_').collect();
        match clean.parse::<f64>() {
            Ok(v) => Token::new(TokenKind::Float(v, suffix), self.span(start)),
            Err(e) => {
                self.errors.push(e0009_invalid_number(e.to_string(), self.span(start)));
                Token::new(TokenKind::Float(0.0, suffix), self.span(start))
            }
        }
    }

    /// Read consecutive digits (and underscores) of the given base.
    fn read_digits(&mut self, base: u32) -> String {
        let mut s = String::new();
        loop {
            let c = self.peek_char();
            if c == '_' {
                s.push(c); self.advance();
            } else if c.is_digit(base) {
                s.push(c); self.advance();
            } else {
                break;
            }
        }
        s
    }

    fn read_int_suffix(&mut self) -> Option<IntSuffix> {
        // Peek ahead to detect suffix
        let rest = &self.src[self.pos..];
        let suffixes = [
            ("i128", IntSuffix::I128), ("i64", IntSuffix::I64),
            ("i32",  IntSuffix::I32),  ("i16", IntSuffix::I16), ("i8", IntSuffix::I8),
            ("u128", IntSuffix::U128), ("u64", IntSuffix::U64),
            ("u32",  IntSuffix::U32),  ("u16", IntSuffix::U16), ("u8", IntSuffix::U8),
            ("usize",IntSuffix::Usize),("isize",IntSuffix::Isize),
        ];
        for (s, suf) in &suffixes {
            if rest.starts_with(s) {
                // Make sure it's not part of a longer ident
                let after = rest.get(s.len()..).unwrap_or("");
                if after.is_empty() || !is_ident_continue(after.chars().next().unwrap()) {
                    self.pos += s.len();
                    return Some(*suf);
                }
            }
        }
        None
    }

    // ── Identifier / keyword ──────────────────────────────────────────────────

    fn lex_ident(&mut self, start: usize) -> Token {
        while is_ident_continue(self.peek_char()) {
            self.advance_char();
        }
        let text = &self.src[start..self.pos];

        // Special: _ alone is a wildcard
        if text == "_" {
            return Token::new(TokenKind::Underscore, self.span(start));
        }

        // Keyword check
        if let Some(kw) = Keyword::from_str(text) {
            return Token::new(TokenKind::Kw(kw), self.span(start));
        }

        Token::new(TokenKind::Ident(text.to_string()), self.span(start))
    }

    // ── Operator lexing ───────────────────────────────────────────────────────

    fn lex_plus(&mut self, start: usize) -> Token {
        self.advance(); // consume +
        match self.peek_char() {
            '=' => { self.advance(); Token::new(TokenKind::PlusEq,   self.span(start)) }
            '%' => { self.advance();
                if self.peek_char() == '=' { self.advance(); Token::new(TokenKind::PlusPctEq, self.span(start)) }
                else { Token::new(TokenKind::PlusPct, self.span(start)) } }
            '|' => { self.advance();
                if self.peek_char() == '=' { self.advance(); Token::new(TokenKind::PlusBarEq, self.span(start)) }
                else { Token::new(TokenKind::PlusBar, self.span(start)) } }
            '!' => { self.advance(); Token::new(TokenKind::PlusExcl, self.span(start)) }
            _   => Token::new(TokenKind::Plus, self.span(start)),
        }
    }

    fn lex_minus(&mut self, start: usize) -> Token {
        self.advance(); // consume -
        match self.peek_char() {
            '>' => { self.advance(); Token::new(TokenKind::Arrow,     self.span(start)) }
            '=' => { self.advance(); Token::new(TokenKind::MinusEq,   self.span(start)) }
            '%' => { self.advance();
                if self.peek_char() == '=' { self.advance(); Token::new(TokenKind::MinusPctEq, self.span(start)) }
                else { Token::new(TokenKind::MinusPct, self.span(start)) } }
            '|' => { self.advance();
                if self.peek_char() == '=' { self.advance(); Token::new(TokenKind::MinusBarEq, self.span(start)) }
                else { Token::new(TokenKind::MinusBar, self.span(start)) } }
            '!' => { self.advance(); Token::new(TokenKind::MinusExcl, self.span(start)) }
            _   => Token::new(TokenKind::Minus, self.span(start)),
        }
    }

    fn lex_star(&mut self, start: usize) -> Token {
        self.advance(); // consume *
        match self.peek_char() {
            '*' => { self.advance();
                if self.peek_char() == '=' { self.advance(); Token::new(TokenKind::StarStarEq, self.span(start)) }
                else { Token::new(TokenKind::StarStar, self.span(start)) } }
            '=' => { self.advance(); Token::new(TokenKind::StarEq,   self.span(start)) }
            '%' => { self.advance();
                if self.peek_char() == '=' { self.advance(); Token::new(TokenKind::StarPctEq, self.span(start)) }
                else { Token::new(TokenKind::StarPct, self.span(start)) } }
            '|' => { self.advance();
                if self.peek_char() == '=' { self.advance(); Token::new(TokenKind::StarBarEq, self.span(start)) }
                else { Token::new(TokenKind::StarBar, self.span(start)) } }
            '!' => { self.advance(); Token::new(TokenKind::StarExcl, self.span(start)) }
            _   => Token::new(TokenKind::Star, self.span(start)),
        }
    }

    fn lex_slash(&mut self, start: usize) -> Token {
        self.advance(); // consume /
        if self.peek_char() == '=' {
            self.advance();
            Token::new(TokenKind::SlashEq, self.span(start))
        } else {
            Token::new(TokenKind::Slash, self.span(start))
        }
    }

    fn lex_percent(&mut self, start: usize) -> Token {
        self.advance(); // consume %
        if self.peek_char() == '=' {
            self.advance();
            Token::new(TokenKind::PercentEq, self.span(start))
        } else {
            Token::new(TokenKind::Percent, self.span(start))
        }
    }

    fn lex_amp(&mut self, start: usize) -> Token {
        self.advance(); // consume &
        if self.peek_char() == '=' {
            self.advance();
            Token::new(TokenKind::AmpEq, self.span(start))
        } else {
            Token::new(TokenKind::Amp, self.span(start))
        }
    }

    fn lex_bar(&mut self, start: usize) -> Token {
        self.advance(); // consume |
        match self.peek_char() {
            '=' => { self.advance(); Token::new(TokenKind::BarEq,   self.span(start)) }
            '>' => { self.advance(); Token::new(TokenKind::PipeGt,  self.span(start)) }
            _   => Token::new(TokenKind::Bar, self.span(start)),
        }
    }

    fn lex_lt(&mut self, start: usize) -> Token {
        self.advance(); // consume <
        match self.peek_char() {
            '=' => { self.advance(); Token::new(TokenKind::LtEq,   self.span(start)) }
            '<' => { self.advance();
                if self.peek_char() == '=' { self.advance(); Token::new(TokenKind::LtLtEq, self.span(start)) }
                else { Token::new(TokenKind::LtLt, self.span(start)) } }
            _   => Token::new(TokenKind::Lt, self.span(start)),
        }
    }

    fn lex_gt(&mut self, start: usize) -> Token {
        self.advance(); // consume >
        match self.peek_char() {
            '=' => { self.advance(); Token::new(TokenKind::GtEq,   self.span(start)) }
            '>' => { self.advance();
                if self.peek_char() == '=' { self.advance(); Token::new(TokenKind::GtGtEq, self.span(start)) }
                else { Token::new(TokenKind::GtGt, self.span(start)) } }
            _   => Token::new(TokenKind::Gt, self.span(start)),
        }
    }

    fn lex_eq(&mut self, start: usize) -> Token {
        self.advance(); // consume =
        match self.peek_char() {
            '=' => { self.advance(); Token::new(TokenKind::EqEq,    self.span(start)) }
            '>' => { self.advance(); Token::new(TokenKind::FatArrow, self.span(start)) }
            _   => Token::new(TokenKind::Eq, self.span(start)),
        }
    }

    fn lex_bang(&mut self, start: usize) -> Token {
        self.advance(); // consume !
        if self.peek_char() == '=' {
            self.advance();
            Token::new(TokenKind::BangEq, self.span(start))
        } else {
            Token::new(TokenKind::Bang, self.span(start))
        }
    }

    fn lex_question(&mut self, start: usize) -> Token {
        self.advance(); // consume ?
        match self.peek_char() {
            '?' => { self.advance(); Token::new(TokenKind::QQ,      self.span(start)) }
            '.' => { self.advance(); Token::new(TokenKind::QDot,    self.span(start)) }
            _   => Token::new(TokenKind::Question, self.span(start)),
        }
    }

    fn lex_dot(&mut self, start: usize) -> Token {
        self.advance(); // consume .
        match self.peek_char() {
            '.' => {
                self.advance();
                if self.peek_char() == '=' {
                    self.advance();
                    Token::new(TokenKind::DotDotEq, self.span(start))
                } else {
                    Token::new(TokenKind::DotDot, self.span(start))
                }
            }
            c if c.is_ascii_digit() => {
                // .5 float
                let frac = self.read_digits(10);
                self.finish_float(start, &format!("0.{}", frac))
            }
            _ => Token::new(TokenKind::Dot, self.span(start)),
        }
    }

    fn lex_colon(&mut self, start: usize) -> Token {
        self.advance(); // consume :
        if self.peek_char() == '=' {
            self.advance();
            Token::new(TokenKind::ColonEq, self.span(start))
        } else {
            Token::new(TokenKind::Colon, self.span(start))
        }
    }

    // ── Helper: maybe_eq ──────────────────────────────────────────────────────

    fn maybe_eq(&mut self, start: usize, base: TokenKind, with_eq: TokenKind) -> Token {
        if self.peek_char() == '=' {
            self.advance();
            Token::new(with_eq, self.span(start))
        } else {
            Token::new(base, self.span(start))
        }
    }

    // ── Source navigation helpers ─────────────────────────────────────────────

    /// Peek current character (ASCII fast path, falls back to UTF-8).
    #[inline]
    fn peek_char(&self) -> char {
        self.src[self.pos..].chars().next().unwrap_or('\0')
    }

    /// Peek character `offset` bytes ahead (byte offset, not char offset).
    #[inline]
    fn peek_at(&self, offset: usize) -> char {
        self.src[self.pos + offset..].chars().next().unwrap_or('\0')
    }

    /// Advance by one byte (ASCII).
    #[inline]
    fn advance(&mut self) {
        if self.pos < self.src.len() {
            self.pos += 1;
        }
    }

    /// Advance by one Unicode character (correct for multi-byte).
    #[inline]
    fn advance_char(&mut self) {
        if let Some(c) = self.src[self.pos..].chars().next() {
            self.pos += c.len_utf8();
        }
    }

    /// Skip spaces and tabs on the current line (NOT newlines).
    fn skip_spaces(&mut self) {
        while self.pos < self.src.len() {
            match self.peek_char() {
                ' ' | '\t' => self.advance(),
                _ => break,
            }
        }
    }

    /// Read until end of line (does not consume the newline).
    fn read_line(&mut self) -> String {
        let start = self.pos;
        while self.pos < self.src.len() {
            match self.peek_char() {
                '\n' | '\r' => break,
                _ => self.advance_char(),
            }
        }
        self.src[start..self.pos].to_string()
    }

    /// Read until a specific character (does not consume that character).
    fn read_until(&mut self, stop: char) -> String {
        let start = self.pos;
        while self.pos < self.src.len() && self.peek_char() != stop {
            self.advance_char();
        }
        self.src[start..self.pos].to_string()
    }

    /// Returns true if the source at the current position starts with `s`.
    fn starts_with(&self, s: &str) -> bool {
        self.src[self.pos..].starts_with(s)
    }

    /// Create a span from `start` to current position.
    #[inline]
    fn span(&self, start: usize) -> Span {
        Span::new(start as u32, self.pos as u32)
    }

    /// Create a zero-length span at current position.
    #[inline]
    fn span_here(&self) -> Span {
        Span::new(self.pos as u32, self.pos as u32)
    }
}

// ── Utility functions ─────────────────────────────────────────────────────────

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Strip common leading whitespace from a multiline string.
fn strip_indent(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();
    if lines.is_empty() { return String::new(); }

    // Find minimum indent (ignoring empty lines)
    let min_indent = lines.iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);

    lines.iter()
        .map(|l| if l.len() >= min_indent { &l[min_indent..] } else { l })
        .collect::<Vec<_>>()
        .join("\n")
}
