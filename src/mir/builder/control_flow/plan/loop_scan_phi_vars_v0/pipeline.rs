//! Pipeline for loop_scan_phi_vars_v0: lowering outer loop with nested loops.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::facts::stmt_view::try_build_stmt_only_block_recipe;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::features::step_mode;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1::lower_nested_loop_depth1_any;
use crate::mir::builder::control_flow::plan::normalizer::{
    lower_loop_header_cond, helpers::LoopBlocksStandard5,
};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;
use crate::mir::builder::control_flow::plan::steps::empty_carriers_args;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::MirType;
use std::collections::BTreeMap;

use super::facts::LoopScanPhiVarsV0Facts;
use super::recipe::{LoopScanPhiSegment, NestedLoopRecipe};

const LOOP_SCAN_PHI_VARS_ERR: &str = "[normalizer] loop_scan_phi_vars_v0";

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

fn lower_nested_loop_plan(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    _ctx: &LoopPatternContext,
) -> Result<LoweredRecipe, String> {
    lower_nested_loop_depth1_any(builder, condition, body, LOOP_SCAN_PHI_VARS_ERR)
}

fn lower_nested_loop_recipe(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    ctx: &LoopPatternContext,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    nested: &NestedLoopRecipe,
) -> Result<Vec<LoweredRecipe>, String> {
    if let Some(plans) = parts::entry::lower_nested_loop_recipe_stmt_only(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        nested,
        LOOP_SCAN_PHI_VARS_ERR,
    )? {
        for plan in &plans {
            apply_loop_final_values_to_bindings(builder, current_bindings, plan);
        }
        Ok(plans)
    } else {
        let plan =
            lower_nested_loop_plan(builder, &nested.cond_view.tail_expr, nested.body.as_ref(), ctx)?;
        apply_loop_final_values_to_bindings(builder, current_bindings, &plan);
        Ok(vec![plan])
    }
}

fn lower_segment(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    ctx: &LoopPatternContext,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    segment: &LoopScanPhiSegment,
) -> Result<Vec<LoweredRecipe>, String> {
    match segment {
        LoopScanPhiSegment::Linear(no_exit) => {
            let verified = parts::entry::verify_no_exit_block_with_pre(
                &no_exit.arena,
                &no_exit.block,
                LOOP_SCAN_PHI_VARS_ERR,
                Some(current_bindings),
            )?;
            parts::entry::lower_no_exit_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                Some(break_phi_dsts),
                verified,
                LOOP_SCAN_PHI_VARS_ERR,
            )
        }
        LoopScanPhiSegment::NestedLoop(nested) => lower_nested_loop_recipe(
            builder,
            current_bindings,
            ctx,
            carrier_step_phis,
            break_phi_dsts,
            nested,
        ),
    }
}

fn lower_found_if_stmt(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    if_stmt: &ASTNode,
    ctx: &LoopPatternContext,
) -> Result<Vec<LoweredRecipe>, String> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = if_stmt
    else {
        return Err(format!(
            "[freeze:contract][loop_scan_phi_vars_v0] expected If: ctx={}",
            LOOP_SCAN_PHI_VARS_ERR
        ));
    };

    let cond_view = CondBlockView::from_expr(condition);

    let mut lower_then =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            lower_found_if_branch_body(
                builder,
                bindings,
                carrier_step_phis,
                break_phi_dsts,
                then_body,
                ctx,
            )
        };

    let mut lower_else_closure =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            let Some(body) = else_body.as_ref() else {
                return Ok(Vec::new());
            };
            lower_found_if_branch_body(
                builder,
                bindings,
                carrier_step_phis,
                break_phi_dsts,
                body,
                ctx,
            )
        };

    let lower_else = else_body.as_ref().map(|_| {
        &mut lower_else_closure
            as &mut dyn FnMut(&mut MirBuilder, &mut BTreeMap<String, crate::mir::ValueId>)
                -> Result<Vec<LoweredRecipe>, String>
    });

    parts::entry::lower_if_join_with_branch_lowerers(
        builder,
        current_bindings,
        &cond_view,
        LOOP_SCAN_PHI_VARS_ERR,
        &mut lower_then,
        lower_else,
        &|_name, _bindings| true,
    )
}

