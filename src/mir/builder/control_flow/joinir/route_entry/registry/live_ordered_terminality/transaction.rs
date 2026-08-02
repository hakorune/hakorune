//! Consumes the private live carrier and proves only scheduler terminality.

use super::{
    LiveLoopFactsV1, LiveOrderedTerminalityDispositionV1, PreEffectSchedulerTerminalV1,
};
use crate::mir::builder::control_flow::joinir::route_entry::registry::direct_simple_while_terminality::certify_direct_simple_while_terminality;
use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
use crate::mir::builder::control_flow::joinir::route_entry::registry::selection::select_recipe_first_routes;
use crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts;

pub(crate) fn qualify_live_loop_facts_v1(
    live: LiveLoopFactsV1<'_>,
) -> LiveOrderedTerminalityDispositionV1 {
    let _bound_source_frame = (live.condition, live.body);
    let canonical = canonicalize_loop_facts(live.facts);
    let selection = select_recipe_first_routes(Some(&canonical));
    qualify_raw_order(&canonical.facts, selection.raw_execution_routes())
}

fn qualify_raw_order(
    facts: &crate::mir::builder::control_flow::plan::facts::LoopFacts,
    raw_order: &[LoopRouteId],
) -> LiveOrderedTerminalityDispositionV1 {
    let Some((&current, tail)) = raw_order.split_first() else {
        return LiveOrderedTerminalityDispositionV1::NoRoute;
    };
    if current != LoopRouteId::LoopSimpleWhile {
        return LiveOrderedTerminalityDispositionV1::BlockedEarlier { route: current };
    }
    if certify_direct_simple_while_terminality(facts).is_none() {
        return LiveOrderedTerminalityDispositionV1::BlockedCurrent { route: current };
    }
    LiveOrderedTerminalityDispositionV1::PreEffectSchedulerTerminal(PreEffectSchedulerTerminalV1 {
        route: current,
        unreached_legacy_tail: tail.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::{qualify_live_loop_facts_v1, qualify_raw_order};
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::live_ordered_terminality::LiveOrderedTerminalityDispositionV1;
    use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
    use crate::mir::builder::control_flow::plan::facts::try_build_live_loop_facts;

    fn variable(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.into(),
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
        let step = ASTNode::Assignment {
            target: Box::new(variable("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(variable("i")),
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
    fn actual_direct_simple_while_retains_generic_v0_tail() {
        let (condition, body) = fixture(false);
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            qualify_live_loop_facts_v1(live),
            LiveOrderedTerminalityDispositionV1::PreEffectSchedulerTerminal(proof)
                if proof.route() == LoopRouteId::LoopSimpleWhile
                && proof.unreached_legacy_tail() == [LoopRouteId::GenericLoopV0]
        ));
    }

    #[test]
    fn scope_box_simple_while_fails_closed() {
        let (condition, body) = fixture(true);
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            qualify_live_loop_facts_v1(live),
            LiveOrderedTerminalityDispositionV1::BlockedCurrent {
                route: LoopRouteId::LoopSimpleWhile
            }
        ));
    }

    #[test]
    fn unknown_earlier_route_blocks_later_simple_while() {
        let (condition, body) = fixture(false);
        let facts =
            crate::mir::builder::control_flow::plan::facts::try_build_loop_facts(&condition, &body)
                .unwrap()
                .unwrap();
        assert!(matches!(
            qualify_raw_order(
                &facts,
                &[LoopRouteId::LoopBreakRecipe, LoopRouteId::LoopSimpleWhile]
            ),
            LiveOrderedTerminalityDispositionV1::BlockedEarlier {
                route: LoopRouteId::LoopBreakRecipe
            }
        ));
    }

    #[test]
    fn empty_raw_order_has_no_route() {
        let (condition, body) = fixture(false);
        let facts =
            crate::mir::builder::control_flow::plan::facts::try_build_loop_facts(&condition, &body)
                .unwrap()
                .unwrap();
        assert!(matches!(
            qualify_raw_order(&facts, &[]),
            LiveOrderedTerminalityDispositionV1::NoRoute
        ));
    }
}
