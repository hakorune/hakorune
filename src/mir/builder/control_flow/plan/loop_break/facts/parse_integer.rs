use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    count_control_flow, ControlFlowDetector,
};
use crate::mir::builder::control_flow::plan::loop_break::facts::helpers_condition::{
    match_acc_update_mul10_plus_d, match_break_if_less_than_zero,
};
use crate::mir::builder::control_flow::plan::loop_break::facts::helpers_local::{
    match_local_substring_char, match_local_this_index_of,
};
use crate::mir::builder::control_flow::plan::loop_break::facts::helpers_loop::{
    extract_loop_increment_at_end, extract_loop_var_for_len_condition,
};
use crate::mir::builder::control_flow::plan::loop_break::facts::helpers_break_if::extract_break_if_parts;
use crate::mir::builder::control_flow::plan::loop_break::facts::helpers_common::{lit_int, lit_str, var};
use crate::mir::builder::control_flow::plan::loop_break::facts::LoopBreakFacts;
use crate::mir::builder::control_flow::plan::LoopBreakStepPlacement;

pub(in crate::mir::builder::control_flow::plan) fn try_extract_loop_break_parse_integer_subset(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<LoopBreakFacts> {
    if let Some(range_based) = try_extract_loop_break_parse_integer_range_subset(condition, body) {
        return Some(range_based);
    }

    let loop_var = extract_loop_var_for_len_condition(condition)
        .or_else(|| extract_loop_var_for_cached_len_condition(condition))?;

    let counts = count_control_flow(body, ControlFlowDetector::default());
    if counts.break_count != 1 || counts.continue_count > 0 || counts.return_count > 0 {
        return None;
    }

    if body.len() != 5 {
        return None;
    }

    // local ch = s.substring(i, i + 1)
    let (ch_var, haystack_var, ch_expr) = match_local_substring_char(&body[0], &loop_var)?;

    // local d = this.index_of(digits, ch)
    let (digits_var, _d_var) = match_local_this_index_of(&body[1], &ch_var)?;

    // if d < 0 { break }
    let break_var = match_break_if_less_than_zero(&body[2])?;

    // acc = acc * 10 + d
    let carrier_var = match_acc_update_mul10_plus_d(&body[3], &break_var)?;

    // i = i + 1
    let loop_increment = extract_loop_increment_at_end(body, &loop_var)?;

    // Rebuild break_condition and carrier_update_in_body without relying on the local `d`.
    // This avoids requiring a local binding in PlanNormalizer while keeping semantics.
    let index_expr = ASTNode::MethodCall {
        object: Box::new(ASTNode::This {
            span: Span::unknown(),
        }),
        method: "index_of".to_string(),
        arguments: vec![var(&digits_var), ch_expr],
        span: Span::unknown(),
    };
    let break_condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(index_expr.clone()),
        right: Box::new(lit_int(0)),
        span: Span::unknown(),
    };
    let carrier_update_in_body = ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Multiply,
            left: Box::new(var(&carrier_var)),
            right: Box::new(lit_int(10)),
            span: Span::unknown(),
        }),
        right: Box::new(index_expr),
        span: Span::unknown(),
    };

    // Also confirm the substring source matches the haystack var and loop var.
    // (We don't need to store them, but the match helps prevent accidental overlap.)
    let _ = &haystack_var;

    Some(LoopBreakFacts {
        loop_var,
        carrier_var,
        loop_condition: condition.clone(),
        break_condition,
        carrier_update_in_break: None,
        carrier_update_in_body,
        loop_increment,
        step_placement: LoopBreakStepPlacement::Last,
    })
}

