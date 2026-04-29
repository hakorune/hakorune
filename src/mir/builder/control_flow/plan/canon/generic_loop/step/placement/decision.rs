use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::generic_loop::step::placement::{
    collect_conditional_step_indices, collect_direct_step_indices,
};
use crate::mir::builder::control_flow::plan::facts::reject_reason::RejectReason;

use super::super::super::{StepPlacement, StepPlacementDecision};

pub(crate) fn classify_step_placement(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> StepPlacementDecision {
    let indices = collect_direct_step_indices(body, loop_var, loop_increment);
    if indices.is_empty() {
        return classify_conditional_only(body, loop_var, loop_increment);
    }
    classify_direct_indices(body.len(), &indices)
}

fn classify_conditional_only(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> StepPlacementDecision {
    let (continue_indices, break_else_indices) =
        collect_conditional_step_indices(body, loop_var, loop_increment);
    if continue_indices.is_empty() && break_else_indices.is_empty() {
        return StepPlacementDecision::default();
    }
    if continue_indices.len() + break_else_indices.len() > 1 {
        return StepPlacementDecision {
            placement: None,
            reject_reason: Some(RejectReason::MultipleConditionalStepAssignments),
        };
    }
    if let Some(idx) = continue_indices.first().copied() {
        return StepPlacementDecision {
            placement: Some(StepPlacement::InContinueIf(idx)),
            reject_reason: None,
        };
    }
    if let Some(idx) = break_else_indices.first().copied() {
        return StepPlacementDecision {
            placement: Some(StepPlacement::InBreakElseIf(idx)),
            reject_reason: None,
        };
    }
    StepPlacementDecision::default()
}

fn classify_direct_indices(body_len: usize, indices: &[usize]) -> StepPlacementDecision {
    if indices.len() > 1 {
        return StepPlacementDecision {
            placement: None,
            reject_reason: Some(RejectReason::MultipleStepAssignments),
        };
    }
    let idx = indices[0];
    if idx + 1 == body_len {
        StepPlacementDecision {
            placement: Some(StepPlacement::Last),
            reject_reason: None,
        }
    } else {
        StepPlacementDecision {
            placement: Some(StepPlacement::InBody(idx)),
            reject_reason: None,
        }
    }
}
