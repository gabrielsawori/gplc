use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub is_mut: bool,
}

pub struct Scope {
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, is_mut: bool) -> Result<(), String> {
        if self.symbols.contains_key(&name) {
            // E0306: Duplicate binding
            return Err(format!("ReferenceError: Symbol '{}' is already defined in this scope.", name));
        }
        self.symbols.insert(name.clone(), Symbol { name, is_mut });
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<Symbol> {
        self.symbols.get(name).cloned()
    }
}

pub struct ScopeStack {
    scopes: Vec<Scope>,
}

impl ScopeStack {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new()], // Global scope at index 0
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn define(&mut self, name: String, is_mut: bool) -> Result<(), String> {
        let current_scope = self.scopes.last_mut().expect("Compiler Bug: Scope stack should never be empty");
        current_scope.define(name, is_mut)
    }

    pub fn lookup(&self, name: &str) -> Option<Symbol> {
        // Search from the innermost scope outwards
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.lookup(name) {
                return Some(symbol);
            }
        }
        None
    }
}