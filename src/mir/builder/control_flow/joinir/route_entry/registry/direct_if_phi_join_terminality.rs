//! Scheduler-terminality proof for the direct IfPhiJoin route.
//!
//! This says only that the selected legacy route cannot return `Ok(None)`.
//! It does not make the route pre-effect or physically safe.

use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
use crate::mir::builder::control_flow::plan::facts::LoopFacts;

/// Opaque proof that the direct IfPhiJoin route is scheduler-terminal.
#[derive(Debug)]
pub(crate) struct DirectIfPhiJoinTerminalityV1 {
    route: LoopRouteId,
}

impl DirectIfPhiJoinTerminalityV1 {
    pub(crate) fn route(&self) -> LoopRouteId {
        self.route
    }
}

/// Issues no product and assumes the caller separately proves the raw schedule.
pub(crate) fn certify_direct_if_phi_join_terminality(
    facts: &LoopFacts,
) -> Option<DirectIfPhiJoinTerminalityV1> {
    let if_phi_join = facts.if_phi_join()?;
    let topology = if_phi_join.source_topology.as_ref()?;
    let if_else = topology.if_else();
    let step = topology.step();
    let direct_observed_pair = matches!(
        (if_else.raw_body_index(), step.raw_body_index()),
        (0, 1) | (1, 0)
    );
    let direct_two_sites = facts.source_receipt().raw_body_statement_count() == Some(2)
        && direct_observed_pair
        && if_else.raw_body_index() != step.raw_body_index()
        && if_else.scope_box_children().is_empty()
        && step.scope_box_children().is_empty();

    direct_two_sites.then_some(DirectIfPhiJoinTerminalityV1 {
        route: LoopRouteId::IfPhiJoin,
    })
}

#[cfg(test)]
mod tests {
    use super::certify_direct_if_phi_join_terminality;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::{
        route_id::LoopRouteId, select_recipe_first_routes,
    };
    use crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts;
    use crate::mir::builder::control_flow::plan::facts::try_build_loop_facts;

    fn variable(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.into(),
            span: Span::unknown(),
        }
    }

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn fixture(reversed: bool, scope_boxed: bool) -> (ASTNode, Vec<ASTNode>) {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("i")),
            right: Box::new(integer(3)),
            span: Span::unknown(),
        };
        let update_sum = |value| ASTNode::Assignment {
            target: Box::new(variable("sum")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(variable("sum")),
                right: Box::new(integer(value)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let if_else = ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Greater,
                left: Box::new(variable("i")),
                right: Box::new(integer(0)),
                span: Span::unknown(),
            }),
            then_body: vec![update_sum(1)],
            else_body: Some(vec![update_sum(0)]),
            span: Span::unknown(),
        };
        let step = ASTNode::Assignment {
            target: Box::new(variable("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(variable("i")),
                right: Box::new(integer(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let statements = if reversed {
            vec![step, if_else]
        } else {
            vec![if_else, step]
        };
        let body = if scope_boxed {
            vec![ASTNode::ScopeBox {
                body: statements,
                span: Span::unknown(),
            }]
        } else {
            statements
        };
        (condition, body)
    }

    #[test]
    fn direct_if_phi_orders_are_sole_raw_route_and_terminal() {
        for reversed in [false, true] {
            let (condition, body) = fixture(reversed, false);
            let facts = try_build_loop_facts(&condition, &body).unwrap().unwrap();
            let canonical = canonicalize_loop_facts(facts.clone());
            assert_eq!(
                select_recipe_first_routes(Some(&canonical)).raw_execution_routes(),
                [LoopRouteId::IfPhiJoin]
            );
            assert_eq!(
                certify_direct_if_phi_join_terminality(&facts)
                    .unwrap()
                    .route(),
                LoopRouteId::IfPhiJoin
            );
        }
    }

    #[test]
    fn scope_box_if_phi_is_not_directly_terminal() {
        let (condition, body) = fixture(false, true);
        let facts = try_build_loop_facts(&condition, &body).unwrap().unwrap();
        assert!(certify_direct_if_phi_join_terminality(&facts).is_none());
    }
}
