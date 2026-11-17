//! Stable semantic owner modules for route detector support.
//!
//! This module owns non-legacy caller paths. Keep new callers on these semantic
//! paths instead of reintroducing historical implementation names.

/// Break-condition structural analysis support.

/// Loop-body local promotion support.
pub mod body_local;

/// Condition-scope analysis support.
pub mod condition_scope;

/// Local-variable analyzer support.

/// Trim-route support.
pub mod trim;
