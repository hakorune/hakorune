//! Recipe definition for loop_scan_methods_v0 (recipes-owned surface).
//!
//! Purpose:
//! - Own the LoopScanMethodsV0Recipe shape at the top-level recipes layer.
//! - Keep plan callers depending on this module instead of a plan-local recipe facade.

use crate::mir::builder::control_flow::facts::no_exit_block::NoExitBlockRecipe;

pub(in crate::mir::builder) type NestedLoopRecipe =
    crate::mir::builder::control_flow::recipes::scan_loop_segments::NestedLoopRecipe;
pub(in crate::mir::builder) type LoopScanSegment =
    crate::mir::builder::control_flow::recipes::scan_loop_segments::LoopScanSegment<
        NoExitBlockRecipe,
    >;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopScanMethodsV0Recipe {
    pub segments: Vec<LoopScanSegment>,
}
