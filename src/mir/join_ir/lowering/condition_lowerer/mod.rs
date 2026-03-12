//! Condition Expression Lowerer
//!
//! This module provides the core logic for lowering AST condition expressions
//! to JoinIR instructions. It handles comparisons, logical operators, and
//! arithmetic expressions.
//!
//! ## Design Philosophy
//!
//! **Single Responsibility**: This module ONLY performs AST → JoinIR lowering.
//! It does NOT:
//! - Manage variable environments (that's condition_env.rs)
//! - Extract variables from AST (that's condition_var_extractor.rs)
//! - Manage HOST ↔ JoinIR bindings (that's inline_boundary.rs)

mod api;
mod condition_ops;
#[cfg(test)]
mod tests;
mod value_expr;

pub use api::{lower_condition_to_joinir, lower_condition_to_joinir_no_body_locals};
pub use value_expr::lower_value_expression;
