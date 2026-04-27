//! Recipe definition for loop_scan_methods_block_v0 (recipes-owned surface).
//!
//! This keeps the scan-segment vocabulary under the shared recipes owner while
//! lowering logic stays in the owner-local family.

use crate::mir::builder::control_flow::facts::no_exit_block::NoExitBlockRecipe;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe;

pub(in crate::mir::builder) type NestedLoopRecipe =
    crate::mir::builder::control_flow::recipes::scan_loop_segments::NestedLoopRecipe;
pub(in crate::mir::builder) type ScanSegment =
    crate::mir::builder::control_flow::recipes::scan_loop_segments::LoopScanSegment<
        LinearBlockRecipe,
    >;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum LinearBlockRecipe {
    NoExit(NoExitBlockRecipe),
    ExitAllowed(ExitAllowedBlockRecipe),
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopScanMethodsBlockV0Recipe {
    pub segments: Vec<ScanSegment>,
}
