use crate::lexer::token::Span;
use super::codes::ErrorCode;

// ── Severity ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
    Help,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Severity::Error   => "error",
            Severity::Warning => "warning",
            Severity::Note    => "note",
            Severity::Help    => "help",
        }
    }

    /// ANSI color code for terminal output.
    pub fn color(self) -> &'static str {
        match self {
            Severity::Error   => "\x1b[1;31m", // bold red
            Severity::Warning => "\x1b[1;33m", // bold yellow
            Severity::Note    => "\x1b[1;36m", // bold cyan
            Severity::Help    => "\x1b[1;32m", // bold green
        }
    }
}

// ── Label ─────────────────────────────────────────────────────────────────────

/// A label attached to a span of source code in a diagnostic.
#[derive(Debug, Clone)]
pub struct Label {
    pub span:    Span,
    pub message: String,
}

impl Label {
    pub fn new(span: Span, msg: impl Into<String>) -> Self {
        Self { span, message: msg.into() }
    }
}

// ── Diagnostic ────────────────────────────────────────────────────────────────

/// A compiler diagnostic — error, warning, note, or help message.
///
/// Per spec §37.13, every diagnostic must include:
/// - Error code
/// - Severity
/// - Primary label (span + message)
/// - Optional secondary labels, notes, hints
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code:      ErrorCode,
    pub severity:  Severity,
    pub message:   String,
    pub primary:   Label,
    pub secondary: Vec<Label>,
    pub notes:     Vec<String>,
    pub hints:     Vec<String>,
    pub see:       Option<String>,
}

impl Diagnostic {
    pub fn error(code: ErrorCode, msg: impl Into<String>, span: Span) -> Self {
        let message = msg.into();
        Self {
            code,
            severity:  Severity::Error,
            message:   message.clone(),
            primary:   Label::new(span, message),
            secondary: Vec::new(),
            notes:     Vec::new(),
            hints:     Vec::new(),
            see:       None,
        }
    }

    pub fn warning(code: ErrorCode, msg: impl Into<String>, span: Span) -> Self {
        let message = msg.into();
        Self {
            code,
            severity:  Severity::Warning,
            message:   message.clone(),
            primary:   Label::new(span, message),
            secondary: Vec::new(),
            notes:     Vec::new(),
            hints:     Vec::new(),
            see:       None,
        }
    }

    pub fn with_label(mut self, span: Span, msg: impl Into<String>) -> Self {
        self.secondary.push(Label::new(span, msg));
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hints.push(hint.into());
        self
    }

    pub fn with_see(mut self, url: impl Into<String>) -> Self {
        self.see = Some(url.into());
        self
    }
}

// ── Reporter ──────────────────────────────────────────────────────────────────

/// The reporter collects diagnostics and renders them to stderr.
pub struct Reporter {
    diagnostics: Vec<Diagnostic>,
    source:      String,
    filename:    String,
}

impl Reporter {
    pub fn new(filename: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            diagnostics: Vec::new(),
            source:      source.into(),
            filename:    filename.into(),
        }
    }

    pub fn report(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == Severity::Error)
    }

    pub fn error_count(&self) -> usize {
        self.diagnostics.iter().filter(|d| d.severity == Severity::Error).count()
    }

    pub fn warning_count(&self) -> usize {
        self.diagnostics.iter().filter(|d| d.severity == Severity::Warning).count()
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Render all diagnostics to stderr in rustc-style format.
    ///
    /// Example output:
    /// ```text
    /// error[E0400]: type mismatch
    ///   --> src/main.gpl:15:12
    ///    |
    /// 15 |     var id: UserId = 42
    ///    |             ------   ^^ expected UserId, found i32
    ///    |
    ///    = hint: wrap with UserId(42) to construct a UserId
    /// ```
    pub fn emit_all(&self) {
        let reset = "\x1b[0m";
        let bold  = "\x1b[1m";

        for diag in &self.diagnostics {
            let color = diag.severity.color();
            let (line, col) = self.offset_to_line_col(diag.primary.span.start as usize);

            // Header: error[E0400]: type mismatch
            eprintln!("{color}{}{reset}[{}]: {bold}{}{reset}",
                diag.severity.as_str(), diag.code, diag.message);

            // Location: --> file:line:col
            eprintln!("  {bold}-->{reset} {}:{}:{}", self.filename, line, col);

            // Source line
            if let Some(src_line) = self.get_line(line) {
                let line_num = format!("{}", line);
                let padding  = " ".repeat(line_num.len());
                eprintln!("{padding} {bold}|{reset}");
                eprintln!("{bold}{line_num} |{reset} {src_line}");

                // Underline
                let col0 = col.saturating_sub(1);
                let len  = (diag.primary.span.end as usize)
                    .saturating_sub(diag.primary.span.start as usize)
                    .max(1);
                let underline = format!("{}{color}{}{reset} {}",
                    " ".repeat(col0),
                    "^".repeat(len),
                    diag.primary.message);
                eprintln!("{padding} {bold}|{reset} {underline}");
            }

            // Secondary labels
            for label in &diag.secondary {
                let (l2, c2) = self.offset_to_line_col(label.span.start as usize);
                if let Some(src_line) = self.get_line(l2) {
                    let line_num = format!("{}", l2);
                    let padding  = " ".repeat(line_num.len());
                    eprintln!("{bold}{line_num} |{reset} {src_line}");
                    let col0 = c2.saturating_sub(1);
                    let len  = (label.span.end as usize)
                        .saturating_sub(label.span.start as usize)
                        .max(1);
                    eprintln!("{padding} {bold}|{reset} {}{color}{}{reset} {}",
                        " ".repeat(col0), "-".repeat(len), label.message);
                }
            }

            // Notes
            for note in &diag.notes {
                eprintln!("   {bold}= note:{reset} {note}");
            }
            // Hints
            for hint in &diag.hints {
                eprintln!("   {bold}= hint:{reset} {hint}");
            }
            // See
            if let Some(url) = &diag.see {
                eprintln!("   {bold}= see:{reset} {url}");
            }
            eprintln!();
        }

        // Summary
        let errors   = self.error_count();
        let warnings = self.warning_count();
        if errors > 0 || warnings > 0 {
            eprint!("{bold}");
            if errors > 0 {
                let color = Severity::Error.color();
                eprint!("{color}{} error(s){reset}", errors);
            }
            if warnings > 0 {
                if errors > 0 { eprint!(", "); }
                let color = Severity::Warning.color();
                eprint!("{color}{} warning(s){reset}", warnings);
            }
            eprintln!("{reset}");
        }
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn offset_to_line_col(&self, offset: usize) -> (usize, usize) {
        let before = &self.source[..offset.min(self.source.len())];
        let line = before.bytes().filter(|&b| b == b'\n').count() + 1;
        let col  = before.rfind('\n').map(|i| offset - i - 1).unwrap_or(offset) + 1;
        (line, col)
    }

    fn get_line(&self, line_num: usize) -> Option<&str> {
        self.source.lines().nth(line_num.saturating_sub(1))
    }
}
