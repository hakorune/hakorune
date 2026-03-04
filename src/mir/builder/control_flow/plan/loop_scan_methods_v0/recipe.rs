use crate::mir::builder::control_flow::plan::facts::no_exit_block::NoExitBlockRecipe;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;

pub(in crate::mir::builder) type NestedLoopRecipe =
    crate::mir::builder::control_flow::plan::scan_loop_segments::NestedLoopRecipe;
pub(in crate::mir::builder) type LoopScanSegment =
    crate::mir::builder::control_flow::plan::scan_loop_segments::LoopScanSegment<NoExitBlockRecipe>;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopScanMethodsV0Recipe {
    pub next_i_var: String,
    pub body: RecipeBody,
    pub segments: Vec<LoopScanSegment>,
}
