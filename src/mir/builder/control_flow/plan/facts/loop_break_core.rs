//! Phase 29ai P11: Core loop_break extraction logic.
//!
//! This module contains the main entry function and fallback extraction logic.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    count_control_flow, extract_loop_increment_plan, ControlFlowDetector,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::LoopBreakStepPlacement;

use super::loop_break_helpers::{
    extract_break_if_parts, extract_loop_var_for_len_condition, extract_loop_var_for_plan_subset,
    has_continue_statement, has_return_statement,
};
use crate::mir::builder::control_flow::plan::loop_break::facts::LoopBreakFacts;

// Import subset extractors
use super::loop_break_body_local_subset::try_extract_loop_break_body_local_subset;
use super::loop_break_parse_integer::try_extract_loop_break_parse_integer_subset;
use super::loop_break_read_digits::try_extract_loop_break_read_digits_subset;
use crate::mir::builder::control_flow::plan::loop_break::facts::realworld::try_extract_loop_break_realworld_subset;
use crate::mir::builder::control_flow::plan::loop_break::facts::step_before_break::try_extract_loop_break_step_before_break_subset;
use crate::mir::builder::control_flow::plan::loop_break::facts::trim_whitespace::try_extract_loop_break_trim_whitespace_subset;

/// Main entry point for loop_break facts extraction.
///
/// Tries each subset extractor in order, then falls back to generic extraction.
pub(in crate::mir::builder) fn try_extract_loop_break_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopBreakFacts>, Freeze> {
    if let Some(read_digits) = try_extract_loop_break_read_digits_subset(condition, body) {
        return Ok(Some(read_digits));
    }

    if let Some(parse_int) = try_extract_loop_break_parse_integer_subset(condition, body) {
        return Ok(Some(parse_int));
    }

    if let Some(trim_whitespace) = try_extract_loop_break_trim_whitespace_subset(condition, body) {
        return Ok(Some(trim_whitespace));
    }

    if let Some(realworld) = try_extract_loop_break_realworld_subset(condition, body) {
        return Ok(Some(realworld));
    }

    if let Some(body_local) = try_extract_loop_break_body_local_subset(condition, body)? {
        return Ok(Some(body_local));
    }

    if let Some(step_first) = try_extract_loop_break_step_before_break_subset(condition, body) {
        return Ok(Some(step_first));
    }

    // Fallback: generic extraction
    try_extract_generic(condition, body)
}

/// Generic extraction for simple 3-statement loops with break.
fn try_extract_generic(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopBreakFacts>, Freeze> {
    let Some(loop_var) = extract_loop_var_for_plan_subset(condition)
        .or_else(|| extract_loop_var_for_len_condition(condition))
    else {
        return Ok(None);
    };

    let break_count = count_control_flow(body, ControlFlowDetector::default()).break_count;
    if break_count != 1 {
        return Ok(None);
    }
    if has_continue_statement(body) {
        return Ok(None);
    }
    if has_return_statement(body) {
        return Ok(None);
    }

    if body.len() != 3 {
        return Ok(None);
    }

    let (break_idx, break_condition, carrier_update_in_break) =
        if let Some(parts) = extract_break_if_parts(&body[0]) {
            (0, parts.0, parts.1)
        } else if matches!(body.get(0), Some(ASTNode::Local { .. })) {
            let Some(parts) = body.get(1).and_then(extract_break_if_parts) else {
                return Ok(None);
            };
            (1, parts.0, parts.1)
        } else {
            return Ok(None);
        };

    let carrier_stmt = match body.get(break_idx + 1) {
        Some(stmt) => stmt,
        None => return Ok(None),
    };

    let (carrier_var, carrier_update_in_body) = match carrier_stmt {
        ASTNode::Assignment { target, value, .. } => {
            let carrier_name = match target.as_ref() {
                ASTNode::Variable { name, .. } => name.clone(),
                _ => return Ok(None),
            };
            (carrier_name, value.as_ref().clone())
        }
        _ => return Ok(None),
    };

    let loop_increment = match extract_loop_increment_plan(body, &loop_var) {
        Ok(Some(inc)) => inc,
        _ => return Ok(None),
    };

    Ok(Some(LoopBreakFacts {
        loop_var,
        carrier_var,
        loop_condition: condition.clone(),
        break_condition,
        carrier_update_in_break,
        carrier_update_in_body,
        loop_increment,
        step_placement: LoopBreakStepPlacement::Last,
    }))
}
