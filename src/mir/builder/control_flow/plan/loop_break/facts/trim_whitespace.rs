use super::trim_whitespace_helpers::{
    build_not_whitespace_condition, extract_trim_break_condition, extract_trim_loop_increment,
    extract_trim_loop_var, match_trim_header_condition,
};
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    count_control_flow, ControlFlowDetector,
};
use crate::mir::builder::control_flow::plan::loop_break::facts::LoopBreakFacts;
use crate::mir::builder::control_flow::plan::LoopBreakStepPlacement;

pub(in crate::mir::builder::control_flow::plan) fn try_extract_loop_break_trim_whitespace_subset(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<LoopBreakFacts> {
    if let Some(facts) = try_extract_trim_header_condition_subset(condition, body) {
        return Some(facts);
    }

    let loop_var = extract_trim_loop_var(condition)?;

    let counts = count_control_flow(body, ControlFlowDetector::default());
    if counts.break_count != 1 || counts.continue_count > 0 || counts.return_count > 0 {
        return None;
    }

    if body.len() != 2 {
        return None;
    }

    let break_condition = extract_trim_break_condition(&body[0], &loop_var)?;
    let loop_increment = extract_trim_loop_increment(&body[1], &loop_var)?;

    Some(LoopBreakFacts {
        loop_var: loop_var.clone(),
        carrier_var: loop_var,
        loop_condition: condition.clone(),
        break_condition,
        carrier_update_in_break: None,
        carrier_update_in_body: loop_increment.clone(),
        loop_increment,
        step_placement: LoopBreakStepPlacement::Last,
    })
}

fn try_extract_trim_header_condition_subset(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<LoopBreakFacts> {
    let (loop_var, loop_condition, haystack_var, direction, delimiters) =
        match_trim_header_condition(condition)?;

    let counts = count_control_flow(body, ControlFlowDetector::default());
    if counts.break_count > 0 || counts.continue_count > 0 || counts.return_count > 0 {
        return None;
    }

    if body.len() != 1 {
        return None;
    }

    let loop_increment = extract_trim_loop_increment(&body[0], &loop_var)?;
    let break_condition =
        build_not_whitespace_condition(&loop_var, &haystack_var, direction, &delimiters);

    Some(LoopBreakFacts {
        loop_var: loop_var.clone(),
        carrier_var: loop_var,
        loop_condition,
        break_condition,
        carrier_update_in_break: None,
        carrier_update_in_body: loop_increment.clone(),
        loop_increment,
        step_placement: LoopBreakStepPlacement::Last,
    })
}
