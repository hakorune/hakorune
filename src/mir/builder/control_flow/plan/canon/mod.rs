//! Phase 29bq: Canon (analysis-only view)
//!
//! Responsibility:
//! - Build conservative, analysis-only views from Facts inputs
//! - No AST rewrite, no behavior changes
//! - Return None when out of scope

pub(in crate::mir::builder) mod generic_loop;
