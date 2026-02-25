//! Phase 29aj P5: Pattern5InfiniteEarlyExitFacts (Facts SSOT)

use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    count_control_flow, extract_loop_increment_plan, is_true_literal, ControlFlowDetector,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::domain::Pattern5ExitKind;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern5InfiniteEarlyExitFacts {
    pub loop_var: String,
    pub exit_kind: Pattern5ExitKind,
    pub exit_condition: ASTNode,
    pub exit_value: Option<ASTNode>,
    pub carrier_var: Option<String>,
    pub carrier_update: Option<ASTNode>,
    pub loop_increment: ASTNode,
}

pub(in crate::mir::builder) fn try_extract_pattern5_infinite_early_exit_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<Pattern5InfiniteEarlyExitFacts>, Freeze> {
    if !is_true_literal(condition) {
        return Ok(None);
    }

    let Some((exit_kind, exit_condition, exit_value)) = extract_exit_if(body) else {
        return Ok(None);
    };

    let mut detector = ControlFlowDetector::default();
    detector.count_returns = true;
    let counts = count_control_flow(body, detector);
    if counts.has_nested_loop || counts.continue_count > 0 {
        return Ok(None);
    }

    match exit_kind {
        Pattern5ExitKind::Return => {
            if counts.return_count != 1 || counts.break_count != 0 {
                return Ok(None);
            }
        }
        Pattern5ExitKind::Break => {
            if counts.break_count != 1 || counts.return_count != 0 {
                return Ok(None);
            }
        }
    }

    let remaining = &body[1..];

    match exit_kind {
        Pattern5ExitKind::Return => {
            if remaining.len() != 1 {
                return Ok(None);
            }

            let loop_var = match extract_assignment_target(&remaining[0]) {
                Some(var) => var,
                None => return Ok(None),
            };

            let loop_increment = match extract_loop_increment_plan(body, &loop_var) {
                Ok(Some(inc)) => inc,
                _ => return Ok(None),
            };

            Ok(Some(Pattern5InfiniteEarlyExitFacts {
                loop_var,
                exit_kind,
                exit_condition,
                exit_value,
                carrier_var: None,
                carrier_update: None,
                loop_increment,
            }))
        }
        Pattern5ExitKind::Break => {
            if remaining.len() != 2 {
                return Ok(None);
            }

            let (carrier_var, carrier_update) = match extract_carrier_update(&remaining[0]) {
                Some(values) => values,
                None => return Ok(None),
            };

            let loop_var = match extract_assignment_target(&remaining[1]) {
                Some(var) => var,
                None => return Ok(None),
            };

            if carrier_var == loop_var {
                return Ok(None);
            }

            let loop_increment = match extract_loop_increment_plan(body, &loop_var) {
                Ok(Some(inc)) => inc,
                _ => return Ok(None),
            };

            Ok(Some(Pattern5InfiniteEarlyExitFacts {
                loop_var,
                exit_kind,
                exit_condition,
                exit_value,
                carrier_var: Some(carrier_var),
                carrier_update: Some(carrier_update),
                loop_increment,
            }))
        }
    }
}

fn extract_exit_if(body: &[ASTNode]) -> Option<(Pattern5ExitKind, ASTNode, Option<ASTNode>)> {
    let first = body.first()?;
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = first
    else {
        return None;
    };

    if else_body.is_some() || then_body.len() != 1 {
        return None;
    }

    match &then_body[0] {
        ASTNode::Return { value, .. } => {
            let exit_value = value.as_ref().map(|boxed| boxed.as_ref().clone());
            Some((
                Pattern5ExitKind::Return,
                condition.as_ref().clone(),
                exit_value,
            ))
        }
        ASTNode::Break { .. } => Some((
            Pattern5ExitKind::Break,
            condition.as_ref().clone(),
            None,
        )),
        _ => None,
    }
}

fn extract_assignment_target(stmt: &ASTNode) -> Option<String> {
    let ASTNode::Assignment { target, .. } = stmt else {
        return None;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return None;
    };
    Some(name.clone())
}

fn extract_carrier_update(stmt: &ASTNode) -> Option<(String, ASTNode)> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return None;
    };
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        ..
    } = value.as_ref()
    else {
        return None;
    };
    if !matches!(left.as_ref(), ASTNode::Variable { name: lhs, .. } if lhs == name) {
        return None;
    }
    Some((name.clone(), value.as_ref().clone()))
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

    fn lit_true() -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        }
    }

    fn increment(var: &str) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(var)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v(var)),
                right: Box::new(lit_int(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn carrier_update(var: &str, rhs: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(var)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v(var)),
                right: Box::new(rhs),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn if_return(cond: ASTNode, value: Option<ASTNode>) -> ASTNode {
        ASTNode::If {
            condition: Box::new(cond),
            then_body: vec![ASTNode::Return {
                value: value.map(Box::new),
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        }
    }

    fn if_break(cond: ASTNode) -> ASTNode {
        ASTNode::If {
            condition: Box::new(cond),
            then_body: vec![ASTNode::Break { span: Span::unknown() }],
            else_body: None,
            span: Span::unknown(),
        }
    }

    fn if_break_else(cond: ASTNode) -> ASTNode {
        ASTNode::If {
            condition: Box::new(cond),
            then_body: vec![ASTNode::Break { span: Span::unknown() }],
            else_body: Some(vec![ASTNode::Continue { span: Span::unknown() }]),
            span: Span::unknown(),
        }
    }

    #[test]
    fn facts_extracts_pattern5_return_success() {
        let condition = lit_true();
        let body = vec![
            if_return(v("done"), Some(v("value"))),
            increment("i"),
        ];

        let facts = try_extract_pattern5_infinite_early_exit_facts(&condition, &body)
            .expect("Ok");
        let facts = facts.expect("Some");

        assert_eq!(facts.loop_var, "i");
        assert_eq!(facts.exit_kind, Pattern5ExitKind::Return);
        assert!(facts.carrier_var.is_none());
    }

    #[test]
    fn facts_extracts_pattern5_break_success() {
        let condition = lit_true();
        let body = vec![
            if_break(v("done")),
            carrier_update("sum", v("i")),
            increment("i"),
        ];

        let facts = try_extract_pattern5_infinite_early_exit_facts(&condition, &body)
            .expect("Ok");
        let facts = facts.expect("Some");

        assert_eq!(facts.loop_var, "i");
        assert_eq!(facts.exit_kind, Pattern5ExitKind::Break);
        assert_eq!(facts.carrier_var.as_deref(), Some("sum"));
    }

    #[test]
    fn facts_rejects_else_branch() {
        let condition = lit_true();
        let body = vec![if_break_else(v("done")), increment("i")];

        let facts = try_extract_pattern5_infinite_early_exit_facts(&condition, &body)
            .expect("Ok");
        assert!(facts.is_none());
    }

    #[test]
    fn facts_rejects_missing_increment() {
        let condition = lit_true();
        let body = vec![if_return(v("done"), None)];

        let facts = try_extract_pattern5_infinite_early_exit_facts(&condition, &body)
            .expect("Ok");
        assert!(facts.is_none());
    }
}
