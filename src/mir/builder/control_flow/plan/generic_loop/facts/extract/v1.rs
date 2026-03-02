use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::generic_loop::{
    canon_condition_for_generic_loop_v0, canon_loop_increment_for_var, classify_step_placement,
    matches_loop_increment, StepPlacement,
};
use crate::mir::builder::control_flow::plan::facts::reject_reason::{
    handoff_tables, log_reject, RejectReason,
};
use crate::mir::builder::control_flow::plan::facts::exit_only_block::{
    try_build_exit_allowed_block_recipe, ExitAllowedBlockRecipe,
};
use crate::mir::builder::control_flow::plan::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::facts::stmt_view::flatten_scope_boxes;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::recipe_tree::{RecipeBlock, RecipeBodies};
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;
use crate::mir::builder::control_flow::plan::single_planner::PlanRuleId;
use crate::mir::policies::BodyLoweringPolicy;

use super::super::super::body_check::shape_resolution::{
    check_body_generic_v0, detect_generic_loop_v1_shape,
};
use super::super::super::facts_helpers::reject_or_none;
use super::super::super::body_check::step_validation::{
    has_control_flow_after_step, validate_break_else_if_step, validate_continue_if_step,
    validate_in_body_step_v1,
};
use super::super::super::facts_types::GenericLoopV1Facts;
use super::collection::{
    body_has_break_or_continue_stmt, collect_loop_var_candidates_from_body, has_continue_recursive,
};

/// Attempts to extract generic loop v1 facts from a loop condition and body
pub(in crate::mir::builder) fn try_extract_generic_loop_v1_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<GenericLoopV1Facts>, Freeze> {
    let flat_body = flatten_scope_boxes(body);
    let strict = crate::config::env::joinir_dev::strict_enabled();
    let strict_or_dev = strict || crate::config::env::joinir_dev_enabled();
    let debug_enabled = crate::config::env::joinir_dev::debug_enabled();
    let planner_required =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    let allow_var_step = planner_required;
    let has_break_or_continue = flat_body.iter().any(body_has_break_or_continue_stmt);
    let base_body_lowering_policy = if planner_required && !has_break_or_continue {
        BodyLoweringPolicy::ExitAllowed {
            allow_join_if: false,
        }
    } else {
        BodyLoweringPolicy::RecipeOnly
    };

    let Some(canon) = canon_condition_for_generic_loop_v0(condition, planner_required) else {
        log_reject(
            "generic_loop_v1",
            RejectReason::UnsupportedCondition,
            handoff_tables::for_generic_loop,
        );
        return Ok(None);
    };
    let cond_profile = canon.cond_profile.clone();

    let raw_candidates = canon.loop_var_candidates.len();
    let mut reject_no_increment = 0usize;
    let mut reject_step_placement_none = 0usize;
    let mut reject_control_flow_after_step = 0usize;
    let mut reject_step_invalid = 0usize;
    let mut reject_recipe_unbuildable = 0usize;
    let mut reject_v0_guard = 0usize;

    let mut matches = Vec::new();
    let mut logged_planner_first = false;
    for loop_var in &canon.loop_var_candidates {
        let Some(loop_increment) =
            canon_loop_increment_for_var(&flat_body, loop_var, allow_var_step)
        else {
            reject_no_increment += 1;
            continue;
        };

        let shape_id =
            detect_generic_loop_v1_shape(&flat_body, loop_var, &loop_increment, condition)?;
        let step_decision = classify_step_placement(&flat_body, loop_var, &loop_increment);
        if let Some(reason) = step_decision.reject_reason {
            log_reject(
                "generic_loop_v1",
                reason,
                handoff_tables::for_generic_loop,
            );
            return reject_or_none(strict, reason.as_freeze_message());
        }
        let Some(step_placement) = step_decision.placement else {
            reject_step_placement_none += 1;
            continue;
        };

        if planner_required
            && matches!(step_placement, StepPlacement::InBody(idx) if has_control_flow_after_step(&flat_body, idx))
        {
            reject_control_flow_after_step += 1;
            continue;
        }

        let step_ok = match step_placement {
            StepPlacement::InBody(idx) => {
                validate_in_body_step_v1(&flat_body, idx, loop_var, &loop_increment, strict)?
            }
            StepPlacement::InContinueIf(idx) => {
                validate_continue_if_step(&flat_body, idx, strict)?
            }
            StepPlacement::InBreakElseIf(idx) => {
                validate_break_else_if_step(&flat_body, idx, strict)?
            }
            _ => true,
        };
        if !step_ok {
            reject_step_invalid += 1;
            continue;
        }

        let mut body_lowering_policy = base_body_lowering_policy;
        let body_for_recipe: Vec<ASTNode> = flat_body
            .iter()
            .filter(|stmt| !matches_loop_increment(stmt, loop_var, &loop_increment))
            .cloned()
            .collect();
        let body_exit_allowed = if body_for_recipe.is_empty() {
            let mut arena = RecipeBodies::new();
            let body_id = arena.register(RecipeBody::new(Vec::new()));
            let block = RecipeBlock::new(body_id, Vec::new());
            Some(ExitAllowedBlockRecipe { arena, block })
        } else {
            try_build_exit_allowed_block_recipe(&body_for_recipe, true)
        };
        if planner_required && body_exit_allowed.is_none() {
            reject_recipe_unbuildable += 1;
            continue;
        }
        let body_no_exit = try_build_no_exit_block_recipe(&body_for_recipe, true);
        if matches!(body_lowering_policy, BodyLoweringPolicy::ExitAllowed { .. })
            && body_exit_allowed.is_some()
        {
            body_lowering_policy = BodyLoweringPolicy::RecipeOnly;
        }
        if shape_id.is_none() && body_exit_allowed.is_none() {
            if check_body_generic_v0(&flat_body, loop_var, &loop_increment).is_some() {
                reject_v0_guard += 1;
                continue;
            }
        }

        if planner_required && (shape_id.is_some() || body_exit_allowed.is_some()) && !logged_planner_first {
            if strict_or_dev {
                let msg = crate::mir::builder::control_flow::plan::planner::tags::planner_first_tag_with_label(
                    PlanRuleId::Pattern1,
                );
                if crate::config::env::joinir_dev::strict_planner_required_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    let _ = ring0.io.stderr_write(format!("{}\n", msg).as_bytes());
                } else if debug_enabled {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&msg);
                }
            }
            logged_planner_first = true;
        }

        matches.push(GenericLoopV1Facts {
            loop_var: loop_var.clone(),
            condition: condition.clone(),
            loop_increment,
            body: RecipeBody::new(flat_body.clone()),
            shape_id,
            body_lowering_policy,
            body_exit_allowed: body_exit_allowed.clone(),
            body_no_exit,
            cond_profile: cond_profile.clone(),
        });
    }

    if matches.is_empty() {
        if debug_enabled {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[plan/trace:loop_var_candidates] ctx=generic_loop_v1 raw={} filtered={} reasons=no_increment={} step_placement_none={} control_flow_after_step={} step_invalid={} recipe_unbuildable={} v0_guard={}",
                raw_candidates,
                matches.len(),
                reject_no_increment,
                reject_step_placement_none,
                reject_control_flow_after_step,
                reject_step_invalid,
                reject_recipe_unbuildable,
                reject_v0_guard
            ));
        }
        if planner_required && reject_recipe_unbuildable > 0 {
            return Err(Freeze::unsupported(
                "generic_loop_v1: cannot build recipe for body",
            ));
        }
        let detailed_reject = format!(
            "box=generic_loop_v1 reason=no_valid_loop_var_candidates raw={} no_increment={} step_placement_none={} control_flow_after_step={} step_invalid={} recipe_unbuildable={} v0_guard={}",
            raw_candidates,
            reject_no_increment,
            reject_step_placement_none,
            reject_control_flow_after_step,
            reject_step_invalid,
            reject_recipe_unbuildable,
            reject_v0_guard
        );
        log_reject(
            "generic_loop_v1",
            RejectReason::NoValidLoopVarCandidates,
            handoff_tables::for_generic_loop,
        );
        crate::mir::builder::control_flow::plan::facts::reject_reason::set_last_plan_reject_detail(
            detailed_reject,
        );
        return Ok(None);
    }

    if matches.len() > 1 {
        log_reject(
            "generic_loop_v1",
            RejectReason::AmbiguousLoopVarCandidates,
            handoff_tables::for_generic_loop,
        );
        return reject_or_none(
            strict,
            RejectReason::AmbiguousLoopVarCandidates.as_freeze_message(),
        );
    }

    Ok(Some(matches.remove(0)))
}

