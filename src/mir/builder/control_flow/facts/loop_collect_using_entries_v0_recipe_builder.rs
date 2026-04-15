use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::recipes::loop_collect_using_entries_v0::LoopCollectUsingEntriesV0Recipe;

pub(in crate::mir::builder) fn try_build_loop_collect_using_entries_v0_recipe(
    body: &[ASTNode],
) -> Option<LoopCollectUsingEntriesV0Recipe> {
    let body_no_exit = try_build_no_exit_block_recipe(body, true)?;
    Some(LoopCollectUsingEntriesV0Recipe { body_no_exit })
}
