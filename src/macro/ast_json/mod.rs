//! AST JSON helpers.
//!
//! This module intentionally separates two concerns:
//! - `joinir_compat`: lossy export for legacy JoinIR frontend JSON shape.
//! - `roundtrip`: decode/encode helpers used by macro/diagnostic pipelines.
//!
//! SSOT:
//! - JoinIR frontend expects the `joinir_compat` export shape.
//! - Macro child / diagnostics should prefer `roundtrip` (schema-tagged).

mod joinir_compat;
mod roundtrip;
mod shared;

pub use joinir_compat::ast_to_json;
pub use roundtrip::ast_to_json_roundtrip;
pub use roundtrip::json_to_ast;

