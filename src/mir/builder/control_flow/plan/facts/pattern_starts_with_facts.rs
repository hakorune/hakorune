//! Phase 29bg P1: PatternStartsWithFacts (Facts SSOT)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct PatternStartsWithFacts {
    pub loop_var: String,
    pub loop_condition: ASTNode,
    pub loop_increment: ASTNode,
    pub mismatch_condition: ASTNode,
}

pub(in crate::mir::builder) fn try_extract_pattern_starts_with_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<PatternStartsWithFacts>, Freeze> {
    let Some(loop_var) = match_loop_condition(condition) else {
        return Ok(None);
    };

    let flat_body = flatten_scope_boxes(body);
    let mut mismatch_condition: Option<ASTNode> = None;
    let mut loop_increment: Option<ASTNode> = None;

    for stmt in &flat_body {
        if mismatch_condition.is_none() {
            if let Some(cond) = match_mismatch_if_return_zero(stmt, &loop_var) {
                mismatch_condition = Some(cond);
                continue;
            }
        }
        if loop_increment.is_none() {
            if let Some(inc) = match_loop_increment_stmt(stmt, &loop_var) {
                loop_increment = Some(inc);
            }
        }
    }

    let Some(mismatch_condition) = mismatch_condition else {
        return Ok(None);
    };
    let Some(loop_increment) = loop_increment else {
        return Ok(None);
    };

    Ok(Some(PatternStartsWithFacts {
        loop_var,
        loop_condition: condition.clone(),
        loop_increment,
        mismatch_condition,
    }))
}

fn flatten_scope_boxes<'a>(body: &'a [ASTNode]) -> Vec<&'a ASTNode> {
    let mut out = Vec::new();
    fn push_node<'a>(node: &'a ASTNode, out: &mut Vec<&'a ASTNode>) {
        match node {
            ASTNode::ScopeBox { body, .. } => {
                for inner in body {
                    push_node(inner, out);
                }
            }
            _ => out.push(node),
        }
    }
    for stmt in body {
        push_node(stmt, &mut out);
    }
    out
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

    let ASTNode::Variable { name: loop_var, .. } = left.as_ref() else {
        return None;
    };

    if !matches!(right.as_ref(), ASTNode::Variable { .. }) {
        return None;
    }

    Some(loop_var.clone())
}

fn match_mismatch_if_return_zero(stmt: &ASTNode, loop_var: &str) -> Option<ASTNode> {
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

    match then_body.first()? {
        ASTNode::Return { value: Some(ret), .. } => {
            if !matches!(
                ret.as_ref(),
                ASTNode::Literal {
                    value: LiteralValue::Integer(0),
                    ..
                }
            ) {
                return None;
            }
        }
        _ => return None,
    }

    if !match_starts_with_mismatch_condition(condition, loop_var) {
        return None;
    }

    Some((**condition).clone())
}

fn match_starts_with_mismatch_condition(condition: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::NotEqual,
        left,
        right,
        ..
    } = condition
    else {
        return false;
    };

    let Some((src_var, idx_var)) = match_left_substring(left, loop_var) else {
        return false;
    };
    let Some(pat_var) = match_right_substring(right, loop_var) else {
        return false;
    };

    if src_var == pat_var {
        return false;
    }
    if idx_var == loop_var {
        return false;
    }

    true
}

fn match_left_substring(expr: &ASTNode, loop_var: &str) -> Option<(String, String)> {
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

    let ASTNode::Variable { name: src_var, .. } = object.as_ref() else {
        return None;
    };

    let start = &arguments[0];
    let end = &arguments[1];
    let (a, b) = match_add_vars(start)?;
    if a != loop_var && b != loop_var {
        return None;
    }
    let idx_var = if a == loop_var { b } else { a };

    if !match_add_vars_plus_one(end, loop_var, idx_var) {
        return None;
    }

    Some((src_var.clone(), idx_var.to_string()))
}

fn match_right_substring(expr: &ASTNode, loop_var: &str) -> Option<String> {
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

    let ASTNode::Variable { name: pat_var, .. } = object.as_ref() else {
        return None;
    };

    if !matches!(&arguments[0], ASTNode::Variable { name, .. } if name == loop_var) {
        return None;
    }
    if !match_add_var_lit(&arguments[1], loop_var, 1) {
        return None;
    }

    Some(pat_var.clone())
}

fn match_add_vars(expr: &ASTNode) -> Option<(&str, &str)> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = expr
    else {
        return None;
    };
    let ASTNode::Variable { name: left_name, .. } = left.as_ref() else {
        return None;
    };
    let ASTNode::Variable { name: right_name, .. } = right.as_ref() else {
        return None;
    };
    Some((left_name.as_str(), right_name.as_str()))
}

fn match_add_vars_plus_one(expr: &ASTNode, var_a: &str, var_b: &str) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = expr
    else {
        return false;
    };
    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(1),
            ..
        }
    ) {
        return false;
    }
    match_add_vars(left)
        .map(|(x, y)| (x == var_a && y == var_b) || (x == var_b && y == var_a))
        .unwrap_or(false)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    fn v(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit_int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn substring_call(obj: &str, start: ASTNode, end: ASTNode) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(v(obj)),
            method: "substring".to_string(),
            arguments: vec![start, end],
            span: Span::unknown(),
        }
    }

    #[test]
    fn starts_with_subset_matches_minimal_shape() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("k")),
            right: Box::new(v("m")),
            span: Span::unknown(),
        };

        let start = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(v("k")),
            span: Span::unknown(),
        };
        let end = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(start.clone()),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };
        let left = substring_call("src", start, end);
        let right = substring_call(
            "pat",
            v("k"),
            ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("k")),
                right: Box::new(lit_int(1)),
                span: Span::unknown(),
            },
        );

        let if_stmt = ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::NotEqual,
                left: Box::new(left),
                right: Box::new(right),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(lit_int(0))),
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        };
        let step = ASTNode::Assignment {
            target: Box::new(v("k")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("k")),
                right: Box::new(lit_int(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let facts = try_extract_pattern_starts_with_facts(&condition, &[if_stmt, step])
            .expect("Ok")
            .expect("Some");
        assert_eq!(facts.loop_var, "k");
    }
}
