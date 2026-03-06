//! Phase 29bg P1: IntToStrFacts (Facts SSOT)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct IntToStrFacts {
    pub loop_var: String,
    pub loop_condition: ASTNode,
    pub loop_increment: ASTNode,
    pub out_var: String,
    pub digits_var: String,
}

pub(in crate::mir::builder) fn try_extract_int_to_str_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<IntToStrFacts>, Freeze> {
    let Some(loop_var) = match_loop_condition(condition) else {
        return Ok(None);
    };

    let flat_body = flatten_scope_boxes(body);
    if flat_body.len() != 4 {
        return Ok(None);
    }

    let Some(digit_var) = match_digit_local(flat_body[0], &loop_var) else {
        return Ok(None);
    };

    let Some((digits_var, ch_var)) = match_char_local(flat_body[1], &digit_var) else {
        return Ok(None);
    };

    let Some(out_var) = match_out_update(flat_body[2], &ch_var) else {
        return Ok(None);
    };

    let Some(loop_increment) = match_loop_increment_stmt(flat_body[3], &loop_var) else {
        return Ok(None);
    };

    Ok(Some(IntToStrFacts {
        loop_var,
        loop_condition: condition.clone(),
        loop_increment,
        out_var,
        digits_var,
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
        operator: BinaryOperator::Greater,
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
    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(0),
            ..
        }
    ) {
        return None;
    }

    Some(loop_var.clone())
}

fn match_digit_local(stmt: &ASTNode, loop_var: &str) -> Option<String> {
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
    let digit_var = variables[0].clone();
    let Some(init) = &initial_values[0] else {
        return None;
    };

    let ASTNode::BinaryOp {
        operator: BinaryOperator::Modulo,
        left,
        right,
        ..
    } = init.as_ref()
    else {
        return None;
    };
    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
        return None;
    }
    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(10),
            ..
        }
    ) {
        return None;
    }

    Some(digit_var)
}

fn match_char_local(stmt: &ASTNode, digit_var: &str) -> Option<(String, String)> {
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
    let Some(init) = &initial_values[0] else {
        return None;
    };

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

    let ASTNode::Variable { name: digits_var, .. } = object.as_ref() else {
        return None;
    };
    if !matches!(&arguments[0], ASTNode::Variable { name, .. } if name == digit_var) {
        return None;
    }
    if !match_add_var_lit(&arguments[1], digit_var, 1) {
        return None;
    }

    Some((digits_var.clone(), ch_var))
}

fn match_out_update(stmt: &ASTNode, ch_var: &str) -> Option<String> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };

    let ASTNode::Variable { name: out_var, .. } = target.as_ref() else {
        return None;
    };

    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return None;
    };

    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == ch_var) {
        return None;
    }
    if !matches!(right.as_ref(), ASTNode::Variable { name, .. } if name == out_var) {
        return None;
    }

    Some(out_var.clone())
}

fn match_loop_increment_stmt(stmt: &ASTNode, loop_var: &str) -> Option<ASTNode> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    if !matches!(target.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
        return None;
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Divide,
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
            value: LiteralValue::Integer(10),
            ..
        }
    ) {
        return None;
    }

    Some(value.as_ref().clone())
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

    #[test]
    fn int_to_str_subset_matches_minimal_shape() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Greater,
            left: Box::new(v("v")),
            right: Box::new(lit_int(0)),
            span: Span::unknown(),
        };
        let digit = ASTNode::Local {
            variables: vec!["d".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Modulo,
                left: Box::new(v("v")),
                right: Box::new(lit_int(10)),
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        };
        let ch = ASTNode::Local {
            variables: vec!["ch".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::MethodCall {
                object: Box::new(v("digits")),
                method: "substring".to_string(),
                arguments: vec![
                    v("d"),
                    ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(v("d")),
                        right: Box::new(lit_int(1)),
                        span: Span::unknown(),
                    },
                ],
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        };
        let out_update = ASTNode::Assignment {
            target: Box::new(v("out")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("ch")),
                right: Box::new(v("out")),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let step = ASTNode::Assignment {
            target: Box::new(v("v")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Divide,
                left: Box::new(v("v")),
                right: Box::new(lit_int(10)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let facts = try_extract_int_to_str_facts(&condition, &[digit, ch, out_update, step])
            .expect("Ok")
            .expect("Some");
        assert_eq!(facts.loop_var, "v");
        assert_eq!(facts.out_var, "out");
        assert_eq!(facts.digits_var, "digits");
    }
}
