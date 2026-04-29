use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::generic_loop::{
    canon_condition_for_generic_loop_v0, canon_loop_increment_for_var,
};
use crate::mir::builder::control_flow::facts::stmt_view::flatten_scope_boxes;
use crate::mir::builder::control_flow::plan::canon::generic_loop::{
    classify_step_placement, StepPlacement,
};
use crate::mir::builder::control_flow::plan::facts::reject_reason::{
    handoff_tables, log_reject, RejectReason,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::recipes::RecipeBody;
use crate::mir::policies::generic_loop_overlap_policy::v1_shape_blocks_v0;

use super::super::super::body_check::shape_resolution::{
    check_body_generic_v1, detect_generic_loop_v1_shape,
};
use super::super::super::body_check::step_validation::{
    has_control_flow_after_step, validate_break_else_if_step, validate_continue_if_step,
    validate_in_body_step,
};
use super::super::super::facts_helpers::reject_or_none;
use super::super::super::facts_types::GenericLoopV0Facts;
use super::collection::body_writes_non_loop_vars;

/// Attempts to extract generic loop v0 facts from a loop condition and body
pub(in crate::mir::builder) fn try_extract_generic_loop_v0_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<GenericLoopV0Facts>, Freeze> {
    let flat_body = flatten_scope_boxes(body);
    let strict = crate::config::env::joinir_dev::strict_enabled();
    let strict_or_dev = strict || crate::config::env::joinir_dev_enabled();
    let planner_required =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    if planner_required {
        return Ok(None);
    }
    let allow_var_step = planner_required;

    let Some(canon) = canon_condition_for_generic_loop_v0(condition, planner_required) else {
        log_reject(
            "generic_loop_v0",
            RejectReason::UnsupportedCondition,
            handoff_tables::for_generic_loop,
        );
        return Ok(None);
    };
    let mut matches = Vec::new();
    for loop_var in &canon.loop_var_candidates {
        let Some(loop_increment) =
            canon_loop_increment_for_var(&flat_body, loop_var, allow_var_step)
        else {
            continue;
        };

        let step_decision = classify_step_placement(&flat_body, loop_var, &loop_increment);
        if let Some(reason) = step_decision.reject_reason {
            log_reject("generic_loop_v0", reason, handoff_tables::for_generic_loop);
            return reject_or_none(strict, reason.as_freeze_message());
        }
        let Some(step_placement) = step_decision.placement else {
            continue;
        };

        if planner_required
            && matches!(step_placement, StepPlacement::InBody(idx) if has_control_flow_after_step(&flat_body, idx))
        {
            continue;
        }

        let step_ok = match step_placement {
            StepPlacement::InBody(idx) => {
                validate_in_body_step(&flat_body, idx, loop_var, &loop_increment, strict)?
            }
            StepPlacement::InContinueIf(idx) => validate_continue_if_step(&flat_body, idx, strict)?,
            StepPlacement::InBreakElseIf(idx) => {
                validate_break_else_if_step(&flat_body, idx, strict)?
            }
            _ => true,
        };
        if !step_ok {
            continue;
        }

        if body_writes_non_loop_vars(&flat_body, loop_var, &loop_increment) {
            continue;
        }

        let v1_shape =
            detect_generic_loop_v1_shape(&flat_body, loop_var, &loop_increment, condition)?;
        if v1_shape_blocks_v0(v1_shape) {
            continue;
        }

        if check_body_generic_v1(&flat_body, loop_var, &loop_increment, condition, false)?.is_some()
        {
            continue;
        }

        matches.push(GenericLoopV0Facts {
            loop_var: loop_var.clone(),
            condition: condition.clone(),
            loop_increment,
            body: RecipeBody::new(flat_body.clone()),
        });
    }

    if matches.is_empty() {
        log_reject(
            "generic_loop_v0",
            RejectReason::NoValidLoopVarCandidates,
            handoff_tables::for_generic_loop,
        );
        return Ok(None);
    }

    if matches.len() > 1 {
        log_reject(
            "generic_loop_v0",
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
