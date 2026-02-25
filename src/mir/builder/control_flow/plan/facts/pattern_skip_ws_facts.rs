//! Phase 29bg P1: PatternSkipWsFacts (Facts SSOT)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct PatternSkipWsFacts {
    pub loop_var: String,
    pub loop_condition: ASTNode,
    pub loop_increment: ASTNode,
}

pub(in crate::mir::builder) fn try_extract_pattern_skip_ws_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<PatternSkipWsFacts>, Freeze> {
    let Some(loop_var) = match_loop_condition(condition) else {
        return Ok(None);
    };
    if body.len() != 2 {
        return Ok(None);
    }

    let Some((space_call, loop_increment)) =
        match_if_space_then_continue(&body[0], &loop_var)
    else {
        return Ok(None);
    };

    if !matches!(body[1], ASTNode::Break { .. }) {
        return Ok(None);
    }

    let loop_condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(space_call),
        right: Box::new(lit_bool(true)),
        span: Span::unknown(),
    };

    Ok(Some(PatternSkipWsFacts {
        loop_var,
        loop_condition,
        loop_increment,
    }))
}

fn match_loop_condition(condition: &ASTNode) -> Option<String> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
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

fn match_if_space_then_continue(
    stmt: &ASTNode,
    loop_var: &str,
) -> Option<(ASTNode, ASTNode)> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return None;
    };
    if else_body.is_some() {
        return None;
    }

    let space_call = match_is_space_call(condition, loop_var)?;

    let mut loop_increment: Option<ASTNode> = None;
    let mut has_continue = false;
    for stmt in then_body {
        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    return None;
                };
                if name != loop_var {
                    return None;
                }
                if !match_add_var_lit(value, loop_var, 1) {
                    return None;
                }
                loop_increment = Some(value.as_ref().clone());
            }
            ASTNode::Continue { .. } => {
                has_continue = true;
            }
            _ => return None,
        }
    }

    let loop_increment = loop_increment?;
    if !has_continue {
        return None;
    }

    Some((space_call, loop_increment))
}

fn match_is_space_call(condition: &ASTNode, loop_var: &str) -> Option<ASTNode> {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = condition
    else {
        return None;
    };
    if method != "is_space" || arguments.len() != 1 {
        return None;
    }
    if !matches!(object.as_ref(), ASTNode::This { .. } | ASTNode::Me { .. }) {
        return None;
    }
    if !match_substring_call(&arguments[0], loop_var) {
        return None;
    }
    Some(condition.clone())
}

fn match_substring_call(expr: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = expr
    else {
        return false;
    };
    if method != "substring" || arguments.len() != 2 {
        return false;
    }
    if !matches!(object.as_ref(), ASTNode::Variable { .. }) {
        return false;
    }
    if !matches!(&arguments[0], ASTNode::Variable { name, .. } if name == loop_var) {
        return false;
    }
    match_add_var_lit(&arguments[1], loop_var, 1)
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

fn lit_bool(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}
