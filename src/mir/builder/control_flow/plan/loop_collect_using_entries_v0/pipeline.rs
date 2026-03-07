use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::MirBuilder;

use super::facts::LoopCollectUsingEntriesV0Facts;

const LOOP_COLLECT_USING_ENTRIES_ERR: &str = "[loop_collect_using_entries_v0]";

pub(in crate::mir::builder) fn lower_loop_collect_using_entries_v0(
    builder: &mut MirBuilder,
    facts: LoopCollectUsingEntriesV0Facts,
    _ctx: &LoopRouteContext,
) -> Result<LoweredRecipe, String> {
    if !builder
        .variable_ctx
        .variable_map
        .contains_key(&facts.limit_var)
    {
        return Err(format!(
            "[freeze:contract][loop_collect_using_entries_v0] limit var {} missing init: ctx={}",
            facts.limit_var, LOOP_COLLECT_USING_ENTRIES_ERR
        ));
    }
    if !builder
        .variable_ctx
        .variable_map
        .contains_key(&facts.loop_var)
    {
        return Err(format!(
            "[freeze:contract][loop_collect_using_entries_v0] loop var {} missing init: ctx={}",
            facts.loop_var, LOOP_COLLECT_USING_ENTRIES_ERR
        ));
    }

    let cond_view = CondBlockView::from_expr(&facts.condition);
    let mut current_bindings = builder.variable_ctx.variable_map.clone();
    parts::entry::lower_loop_v0(
        builder,
        &mut current_bindings,
        &cond_view,
        BlockContractKind::NoExit,
        &facts.recipe.body_no_exit.arena,
        &facts.recipe.body_no_exit.block,
        LOOP_COLLECT_USING_ENTRIES_ERR,
    )
}
