//! Pre-effect scheduler-terminality proof for direct AccumConstLoop.

use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
use crate::mir::builder::control_flow::plan::facts::LoopFacts;

/// Opaque proof that the direct Accum route is the sole raw scheduled route.
#[derive(Debug)]
pub(crate) struct DirectAccumConstLoopTerminalityV1 {
    route: LoopRouteId,
}

impl DirectAccumConstLoopTerminalityV1 {
    pub(crate) fn route(&self) -> LoopRouteId {
        self.route
    }
}

/// Issues no product and assumes the caller separately proves the raw schedule.
pub(crate) fn certify_direct_accum_const_loop_terminality(
    facts: &LoopFacts,
) -> Option<DirectAccumConstLoopTerminalityV1> {
    let accum = facts.accum_const_loop()?;
    let topology = accum.source_topology.as_ref()?;
    let update = topology.acc_update();
    let step = topology.step();
    let direct_two_sites = facts.source_receipt().raw_body_statement_count() == Some(2)
        && update.raw_body_index() == 0
        && step.raw_body_index() == 1
        && update.raw_body_index() != step.raw_body_index()
        && update.scope_box_children().is_empty()
        && step.scope_box_children().is_empty();

    direct_two_sites.then_some(DirectAccumConstLoopTerminalityV1 {
        route: LoopRouteId::AccumConstLoop,
    })
}

#[cfg(test)]
mod tests {
    use super::certify_direct_accum_const_loop_terminality;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::{
        route_id::LoopRouteId, select_recipe_first_routes,
    };
    use crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts;
    use crate::mir::builder::control_flow::plan::facts::try_build_loop_facts;

    fn v(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.into(),
            span: Span::unknown(),
        }
    }

    fn fixture(scope_boxed: bool) -> (ASTNode, Vec<ASTNode>) {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(3),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let update = ASTNode::Assignment {
            target: Box::new(v("sum")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("sum")),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let step = ASTNode::Assignment {
            target: Box::new(v("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("i")),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let body = if scope_boxed {
            vec![ASTNode::ScopeBox {
                body: vec![update, step],
                span: Span::unknown(),
            }]
        } else {
            vec![update, step]
        };
        (condition, body)
    }

    #[test]
    fn direct_accum_is_sole_raw_route_and_terminal() {
        let (condition, body) = fixture(false);
        let facts = try_build_loop_facts(&condition, &body).unwrap().unwrap();
        let canonical = canonicalize_loop_facts(facts.clone());
        assert_eq!(
            select_recipe_first_routes(Some(&canonical)).raw_execution_routes(),
            [LoopRouteId::AccumConstLoop]
        );
        assert_eq!(
            certify_direct_accum_const_loop_terminality(&facts)
                .unwrap()
                .route(),
            LoopRouteId::AccumConstLoop
        );
    }

    #[test]
    fn scope_box_accum_is_not_directly_terminal() {
        let (condition, body) = fixture(true);
        let facts = try_build_loop_facts(&condition, &body).unwrap().unwrap();
        assert!(certify_direct_accum_const_loop_terminality(&facts).is_none());
    }
}
