use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::nested_loop_plan;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn lower_loop_scan_methods_nested_loop_fallback(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    ctx: &LoopRouteContext,
    error_prefix: &str,
) -> Result<LoweredRecipe, String> {
    nested_loop_plan::lower_nested_loop_plan_with_recipe_first(
        builder,
        condition,
        body,
        ctx,
        error_prefix,
        "loop_scan_methods_v0",
    )
}
