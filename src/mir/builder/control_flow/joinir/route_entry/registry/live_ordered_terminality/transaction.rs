//! Consumes the private live carrier and proves only scheduler terminality.

use super::{
    DirectSimpleWhileSourceLeaseV1, LiveLoopFactsV1, LiveOrderedTerminalityDispositionV1,
    PreEffectSchedulerTerminalV1,
};
use crate::mir::builder::control_flow::joinir::route_entry::registry::direct_simple_while_terminality::certify_direct_simple_while_terminality;
use crate::mir::builder::control_flow::joinir::route_entry::registry::direct_accum_const_loop_terminality::certify_direct_accum_const_loop_terminality;
use crate::mir::builder::control_flow::joinir::route_entry::registry::direct_loop_break_terminality::certify_direct_loop_break_terminality;
use crate::mir::builder::control_flow::joinir::route_entry::registry::direct_loop_continue_only_terminality::certify_direct_loop_continue_only_terminality;
use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
use crate::mir::builder::control_flow::joinir::route_entry::registry::selection::select_recipe_first_routes;
use crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts;

pub(crate) fn qualify_live_loop_facts_v1(
    live: LiveLoopFactsV1<'_>,
) -> LiveOrderedTerminalityDispositionV1<'_> {
    let source = (live.condition, live.body);
    let canonical = canonicalize_loop_facts(live.facts);
    let selection = select_recipe_first_routes(Some(&canonical));
    qualify_raw_order(&canonical.facts, selection.raw_execution_routes(), source)
}

