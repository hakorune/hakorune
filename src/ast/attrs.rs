//! Compatibility re-export for passive AST declaration attributes.
//!
//! The active main-crate AST keeps the historical `crate::ast::{RuneAttr,
//! DeclarationAttrs}` import paths while the data definitions live in
//! `hakorune-frontend-ast`.

pub use hakorune_frontend_ast::{DeclarationAttrs, RuneAttr};
