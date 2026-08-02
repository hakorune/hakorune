//! Scheduler-terminality proof for the direct generic LoopBreak route.
//!
//! This proof says only that the selected legacy route cannot return `Ok(None)`.
//! It does not make the route pre-effect or physically safe.

use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
use crate::mir::builder::control_flow::plan::facts::LoopFacts;

/// Opaque proof that the direct generic LoopBreak route is scheduler-terminal.
#[derive(Debug)]
pub(crate) struct DirectLoopBreakTerminalityV1 {
    route: LoopRouteId,
}

impl DirectLoopBreakTerminalityV1 {
    pub(crate) fn route(&self) -> LoopRouteId {
        self.route
    }
}

/// Issues no product and assumes the caller separately proves the raw schedule.
pub(crate) fn certify_direct_loop_break_terminality(
    facts: &LoopFacts,
) -> Option<DirectLoopBreakTerminalityV1> {
    let loop_break = facts.loop_break()?;
    let topology = loop_break.source_topology.as_ref()?;
    let break_if = topology.break_if();
    let carrier_update = topology.carrier_update();
    let step = topology.step();
    let direct_three_sites = facts.source_receipt().raw_body_statement_count() == Some(3)
        && break_if.raw_body_index() == 0
        && carrier_update.raw_body_index() == 1
        && step.raw_body_index() == 2
        && break_if.raw_body_index() != carrier_update.raw_body_index()
        && carrier_update.raw_body_index() != step.raw_body_index()
        && break_if.scope_box_children().is_empty()
        && carrier_update.scope_box_children().is_empty()
        && step.scope_box_children().is_empty();

    direct_three_sites.then_some(DirectLoopBreakTerminalityV1 {
        route: LoopRouteId::LoopBreakRecipe,
    })
}

#[cfg(test)]
mod tests {
    use super::certify_direct_loop_break_terminality;
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
                condition: Box::new(variable("stop")),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            increment("sum"),
            increment("i"),
        ];
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
    fn direct_generic_loop_break_is_sole_raw_route_and_terminal() {
        let (condition, body) = fixture(false);
        let facts = try_build_loop_facts(&condition, &body).unwrap().unwrap();
        let canonical = canonicalize_loop_facts(facts.clone());
        assert_eq!(
            select_recipe_first_routes(Some(&canonical)).raw_execution_routes(),
            [LoopRouteId::LoopBreakRecipe]
        );
        assert_eq!(
            certify_direct_loop_break_terminality(&facts)
                .unwrap()
                .route(),
            LoopRouteId::LoopBreakRecipe
        );
    }

    #[test]
    fn scope_box_loop_break_is_not_directly_terminal() {
        let (condition, body) = fixture(true);
        let facts = try_build_loop_facts(&condition, &body).unwrap().unwrap();
        assert!(certify_direct_loop_break_terminality(&facts).is_none());
    }
}
