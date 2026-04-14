use crate::ast::{ASTNode, BinaryOperator, Span};
use crate::mir::builder::control_flow::plan::loop_break::facts::helpers_local::{
    match_indexof_local, match_local_empty_string,
};
use crate::mir::builder::control_flow::plan::facts::loop_break_helpers_realworld::{
    match_break_if, match_loop_increment, match_seg_if_else,
};
use crate::mir::builder::control_flow::plan::loop_break::facts::helpers_common::{
    index_of_call, length_call, lit_int, lit_str, substring_call, var,
};
use crate::mir::builder::control_flow::plan::loop_break::facts::LoopBreakFacts;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    count_control_flow, is_true_literal, ControlFlowDetector,
};
use crate::mir::builder::control_flow::plan::LoopBreakStepPlacement;

pub(in crate::mir::builder::control_flow::plan) fn try_extract_loop_break_realworld_subset(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<LoopBreakFacts> {
    if !is_true_literal(condition) {
        return None;
    }

    let counts = count_control_flow(body, ControlFlowDetector::default());
    if counts.break_count != 1 || counts.continue_count > 0 || counts.return_count > 0 {
        return None;
    }

    if body.len() != 5 {
        return None;
    }

    let (j_var, haystack_var, sep_lit, loop_var) = match_indexof_local(&body[0])?;
    let seg_var = match_local_empty_string(&body[1])?;

    if !match_seg_if_else(&body[2], &j_var, &seg_var, &haystack_var, &loop_var)? {
        return None;
    }

    if !match_break_if(&body[3], &seg_var)? {
        return None;
    }

    let sep_len = sep_lit.len() as i64;
    if !match_loop_increment(&body[4], &loop_var, &j_var, sep_len)? {
        return None;
    }

    let loop_condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(var(&loop_var)),
        right: Box::new(length_call(&haystack_var)),
        span: Span::unknown(),
    };

    let index_expr = index_of_call(&haystack_var, &sep_lit, &loop_var);
    let break_condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(substring_call(
            &haystack_var,
            var(&loop_var),
            index_expr.clone(),
        )),
        right: Box::new(lit_str("")),
        span: Span::unknown(),
    };

    let loop_increment = ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(index_expr),
        right: Box::new(lit_int(sep_len)),
        span: Span::unknown(),
    };

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
