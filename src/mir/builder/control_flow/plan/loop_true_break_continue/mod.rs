//! Phase 29bq P2: loop(true) break/continue coverage (Facts + Normalizer)
//!
//! Responsibility:
//! - Facts: loop(true) with multiple if(cond) break/continue, no return
//! - Normalizer: CorePlan::Loop + CorePlan::If + Exit (Break/Continue)
//!
//! Boundaries:
//! - analysis-only (no AST rewrite)
//! - strict/dev + planner_required only (release default unchanged)
//!
//! Facts are re-exported from `loop_cond::true_break_continue`.

pub(in crate::mir::builder) mod facts;
pub(in crate::mir::builder) mod normalizer;
pub(in crate::mir::builder) mod recipe;
