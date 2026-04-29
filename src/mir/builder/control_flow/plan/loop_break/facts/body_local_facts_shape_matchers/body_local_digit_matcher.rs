use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

use super::body_local_common::{extract_indexof_expr, extract_substring_loop_slice};
use crate::mir::builder::control_flow::plan::loop_break::facts::body_local_facts::LoopBodyLocalShape;
use crate::mir::builder::control_flow::plan::loop_break::facts::body_local_facts_helpers::find_local_init_expr;

pub(super) fn try_match_digit_pos(
    break_condition: &ASTNode,
    body: &[ASTNode],
    break_idx: usize,
    loop_var: &str,
) -> Option<(String, LoopBodyLocalShape)> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left,
        right,
        ..
    } = break_condition
    else {
        return None;
    };
    let ASTNode::Variable {
        name: body_local_var,
        ..
    } = left.as_ref()
    else {
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

    let (digit_idx, digit_expr) = find_local_init_expr(body, body_local_var)?;
    if digit_idx >= break_idx {
        return None;
    }
    let (digits_var, ch_var) = extract_indexof_expr(&digit_expr)?;

    let (ch_idx, ch_expr) = find_local_init_expr(body, &ch_var)?;
    if ch_idx >= break_idx || ch_idx >= digit_idx {
        return None;
    }
    let _s_var = extract_substring_loop_slice(&ch_expr, loop_var)?;

    Some((
        body_local_var.clone(),
        LoopBodyLocalShape::DigitPos { digits_var, ch_var },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};

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

    fn local(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![Some(Box::new(value))],
            span: Span::unknown(),
        }
    }

    fn assign(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(name)),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    fn add(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn less(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn substring(obj: &str, i_var: &str, step: i64) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(v(obj)),
            method: "substring".to_string(),
            arguments: vec![v(i_var), add(v(i_var), lit_int(step))],
            span: Span::unknown(),
        }
    }

    fn index_of(obj: &str, arg: &str) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(v(obj)),
            method: "indexOf".to_string(),
            arguments: vec![v(arg)],
            span: Span::unknown(),
        }
    }

    fn length_call(obj: &str) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(v(obj)),
            method: "length".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }
    }

    #[test]
    fn loop_break_body_local_facts_detect_digit_pos() {
        let condition = less(v("p"), length_call("s"));
        let body = vec![
            local("ch", substring("s", "p", 1)),
            local("digit_pos", index_of("digits", "ch")),
            ASTNode::If {
                condition: Box::new(less(v("digit_pos"), lit_int(0))),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            assign("p", add(v("p"), lit_int(1))),
        ];

        let facts =
            crate::mir::builder::control_flow::plan::loop_break::facts::body_local_facts_helpers::try_extract_loop_break_body_local_facts_inner(
                &condition,
                &body,
            )
            .expect("Ok");
        let facts = facts.expect("Some facts");
        assert_eq!(facts.loop_var, "p");
        assert_eq!(facts.body_local_var, "digit_pos");
        match facts.shape {
            LoopBodyLocalShape::DigitPos { digits_var, ch_var } => {
                assert_eq!(digits_var, "digits");
                assert_eq!(ch_var, "ch");
            }
            other => panic!("expected DigitPos, got {:?}", other),
        }
    }
}
