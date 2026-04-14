use super::loop_break_helpers::{extract_break_if_parts, lit_str, var};
use crate::mir::builder::control_flow::plan::loop_break::facts::LoopBreakFacts;
use crate::ast::ASTNode;
use crate::ast::{BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::extractors::common_helpers::is_true_literal;
use crate::mir::builder::control_flow::plan::LoopBreakStepPlacement;

pub(super) fn try_extract_loop_break_read_digits_subset(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<LoopBreakFacts> {
    if !is_true_literal(condition) {
        return None;
    }
    if body.len() != 3 {
        return None;
    }

    let (loop_var, haystack_var, ch_var, ch_expr) = match_local_substring_any(&body[0])?;
    if !match_break_if_empty(&body[1], &ch_var) {
        return None;
    }

    let (carrier_var, carrier_update_in_body, loop_increment) =
        match_digit_if_update_else_break(&body[2], &loop_var, &ch_var, &ch_expr)?;

    let loop_condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(var(&loop_var)),
        right: Box::new(ASTNode::MethodCall {
            object: Box::new(var(&haystack_var)),
            method: "length".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let break_condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Or,
        left: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(ch_expr.clone()),
            right: Box::new(lit_str("")),
            span: Span::unknown(),
        }),
        right: Box::new(build_non_digit_condition(&ch_expr)),
        span: Span::unknown(),
    };

    Some(LoopBreakFacts {
        loop_var,
        carrier_var,
        loop_condition,
        break_condition,
        carrier_update_in_break: None,
        carrier_update_in_body,
        loop_increment,
        step_placement: LoopBreakStepPlacement::Last,
    })
}

fn match_local_substring_any(stmt: &ASTNode) -> Option<(String, String, String, ASTNode)> {
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
    let ch_var = variables[0].clone();
    let init = initial_values[0].as_ref()?;

    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = init.as_ref()
    else {
        return None;
    };
    if method != "substring" || arguments.len() != 2 {
        return None;
    }

    let ASTNode::Variable {
        name: haystack_var, ..
    } = object.as_ref()
    else {
        return None;
    };
    let ASTNode::Variable { name: loop_var, .. } = &arguments[0] else {
        return None;
    };
    if !match_add_var_lit(&arguments[1], loop_var, 1) {
        return None;
    }

    Some((
        loop_var.clone(),
        haystack_var.clone(),
        ch_var,
        init.as_ref().clone(),
    ))
}

fn match_break_if_empty(stmt: &ASTNode, ch_var: &str) -> bool {
    let Some((cond, update_opt)) = extract_break_if_parts(stmt) else {
        return false;
    };
    if update_opt.is_some() {
        return false;
    }

    match_char_cmp(&cond, ch_var, BinaryOperator::Equal, "")
}

fn match_digit_if_update_else_break(
    stmt: &ASTNode,
    loop_var: &str,
    ch_var: &str,
    ch_expr: &ASTNode,
) -> Option<(String, ASTNode, ASTNode)> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return None;
    };
    let Some(else_body_vec) = else_body else {
        return None;
    };
    if else_body_vec.len() != 1 || !matches!(else_body_vec[0], ASTNode::Break { .. }) {
        return None;
    }
    if !match_digit_range_condition(condition, ch_var) {
        return None;
    }

    let mut carrier_var: Option<String> = None;
    let mut carrier_update_in_body: Option<ASTNode> = None;
    let mut loop_increment: Option<ASTNode> = None;

    for stmt in then_body {
        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    return None;
                };
                if name == loop_var {
                    if !match_add_var_lit(value, loop_var, 1) {
                        return None;
                    }
                    loop_increment = Some(value.as_ref().clone());
                } else {
                    let ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left,
                        right,
                        ..
                    } = value.as_ref()
                    else {
                        return None;
                    };
                    if !matches!(left.as_ref(), ASTNode::Variable { name: lhs, .. } if lhs == name)
                    {
                        return None;
                    }
                    if !matches!(right.as_ref(), ASTNode::Variable { name: rhs, .. } if rhs == ch_var)
                    {
                        return None;
                    }
                    carrier_var = Some(name.clone());
                    carrier_update_in_body = Some(ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(var(name)),
                        right: Box::new(ch_expr.clone()),
                        span: Span::unknown(),
                    });
                }
            }
            _ => return None,
        }
    }

    let carrier_var = carrier_var?;
    let carrier_update_in_body = carrier_update_in_body?;
    let loop_increment = loop_increment?;

    Some((carrier_var, carrier_update_in_body, loop_increment))
}

fn match_digit_range_condition(condition: &ASTNode, ch_var: &str) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::And,
        left,
        right,
        ..
    } = condition
    else {
        return false;
    };

    (match_char_cmp(left.as_ref(), ch_var, BinaryOperator::GreaterEqual, "0")
        && match_char_cmp(right.as_ref(), ch_var, BinaryOperator::LessEqual, "9"))
        || (match_char_cmp(left.as_ref(), ch_var, BinaryOperator::LessEqual, "9")
            && match_char_cmp(right.as_ref(), ch_var, BinaryOperator::GreaterEqual, "0"))
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

fn build_non_digit_condition(ch_expr: &ASTNode) -> ASTNode {
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

fn match_add_var_lit(expr: &ASTNode, var: &str, lit: i64) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = expr
    else {
        return false;
    };
    let (var_side, lit_side) = match (left.as_ref(), right.as_ref()) {
        (ASTNode::Variable { name, .. }, ASTNode::Literal { value, .. }) => (name, value),
        (ASTNode::Literal { value, .. }, ASTNode::Variable { name, .. }) => (name, value),
        _ => return false,
    };
    matches!(lit_side, LiteralValue::Integer(v) if *v == lit) && var_side == var
}
