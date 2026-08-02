//! One-shot all-route preflight rejection from the existing live capability.

use super::LiveLoopFactsV1;
use crate::mir::builder::control_flow::joinir::route_entry::registry::loop_preflight::{
    LoopPreflightDispositionV1, LoopPreflightRejectV1,
};
use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
use crate::mir::builder::control_flow::joinir::route_entry::registry::selection::select_recipe_first_routes;
use crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts;
use crate::mir::builder::control_flow::plan::facts::LoopFacts;

/// Consumes the only live pair. Current all-route preflight has no Qualified arm.
pub(crate) fn issue_all_route_preflight_v1(
    live: LiveLoopFactsV1<'_>,
) -> LoopPreflightDispositionV1 {
    let _bound_source_frame = (live.condition, live.body);
    let canonical = canonicalize_loop_facts(live.facts);
    let selection = select_recipe_first_routes(Some(&canonical));
    let Some(&front) = selection.raw_execution_routes().first() else {
        return LoopPreflightDispositionV1::NoCandidate;
    };
    LoopPreflightDispositionV1::Rejected(classify_front(&canonical.facts, front))
}

fn classify_front(facts: &LoopFacts, route: LoopRouteId) -> LoopPreflightRejectV1 {
    match route {
        LoopRouteId::LoopSimpleWhile => classify_simple_while(facts),
        LoopRouteId::AccumConstLoop => classify_accum_const(facts),
        _ => LoopPreflightRejectV1::SourceTopologyUnavailable { route },
    }
}

fn classify_simple_while(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(simple) = facts.loop_simple_while() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopSimpleWhile,
        };
    };
    let Some(topology) = simple.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopSimpleWhile,
        };
    };
    if !topology.step().scope_box_children().is_empty() {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopSimpleWhile,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::LoopSimpleWhile,
    }
}

fn classify_accum_const(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(accum) = facts.accum_const_loop() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::AccumConstLoop,
        };
    };
    let Some(topology) = accum.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::AccumConstLoop,
        };
    };
    if !topology.acc_update().scope_box_children().is_empty()
        || !topology.step().scope_box_children().is_empty()
    {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::AccumConstLoop,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::AccumConstLoop,
    }
}

#[cfg(test)]
mod tests {
    use super::issue_all_route_preflight_v1;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::loop_preflight::{
        LoopPreflightDispositionV1, LoopPreflightRejectV1,
    };
    use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
    use crate::mir::builder::control_flow::plan::facts::try_build_live_loop_facts;

    fn fixture(scope_boxed: bool) -> (ASTNode, Vec<ASTNode>) {
        let v = |name: &str| ASTNode::Variable {
            name: name.into(),
            span: Span::unknown(),
        };
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(3),
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
                body: vec![step],
                span: Span::unknown(),
            }]
        } else {
            vec![step]
        };
        (condition, body)
    }

    #[test]
    fn direct_simple_is_policy_rejected_not_qualified() {
        let (condition, body) = fixture(false);
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            issue_all_route_preflight_v1(live),
            LoopPreflightDispositionV1::Rejected(
                LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
                    route: LoopRouteId::LoopSimpleWhile
                }
            )
        ));
    }

    #[test]
    fn scope_box_precedes_direct_policy_rejection() {
        let (condition, body) = fixture(true);
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            issue_all_route_preflight_v1(live),
            LoopPreflightDispositionV1::Rejected(
                LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
                    route: LoopRouteId::LoopSimpleWhile
                }
            )
        ));
    }
}
