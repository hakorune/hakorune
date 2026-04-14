use super::loop_break_body_local_facts::{
    try_extract_loop_break_body_local_facts, LoopBodyLocalShape,
};
use super::loop_break_helpers::*;
use super::loop_break_helpers_local::find_local_init_expr;
use crate::mir::builder::control_flow::plan::loop_break::facts::LoopBreakFacts;
use crate::ast::{ASTNode, BinaryOperator, Span};
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    count_control_flow, ControlFlowDetector,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::LoopBreakStepPlacement;

pub(super) fn try_extract_loop_break_body_local_subset(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopBreakFacts>, Freeze> {
    let strict = crate::config::env::joinir_dev::strict_enabled();
    let strict_or_dev = strict || crate::config::env::joinir_dev_enabled();
    let Some(loop_var) = extract_loop_var_for_len_condition(condition) else {
        return Ok(None);
    };

    let counts = count_control_flow(body, ControlFlowDetector::default());
    if counts.break_count != 1 || counts.continue_count > 0 || counts.return_count > 0 {
        return Ok(None);
    }

    if body.len() != 3 && body.len() != 4 {
        return Ok(None);
    }

    let body_local = match try_extract_loop_break_body_local_facts(condition, body)? {
        Some(facts) => facts,
        None => return Ok(None),
    };
    if body_local.loop_var != loop_var {
        return Ok(None);
    }

    let (break_idx, _, carrier_update_in_break) = match find_break_if_parts(body) {
        Some(parts) => parts,
        None => return Ok(None),
    };
    if carrier_update_in_break.is_some() {
        return Ok(None);
    }
    if has_assignment_after(body, break_idx, &body_local.body_local_var) {
        if strict_or_dev {
            return Err(Freeze::contract(
                "loop_break body_local: read-only variable is reassigned in loop body",
            ));
        }
        return Ok(None);
    }

    let break_condition = match body_local.shape {
        LoopBodyLocalShape::TrimSeg { s_var, i_var } => {
            if i_var != loop_var {
                return Ok(None);
            }
            let seg_expr = substring_call(&s_var, var(&loop_var), add(var(&loop_var), lit_int(1)));
            let is_space = ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left: Box::new(seg_expr.clone()),
                right: Box::new(lit_str(" ")),
                span: Span::unknown(),
            };
            let is_tab = ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left: Box::new(seg_expr),
                right: Box::new(lit_str("\t")),
                span: Span::unknown(),
            };
            ASTNode::BinaryOp {
                operator: BinaryOperator::Or,
                left: Box::new(is_space),
                right: Box::new(is_tab),
                span: Span::unknown(),
            }
        }
        LoopBodyLocalShape::DigitPos { digits_var, ch_var } => {
            let ch_expr = match find_local_init_expr(body, &ch_var) {
                Some(expr) => expr,
                None => return Ok(None),
            };
            let index_expr = index_of_call_expr(&digits_var, ch_expr);
            ASTNode::BinaryOp {
                operator: BinaryOperator::Less,
                left: Box::new(index_expr),
                right: Box::new(lit_int(0)),
                span: Span::unknown(),
            }
        }
    };

    let loop_increment = match extract_loop_increment_at_end(body, &loop_var) {
        Some(inc) => inc,
        None => return Ok(None),
    };

    Ok(Some(LoopBreakFacts {
        loop_var: loop_var.clone(),
        carrier_var: loop_var,
        loop_condition: condition.clone(),
        break_condition,
        carrier_update_in_break: None,
        carrier_update_in_body: loop_increment.clone(),
        loop_increment,
        step_placement: LoopBreakStepPlacement::Last,
    }))
}
