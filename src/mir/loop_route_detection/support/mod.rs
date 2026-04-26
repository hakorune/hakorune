//! Stable semantic owner modules for route detector support.
//!
//! This module owns non-legacy caller paths. Keep new callers on these semantic
//! paths instead of reintroducing historical implementation names.

/// Break-condition structural analysis support.
pub mod break_condition;

/// Loop-body local promotion support.
pub mod body_local;

/// Condition-scope analysis support.
pub mod condition_scope;

/// Function-scope capture analysis support.
pub mod function_scope;

/// Local-variable analyzer support.
pub mod locals;

/// Trim-route support.
pub mod trim;
