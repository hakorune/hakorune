use crate::ast::{ASTNode, BinaryOperator};

use super::body_local_common::{extract_eq_whitespace, extract_substring_loop_slice};
use crate::mir::builder::control_flow::plan::loop_break::facts::body_local_facts::LoopBodyLocalShape;
use crate::mir::builder::control_flow::plan::loop_break::facts::body_local_facts_helpers::find_local_init_expr;

pub(super) fn try_match_trim_seg(
    break_condition: &ASTNode,
    body: &[ASTNode],
    break_idx: usize,
    loop_var: &str,
) -> Option<(String, LoopBodyLocalShape)> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Or,
        left,
        right,
        ..
    } = break_condition
    else {
        return None;
    };
    let left_var = extract_eq_whitespace(left.as_ref())?;
    let right_var = extract_eq_whitespace(right.as_ref())?;
    if left_var != right_var {
        return None;
    }
    let (seg_idx, seg_expr) = find_local_init_expr(body, &left_var)?;
    if seg_idx >= break_idx {
        return None;
    }
    let s_var = extract_substring_loop_slice(&seg_expr, loop_var)?;
    Some((
        left_var.clone(),
        LoopBodyLocalShape::TrimSeg {
            s_var,
            i_var: loop_var.to_string(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, LiteralValue, Span};

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

    fn lit_str(value: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(value.to_string()),
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

    fn eq(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn or(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Or,
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

    fn length_call(obj: &str) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(v(obj)),
            method: "length".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }
    }

    #[test]
    fn loop_break_body_local_facts_detect_trim_seg() {
        let condition = less(v("i"), length_call("s"));
        let body = vec![
            local("seg", substring("s", "i", 1)),
            ASTNode::If {
                condition: Box::new(or(eq(v("seg"), lit_str(" ")), eq(v("seg"), lit_str("\t")))),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            assign("i", add(v("i"), lit_int(1))),
        ];

        let facts =
            crate::mir::builder::control_flow::plan::loop_break::facts::body_local_facts_helpers::try_extract_loop_break_body_local_facts_inner(
                &condition,
                &body,
            )
            .expect("Ok");
        let facts = facts.expect("Some facts");
        assert_eq!(facts.loop_var, "i");
        assert_eq!(facts.body_local_var, "seg");
        assert!(facts.break_uses_body_local);
        match facts.shape {
            LoopBodyLocalShape::TrimSeg { s_var, i_var } => {
                assert_eq!(s_var, "s");
                assert_eq!(i_var, "i");
            }
            other => panic!("expected TrimSeg, got {:?}", other),
        }
    }

    #[test]
    fn loop_break_body_local_facts_none_when_substring_step_not_one() {
        let condition = less(v("i"), length_call("s"));
        let body = vec![
            local("seg", substring("s", "i", 2)),
            ASTNode::If {
                condition: Box::new(or(eq(v("seg"), lit_str(" ")), eq(v("seg"), lit_str("\t")))),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
        ];

        let facts =
            crate::mir::builder::control_flow::plan::loop_break::facts::body_local_facts_helpers::try_extract_loop_break_body_local_facts_inner(
                &condition,
                &body,
            )
            .expect("Ok");
        assert!(facts.is_none());
    }
}
