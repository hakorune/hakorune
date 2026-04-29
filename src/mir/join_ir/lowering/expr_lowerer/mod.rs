//! Phase 231: Expression Lowering with Unified Scope Management
//!
//! This module provides a pilot implementation of expression lowering that uses
//! ScopeManager for variable resolution. It's a thin wrapper around existing
//! condition_lowerer logic, focusing on API unification rather than reimplementation.
//!
//! ## Design Philosophy
//!
//! **Box-First**: ExprLowerer is a "box" that encapsulates expression lowering
//! logic with clean boundaries: takes AST + ScopeManager, returns ValueId + instructions.
//!
//! **Incremental Adoption**: Phase 231 exposes Condition context only.
//! Future phases should add a new context variant together with a live lowering
//! contract and tests.
//!
//! **Fail-Safe**: Unsupported AST nodes return explicit errors, allowing callers
//! to fall back to legacy paths.

mod ast_support;
mod lowerer;
mod scope_resolution;
#[cfg(test)]
mod test_helpers;
#[cfg(test)]
mod tests;
mod types;

pub use lowerer::ExprLowerer;
pub use types::{ExprContext, ExprLoweringError};