/// Recipe-based hint for generic_loop_v1
pub(in crate::mir::builder) fn has_generic_loop_v1_recipe_hint(
    condition: &ASTNode,
    body: &[ASTNode],
) -> bool {
    let flat_body = flatten_scope_boxes(body);
    let allow_var_step = true;

    let Some(canon) = canon_condition_for_generic_loop_v0(condition, true) else {
        return false;
    };

    let mut candidates = canon.loop_var_candidates;

    if candidates.is_empty() {
        candidates = collect_loop_var_candidates_from_body(&flat_body);
    }
    if candidates.is_empty() {
        return false;
    }

    for loop_var in &candidates {
        let Some(loop_increment) =
            canon_loop_increment_for_var(&flat_body, loop_var, allow_var_step)
        else {
            continue;
        };

        let step_decision = classify_step_placement(&flat_body, loop_var, &loop_increment);
        let placement_ok = match step_decision.placement {
            Some(StepPlacement::Last) => true,
            Some(StepPlacement::InContinueIf(_)) | Some(StepPlacement::InBreakElseIf(_)) => true,
            Some(StepPlacement::InBody(_)) => false,
            None => false,
        };
        if !placement_ok {
            continue;
        }

        let has_nested_loop = flat_body.iter().any(|stmt| matches!(stmt, ASTNode::Loop { .. } | ASTNode::While { .. } | ASTNode::ForRange { .. }));
        if has_nested_loop {
            continue;
        }
        let has_continue = flat_body.iter().any(has_continue_recursive);
        if has_continue {
            continue;
        }

        let body_for_recipe: Vec<ASTNode> = flat_body
            .iter()
            .filter(|stmt| !matches_loop_increment(stmt, loop_var, &loop_increment))
            .cloned()
            .collect();
        let body_exit_allowed = if body_for_recipe.is_empty() {
            true
        } else {
            try_build_exit_allowed_block_recipe(&body_for_recipe, false).is_some()
        };
        if body_exit_allowed {
            return true;
        }
    }
    false
}
