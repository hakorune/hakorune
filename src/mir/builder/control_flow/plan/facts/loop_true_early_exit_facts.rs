//! Phase 29aj P5: loop_true_early_exit facts (SSOT)

use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::builder::control_flow::facts::extractors::common_helpers::{
    count_control_flow, extract_loop_increment_plan, is_true_literal, ControlFlowDetector,
};
use crate::mir::builder::control_flow::facts::stmt_view::{
    LoopSourceBodySiteV1, LoopSourceProjectionV1,
};
use crate::mir::builder::control_flow::plan::domain::LoopTrueEarlyExitKind;
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopTrueEarlyExitFacts {
    pub loop_var: String,
    pub exit_kind: LoopTrueEarlyExitKind,
    pub exit_condition: ASTNode,
    pub exit_value: Option<ASTNode>,
    pub carrier_var: Option<String>,
    pub carrier_update: Option<ASTNode>,
    pub loop_increment: ASTNode,
    pub source_topology: Option<LoopTrueEarlyExitSourceTopologyV1>,
}

/// Opaque whole-statement observations for one accepted early-exit loop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct LoopTrueEarlyExitSourceTopologyV1 {
    exit_kind: LoopTrueEarlyExitKind,
    exit_if: LoopSourceBodySiteV1,
    carrier_update: Option<LoopSourceBodySiteV1>,
    step: LoopSourceBodySiteV1,
}

pub(in crate::mir::builder) fn try_extract_loop_true_early_exit_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopTrueEarlyExitFacts>, Freeze> {
    try_extract_loop_true_early_exit_facts_with_projection(
        condition,
        body,
        &LoopSourceProjectionV1::default(),
    )
}

pub(in crate::mir::builder) fn try_extract_loop_true_early_exit_facts_with_projection(
    condition: &ASTNode,
    body: &[ASTNode],
    source_projection: &LoopSourceProjectionV1,
) -> Result<Option<LoopTrueEarlyExitFacts>, Freeze> {
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
        LoopTrueEarlyExitKind::Return => {
            if counts.return_count != 1 || counts.break_count != 0 {
                return Ok(None);
            }
        }
        LoopTrueEarlyExitKind::Break => {
            if counts.break_count != 1 || counts.return_count != 0 {
                return Ok(None);
            }
        }
    }

    let remaining = &body[1..];

    match exit_kind {
        LoopTrueEarlyExitKind::Return => {
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

            Ok(Some(LoopTrueEarlyExitFacts {
                loop_var,
                exit_kind,
                exit_condition,
                exit_value,
                carrier_var: None,
                carrier_update: None,
                loop_increment,
                source_topology: source_topology_for(body, source_projection, exit_kind, None),
            }))
        }
        LoopTrueEarlyExitKind::Break => {
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

            Ok(Some(LoopTrueEarlyExitFacts {
                loop_var,
                exit_kind,
                exit_condition,
                exit_value,
                carrier_var: Some(carrier_var),
                carrier_update: Some(carrier_update),
                loop_increment,
                source_topology: source_topology_for(body, source_projection, exit_kind, Some(1)),
            }))
        }
    }
}

fn source_topology_for(
    body: &[ASTNode],
    projection: &LoopSourceProjectionV1,
    exit_kind: LoopTrueEarlyExitKind,
    carrier_index: Option<usize>,
) -> Option<LoopTrueEarlyExitSourceTopologyV1> {
    let expected_len = match exit_kind {
        LoopTrueEarlyExitKind::Return => 2,
        LoopTrueEarlyExitKind::Break => 3,
    };
    if body.len() != expected_len || projection.flattened_body_len() != Some(expected_len) {
        return None;
    }
    Some(LoopTrueEarlyExitSourceTopologyV1 {
        exit_kind,
        exit_if: projection.site_for_flattened_index(0)?.clone(),
        carrier_update: match carrier_index {
            Some(index) => Some(projection.site_for_flattened_index(index)?.clone()),
            None => None,
        },
        step: projection
            .site_for_flattened_index(expected_len - 1)?
            .clone(),
    })
}

