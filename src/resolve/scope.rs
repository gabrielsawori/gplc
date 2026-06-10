/// Scope stack and symbol table for name resolution.
///
/// COMPILER.md §6.1

use std::collections::HashMap;
use crate::lexer::token::Span;

// ── IDs ───────────────────────────────────────────────────────────────────────

/// Index into SymbolTable.symbols
pub type SymbolId = u32;
/// Index into SymbolTable.scopes
pub type ScopeId = u32;
/// Module identity (for now just a number)
pub type ModuleId = u32;

// ── Symbol kinds ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Var,
    Let,       // immutable binding
    Fn,
    Struct,
    Enum,
    Union,
    Interface,
    TypeAlias,
    Newtype,
    Const,
    Module,
    Param,
    EnumVariant,
}

// ── Scope kinds ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    Module,
    Fn,
    Block,
    Loop,
    If,
    Match,
    Struct,
    Enum,
    Impl,
}

// ── Symbol ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Symbol {
    pub id:     SymbolId,
    pub name:   String,
    pub kind:   SymbolKind,
    pub span:   Span,
    pub is_pub: bool,
    pub module: ModuleId,
    /// Marked true once the symbol is referenced.
    pub used:   bool,
}

// ── Scope ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Scope {
    pub kind:    ScopeKind,
    pub symbols: HashMap<String, SymbolId>,
}

impl Scope {
    pub fn new(kind: ScopeKind) -> Self {
        Self { kind, symbols: HashMap::new() }
    }
}

// ── Symbol Table ──────────────────────────────────────────────────────────────

/// The symbol table holds all symbols and a stack of scopes.
#[derive(Debug)]
pub struct SymbolTable {
    pub symbols: Vec<Symbol>,
    scopes:      Vec<Scope>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
            scopes:  vec![Scope::new(ScopeKind::Module)],
        }
    }

    /// Push a new scope onto the scope stack.
    pub fn push_scope(&mut self, kind: ScopeKind) {
        self.scopes.push(Scope::new(kind));
    }

    /// Pop the topmost scope.
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Define a new symbol in the current scope. Returns the SymbolId,
    /// or None if a symbol with the same name already exists in this scope.
    pub fn define(
        &mut self,
        name: &str,
        kind: SymbolKind,
        span: Span,
        is_pub: bool,
        module: ModuleId,
    ) -> Result<SymbolId, SymbolId> {
        let scope = self.scopes.last_mut().expect("no scope");

        // Check for duplicate in current scope
        if let Some(&existing) = scope.symbols.get(name) {
            return Err(existing);
        }

        let id = self.symbols.len() as SymbolId;
        self.symbols.push(Symbol {
            id,
            name: name.to_string(),
            kind,
            span,
            is_pub,
            module,
            used: false,
        });
        scope.symbols.insert(name.to_string(), id);
        Ok(id)
    }

    /// Look up a name by searching from the innermost scope outward.
    pub fn lookup(&self, name: &str) -> Option<SymbolId> {
        for scope in self.scopes.iter().rev() {
            if let Some(&id) = scope.symbols.get(name) {
                return Some(id);
            }
        }
        None
    }

    /// Look up only in the current (innermost) scope.
    pub fn lookup_current(&self, name: &str) -> Option<SymbolId> {
        self.scopes.last()
            .and_then(|s| s.symbols.get(name).copied())
    }

    /// Get a symbol by ID.
    pub fn get(&self, id: SymbolId) -> &Symbol {
        &self.symbols[id as usize]
    }

    /// Get a mutable symbol by ID.
    pub fn get_mut(&mut self, id: SymbolId) -> &mut Symbol {
        &mut self.symbols[id as usize]
    }

    /// Mark a symbol as used.
    pub fn mark_used(&mut self, id: SymbolId) {
        self.symbols[id as usize].used = true;
    }

    /// Current scope depth (0 = module scope).
    pub fn depth(&self) -> usize {
        self.scopes.len().saturating_sub(1)
    }

    /// Collect unused symbols (for W0001/W0002/W0003 warnings).
    pub fn unused_symbols(&self) -> Vec<&Symbol> {
        self.symbols.iter()
            .filter(|s| !s.used && !s.name.starts_with('_'))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_and_lookup() {
        let mut table = SymbolTable::new();
        let span = Span::new(0, 0);
        let id = table.define("x", SymbolKind::Var, span, false, 0).unwrap();
        assert_eq!(table.lookup("x"), Some(id));
        assert_eq!(table.lookup("y"), None);
    }

    #[test]
    fn test_nested_scopes() {
        let mut table = SymbolTable::new();
        let span = Span::new(0, 0);
        let outer = table.define("x", SymbolKind::Var, span, false, 0).unwrap();
        table.push_scope(ScopeKind::Block);
        let inner = table.define("y", SymbolKind::Var, span, false, 0).unwrap();
        assert_eq!(table.lookup("x"), Some(outer)); // finds outer
        assert_eq!(table.lookup("y"), Some(inner));
        table.pop_scope();
        assert_eq!(table.lookup("y"), None); // gone
        assert_eq!(table.lookup("x"), Some(outer));
    }

    #[test]
    fn test_duplicate_in_scope() {
        let mut table = SymbolTable::new();
        let span = Span::new(0, 0);
        table.define("x", SymbolKind::Var, span, false, 0).unwrap();
        assert!(table.define("x", SymbolKind::Var, span, false, 0).is_err());
    }

    #[test]
    fn test_shadowing_across_scopes() {
        let mut table = SymbolTable::new();
        let span = Span::new(0, 0);
        let outer = table.define("x", SymbolKind::Var, span, false, 0).unwrap();
        table.push_scope(ScopeKind::Block);
        let inner = table.define("x", SymbolKind::Var, span, false, 0).unwrap();
        assert_ne!(outer, inner);
        assert_eq!(table.lookup("x"), Some(inner)); // inner shadows
        table.pop_scope();
        assert_eq!(table.lookup("x"), Some(outer)); // back to outer
    }
}
