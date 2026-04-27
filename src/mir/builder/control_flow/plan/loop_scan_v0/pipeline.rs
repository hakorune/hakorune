use crate::mir::builder::control_flow::edgecfg::api::Frag;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::stmt_view::try_build_stmt_only_block_recipe;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::step_mode;
use crate::mir::builder::control_flow::plan::normalizer::{
    helpers::LoopBlocksStandard5, lower_loop_header_cond, PlanNormalizer,
};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::steps::empty_carriers_args;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::policies::BodyLoweringPolicy;
use crate::mir::MirType;
use std::collections::BTreeMap;

use super::facts::LoopScanV0Facts;
use super::recipe::LoopScanSegment;
use super::route_finalize::finalize_loop_scan_v0_route;
use super::segment_linear::lower_loop_scan_v0_linear_segment;
use super::segment_nested_loop::lower_loop_scan_v0_nested_segment;

const LOOP_SCAN_ERR: &str = "[normalizer] loop_scan_v0";

pub(in crate::mir::builder) fn lower_loop_scan_v0(
    builder: &mut MirBuilder,
    facts: LoopScanV0Facts,
    ctx: &LoopRouteContext,
) -> Result<LoweredRecipe, String> {
    let blocks = LoopBlocksStandard5::allocate(builder)?;
    let LoopBlocksStandard5 {
        preheader_bb,
        header_bb,
        body_bb,
        step_bb,
        after_bb,
    } = blocks;

    let loop_var = facts.loop_var;
    if !builder
        .variable_ctx
        .variable_map
        .contains_key(&facts.limit_var)
    {
        return Err(format!(
            "[freeze:contract][loop_scan_v0] limit var {} missing init: ctx={LOOP_SCAN_ERR}",
            facts.limit_var
        ));
    }
    let init_val = builder
        .variable_ctx
        .variable_map
        .get(&loop_var)
        .copied()
        .ok_or_else(|| format!("{LOOP_SCAN_ERR}: loop var {loop_var} missing init"))?;
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
        builder
            .variable_ctx
            .variable_map
            .insert(name.clone(), *value_id);
    }

    let cond_view = CondBlockView::from_expr(&facts.condition);
    let header_result = lower_loop_header_cond(
        builder,
        &current_bindings,
        &cond_view,
        header_bb,
        body_bb,
        after_bb,
        empty_carriers_args(),
        empty_carriers_args(),
        LOOP_SCAN_ERR,
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

    let mut body_plans: Vec<LoweredRecipe> = Vec::new();

    match facts.body_lowering_policy {
        BodyLoweringPolicy::ExitAllowed { .. } => {
            if facts.segments.is_empty() {
                return Err(format!(
                    "[freeze:contract][loop_scan_v0] body_lowering_policy=ExitAllowed but segments empty: ctx={LOOP_SCAN_ERR}"
                ));
            }
            for segment in &facts.segments {
                match segment {
                    LoopScanSegment::Linear(exit_allowed) => {
                        body_plans.extend(lower_loop_scan_v0_linear_segment(
                            builder,
                            &mut current_bindings,
                            &carrier_step_phis,
                            &break_phi_dsts,
                            exit_allowed,
                        )?);
                    }
                    LoopScanSegment::NestedLoop(nested) => {
                        body_plans.extend(lower_loop_scan_v0_nested_segment(
                            builder,
                            &mut current_bindings,
                            &carrier_step_phis,
                            &break_phi_dsts,
                            nested,
                            ctx,
                        )?)
                    }
                }
            }
        }
        BodyLoweringPolicy::RecipeOnly => {
            // Fallback: legacy hand-lowering (acceptance-preserving).
            //
            // Note: Prefer Facts-provided `segments` for recipe-first lowering.
            let local_ch_recipe =
                try_build_stmt_only_block_recipe(std::slice::from_ref(&facts.recipe.local_ch_stmt))
                    .ok_or_else(|| {
                        format!(
                    "[freeze:contract][loop_scan_v0] local_ch not stmt-only: ctx={LOOP_SCAN_ERR}"
                )
                    })?;

            body_plans.extend(parts::entry::lower_loop_with_body_block(
                builder,
                &mut current_bindings,
                &carrier_step_phis,
                &local_ch_recipe.arena,
                &local_ch_recipe.block,
                parts::LoopBodyContractKind::StmtOnly,
                LOOP_SCAN_ERR,
            )?);

            // 2) if ch == "," { i = i + 1; continue }
            let comma_view = CondBlockView::from_expr(&facts.recipe.comma_if_cond);
            let (comma_cond_id, comma_cond_effects) =
                crate::mir::builder::control_flow::plan::normalizer::lower_bool_expr_value_id(
                    builder,
                    &current_bindings,
                    &comma_view,
                    LOOP_SCAN_ERR,
                )?;
            body_plans.extend(
                crate::mir::builder::control_flow::plan::steps::effects_to_plans(
                    comma_cond_effects,
                ),
            );

            let (new_i_id, mut inc_effects) = match &facts.recipe.comma_inc_stmt {
                crate::ast::ASTNode::Assignment { value, .. } => {
                    PlanNormalizer::lower_value_ast(value, builder, &current_bindings)?
                }
                _ => {
                    return Err(format!(
                        "[freeze:contract][loop_scan_v0] comma_inc_stmt not Assignment: ctx={LOOP_SCAN_ERR}"
                    ));
                }
            };

            let mut then_bindings = current_bindings.clone();
            then_bindings.insert(loop_var.clone(), new_i_id);
            let continue_exit = parts::exit::build_continue_with_phi_args(
                builder,
                &carrier_step_phis,
                &then_bindings,
                LOOP_SCAN_ERR,
            )?;

            inc_effects.push(
                crate::mir::builder::control_flow::plan::CoreEffectPlan::ExitIf {
                    cond: comma_cond_id,
                    exit: continue_exit,
                },
            );
            body_plans.push(CorePlan::Effect(
                crate::mir::builder::control_flow::plan::CoreEffectPlan::IfEffect {
                    cond: comma_cond_id,
                    then_effects: inc_effects,
                    else_effects: None,
                },
            ));

            // 3) if ch == "]" { break }
            let close_view = CondBlockView::from_expr(&facts.recipe.close_if_cond);
            let (close_cond_id, close_cond_effects) =
                crate::mir::builder::control_flow::plan::normalizer::lower_bool_expr_value_id(
                    builder,
                    &current_bindings,
                    &close_view,
                    LOOP_SCAN_ERR,
                )?;
            body_plans.extend(
                crate::mir::builder::control_flow::plan::steps::effects_to_plans(
                    close_cond_effects,
                ),
            );
            body_plans.push(CorePlan::Effect(
                crate::mir::builder::control_flow::plan::CoreEffectPlan::ExitIf {
                    cond: close_cond_id,
                    exit: parts::exit::build_break_with_phi_args(
                        &break_phi_dsts,
                        &current_bindings,
                        LOOP_SCAN_ERR,
                    )?,
                },
            ));

            // 4) i = i + 1 (fallthrough step)
            let step_inc_recipe =
                try_build_stmt_only_block_recipe(std::slice::from_ref(&facts.recipe.step_inc_stmt))
                    .ok_or_else(|| {
                        format!(
                    "[freeze:contract][loop_scan_v0] step_inc not stmt-only: ctx={LOOP_SCAN_ERR}"
                )
                    })?;

            body_plans.extend(parts::entry::lower_loop_with_body_block(
                builder,
                &mut current_bindings,
                &carrier_step_phis,
                &step_inc_recipe.arena,
                &step_inc_recipe.block,
                parts::LoopBodyContractKind::StmtOnly,
                LOOP_SCAN_ERR,
            )?);
        }
    }

    let finalized = finalize_loop_scan_v0_route(
        builder,
        &mut body_plans,
        &carrier_inits,
        &carrier_phis,
        &carrier_step_phis,
        &break_phi_dsts,
        &current_bindings,
        preheader_bb,
        header_bb,
        step_bb,
        after_bb,
    )?;

    // Build block_effects: merge header_result.block_effects + static entries
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
        phis: finalized.phis,
        frag,
        final_values: finalized.final_values,
        step_mode,
        has_explicit_step,
    }))
}