fn lower_found_if_branch_body(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    stmts: &[ASTNode],
    ctx: &LoopPatternContext,
) -> Result<Vec<LoweredRecipe>, String> {
    const ALLOW_EXTENDED: bool = true;
    let mut plans = Vec::new();

    let mut idx = 0;
    while idx < stmts.len() {
        if matches!(stmts[idx], ASTNode::Loop { .. } | ASTNode::While { .. }) {
            let nested = match &stmts[idx] {
                ASTNode::Loop { condition, body, .. } | ASTNode::While { condition, body, .. } => {
                    NestedLoopRecipe {
                        cond_view: CondBlockView::from_expr(condition),
                        loop_stmt: stmts[idx].clone(),
                        body: RecipeBody::new(body.to_vec()),
                        body_stmt_only: try_build_stmt_only_block_recipe(body),
                    }
                }
                _ => unreachable!(),
            };
            plans.extend(lower_nested_loop_recipe(
                builder,
                current_bindings,
                ctx,
                carrier_step_phis,
                break_phi_dsts,
                &nested,
            )?);
            idx += 1;
            continue;
        }

        let start = idx;
        idx += 1;
        while idx < stmts.len() {
            if matches!(stmts[idx], ASTNode::Loop { .. } | ASTNode::While { .. }) {
                break;
            }
            idx += 1;
        }

        let slice = &stmts[start..idx];
        let Some(no_exit) = try_build_no_exit_block_recipe(slice, ALLOW_EXTENDED) else {
            return Err(format!(
                "[freeze:contract][loop_scan_phi_vars_v0] found_if_branch_linear_not_no_exit: ctx={}",
                LOOP_SCAN_PHI_VARS_ERR
            ));
        };
        let verified = parts::entry::verify_no_exit_block_with_pre(
            &no_exit.arena,
            &no_exit.block,
            LOOP_SCAN_PHI_VARS_ERR,
            Some(current_bindings),
        )?;
        plans.extend(parts::entry::lower_no_exit_block_verified(
            builder,
            current_bindings,
            carrier_step_phis,
            Some(break_phi_dsts),
            verified,
            LOOP_SCAN_PHI_VARS_ERR,
        )?);
    }

    Ok(plans)
}

