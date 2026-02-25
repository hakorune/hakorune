//! Phase 29ar P1: PatternIsIntegerFacts (Facts SSOT)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, UnaryOperator};
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct PatternIsIntegerFacts {
    pub loop_var: String,
    pub s_var: String,
    pub loop_condition: ASTNode,
    pub loop_increment: ASTNode,
    pub digit_call: ASTNode,
    pub return_zero_on_fail: bool,
}

pub(in crate::mir::builder) fn try_extract_pattern_is_integer_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<PatternIsIntegerFacts>, Freeze> {
    let Some((loop_var, bound)) = match_loop_condition(condition) else {
        return Ok(None);
    };

    let Some((s_var, digit_call, loop_increment, return_zero_on_fail)) =
        match_loop_body(body, &loop_var)
    else {
        return Ok(None);
    };

    if let LoopBound::LengthCall { s_var: bound_s_var } = bound {
        if bound_s_var != s_var {
            return Ok(None);
        }
    }

    Ok(Some(PatternIsIntegerFacts {
        loop_var,
        s_var,
        loop_condition: condition.clone(),
        loop_increment,
        digit_call,
        return_zero_on_fail,
    }))
}

enum LoopBound {
    LengthCall { s_var: String },
    LengthVar { len_var: String },
}

fn match_loop_condition(condition: &ASTNode) -> Option<(String, LoopBound)> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };

    let ASTNode::Variable { name: loop_var, .. } = left.as_ref() else {
        return None;
    };

    match right.as_ref() {
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => {
            if method != "length" || !arguments.is_empty() {
                return None;
            }
            let ASTNode::Variable { name: s_var, .. } = object.as_ref() else {
                return None;
            };
            Some((loop_var.clone(), LoopBound::LengthCall { s_var: s_var.clone() }))
        }
        ASTNode::Variable { name: len_var, .. } => {
            Some((loop_var.clone(), LoopBound::LengthVar { len_var: len_var.clone() }))
        }
        _ => None,
    }
}

fn match_loop_body(
    body: &[ASTNode],
    loop_var: &str,
    ) -> Option<(String, ASTNode, ASTNode, bool)> {
    match body {
        [digit_stmt, increment_stmt] => {
            let (s_var, digit_call, return_zero_on_fail) =
                match_digit_if_return_false(digit_stmt, loop_var)?;
            let loop_increment = match_loop_increment_stmt(increment_stmt, loop_var)?;
            Some((s_var, digit_call, loop_increment, return_zero_on_fail))
        }
        [local_stmt, digit_stmt, increment_stmt] => {
            let (s_var, substring_expr, ch_var) =
                match_local_substring(local_stmt, loop_var)?;
            let return_zero_on_fail =
                match_range_if_return_false(digit_stmt, &ch_var)?;
            let loop_increment = match_loop_increment_stmt(increment_stmt, loop_var)?;
            let digit_call = build_is_digit_call(substring_expr);
            Some((s_var, digit_call, loop_increment, return_zero_on_fail))
        }
        _ => None,
    }
}

fn match_local_substring(
    stmt: &ASTNode,
    loop_var: &str,
) -> Option<(String, ASTNode, String)> {
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
    let ch_var = variables.first()?.clone();
    let substring_expr = initial_values.first()?.as_ref()?.as_ref().clone();
    let s_var = match_substring_call(&substring_expr, loop_var)?;
    Some((s_var, substring_expr, ch_var))
}

fn match_range_if_return_false(stmt: &ASTNode, ch_var: &str) -> Option<bool> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return None;
    };
    if else_body.is_some() || then_body.len() != 1 {
        return None;
    }

    let return_zero_on_fail = match_return_false_or_zero(then_body.first()?)?;
    if !match_range_check_condition(condition, ch_var) {
        return None;
    }
    Some(return_zero_on_fail)
}

fn match_digit_if_return_false(
    stmt: &ASTNode,
    loop_var: &str,
) -> Option<(String, ASTNode, bool)> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return None;
    };
    if else_body.is_some() || then_body.len() != 1 {
        return None;
    }

    let return_zero_on_fail = match_return_false_or_zero(then_body.first()?)?;
    let (digit_call, s_var) = match_not_is_digit_condition(condition, loop_var)?;
    Some((s_var, digit_call, return_zero_on_fail))
}

fn match_not_is_digit_condition(
    condition: &ASTNode,
    loop_var: &str,
) -> Option<(ASTNode, String)> {
    let ASTNode::UnaryOp {
        operator: UnaryOperator::Not,
        operand,
        ..
    } = condition
    else {
        return None;
    };

    match_is_digit_call(operand, loop_var)
}

fn match_is_digit_call(expr: &ASTNode, loop_var: &str) -> Option<(ASTNode, String)> {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr
    else {
        return None;
    };
    if method != "is_digit" || arguments.len() != 1 {
        return None;
    }
    if !matches!(object.as_ref(), ASTNode::This { .. } | ASTNode::Me { .. }) {
        return None;
    }

    let arg = arguments.first()?;
    let s_var = match_substring_call(arg, loop_var)?;

    Some((expr.clone(), s_var))
}

fn match_substring_call(expr: &ASTNode, loop_var: &str) -> Option<String> {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr
    else {
        return None;
    };
    if method != "substring" || arguments.len() != 2 {
        return None;
    }

    let ASTNode::Variable { name, .. } = object.as_ref() else {
        return None;
    };

    if !matches!(&arguments[0], ASTNode::Variable { name, .. } if name == loop_var) {
        return None;
    }

    let ok = matches!(
        &arguments[1],
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left,
            right,
            ..
        }
            if matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var)
                && matches!(
                    right.as_ref(),
                    ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        ..
                    }
                )
    )
    ;

    if !ok {
        return None;
    }

    Some(name.clone())
}

fn match_return_false_or_zero(stmt: &ASTNode) -> Option<bool> {
    let ASTNode::Return { value: Some(ret), .. } = stmt else {
        return None;
    };
    match ret.as_ref() {
        ASTNode::Literal {
            value: LiteralValue::Bool(false),
            ..
        } => Some(false),
        ASTNode::Literal {
            value: LiteralValue::Integer(0),
            ..
        } => Some(true),
        _ => None,
    }
}

fn match_range_check_condition(condition: &ASTNode, ch_var: &str) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Or,
        left,
        right,
        ..
    } = condition
    else {
        return false;
    };

    (match_range_compare(left, ch_var, RangeSide::Low)
        && match_range_compare(right, ch_var, RangeSide::High))
        || (match_range_compare(left, ch_var, RangeSide::High)
            && match_range_compare(right, ch_var, RangeSide::Low))
}

enum RangeSide {
    Low,
    High,
}

fn match_range_compare(expr: &ASTNode, ch_var: &str, side: RangeSide) -> bool {
    let (op, lit) = match side {
        RangeSide::Low => (BinaryOperator::Less, "0"),
        RangeSide::High => (BinaryOperator::Greater, "9"),
    };

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
    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == ch_var) {
        return false;
    }

    matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::String(s),
            ..
        } if s == lit
    )
}

fn build_is_digit_call(substring_expr: ASTNode) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(ASTNode::This {
            span: crate::ast::Span::unknown(),
        }),
        method: "is_digit".to_string(),
        arguments: vec![substring_expr],
        span: crate::ast::Span::unknown(),
    }
}

fn match_loop_increment_stmt(stmt: &ASTNode, loop_var: &str) -> Option<ASTNode> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    if !matches!(target.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
        return None;
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return None;
    };
    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
        return None;
    }
    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(1),
            ..
        }
    ) {
        return None;
    }
    Some(value.as_ref().clone())
}