fn extract_exit_if(body: &[ASTNode]) -> Option<(LoopTrueEarlyExitKind, ASTNode, Option<ASTNode>)> {
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
                LoopTrueEarlyExitKind::Return,
                condition.as_ref().clone(),
                exit_value,
            ))
        }
        ASTNode::Break { .. } => Some((
            LoopTrueEarlyExitKind::Break,
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
    use crate::mir::builder::control_flow::facts::stmt_view::flatten_scope_boxes_with_projection;

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
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        }
    }

    fn if_break_else(cond: ASTNode) -> ASTNode {
        ASTNode::If {
            condition: Box::new(cond),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: Some(vec![ASTNode::Continue {
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        }
    }

    #[test]
    fn facts_extracts_loop_true_early_exit_return_success() {
        let condition = lit_true();
        let body = vec![if_return(v("done"), Some(v("value"))), increment("i")];

        let facts = try_extract_loop_true_early_exit_facts(&condition, &body).expect("Ok");
        let facts = facts.expect("Some");

        assert_eq!(facts.loop_var, "i");
        assert_eq!(facts.exit_kind, LoopTrueEarlyExitKind::Return);
        assert!(facts.carrier_var.is_none());
    }

    #[test]
    fn facts_extracts_loop_true_early_exit_break_success() {
        let condition = lit_true();
        let body = vec![
            if_break(v("done")),
            carrier_update("sum", v("i")),
            increment("i"),
        ];

        let facts = try_extract_loop_true_early_exit_facts(&condition, &body).expect("Ok");
        let facts = facts.expect("Some");

        assert_eq!(facts.loop_var, "i");
        assert_eq!(facts.exit_kind, LoopTrueEarlyExitKind::Break);
        assert_eq!(facts.carrier_var.as_deref(), Some("sum"));
    }

    #[test]
    fn facts_retains_only_whole_direct_return_and_break_sites() {
        for (raw_body, expected_len, has_carrier) in [
            (
                vec![if_return(v("done"), Some(v("value"))), increment("i")],
                2,
                false,
            ),
            (
                vec![
                    if_break(v("done")),
                    carrier_update("sum", v("i")),
                    increment("i"),
                ],
                3,
                true,
            ),
        ] {
            let (body, projection) = flatten_scope_boxes_with_projection(&raw_body).into_parts();
            let facts = try_extract_loop_true_early_exit_facts_with_projection(
                &lit_true(),
                &body,
                &projection,
            )
            .expect("Ok")
            .expect("accepted early exit");
            let topology = facts.source_topology.expect("whole-statement topology");
            assert_eq!(topology.exit_if.raw_body_index(), 0);
            assert_eq!(topology.step.raw_body_index(), (expected_len - 1) as u32);
            assert_eq!(topology.carrier_update.is_some(), has_carrier);
            assert!(topology.exit_if.scope_box_children().is_empty());
            assert!(topology.step.scope_box_children().is_empty());
        }
    }

    #[test]
    fn facts_retains_scope_box_lineage_without_borrowing_sites() {
        let raw_body = vec![ASTNode::ScopeBox {
            body: vec![
                if_break(v("done")),
                carrier_update("sum", v("i")),
                increment("i"),
            ],
            span: Span::unknown(),
        }];
        let (body, projection) = flatten_scope_boxes_with_projection(&raw_body).into_parts();
        let facts =
            try_extract_loop_true_early_exit_facts_with_projection(&lit_true(), &body, &projection)
                .expect("Ok")
                .expect("accepted early exit");
        let topology = facts.source_topology.expect("observational topology");
        assert_eq!(topology.exit_if.scope_box_children(), [0]);
        assert_eq!(
            topology
                .carrier_update
                .expect("break carrier")
                .scope_box_children(),
            [1]
        );
        assert_eq!(topology.step.scope_box_children(), [2]);
    }

    #[test]
    fn facts_rejects_else_branch() {
        let condition = lit_true();
        let body = vec![if_break_else(v("done")), increment("i")];

        let facts = try_extract_loop_true_early_exit_facts(&condition, &body).expect("Ok");
        assert!(facts.is_none());
    }

    #[test]
    fn facts_rejects_missing_increment() {
        let condition = lit_true();
        let body = vec![if_return(v("done"), None)];

        let facts = try_extract_loop_true_early_exit_facts(&condition, &body).expect("Ok");
        assert!(facts.is_none());
    }
}
