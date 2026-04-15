use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::loop_bundle_resolver_v0::LoopBundleResolverV0Facts;
use crate::mir::builder::control_flow::facts::loop_bundle_resolver_v0_shape_routes::LoopBundleResolverV0ShapePins;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::recipes::loop_bundle_resolver_v0::LoopBundleResolverV0Recipe;

pub(in crate::mir::builder) fn try_build_loop_bundle_resolver_v0_facts(
    condition: &ASTNode,
    body: &[ASTNode],
    shape_pins: LoopBundleResolverV0ShapePins,
    debug_reject: &dyn Fn(&str),
) -> Option<LoopBundleResolverV0Facts> {
    let Some(body_exit_allowed) = try_build_exit_allowed_block_recipe(body, true) else {
        debug_reject("exit_allowed_recipe_failed");
        return None;
    };

    Some(LoopBundleResolverV0Facts {
        loop_var: shape_pins.loop_var,
        limit_var: shape_pins.limit_var,
        condition: condition.clone(),
        recipe: LoopBundleResolverV0Recipe {
            step_var: shape_pins.step_var,
            body_exit_allowed,
        },
    })
}
