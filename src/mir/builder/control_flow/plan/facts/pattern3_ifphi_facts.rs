//! Phase 29aj P3: if_phi_join facts (legacy type: Pattern3IfPhiFacts, SSOT)

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    extract_loop_increment_plan, find_if_else_statement,
};
use crate::mir::builder::control_flow::plan::extractors::pattern3::extract_loop_with_if_phi_parts;
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern3IfPhiFacts {
    pub loop_var: String,
    pub carrier_var: String,
    pub condition: ASTNode,
    pub if_condition: ASTNode,
    pub then_update: ASTNode,
    pub else_update: ASTNode,
    pub loop_increment: ASTNode,
}

pub(in crate::mir::builder) fn try_extract_pattern3_ifphi_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<Pattern3IfPhiFacts>, Freeze> {
    let parts = match extract_loop_with_if_phi_parts(condition, body) {
        Ok(Some(parts)) => parts,
        Ok(None) => return Ok(None),
        Err(_) => return Ok(None),
    };

    let if_stmt = match find_if_else_statement(body) {
        Some(stmt) => stmt,
        None => return Ok(None),
    };

    let (if_condition, then_update, else_update) = match if_stmt {
        ASTNode::If {
            condition: if_cond,
            then_body,
            else_body: Some(else_body),
            ..
        } => {
            let then_update = match extract_single_update(then_body, &parts.merged_var) {
                Some(update) => update,
                None => return Ok(None),
            };
            let else_update = match extract_single_update(else_body, &parts.merged_var) {
                Some(update) => update,
                None => return Ok(None),
            };
            (if_cond.as_ref().clone(), then_update, else_update)
        }
        _ => return Ok(None),
    };

    let loop_increment = match extract_loop_increment_plan(body, &parts.loop_var) {
        Ok(Some(inc)) => inc,
        _ => return Ok(None),
    };

    Ok(Some(Pattern3IfPhiFacts {
        loop_var: parts.loop_var,
        carrier_var: parts.merged_var,
        condition: condition.clone(),
        if_condition,
        then_update,
        else_update,
        loop_increment,
    }))
}

fn extract_single_update(body: &[ASTNode], carrier_var: &str) -> Option<ASTNode> {
    for stmt in body {
        if let ASTNode::Assignment { target, value, .. } = stmt {
            if let ASTNode::Variable { name, .. } = target.as_ref() {
                if name == carrier_var {
                    return Some(value.as_ref().clone());
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};

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

    fn assign(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(name)),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    fn if_else(cond: ASTNode, then_body: Vec<ASTNode>, else_body: Vec<ASTNode>) -> ASTNode {
        ASTNode::If {
            condition: Box::new(cond),
            then_body,
            else_body: Some(else_body),
            span: Span::unknown(),
        }
    }

    fn loop_condition() -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(lit_int(3)),
            span: Span::unknown(),
        }
    }

    fn loop_increment() -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        }
    }

    #[test]
    fn facts_extracts_pattern3_ifphi_success() {
        let if_stmt = if_else(
            ASTNode::BinaryOp {
                operator: BinaryOperator::Greater,
                left: Box::new(v("i")),
                right: Box::new(lit_int(0)),
                span: Span::unknown(),
            },
            vec![assign(
                "sum",
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(v("sum")),
                    right: Box::new(lit_int(1)),
                    span: Span::unknown(),
                },
            )],
            vec![assign(
                "sum",
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(v("sum")),
                    right: Box::new(lit_int(0)),
                    span: Span::unknown(),
                },
            )],
        );

        let body = vec![if_stmt, assign("i", loop_increment())];
        let facts = try_extract_pattern3_ifphi_facts(&loop_condition(), &body).expect("Ok");
        let facts = facts.expect("Some");
        assert_eq!(facts.loop_var, "i");
        assert_eq!(facts.carrier_var, "sum");
    }

    #[test]
    fn facts_rejects_if_without_else() {
        let if_stmt = ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Greater,
                left: Box::new(v("i")),
                right: Box::new(lit_int(0)),
                span: Span::unknown(),
            }),
            then_body: vec![assign(
                "sum",
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(v("sum")),
                    right: Box::new(lit_int(1)),
                    span: Span::unknown(),
                },
            )],
            else_body: None,
            span: Span::unknown(),
        };

        let body = vec![if_stmt, assign("i", loop_increment())];
        let facts = try_extract_pattern3_ifphi_facts(&loop_condition(), &body).expect("Ok");
        assert!(facts.is_none());
    }

    #[test]
    fn facts_rejects_mismatched_carrier_vars() {
        let if_stmt = if_else(
            ASTNode::BinaryOp {
                operator: BinaryOperator::Greater,
                left: Box::new(v("i")),
                right: Box::new(lit_int(0)),
                span: Span::unknown(),
            },
            vec![assign(
                "sum",
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(v("sum")),
                    right: Box::new(lit_int(1)),
                    span: Span::unknown(),
                },
            )],
            vec![assign(
                "acc",
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(v("acc")),
                    right: Box::new(lit_int(1)),
                    span: Span::unknown(),
                },
            )],
        );

        let body = vec![if_stmt, assign("i", loop_increment())];
        let facts = try_extract_pattern3_ifphi_facts(&loop_condition(), &body).expect("Ok");
        assert!(facts.is_none());
    }

    #[test]
    fn facts_rejects_break_continue_or_return() {
        let if_stmt = if_else(
            ASTNode::BinaryOp {
                operator: BinaryOperator::Greater,
                left: Box::new(v("i")),
                right: Box::new(lit_int(0)),
                span: Span::unknown(),
            },
            vec![ASTNode::Break { span: Span::unknown() }],
            vec![assign(
                "sum",
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(v("sum")),
                    right: Box::new(lit_int(1)),
                    span: Span::unknown(),
                },
            )],
        );

        let body = vec![
            if_stmt,
            ASTNode::Return {
                value: Some(Box::new(lit_int(0))),
                span: Span::unknown(),
            },
            assign("i", loop_increment()),
        ];
        let facts = try_extract_pattern3_ifphi_facts(&loop_condition(), &body).expect("Ok");
        assert!(facts.is_none());
    }

    #[test]
    fn facts_rejects_nested_if() {
        let nested_if = if_else(
            ASTNode::BinaryOp {
                operator: BinaryOperator::Greater,
                left: Box::new(v("i")),
                right: Box::new(lit_int(0)),
                span: Span::unknown(),
            },
            vec![assign(
                "sum",
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(v("sum")),
                    right: Box::new(lit_int(1)),
                    span: Span::unknown(),
                },
            )],
            vec![assign(
                "sum",
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(v("sum")),
                    right: Box::new(lit_int(0)),
                    span: Span::unknown(),
                },
            )],
        );

        let if_stmt = if_else(
            ASTNode::BinaryOp {
                operator: BinaryOperator::Greater,
                left: Box::new(v("i")),
                right: Box::new(lit_int(0)),
                span: Span::unknown(),
            },
            vec![nested_if],
            vec![assign(
                "sum",
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(v("sum")),
                    right: Box::new(lit_int(1)),
                    span: Span::unknown(),
                },
            )],
        );

        let body = vec![if_stmt, assign("i", loop_increment())];
        let facts = try_extract_pattern3_ifphi_facts(&loop_condition(), &body).expect("Ok");
        assert!(facts.is_none());
    }
}
