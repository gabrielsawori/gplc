pub mod ty;
pub mod checker;
pub mod unify;

pub use ty::{Ty, TyId, TypeArena};
pub use checker::TypeChecker;