fn qualify_raw_order<'src>(
    facts: &crate::mir::builder::control_flow::plan::facts::LoopFacts,
    raw_order: &[LoopRouteId],
    source: (&'src crate::ast::ASTNode, &'src [crate::ast::ASTNode]),
) -> LiveOrderedTerminalityDispositionV1<'src> {
    let Some((&current, tail)) = raw_order.split_first() else {
        return LiveOrderedTerminalityDispositionV1::NoRoute;
    };
    match current {
        LoopRouteId::LoopSimpleWhile => {
            if certify_direct_simple_while_terminality(facts).is_none() {
                return LiveOrderedTerminalityDispositionV1::BlockedCurrent { route: current };
            }
            LiveOrderedTerminalityDispositionV1::PreEffectSchedulerTerminal(
                PreEffectSchedulerTerminalV1 {
                    route: current,
                    unreached_legacy_tail: tail.into(),
                    source_lease: super::DirectTerminalSourceLeaseV1::SimpleWhile(
                        DirectSimpleWhileSourceLeaseV1 {
                            condition: source.0,
                            step: &source.1[0],
                        },
                    ),
                },
            )
        }
        LoopRouteId::AccumConstLoop => {
            if !tail.is_empty() || certify_direct_accum_const_loop_terminality(facts).is_none() {
                return LiveOrderedTerminalityDispositionV1::BlockedCurrent { route: current };
            }
            LiveOrderedTerminalityDispositionV1::PreEffectSchedulerTerminal(
                PreEffectSchedulerTerminalV1 {
                    route: current,
                    unreached_legacy_tail: Box::default(),
                    source_lease: super::DirectTerminalSourceLeaseV1::AccumConstLoop(
                        super::DirectAccumConstLoopSourceLeaseV1 {
                            condition: source.0,
                            acc_update: &source.1[0],
                            step: &source.1[1],
                        },
                    ),
                },
            )
        }
        LoopRouteId::LoopBreakRecipe => {
            if facts.loop_break().is_none() {
                return LiveOrderedTerminalityDispositionV1::BlockedEarlier { route: current };
            }
            if !tail.is_empty() || certify_direct_loop_break_terminality(facts).is_none() {
                return LiveOrderedTerminalityDispositionV1::BlockedCurrent { route: current };
            }
            LiveOrderedTerminalityDispositionV1::PreEffectSchedulerTerminal(
                PreEffectSchedulerTerminalV1 {
                    route: current,
                    unreached_legacy_tail: Box::default(),
                    source_lease: super::DirectTerminalSourceLeaseV1::LoopBreak(
                        super::DirectLoopBreakSourceLeaseV1 {
                            condition: source.0,
                            break_if: &source.1[0],
                            carrier_update: &source.1[1],
                            step: &source.1[2],
                        },
                    ),
                },
            )
        }
        LoopRouteId::LoopContinueOnly => {
            if !tail.is_empty() || certify_direct_loop_continue_only_terminality(facts).is_none() {
                return LiveOrderedTerminalityDispositionV1::BlockedCurrent { route: current };
            }
            LiveOrderedTerminalityDispositionV1::PreEffectSchedulerTerminal(
                PreEffectSchedulerTerminalV1 {
                    route: current,
                    unreached_legacy_tail: Box::default(),
                    source_lease: super::DirectTerminalSourceLeaseV1::LoopContinueOnly(
                        super::DirectLoopContinueOnlySourceLeaseV1 {
                            condition: source.0,
                            body: source.1,
                        },
                    ),
                },
            )
        }
        _ => LiveOrderedTerminalityDispositionV1::BlockedEarlier { route: current },
    }
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

    fn accum_fixture(scope_boxed: bool) -> (ASTNode, Vec<ASTNode>) {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(3),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let increment = |name: &str, value| ASTNode::Assignment {
            target: Box::new(variable(name)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(variable(name)),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(value),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let statements = vec![increment("sum", 1), increment("i", 1)];
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

    fn continue_only_fixture(scope_boxed: bool) -> (ASTNode, Vec<ASTNode>) {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(3),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let increment = |name: &str| ASTNode::Assignment {
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
    fn actual_direct_accum_is_singleton_terminal() {
        let (condition, body) = accum_fixture(false);
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            qualify_live_loop_facts_v1(live),
            LiveOrderedTerminalityDispositionV1::PreEffectSchedulerTerminal(proof)
                if proof.route() == LoopRouteId::AccumConstLoop
                && proof.unreached_legacy_tail().is_empty()
        ));
    }

    #[test]
    fn scope_box_accum_fails_closed() {
        let (condition, body) = accum_fixture(true);
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            qualify_live_loop_facts_v1(live),
            LiveOrderedTerminalityDispositionV1::BlockedCurrent {
                route: LoopRouteId::AccumConstLoop
            }
        ));
    }

    #[test]
    fn actual_direct_continue_only_is_singleton_terminal() {
        let (condition, body) = continue_only_fixture(false);
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            qualify_live_loop_facts_v1(live),
            LiveOrderedTerminalityDispositionV1::PreEffectSchedulerTerminal(proof)
                if proof.route() == LoopRouteId::LoopContinueOnly
                && proof.unreached_legacy_tail().is_empty()
        ));
    }

    #[test]
    fn scope_box_continue_only_fails_closed() {
        let (condition, body) = continue_only_fixture(true);
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            qualify_live_loop_facts_v1(live),
            LiveOrderedTerminalityDispositionV1::BlockedCurrent {
                route: LoopRouteId::LoopContinueOnly
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
                &[LoopRouteId::LoopBreakRecipe, LoopRouteId::LoopSimpleWhile],
                (&condition, &body),
            ),
            LiveOrderedTerminalityDispositionV1::BlockedEarlier {
                route: LoopRouteId::LoopBreakRecipe
            }
        ));
    }

    #[test]
    fn injected_earlier_route_blocks_continue_only() {
        let (condition, body) = continue_only_fixture(false);
        let facts =
            crate::mir::builder::control_flow::plan::facts::try_build_loop_facts(&condition, &body)
                .unwrap()
                .unwrap();
        assert!(matches!(
            qualify_raw_order(
                &facts,
                &[LoopRouteId::LoopBreakRecipe, LoopRouteId::LoopContinueOnly],
                (&condition, &body),
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
            qualify_raw_order(&facts, &[], (&condition, &body)),
            LiveOrderedTerminalityDispositionV1::NoRoute
        ));
    }
}
