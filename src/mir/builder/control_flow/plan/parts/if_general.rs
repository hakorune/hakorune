//! General if lowering helpers (Parts).
//!
//! Scope: behavior-preserving extraction of existing lowering logic.
//! SSOT for try_lower_general_if.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    count_control_flow, ControlFlowDetector,
};
use super::join_scope::{collect_branch_local_vars_from_body, filter_branch_locals_from_maps};
use super::super::steps::build_join_payload;
use crate::mir::builder::control_flow::plan::normalizer::lower_cond_branch;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

/// View-first general if lowering (SSOT).
pub(in crate::mir::builder) fn try_lower_general_if_view<F>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    cond_view: &CondBlockView,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
    mut lower_block: F,
) -> Result<Option<Vec<LoweredRecipe>>, String>
where
    F: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &[ASTNode],
    ) -> Result<Vec<LoweredRecipe>, String>,
{
    let strict_or_dev =
        crate::config::env::joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
    let planner_required =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    if !planner_required {
        return Ok(None);
    }
    if then_body.is_empty() {
        return Ok(None);
    }
    if body_has_exit(then_body) || else_body.map_or(false, |body| body_has_exit(body)) {
        return Ok(None);
    }

    let mut pre_if_map = builder.variable_ctx.variable_map.clone();
    for (name, value_id) in current_bindings.iter() {
        pre_if_map.insert(name.clone(), *value_id);
    }
    let pre_bindings = current_bindings.clone();

    let then_plans = lower_block(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        then_body,
    )?;
    let then_map = builder.variable_ctx.variable_map.clone();

    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();
    let else_plans = match else_body {
        Some(body) => Some(lower_block(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            body,
        )?),
        None => None,
    };
    let else_map = builder.variable_ctx.variable_map.clone();

    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings;

    let branch_locals =
        collect_branch_local_vars_from_body(then_body, else_body.map(|body| body.as_slice()));
    let (then_map, else_map) = filter_branch_locals_from_maps(
        &pre_if_map,
        &then_map,
        &else_map,
        &branch_locals,
    );

    // Phase 8: use step for join payload generation (3-map diff)
    let joins = build_join_payload(builder, &pre_if_map, &then_map, &else_map)?;

    let plans = lower_cond_branch(
        builder,
        current_bindings,
        cond_view,
        then_plans,
        else_plans,
        joins.clone(),
        error_prefix,
    )?;

    for join in &joins {
        builder
            .variable_ctx
            .variable_map
            .insert(join.name.clone(), join.dst);
        if carrier_phis.contains_key(&join.name) || current_bindings.contains_key(&join.name) {
            current_bindings.insert(join.name.clone(), join.dst);
        }
    }

    Ok(Some(plans))
}

/// ASTNode-based wrapper (delegates to view-first SSOT).
pub(in crate::mir::builder) fn try_lower_general_if<F>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
    lower_block: F,
) -> Result<Option<Vec<LoweredRecipe>>, String>
where
    F: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &[ASTNode],
    ) -> Result<Vec<LoweredRecipe>, String>,
{
    let cond_view = CondBlockView::from_expr(condition);
    try_lower_general_if_view(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        &cond_view,
        then_body,
        else_body,
        error_prefix,
        lower_block,
    )
}

fn body_has_exit(body: &[ASTNode]) -> bool {
    let mut detector = ControlFlowDetector::default();
    detector.count_returns = true;
    let counts = count_control_flow(body, detector);
    counts.break_count > 0 || counts.continue_count > 0 || counts.return_count > 0
}
