//! Pipeline for loop_scan_phi_vars_v0: lowering outer loop with nested loops.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::features::step_mode;
use crate::mir::builder::control_flow::plan::normalizer::{
    helpers::LoopBlocksStandard5, lower_loop_header_cond,
};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::steps::empty_carriers_args;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::control_flow::recipes::loop_scan_phi_vars_v0::LoopScanPhiSegment;
use crate::mir::builder::MirBuilder;
use crate::mir::MirType;
use std::collections::{BTreeMap, BTreeSet};

use super::if_branch_scan::lower_loop_scan_phi_vars_found_if_branch_body;
use super::segment_linear::lower_loop_scan_phi_vars_linear_segment;
use super::segment_nested_loop::lower_loop_scan_phi_vars_nested_segment;
use super::LoopScanPhiVarsV0Facts;

const LOOP_SCAN_PHI_VARS_ERR: &str = "[normalizer] loop_scan_phi_vars_v0";

fn collect_assigned_vars_in_stmt(stmt: &ASTNode, out: &mut BTreeSet<String>) {
    match stmt {
        ASTNode::Assignment { target, .. } => {
            if let ASTNode::Variable { name, .. } = target.as_ref() {
                out.insert(name.clone());
            }
        }
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            for s in then_body {
                collect_assigned_vars_in_stmt(s, out);
            }
            if let Some(else_body) = else_body.as_ref() {
                for s in else_body {
                    collect_assigned_vars_in_stmt(s, out);
                }
            }
        }
        ASTNode::Loop { body, .. }
        | ASTNode::While { body, .. }
        | ASTNode::ForRange { body, .. } => {
            for s in body {
                collect_assigned_vars_in_stmt(s, out);
            }
        }
        ASTNode::Program { statements, .. } => {
            for s in statements {
                collect_assigned_vars_in_stmt(s, out);
            }
        }
        ASTNode::ScopeBox { body, .. } => {
            for s in body {
                collect_assigned_vars_in_stmt(s, out);
            }
        }
        _ => {}
    }
}

fn lower_segment(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    segment: &LoopScanPhiSegment,
) -> Result<Vec<LoweredRecipe>, String> {
    match segment {
        LoopScanPhiSegment::Linear(no_exit) => lower_loop_scan_phi_vars_linear_segment(
            builder,
            current_bindings,
            carrier_step_phis,
            break_phi_dsts,
            no_exit,
        ),
        LoopScanPhiSegment::NestedLoop(nested) => lower_loop_scan_phi_vars_nested_segment(
            builder,
            current_bindings,
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
            lower_loop_scan_phi_vars_found_if_branch_body(
                builder,
                bindings,
                carrier_step_phis,
                break_phi_dsts,
                then_body,
            )
        };

    let mut lower_else_closure =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            let Some(body) = else_body.as_ref() else {
                return Ok(Vec::new());
            };
            lower_loop_scan_phi_vars_found_if_branch_body(
                builder,
                bindings,
                carrier_step_phis,
                break_phi_dsts,
                body,
            )
        };

    let lower_else = else_body.as_ref().map(|_| {
        &mut lower_else_closure
            as &mut dyn FnMut(
                &mut MirBuilder,
                &mut BTreeMap<String, crate::mir::ValueId>,
            ) -> Result<Vec<LoweredRecipe>, String>
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

pub(in crate::mir::builder) fn lower_loop_scan_phi_vars_v0(
    builder: &mut MirBuilder,
    facts: LoopScanPhiVarsV0Facts,
    _ctx: &LoopRouteContext,
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

    let mut carrier_vars = vec![loop_var.clone()];
    if recipe.found_if_stmt.is_none() {
        let mut assigned = BTreeSet::new();
        collect_assigned_vars_in_stmt(&recipe.inner_loop_search, &mut assigned);
        for var in assigned {
            if var == loop_var || var == limit_var {
                continue;
            }
            if builder.variable_ctx.variable_map.contains_key(&var) {
                carrier_vars.push(var);
            }
        }
    }

    let mut carrier_inits = BTreeMap::new();
    let mut carrier_phis = BTreeMap::new();
    let mut carrier_step_phis = BTreeMap::new();
    let mut break_phi_dsts = BTreeMap::new();

    for var in carrier_vars {
        let init_val = builder
            .variable_ctx
            .variable_map
            .get(&var)
            .copied()
            .ok_or_else(|| {
                if var == loop_var {
                    format!(
                        "[freeze:contract][loop_scan_phi_vars_v0] loop var {} missing init: ctx={}",
                        var, LOOP_SCAN_PHI_VARS_ERR
                    )
                } else {
                    format!(
                        "[freeze:contract][loop_scan_phi_vars_v0] carrier var {} missing init: ctx={}",
                        var, LOOP_SCAN_PHI_VARS_ERR
                    )
                }
            })?;
        let ty = builder
            .type_ctx
            .get_type(init_val)
            .cloned()
            .unwrap_or(MirType::Integer);
        let header_phi_dst = builder.alloc_typed(ty.clone());
        let step_phi_dst = builder.alloc_typed(ty.clone());
        let after_phi_dst = builder.alloc_typed(ty);

        carrier_inits.insert(var.clone(), init_val);
        carrier_phis.insert(var.clone(), header_phi_dst);
        carrier_step_phis.insert(var.clone(), step_phi_dst);
        break_phi_dsts.insert(var, after_phi_dst);
    }

    let mut current_bindings = carrier_phis.clone();
    for (name, value_id) in &current_bindings {
        builder
            .variable_ctx
            .variable_map
            .insert(name.clone(), *value_id);
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

    body_lowering_policy.expect_recipe_only("[loop_scan_phi_vars_v0]", LOOP_SCAN_PHI_VARS_ERR)?;

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
        &carrier_step_phis,
        &break_phi_dsts,
        &segments[0],
    )?);
    body_plans.extend(lower_segment(
        builder,
        &mut current_bindings,
        &carrier_step_phis,
        &break_phi_dsts,
        &segments[1],
    )?);
    if let Some(found_if_stmt) = recipe.found_if_stmt.as_ref() {
        body_plans.extend(lower_found_if_stmt(
            builder,
            &mut current_bindings,
            &carrier_step_phis,
            &break_phi_dsts,
            found_if_stmt,
        )?);
    }
    body_plans.extend(lower_segment(
        builder,
        &mut current_bindings,
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
        let init_val = *carrier_inits.get(var).ok_or_else(|| {
            format!("[freeze:contract][loop_scan_phi_vars_v0] missing init for {var}")
        })?;
        let step_phi_dst = *carrier_step_phis.get(var).ok_or_else(|| {
            format!("[freeze:contract][loop_scan_phi_vars_v0] missing step phi for {var}")
        })?;
        let after_phi_dst = *break_phi_dsts.get(var).ok_or_else(|| {
            format!("[freeze:contract][loop_scan_phi_vars_v0] missing after phi for {var}")
        })?;

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
