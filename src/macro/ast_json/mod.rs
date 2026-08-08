//! AST JSON helpers.
//!
//! This module intentionally separates two concerns:
//! - `joinir_compat`: lossy export for legacy JoinIR frontend JSON shape.
//! - `roundtrip`: decode/encode helpers used by macro/diagnostic pipelines.
//!
//! SSOT:
//! - JoinIR frontend expects the `joinir_compat` export shape.
//! - Macro child / diagnostics should prefer `roundtrip` (schema-tagged).

mod box_inventory_v2;
mod joinir_compat;
mod roundtrip;
mod roundtrip_decoder;
mod shared;

pub use joinir_compat::ast_to_json;
pub use roundtrip::ast_to_json_roundtrip;
pub use roundtrip::json_to_ast;
