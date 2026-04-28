//! Phase 29bq P2: loop(true) break/continue coverage (Facts + Recipe)
//!
//! Responsibility:
//! - Facts: loop(true) with multiple if(cond) break/continue, no return
//! - Lowering: feature pipeline consumes the recipe directly
//!
//! Boundaries:
//! - analysis-only (no AST rewrite)
//! - strict/dev + planner_required only (release default unchanged)
//!
pub(in crate::mir::builder) mod recipe;