pub(in crate::mir::builder) fn lower_loop_scan_phi_vars_v0(
    builder: &mut MirBuilder,
    facts: LoopScanPhiVarsV0Facts,
    ctx: &LoopPatternContext,
) -> Result<LoweredRecipe, String> {
    let blocks = LoopBlocksStandard5::allocate(builder)?;
    let LoopBlocksStandard5 {
        preheader_bb,
        header_bb,
        body_bb,
        step_bb,
        after_bb,
    } = blocks;

    let LoopScanPhiVarsV0Facts {
        loop_var,
        limit_var,
        condition,
        body_lowering_policy,
        recipe,
        segments,
    } = facts;

    if !builder.variable_ctx.variable_map.contains_key(&limit_var) {
        return Err(format!(
            "[freeze:contract][loop_scan_phi_vars_v0] limit var {} missing init: ctx={}",
            limit_var, LOOP_SCAN_PHI_VARS_ERR
        ));
    }
    let init_val = builder
        .variable_ctx
        .variable_map
        .get(&loop_var)
        .copied()
        .ok_or_else(|| {
            format!(
                "[freeze:contract][loop_scan_phi_vars_v0] loop var {} missing init: ctx={}",
                loop_var, LOOP_SCAN_PHI_VARS_ERR
            )
        })?;
    let ty = builder
        .type_ctx
        .get_type(init_val)
        .cloned()
        .unwrap_or(MirType::Integer);

    let header_phi_dst = builder.alloc_typed(ty.clone());
    let step_phi_dst = builder.alloc_typed(ty.clone());
    let after_phi_dst = builder.alloc_typed(ty);

    let mut carrier_inits = BTreeMap::new();
    carrier_inits.insert(loop_var.clone(), init_val);

    let mut carrier_phis = BTreeMap::new();
    carrier_phis.insert(loop_var.clone(), header_phi_dst);

    let mut carrier_step_phis = BTreeMap::new();
    carrier_step_phis.insert(loop_var.clone(), step_phi_dst);

    let mut break_phi_dsts = BTreeMap::new();
    break_phi_dsts.insert(loop_var.clone(), after_phi_dst);

    let mut current_bindings = carrier_phis.clone();
    for (name, value_id) in &current_bindings {
        builder.variable_ctx.variable_map.insert(name.clone(), *value_id);
    }

    let cond_view = CondBlockView::from_expr(&condition);
    let header_result = lower_loop_header_cond(
        builder,
        &current_bindings,
        &cond_view,
        header_bb,
        body_bb,
        after_bb,
        empty_carriers_args(),
        empty_carriers_args(),
        LOOP_SCAN_PHI_VARS_ERR,
    )?;

    let wires = vec![
        edgecfg_stubs::build_loop_back_edge(body_bb, step_bb),
        edgecfg_stubs::build_loop_back_edge(step_bb, header_bb),
    ];

    let frag = Frag {
        entry: header_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires,
        branches: header_result.branches,
    };

    body_lowering_policy.expect_recipe_only(
        "[loop_scan_phi_vars_v0]",
        LOOP_SCAN_PHI_VARS_ERR,
    )?;

    if segments.len() != 3 {
        return Err(format!(
            "[freeze:contract][loop_scan_phi_vars_v0] unexpected_segments_len={} ctx={}",
            segments.len(),
            LOOP_SCAN_PHI_VARS_ERR
        ));
    }

    let mut body_plans: Vec<LoweredRecipe> = Vec::new();
    body_plans.extend(lower_segment(
        builder,
        &mut current_bindings,
        ctx,
        &carrier_step_phis,
        &break_phi_dsts,
        &segments[0],
    )?);
    body_plans.extend(lower_segment(
        builder,
        &mut current_bindings,
        ctx,
        &carrier_step_phis,
        &break_phi_dsts,
        &segments[1],
    )?);
    body_plans.extend(lower_found_if_stmt(
        builder,
        &mut current_bindings,
        &carrier_step_phis,
        &break_phi_dsts,
        &recipe.found_if_stmt,
        ctx,
    )?);
    body_plans.extend(lower_segment(
        builder,
        &mut current_bindings,
        ctx,
        &carrier_step_phis,
        &break_phi_dsts,
        &segments[2],
    )?);

    body_plans.push(CorePlan::Exit(parts::exit::build_continue_with_phi_args(
        builder,
        &carrier_step_phis,
        &current_bindings,
        LOOP_SCAN_PHI_VARS_ERR,
    )?));

    let mut phis = Vec::new();
    let mut final_values = Vec::new();
    for (var, header_phi_dst) in &carrier_phis {
        let init_val = *carrier_inits
            .get(var)
            .ok_or_else(|| format!("[freeze:contract][loop_scan_phi_vars_v0] missing init for {var}"))?;
        let step_phi_dst = *carrier_step_phis
            .get(var)
            .ok_or_else(|| format!("[freeze:contract][loop_scan_phi_vars_v0] missing step phi for {var}"))?;
        let after_phi_dst = *break_phi_dsts
            .get(var)
            .ok_or_else(|| format!("[freeze:contract][loop_scan_phi_vars_v0] missing after phi for {var}"))?;

        phis.push(loop_carriers::build_step_join_phi_info(
            step_bb,
            step_phi_dst,
            format!("loop_scan_phi_vars_v0_step_join_{}", var),
        ));
        phis.push(loop_carriers::build_loop_phi_info(
            header_bb,
            preheader_bb,
            step_bb,
            *header_phi_dst,
            init_val,
            step_phi_dst,
            format!("loop_scan_phi_vars_v0_carrier_{}", var),
        ));
        phis.push(loop_carriers::build_after_merge_phi_info(
            after_bb,
            after_phi_dst,
            [header_bb],
            *header_phi_dst,
            format!("loop_scan_phi_vars_v0_after_{}", var),
        ));
        final_values.push((var.clone(), after_phi_dst));
    }

    let mut block_effects: Vec<(crate::mir::BasicBlockId, Vec<CoreEffectPlan>)> =
        vec![(preheader_bb, vec![])];
    for (bb, effects) in header_result.block_effects {
        block_effects.push((bb, effects));
    }
    block_effects.push((body_bb, vec![]));
    block_effects.push((step_bb, vec![]));
    block_effects.push((after_bb, vec![]));

    let (step_mode, has_explicit_step) = step_mode::inline_in_body_no_explicit_step();

    Ok(CorePlan::Loop(CoreLoopPlan {
        preheader_bb,
        preheader_is_fresh: false,
        header_bb,
        body_bb,
        step_bb,
        continue_target: step_bb,
        after_bb,
        found_bb: after_bb,
        body: body_plans,
        cond_loop: header_result.first_cond,
        cond_match: header_result.first_cond,
        block_effects,
        phis,
        frag,
        final_values,
        step_mode,
        has_explicit_step,
    }))
}
