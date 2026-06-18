//! Compatibility re-export for reserved `@rune Profile(...)` expansion targets.
//!
//! The vocabulary is frontend/MIR-neutral and now lives with passive frontend
//! AST data. Main-crate users keep the historical `crate::rune_profile_registry`
//! path through this facade.

pub use hakorune_frontend_ast::rune_profile::*;
