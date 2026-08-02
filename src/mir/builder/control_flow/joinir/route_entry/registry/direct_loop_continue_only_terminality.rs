//! Pre-effect scheduler-terminality proof for the direct ContinueOnly route.
//!
//! This certificate does not claim a lower result, source-order equivalence,
//! physical safety, or rollback.

use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
use crate::mir::builder::control_flow::plan::facts::LoopFacts;

/// Opaque proof for one complete direct ContinueOnly source schedule.
#[derive(Debug)]
pub(crate) struct DirectLoopContinueOnlyTerminalityV1 {
    route: LoopRouteId,
}

impl DirectLoopContinueOnlyTerminalityV1 {
    pub(crate) fn route(&self) -> LoopRouteId {
        self.route
    }
}

/// Accepts only the facts-owned complete direct whole-statement schedule.
pub(crate) fn certify_direct_loop_continue_only_terminality(
    facts: &LoopFacts,
) -> Option<DirectLoopContinueOnlyTerminalityV1> {
    let topology = facts.loop_continue_only()?.source_topology.as_ref()?;
    topology
        .is_direct_complete_schedule(facts.source_receipt().raw_body_statement_count())
        .then_some(DirectLoopContinueOnlyTerminalityV1 {
            route: LoopRouteId::LoopContinueOnly,
        })
}

#[cfg(test)]
mod tests {
    use super::certify_direct_loop_continue_only_terminality;
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

    fn increment(name: &str) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(variable(name)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(variable(name)),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn fixture(scope_boxed: bool) -> (ASTNode, Vec<ASTNode>) {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(3),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let statements = vec![
            ASTNode::If {
                condition: Box::new(variable("skip")),
                then_body: vec![ASTNode::Continue {
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            increment("sum"),
            increment("i"),
        ];
        let body = scope_boxed
            .then(|| {
                vec![ASTNode::ScopeBox {
                    body: statements.clone(),
                    span: Span::unknown(),
                }]
            })
            .unwrap_or(statements);
        (condition, body)
    }

    #[test]
    fn direct_continue_only_is_sole_raw_route_and_terminal() {
        let (condition, body) = fixture(false);
        let facts = try_build_loop_facts(&condition, &body).unwrap().unwrap();
        let canonical = canonicalize_loop_facts(facts.clone());
        assert_eq!(
            select_recipe_first_routes(Some(&canonical)).raw_execution_routes(),
            [LoopRouteId::LoopContinueOnly]
        );
        assert_eq!(
            certify_direct_loop_continue_only_terminality(&facts)
                .unwrap()
                .route(),
            LoopRouteId::LoopContinueOnly
        );
    }

    #[test]
    fn scope_box_continue_only_is_not_directly_terminal() {
        let (condition, body) = fixture(true);
        let facts = try_build_loop_facts(&condition, &body).unwrap().unwrap();
        assert!(certify_direct_loop_continue_only_terminality(&facts).is_none());
    }

    #[test]
    fn nonfinal_step_leaves_the_certificate_unavailable() {
        let (condition, mut body) = fixture(false);
        body.swap(1, 2);
        let facts = try_build_loop_facts(&condition, &body).unwrap().unwrap();
        assert!(certify_direct_loop_continue_only_terminality(&facts).is_none());
    }
}
