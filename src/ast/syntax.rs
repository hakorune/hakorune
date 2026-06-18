//! Compatibility re-export for passive AST syntax vocabulary.
//!
//! The historical `crate::ast::syntax::*` path remains available while passive
//! syntax data lives in `hakorune-frontend-ast`.

pub use hakorune_frontend_ast::{BinaryOperator, BuildPredicate, LiteralValue, UnaryOperator};
