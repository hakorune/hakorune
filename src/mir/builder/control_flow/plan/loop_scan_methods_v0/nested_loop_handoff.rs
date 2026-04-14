use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::nested_loop_plan_bridge::lower_nested_loop_plan_with_recipe_first_bridge;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::helpers::apply_loop_final_values_to_bindings;
use super::recipe::NestedLoopRecipe;

const LOOP_SCAN_METHODS_STAGE: &str = "loop_scan_methods_v0::nested_loop_fallback";

pub(in crate::mir::builder) fn lower_loop_scan_methods_nested_loop_fallback(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    ctx: &LoopRouteContext,
    error_prefix: &str,
) -> Result<LoweredRecipe, String> {
    lower_nested_loop_plan_with_recipe_first_bridge(
        builder,
        condition,
        body,
        ctx,
        error_prefix,
        LOOP_SCAN_METHODS_STAGE,
        "loop_scan_methods_v0",
    )
}

pub(in crate::mir::builder) fn lower_loop_scan_methods_nested_fallback_segment(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    nested: &NestedLoopRecipe,
    ctx: &LoopRouteContext,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let plan = lower_loop_scan_methods_nested_loop_fallback(
        builder,
        &nested.cond_view.tail_expr,
        &nested.body.body,
        ctx,
        error_prefix,
    )?;
    apply_loop_final_values_to_bindings(builder, current_bindings, &plan);
    Ok(vec![plan])
}
