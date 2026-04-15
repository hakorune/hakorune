use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

use super::LoopBundleResolverV0Facts;

const LOOP_BUNDLE_RESOLVER_ERR: &str = "[loop_bundle_resolver_v0]";

pub(in crate::mir::builder) fn lower_loop_bundle_resolver_v0(
    builder: &mut MirBuilder,
    facts: LoopBundleResolverV0Facts,
    _ctx: &LoopRouteContext,
) -> Result<LoweredRecipe, String> {
    if !builder
        .variable_ctx
        .variable_map
        .contains_key(&facts.limit_var)
    {
        return Err(format!(
            "[freeze:contract][loop_bundle_resolver_v0] limit var {} missing init: ctx={}",
            facts.limit_var, LOOP_BUNDLE_RESOLVER_ERR
        ));
    }
    if !builder
        .variable_ctx
        .variable_map
        .contains_key(&facts.loop_var)
    {
        return Err(format!(
            "[freeze:contract][loop_bundle_resolver_v0] loop var {} missing init: ctx={}",
            facts.loop_var, LOOP_BUNDLE_RESOLVER_ERR
        ));
    }

    // Delegate loop skeleton + carriers + verified body lowering to LoopV0 Parts.
    // Facts already pinned this family to the exit-allowed recipe shape.
    let cond_view = CondBlockView::from_expr(&facts.condition);
    let mut current_bindings = builder.variable_ctx.variable_map.clone();
    parts::entry::lower_loop_v0(
        builder,
        &mut current_bindings,
        &cond_view,
        BlockContractKind::ExitAllowed,
        &facts.recipe.body_exit_allowed.arena,
        &facts.recipe.body_exit_allowed.block,
        LOOP_BUNDLE_RESOLVER_ERR,
    )
}