fn try_extract_loop_break_parse_integer_range_subset(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<LoopBreakFacts> {
    let loop_var = extract_loop_var_for_len_condition(condition)
        .or_else(|| extract_loop_var_for_cached_len_condition(condition))?;

    let counts = count_control_flow(body, ControlFlowDetector::default());
    if counts.break_count != 2 || counts.continue_count > 0 || counts.return_count > 0 {
        return None;
    }

    if body.len() != 7 {
        return None;
    }

    let (ch_var, _haystack_var, ch_expr) = match_local_substring_char(&body[0], &loop_var)?;
    if !match_break_if_non_digit(&body[1], &ch_var) {
        return None;
    }
    let range_break_cond = build_range_break_condition(&ch_expr);
    let (digits_var, digits_lit) = match_local_digits_literal(&body[2])?;
    let dpos_var = match_local_indexof(&body[3], &digits_var, &ch_var)?;
    let break_var = match_break_if_less_than_zero(&body[4])?;
    if break_var != dpos_var {
        return None;
    }

    let carrier_var = match_acc_update_mul10_plus_d(&body[5], &break_var)?;
    let loop_increment = extract_loop_increment_at_end(body, &loop_var)?;

    let index_expr = ASTNode::MethodCall {
        object: Box::new(ASTNode::This {
            span: Span::unknown(),
        }),
        method: "index_of".to_string(),
        arguments: vec![lit_str(&digits_lit), lit_int(0), ch_expr],
        span: Span::unknown(),
    };
    let index_break = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(index_expr.clone()),
        right: Box::new(lit_int(0)),
        span: Span::unknown(),
    };
    let break_condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Or,
        left: Box::new(range_break_cond),
        right: Box::new(index_break),
        span: Span::unknown(),
    };
    let carrier_update_in_body = ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Multiply,
            left: Box::new(var(&carrier_var)),
            right: Box::new(lit_int(10)),
            span: Span::unknown(),
        }),
        right: Box::new(index_expr),
        span: Span::unknown(),
    };

    Some(LoopBreakFacts {
        loop_var,
        carrier_var,
        loop_condition: condition.clone(),
        break_condition,
        carrier_update_in_break: None,
        carrier_update_in_body,
        loop_increment,
        step_placement: LoopBreakStepPlacement::Last,
    })
}

fn extract_loop_var_for_cached_len_condition(condition: &ASTNode) -> Option<String> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less | BinaryOperator::LessEqual,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };
    let ASTNode::Variable { name, .. } = left.as_ref() else {
        return None;
    };
    if !matches!(right.as_ref(), ASTNode::Variable { .. }) {
        return None;
    }
    Some(name.clone())
}

fn match_break_if_non_digit(stmt: &ASTNode, ch_var: &str) -> bool {
    let Some((cond, update_opt)) = extract_break_if_parts(stmt) else {
        return false;
    };
    if update_opt.is_some() {
        return false;
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Or,
        left,
        right,
        ..
    } = &cond
    else {
        return false;
    };

    if match_char_cmp(left.as_ref(), ch_var, BinaryOperator::Less, "0")
        && match_char_cmp(right.as_ref(), ch_var, BinaryOperator::Greater, "9")
    {
        return true;
    }
    if match_char_cmp(left.as_ref(), ch_var, BinaryOperator::Greater, "9")
        && match_char_cmp(right.as_ref(), ch_var, BinaryOperator::Less, "0")
    {
        return true;
    }
    false
}

fn build_range_break_condition(ch_expr: &ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Or,
        left: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ch_expr.clone()),
            right: Box::new(lit_str("0")),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Greater,
            left: Box::new(ch_expr.clone()),
            right: Box::new(lit_str("9")),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }
}

fn match_char_cmp(expr: &ASTNode, ch_var: &str, op: BinaryOperator, lit: &str) -> bool {
    let ASTNode::BinaryOp {
        operator,
        left,
        right,
        ..
    } = expr
    else {
        return false;
    };
    if *operator != op {
        return false;
    }
    matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == ch_var)
        && matches!(
            right.as_ref(),
            ASTNode::Literal {
                value: LiteralValue::String(s),
                ..
            } if s == lit
        )
}

fn match_local_digits_literal(stmt: &ASTNode) -> Option<(String, String)> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return None;
    };
    if variables.len() != 1 || initial_values.len() != 1 {
        return None;
    }
    let name = variables[0].clone();
    let Some(expr) = initial_values[0].as_ref() else {
        return None;
    };
    let ASTNode::Literal {
        value: LiteralValue::String(lit),
        ..
    } = expr.as_ref()
    else {
        return None;
    };
    Some((name, lit.clone()))
}

fn match_local_indexof(stmt: &ASTNode, digits_var: &str, ch_var: &str) -> Option<String> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return None;
    };
    if variables.len() != 1 || initial_values.len() != 1 {
        return None;
    }
    let dpos_var = variables[0].clone();
    let Some(expr) = initial_values[0].as_ref() else {
        return None;
    };
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr.as_ref()
    else {
        return None;
    };
    if method != "indexOf" || arguments.len() != 1 {
        return None;
    }
    let ASTNode::Variable { name: obj, .. } = object.as_ref() else {
        return None;
    };
    if obj != digits_var {
        return None;
    }
    if !matches!(&arguments[0], ASTNode::Variable { name, .. } if name == ch_var) {
        return None;
    }
    Some(dpos_var)
}
