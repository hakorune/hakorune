//! Compatibility re-export for passive AST span data.
//!
//! The active main-crate AST keeps the historical `crate::ast::Span` import
//! path, while the data definition lives in `hakorune-frontend-ast`.

pub use hakorune_frontend_ast::Span;
