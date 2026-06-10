/// Compilation session — holds configuration, source file map, and
/// shared state for all compiler passes.
///
/// Spec §2 / COMPILER.md §2

use std::collections::HashMap;
use std::path::{Path, PathBuf};

// ── Source file map ───────────────────────────────────────────────────────────

/// A unique ID for a source file within this compilation session.
pub type FileId = u32;

/// Metadata about a loaded source file.
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub id:       FileId,
    pub path:     PathBuf,
    pub name:     String,      // display name (relative path or module path)
    pub source:   String,      // full source text
    pub line_starts: Vec<u32>, // byte offset of each line start
}

impl SourceFile {
    /// Compute (line, col) from a byte offset (both 1-indexed).
    pub fn offset_to_line_col(&self, offset: u32) -> (usize, usize) {
        let offset = offset as usize;
        // binary search for the line
        let line_idx = match self.line_starts.binary_search(&(offset as u32)) {
            Ok(idx)  => idx,
            Err(idx) => idx.saturating_sub(1),
        };
        let line_start = self.line_starts[line_idx] as usize;
        let col = offset - line_start + 1;
        (line_idx + 1, col)
    }

    /// Retrieve the text of a specific line (1-indexed).
    pub fn get_line(&self, line_num: usize) -> Option<&str> {
        if line_num == 0 || line_num > self.line_starts.len() {
            return None;
        }
        let start = self.line_starts[line_num - 1] as usize;
        let end = if line_num < self.line_starts.len() {
            self.line_starts[line_num] as usize
        } else {
            self.source.len()
        };
        // Trim trailing newline
        let line = &self.source[start..end];
        Some(line.trim_end_matches('\n').trim_end_matches('\r'))
    }
}

// ── Optimization / build level ────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptLevel {
    Debug,      // -O0: no optimizations, full debug info
    Release,    // -O2: standard optimizations
    Size,       // -Os: optimize for size
    Aggressive, // -O3: aggressive optimizations
}

impl Default for OptLevel {
    fn default() -> Self {
        OptLevel::Debug
    }
}

// ── Target triple ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Target {
    pub arch:    String,    // x86_64, aarch64, riscv64
    pub os:      String,    // linux, darwin, windows, none
    pub env:     String,    // gnu, musl, none
    pub pointer_size: u8,   // 8 for 64-bit
}

impl Default for Target {
    fn default() -> Self {
        Self {
            arch:    "x86_64".to_string(),
            os:      "linux".to_string(),
            env:     "gnu".to_string(),
            pointer_size: 8,
        }
    }
}

// ── Session ───────────────────────────────────────────────────────────────────

/// The compilation session — top-level state for the entire compilation.
///
/// Holds all source files, configuration, and shared data needed by each
/// compiler pass.
#[derive(Debug)]
pub struct Session {
    /// All loaded source files, indexed by FileId.
    files: Vec<SourceFile>,
    /// Map from file path → FileId for deduplication.
    path_to_id: HashMap<PathBuf, FileId>,

    // ── Configuration ─────────────────────────────────────────────────────
    /// Optimization level.
    pub opt_level: OptLevel,
    /// Target platform.
    pub target: Target,
    /// Output path for the final binary.
    pub output: Option<PathBuf>,
    /// Whether to emit colored diagnostics.
    pub color: bool,
    /// Whether to treat warnings as errors.
    pub warnings_as_errors: bool,
    /// No-std mode (kernel / freestanding).
    pub no_std: bool,
    /// Include paths for imports.
    pub include_paths: Vec<PathBuf>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            files:       Vec::new(),
            path_to_id:  HashMap::new(),
            opt_level:   OptLevel::default(),
            target:      Target::default(),
            output:      None,
            color:       true,
            warnings_as_errors: false,
            no_std:      false,
            include_paths: Vec::new(),
        }
    }

    /// Load a source file and return its FileId.
    /// If the file was already loaded, returns the existing FileId.
    pub fn load_file(&mut self, path: &Path) -> Result<FileId, String> {
        let canon = path.canonicalize().map_err(|e| {
            format!("cannot resolve path `{}`: {}", path.display(), e)
        })?;

        if let Some(&id) = self.path_to_id.get(&canon) {
            return Ok(id);
        }

        let source = std::fs::read_to_string(&canon).map_err(|e| {
            format!("cannot read `{}`: {}", canon.display(), e)
        })?;

        let id = self.files.len() as FileId;
        let name = path.to_string_lossy().into_owned();

        // Compute line starts
        let mut line_starts = vec![0u32];
        for (i, b) in source.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push((i + 1) as u32);
            }
        }

        self.files.push(SourceFile {
            id,
            path: canon.clone(),
            name,
            source,
            line_starts,
        });
        self.path_to_id.insert(canon, id);
        Ok(id)
    }

    /// Load source from a string (useful for tests and REPL).
    pub fn load_string(&mut self, name: impl Into<String>, source: impl Into<String>) -> FileId {
        let id = self.files.len() as FileId;
        let source = source.into();
        let name = name.into();

        let mut line_starts = vec![0u32];
        for (i, b) in source.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push((i + 1) as u32);
            }
        }

        self.files.push(SourceFile {
            id,
            path: PathBuf::from(&name),
            name,
            source,
            line_starts,
        });
        id
    }

    /// Get a source file by its ID.
    pub fn get_file(&self, id: FileId) -> &SourceFile {
        &self.files[id as usize]
    }

    /// Get the source text for a file.
    pub fn source(&self, id: FileId) -> &str {
        &self.files[id as usize].source
    }

    /// Get (line, col) for a byte offset in a file.
    pub fn offset_to_line_col(&self, file_id: FileId, offset: u32) -> (usize, usize) {
        self.files[file_id as usize].offset_to_line_col(offset)
    }

    /// Get the display name of a file.
    pub fn file_name(&self, id: FileId) -> &str {
        &self.files[id as usize].name
    }

    /// Number of loaded files.
    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_string() {
        let mut sess = Session::new();
        let id = sess.load_string("test.gpl", "fn main() -> i32:\n    return 0\n");
        assert_eq!(id, 0);
        assert_eq!(sess.file_count(), 1);
        assert_eq!(sess.file_name(id), "test.gpl");
    }

    #[test]
    fn test_line_col() {
        let mut sess = Session::new();
        let id = sess.load_string("test.gpl", "line1\nline2\nline3\n");
        assert_eq!(sess.offset_to_line_col(id, 0), (1, 1));
        assert_eq!(sess.offset_to_line_col(id, 6), (2, 1));
        assert_eq!(sess.offset_to_line_col(id, 8), (2, 3));
    }

    #[test]
    fn test_get_line() {
        let mut sess = Session::new();
        let id = sess.load_string("test.gpl", "hello\nworld\n");
        let file = sess.get_file(id);
        assert_eq!(file.get_line(1), Some("hello"));
        assert_eq!(file.get_line(2), Some("world"));
        assert_eq!(file.get_line(3), None);
    }
}
