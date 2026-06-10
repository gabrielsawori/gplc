/// Type unification algorithm.
///
/// COMPILER.md §7.2
///
/// Implements union-find based unification for type inference variables.

use super::ty::{Ty, TyId, TypeArena, InferVarId};
use std::collections::HashMap;

/// Union-Find based unifier for type inference.
pub struct Unifier {
    /// Maps inference variable ID → resolved type (or another infer var).
    bindings: HashMap<InferVarId, TyId>,
}

impl Unifier {
    pub fn new() -> Self {
        Self { bindings: HashMap::new() }
    }

    /// Unify two types. Returns Ok(()) if they can be unified, or
    /// Err((a_resolved, b_resolved)) with the conflicting concrete types.
    pub fn unify(&mut self, a: TyId, b: TyId, arena: &TypeArena) -> Result<(), (TyId, TyId)> {
        let a = self.resolve(a, arena);
        let b = self.resolve(b, arena);

        if a == b {
            return Ok(());
        }

        let ty_a = arena.get(a);
        let ty_b = arena.get(b);

        match (ty_a, ty_b) {
            // Error type unifies with anything (suppress cascades)
            (Ty::Error, _) | (_, Ty::Error) => Ok(()),

            // Infer variables: bind to the other type
            (Ty::Infer(var_id), _) => {
                self.bindings.insert(*var_id, b);
                Ok(())
            }
            (_, Ty::Infer(var_id)) => {
                self.bindings.insert(*var_id, a);
                Ok(())
            }

            // Never unifies with any type (it's the bottom type)
            (Ty::Never, _) | (_, Ty::Never) => Ok(()),

            // Same primitive types
            _ if ty_a == ty_b => Ok(()),

            // Structural comparison for compound types
            (Ty::Optional(inner_a), Ty::Optional(inner_b)) => {
                self.unify(*inner_a, *inner_b, arena)
            }
            (Ty::Slice(inner_a), Ty::Slice(inner_b)) => {
                self.unify(*inner_a, *inner_b, arena)
            }
            (Ty::Pointer { mutable: m_a, inner: i_a },
             Ty::Pointer { mutable: m_b, inner: i_b }) if m_a == m_b => {
                self.unify(*i_a, *i_b, arena)
            }
            (Ty::Array { size: s_a, elem: e_a },
             Ty::Array { size: s_b, elem: e_b }) if s_a == s_b => {
                self.unify(*e_a, *e_b, arena)
            }
            (Ty::Tuple(a_elems), Ty::Tuple(b_elems)) if a_elems.len() == b_elems.len() => {
                for (&ea, &eb) in a_elems.iter().zip(b_elems.iter()) {
                    self.unify(ea, eb, arena)?;
                }
                Ok(())
            }
            (Ty::FnPtr { params: pa, ret: ra, .. },
             Ty::FnPtr { params: pb, ret: rb, .. }) if pa.len() == pb.len() => {
                for (&a, &b) in pa.iter().zip(pb.iter()) {
                    self.unify(a, b, arena)?;
                }
                self.unify(*ra, *rb, arena)
            }

            // Mismatch
            _ => Err((a, b)),
        }
    }

    /// Resolve an inference variable to its bound type (following the chain).
    pub fn resolve(&self, ty_id: TyId, arena: &TypeArena) -> TyId {
        let ty = arena.get(ty_id);
        if let Ty::Infer(var_id) = ty {
            if let Some(&bound) = self.bindings.get(var_id) {
                return self.resolve(bound, arena);
            }
        }
        ty_id
    }

    /// Fully resolve a type by substituting all bound inference variables.
    /// Returns the final concrete type (or leaves unresolved vars as-is).
    pub fn apply(&self, ty_id: TyId, arena: &TypeArena) -> TyId {
        self.resolve(ty_id, arena)
    }
}

impl Default for Unifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unify_same_type() {
        let mut arena = TypeArena::new();
        let i32_a = arena.intern(Ty::I32);
        let i32_b = arena.intern(Ty::I32);
        let mut u = Unifier::new();
        assert!(u.unify(i32_a, i32_b, &arena).is_ok());
    }

    #[test]
    fn test_unify_infer_with_concrete() {
        let mut arena = TypeArena::new();
        let infer = arena.fresh_infer();
        let i32_id = arena.intern(Ty::I32);
        let mut u = Unifier::new();
        assert!(u.unify(infer, i32_id, &arena).is_ok());
        assert_eq!(u.resolve(infer, &arena), i32_id);
    }

    #[test]
    fn test_unify_mismatch() {
        let mut arena = TypeArena::new();
        let i32_id = arena.intern(Ty::I32);
        let f64_id = arena.intern(Ty::F64);
        let mut u = Unifier::new();
        assert!(u.unify(i32_id, f64_id, &arena).is_err());
    }

    #[test]
    fn test_unify_optional() {
        let mut arena = TypeArena::new();
        let i32_id = arena.intern(Ty::I32);
        let opt_a = arena.intern(Ty::Optional(i32_id));
        let opt_b = arena.intern(Ty::Optional(i32_id));
        let mut u = Unifier::new();
        assert!(u.unify(opt_a, opt_b, &arena).is_ok());
    }
}
