//! loop_collect_using_entries_v0: Stage1UsingResolverBox._collect_using_entries loop plan (BoxCount).
//!
//! Accepts a one-shape `loop(pos < n)` loop where:
//! - the loop step is `pos = next_pos` (var-to-var),
//! - `next_pos` is declared as a loop-local (first stmt),
//! - body has a top-level if/else chain (shape pin),
//! - body can be lowered via ExitAllowed RecipeBlock (no exits required).
//!
//! Intended for strict/dev + planner_required only.

mod facts;
mod facts_helpers;
mod facts_recipe_builder;
mod facts_shape_routes;
mod facts_types;
mod pipeline;
mod recipe;

pub(in crate::mir::builder) use facts::try_extract_loop_collect_using_entries_v0_facts;
pub(in crate::mir::builder) use facts_types::LoopCollectUsingEntriesV0Facts;
pub(in crate::mir::builder) use pipeline::lower_loop_collect_using_entries_v0;
