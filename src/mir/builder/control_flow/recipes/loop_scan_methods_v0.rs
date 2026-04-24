//! Recipe definition for loop_scan_methods_v0 (recipes-owned surface).
//!
//! Purpose:
//! - Move LoopScanMethodsV0Recipe from plan/loop_scan_methods_v0/recipe.rs to the top-level recipes owner.
//! - Non-plan callers should depend on this module first.
//! - plan/loop_scan_methods_v0/recipe.rs keeps a compat wrapper/re-export for local callers.

use crate::mir::builder::control_flow::facts::no_exit_block::NoExitBlockRecipe;
use crate::mir::builder::control_flow::recipes::RecipeBody;

pub(in crate::mir::builder) type NestedLoopRecipe =
    crate::mir::builder::control_flow::recipes::scan_loop_segments::NestedLoopRecipe;
pub(in crate::mir::builder) type LoopScanSegment =
    crate::mir::builder::control_flow::recipes::scan_loop_segments::LoopScanSegment<
        NoExitBlockRecipe,
    >;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopScanMethodsV0Recipe {
    #[allow(dead_code)] // Phase 291x-126: scan recipe evidence retained for handoff/debug.
    pub next_i_var: String,
    #[allow(dead_code)]
    // Phase 291x-126: original body evidence retained while segments own lowering.
    pub body: RecipeBody,
    pub segments: Vec<LoopScanSegment>,
}
