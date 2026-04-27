//! loop_collect_using_entries_v0: Stage1UsingResolverBox._collect_using_entries loop plan (BoxCount).
//!
//! Accepts a one-shape `loop(pos < n)` loop where:
//! - the loop step is `pos = next_pos` (var-to-var),
//! - `next_pos` is declared as a loop-local (first stmt),
//! - body has a top-level if/else chain (shape pin),
//! - body can be lowered via ExitAllowed RecipeBlock (no exits required).
//!
//! Intended for strict/dev + planner_required only.

pub(in crate::mir::builder) mod pipeline;
