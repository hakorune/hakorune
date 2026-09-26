//! Located-source LoopTrue physical adapter.
//!
//! This adapter consumes the co-sealed source product through the existing
//! LoopTrue physical owner.  The source port supplies located carriers to the
//! neutral Parts dispatcher; no LoopRouteContext or raw AST lowering is
//! re-entered here.

use std::collections::BTreeMap;

use crate::mir::builder::control_flow::plan::features::loop_true_break_continue_cleanup::{
    apply_fallthrough_continue_exit, requires_fallthrough_continue,
};
use crate::mir::builder::control_flow::plan::features::loop_true_break_continue_phi_materializer::LoopTrueBreakContinuePhiMaterializer;
use crate::mir::builder::control_flow::plan::features::loop_true_break_continue_verifier::verify_loop_true_break_continue_phi_closure;
use crate::mir::builder::control_flow::plan::features::step_mode;
use crate::mir::builder::control_flow::plan::parts::{
    lower_callable_loop_source_parts_block, CallableLoopSourcePartsBlockV1,
    PartsAssociatedBlockModeV1,
};
use crate::mir::builder::control_flow::plan::skeletons::loop_true::alloc_loop_true_skeleton;
use crate::mir::builder::control_flow::plan::{CoreLoopPlan, CorePhiInfo, CorePlan, LoweredRecipe};
use crate::mir::builder::normal_callable_loop_source_facts::SourceLoopTruePhysicalInputV1;
use crate::mir::builder::MirBuilder;

const LOOP_TRUE_SOURCE_ERR: &str = "[freeze:contract][callable-loop/loop-true/source-physical]";

pub(in crate::mir::builder) fn lower_loop_true_break_continue_source(
    builder: &mut MirBuilder,
    input: &SourceLoopTruePhysicalInputV1<'_, '_>,
) -> Result<LoweredRecipe, String> {
    input.validate_for_source_port()?;
    let facts = input.loop_true_facts()?;
    let crate::mir::builder::control_flow::plan::loop_cond::true_break_continue::
        LoopTrueBreakContinueLowering::ExitAllowed(body_exit_allowed) = &facts.lowering
    else {
        return Err(format!("{LOOP_TRUE_SOURCE_ERR}/recipe-only-unported"));
    };

    let skeleton = alloc_loop_true_skeleton(builder)?;
    let preheader_bb = skeleton.preheader_bb;
    let header_bb = skeleton.header_bb;
    let body_bb = skeleton.body_bb;
    let step_bb = skeleton.step_bb;
    let after_bb = skeleton.after_bb;
    let cond_loop = skeleton.cond_loop;
    let block_effects = skeleton.block_effects;
    let frag = skeleton.frag;

    let carrier_vars =
        crate::mir::builder::control_flow::plan::features::carriers::collect_outer_from_body(
            builder,
            &facts.recipe.body.body,
        )
        .vars;
    let phi_materializer = LoopTrueBreakContinuePhiMaterializer::prepare(
        builder,
        &carrier_vars,
        LOOP_TRUE_SOURCE_ERR,
    )?;
    let carrier_phis = phi_materializer.carrier_phis().clone();
    let carrier_step_phis = phi_materializer.carrier_step_phis().clone();
    let mut current_bindings = phi_materializer.phi_bindings();

    let has_break = facts.recipe.body.body.iter().any(body_has_break_stmt);
    let mut body_break_phi_dsts: Option<BTreeMap<String, crate::mir::ValueId>> = None;
    let mut body_after_phis: Vec<CorePhiInfo> = Vec::new();
    let break_phi_dsts = if has_break {
        let (out, after_phis) =
            phi_materializer.plan_break_after_phis(builder, &carrier_vars, header_bb, after_bb);
        body_after_phis = after_phis;
        if !out.is_empty() {
            body_break_phi_dsts = Some(out.clone());
        }
        out
    } else {
        BTreeMap::new()
    };

    let port = *input.source_port();
    for (name, value_id) in &carrier_phis {
        current_bindings.insert(name.clone(), *value_id);
        crate::mir::builder::control_flow::plan::parts::var_map_scope::publish_emission_cache(
            builder,
            name.clone(),
            *value_id,
        );
        port.publish_loop_carrier_value(name, *value_id)
            .map_err(|error| format!("{LOOP_TRUE_SOURCE_ERR}/carrier-header-publish: {error}"))?;
    }

    let body_carrier = port
        .body(input.body(), input.body_source())
        .map_err(|error| format!("{LOOP_TRUE_SOURCE_ERR}/body-input: {error}"))?;
    let source_block = CallableLoopSourcePartsBlockV1::located_body(
        &body_exit_allowed.arena,
        &body_exit_allowed.block,
        body_carrier,
        &port,
    )
    .map_err(|error| format!("{LOOP_TRUE_SOURCE_ERR}/body-seal: {error:?}"))?;
    let mut body_plans = lower_callable_loop_source_parts_block(
        port,
        &source_block,
        PartsAssociatedBlockModeV1::ExitAllowed,
        builder,
        &mut current_bindings,
        &carrier_phis,
        &carrier_step_phis,
        &break_phi_dsts,
        &mut BTreeMap::new(),
        LOOP_TRUE_SOURCE_ERR,
    )?;

    let requires_fallthrough = requires_fallthrough_continue(
        &body_plans,
        matches!(
            facts.recipe.items.last(),
            Some(
                crate::mir::builder::control_flow::plan::loop_true_break_continue::recipe::
                    LoopTrueItem::TailReturn(_)
            )
        ),
    );
    if requires_fallthrough {
        apply_fallthrough_continue_exit(
            builder,
            &mut body_plans,
            &carrier_step_phis,
            &current_bindings,
            LOOP_TRUE_SOURCE_ERR,
        )?;
    }

    let body_after_phi_count = body_after_phis.len();
    let phi_closure = phi_materializer.close(
        preheader_bb,
        header_bb,
        step_bb,
        body_break_phi_dsts.as_ref(),
        body_after_phis,
        LOOP_TRUE_SOURCE_ERR,
    )?;
    verify_loop_true_break_continue_phi_closure(
        &phi_closure,
        &body_plans,
        body_break_phi_dsts.as_ref(),
        body_after_phi_count,
        carrier_phis.len(),
        requires_fallthrough,
        LOOP_TRUE_SOURCE_ERR,
    )?;

    for (name, value_id) in phi_closure.final_values() {
        port.publish_loop_carrier_value(name, *value_id)
            .map_err(|error| format!("{LOOP_TRUE_SOURCE_ERR}/carrier-final-publish: {error}"))?;
    }

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
        cond_loop,
        cond_match: cond_loop,
        block_effects,
        phis: phi_closure.phis().to_vec(),
        frag,
        final_values: phi_closure.final_values().to_vec().into(),
        step_mode,
        has_explicit_step,
    }))
}

fn body_has_break_stmt(stmt: &crate::ast::ASTNode) -> bool {
    match stmt {
        crate::ast::ASTNode::Break { .. } => true,
        crate::ast::ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            then_body.iter().any(body_has_break_stmt)
                || else_body
                    .as_ref()
                    .is_some_and(|body| body.iter().any(body_has_break_stmt))
        }
        crate::ast::ASTNode::Loop { body, .. } => body.iter().any(body_has_break_stmt),
        crate::ast::ASTNode::Program { statements, .. } => {
            statements.iter().any(body_has_break_stmt)
        }
        _ => false,
    }
}
