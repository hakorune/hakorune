use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::nested_loop_plan;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::recipe::NestedLoopRecipe;

fn apply_loop_final_values_to_bindings(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    plan: &LoweredRecipe,
) {
    let CorePlan::Loop(loop_plan) = plan else {
        return;
    };
    for (name, value_id) in &loop_plan.final_values {
        builder
            .variable_ctx
            .variable_map
            .insert(name.clone(), *value_id);
        if current_bindings.contains_key(name) {
            current_bindings.insert(name.clone(), *value_id);
        }
    }
}

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
