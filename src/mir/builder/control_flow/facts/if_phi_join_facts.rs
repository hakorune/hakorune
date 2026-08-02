//! Phase 29aj P3: if_phi_join facts

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::extractors::common_helpers::extract_loop_increment_plan;
use crate::mir::builder::control_flow::facts::extractors::if_phi_join::extract_loop_with_if_phi_parts;
use crate::mir::builder::control_flow::facts::stmt_view::{
    LoopSourceBodySiteV1, LoopSourceProjectionV1,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct IfPhiJoinFacts {
    pub loop_var: String,
    pub carrier_var: String,
    pub condition: ASTNode,
    pub if_condition: ASTNode,
    pub then_update: ASTNode,
    pub else_update: ASTNode,
    pub loop_increment: ASTNode,
    pub source_topology: Option<IfPhiJoinSourceTopologyV1>,
}

/// Opaque original-body sites retained by the IfPhiJoin extractor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct IfPhiJoinSourceTopologyV1 {
    if_else: LoopSourceBodySiteV1,
    step: LoopSourceBodySiteV1,
}

impl IfPhiJoinSourceTopologyV1 {
    pub(in crate::mir::builder) fn if_else(&self) -> &LoopSourceBodySiteV1 {
        &self.if_else
    }

    pub(in crate::mir::builder) fn step(&self) -> &LoopSourceBodySiteV1 {
        &self.step
    }
}

pub(in crate::mir::builder) fn try_extract_if_phi_join_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<IfPhiJoinFacts>, Freeze> {
    try_extract_if_phi_join_facts_with_projection(
        condition,
        body,
        &LoopSourceProjectionV1::default(),
    )
}

/// Extract IfPhiJoin facts while retaining the existing flattened-body sites.
pub(in crate::mir::builder) fn try_extract_if_phi_join_facts_with_projection(
    condition: &ASTNode,
    body: &[ASTNode],
    source_projection: &LoopSourceProjectionV1,
) -> Result<Option<IfPhiJoinFacts>, Freeze> {
    let parts = match extract_loop_with_if_phi_parts(condition, body) {
        Ok(Some(parts)) => parts,
        Ok(None) => return Ok(None),
        Err(_) => return Ok(None),
    };

    let if_stmt = match body.get(parts.if_else_index) {
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
    let source_topology = source_topology_for(body, source_projection, &parts);

    Ok(Some(IfPhiJoinFacts {
        loop_var: parts.loop_var,
        carrier_var: parts.merged_var,
        condition: condition.clone(),
        if_condition,
        then_update,
        else_update,
        loop_increment,
        source_topology,
    }))
}

fn source_topology_for(
    body: &[ASTNode],
    source_projection: &LoopSourceProjectionV1,
    parts: &crate::mir::builder::control_flow::facts::extractors::if_phi_join::IfPhiJoinParts,
) -> Option<IfPhiJoinSourceTopologyV1> {
    if source_projection.flattened_body_len() != Some(body.len()) {
        return None;
    }
    Some(IfPhiJoinSourceTopologyV1 {
        if_else: source_projection
            .site_for_flattened_index(parts.if_else_index)?
            .clone(),
        step: source_projection
            .site_for_flattened_index(parts.step_index)?
            .clone(),
    })
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
    use crate::mir::builder::control_flow::facts::stmt_view::flatten_scope_boxes_with_projection;
    use crate::mir::builder::control_flow::plan::facts::try_build_loop_facts;

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

    fn valid_if_phi_statement() -> ASTNode {
        if_else(
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
        )
    }

    fn valid_step() -> ASTNode {
        assign("i", loop_increment())
    }

    #[test]
    fn facts_extracts_if_phi_join_success() {
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
        let facts = try_extract_if_phi_join_facts(&loop_condition(), &body).expect("Ok");
        let facts = facts.expect("Some");
        assert_eq!(facts.loop_var, "i");
        assert_eq!(facts.carrier_var, "sum");
        assert!(facts.source_topology.is_none());
    }

    #[test]
    fn facts_retains_direct_and_reversed_extractor_sites() {
        for (body, expected_if_index, expected_step_index) in [
            (vec![valid_if_phi_statement(), valid_step()], 0, 1),
            (vec![valid_step(), valid_if_phi_statement()], 1, 0),
        ] {
            let (body, projection) = flatten_scope_boxes_with_projection(&body).into_parts();
            let facts = try_extract_if_phi_join_facts_with_projection(
                &loop_condition(),
                &body,
                &projection,
            )
            .expect("Ok")
            .expect("accepted IfPhiJoin");
            let topology = facts.source_topology.expect("extractor-observed sites");
            assert_eq!(topology.if_else().raw_body_index(), expected_if_index);
            assert_eq!(topology.step().raw_body_index(), expected_step_index);
            assert!(topology.if_else().scope_box_children().is_empty());
            assert!(topology.step().scope_box_children().is_empty());
        }
    }

    #[test]
    fn facts_retains_scope_box_lineage_for_if_and_step() {
        let raw_body = vec![ASTNode::ScopeBox {
            body: vec![valid_if_phi_statement(), valid_step()],
            span: Span::unknown(),
        }];
        let (body, projection) = flatten_scope_boxes_with_projection(&raw_body).into_parts();
        let facts =
            try_extract_if_phi_join_facts_with_projection(&loop_condition(), &body, &projection)
                .expect("Ok")
                .expect("accepted IfPhiJoin");
        let topology = facts.source_topology.expect("extractor-observed sites");
        assert_eq!(topology.if_else().raw_body_index(), 0);
        assert_eq!(topology.if_else().scope_box_children(), [0]);
        assert_eq!(topology.step().raw_body_index(), 0);
        assert_eq!(topology.step().scope_box_children(), [1]);
    }

    #[test]
    fn loop_facts_builder_preserves_if_phi_scope_box_lineage() {
        let raw_body = vec![ASTNode::ScopeBox {
            body: vec![valid_if_phi_statement(), valid_step()],
            span: Span::unknown(),
        }];
        let facts = try_build_loop_facts(&loop_condition(), &raw_body)
            .expect("Ok")
            .expect("loop facts");
        let topology = facts
            .if_phi_join()
            .expect("IfPhiJoin facts")
            .source_topology
            .as_ref()
            .expect("extractor-observed sites");
        assert_eq!(topology.if_else().raw_body_index(), 0);
        assert_eq!(topology.if_else().scope_box_children(), [0]);
        assert_eq!(topology.step().raw_body_index(), 0);
        assert_eq!(topology.step().scope_box_children(), [1]);
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
        let facts = try_extract_if_phi_join_facts(&loop_condition(), &body).expect("Ok");
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
        let facts = try_extract_if_phi_join_facts(&loop_condition(), &body).expect("Ok");
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
            vec![ASTNode::Break {
                span: Span::unknown(),
            }],
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
        let facts = try_extract_if_phi_join_facts(&loop_condition(), &body).expect("Ok");
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
        let facts = try_extract_if_phi_join_facts(&loop_condition(), &body).expect("Ok");
        assert!(facts.is_none());
    }
}
