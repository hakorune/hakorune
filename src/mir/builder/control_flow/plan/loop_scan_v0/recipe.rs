use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopScanV0Recipe {
    pub local_ch_stmt: ASTNode,
    pub comma_if_cond: ASTNode,
    pub comma_inc_stmt: ASTNode,
    pub close_if_cond: ASTNode,
    pub step_inc_stmt: ASTNode,
}

pub(in crate::mir::builder) type NestedLoopRecipe =
    crate::mir::builder::control_flow::recipes::scan_loop_segments::NestedLoopRecipe;

pub(in crate::mir::builder) type LoopScanSegment =
    crate::mir::builder::control_flow::recipes::scan_loop_segments::LoopScanSegment<
        ExitAllowedBlockRecipe,
    >;
