use super::route_finalize::finalize_loop_scan_methods_block_route;
use super::segment_linear::lower_loop_scan_methods_block_linear_segment;
use super::segment_nested_loop::lower_loop_scan_methods_block_nested_segment;
use super::LoopScanMethodsBlockV0Facts;
use crate::mir::builder::control_flow::edgecfg::api::Frag;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::step_mode;
use crate::mir::builder::control_flow::plan::normalizer::cond_lowering_loop_header::lower_loop_header_cond;
use crate::mir::builder::control_flow::plan::normalizer::helpers::LoopBlocksStandard5;
use crate::mir::builder::control_flow::plan::steps::empty_carriers_args;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::control_flow::recipes::loop_scan_methods_block_v0::ScanSegment;
use crate::mir::builder::MirBuilder;
use crate::mir::MirType;
use std::collections::BTreeMap;

const LOOP_SCAN_METHODS_BLOCK_ERR: &str = "[normalizer] loop_scan_methods_block_v0";

pub(in crate::mir::builder) fn lower_loop_scan_methods_block_v0(
    builder: &mut MirBuilder,
    facts: LoopScanMethodsBlockV0Facts,
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

    if !builder
        .variable_ctx
        .variable_map
        .contains_key(&facts.limit_var)
    {
        return Err(format!(
            "[freeze:contract][loop_scan_methods_block_v0] limit var {} missing init: ctx={}",
            facts.limit_var, LOOP_SCAN_METHODS_BLOCK_ERR
        ));
    }

    let init_val = builder
        .variable_ctx
        .variable_map
        .get(&facts.loop_var)
        .copied()
        .ok_or_else(|| {
            format!(
                "[freeze:contract][loop_scan_methods_block_v0] loop var {} missing init: ctx={}",
                facts.loop_var, LOOP_SCAN_METHODS_BLOCK_ERR
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

    let mut carrier_phis = BTreeMap::new();
    carrier_phis.insert(facts.loop_var.clone(), header_phi_dst);

    let mut carrier_step_phis = BTreeMap::new();
    carrier_step_phis.insert(facts.loop_var.clone(), step_phi_dst);

    let mut break_phi_dsts = BTreeMap::new();
    break_phi_dsts.insert(facts.loop_var.clone(), after_phi_dst);

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
        LOOP_SCAN_METHODS_BLOCK_ERR,
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
    facts
        .body_lowering_policy
        .expect_exit_allowed("[loop_scan_methods_block_v0]", LOOP_SCAN_METHODS_BLOCK_ERR)?;
    for segment in &facts.recipe.segments {
        match segment {
            ScanSegment::Linear(linear) => {
                body_plans.extend(lower_loop_scan_methods_block_linear_segment(
                    builder,
                    &mut current_bindings,
                    &carrier_step_phis,
                    &break_phi_dsts,
                    &linear,
                )?)
            }
            ScanSegment::NestedLoop(nested) => {
                body_plans.extend(lower_loop_scan_methods_block_nested_segment(
                    builder,
                    &mut current_bindings,
                    &carrier_step_phis,
                    &break_phi_dsts,
                    &nested,
                    ctx,
                )?);
            }
        }
    }

    let finalized = finalize_loop_scan_methods_block_route(
        builder,
        &mut body_plans,
        init_val,
        &carrier_phis,
        &carrier_step_phis,
        &break_phi_dsts,
        &current_bindings,
        preheader_bb,
        header_bb,
        step_bb,
        after_bb,
    )?;

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
        frag,
        block_effects,
        phis: finalized.phis,
        final_values: finalized.final_values,
        step_mode,
        has_explicit_step,
    }))
}
