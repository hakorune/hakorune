//! loop_collect_using_entries_v0: Stage1UsingResolverBox._collect_using_entries loop plan (BoxCount).
//!
//! Accepts a one-shape `loop(pos < n)` loop where:
//! - the loop step is `pos = next_pos` (var-to-var),
//! - `next_pos` is declared as a loop-local (first stmt),
//! - body has a top-level if/else chain (shape pin),
//! - body can be lowered via ExitAllowed RecipeBlock (no exits required).
//!
//! Intended for strict/dev + planner_required only.

mod pipeline;

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::facts::loop_collect_using_entries_v0::{
    try_extract_loop_collect_using_entries_v0_facts, LoopCollectUsingEntriesV0Facts,
};
pub(in crate::mir::builder) use pipeline::lower_loop_collect_using_entries_v0;
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::recipes::loop_collect_using_entries_v0::LoopCollectUsingEntriesV0Recipe;
