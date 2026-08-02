//! One-shot all-route preflight rejection from the existing live capability.

use super::LiveLoopFactsV1;
use crate::mir::builder::control_flow::joinir::route_entry::registry::loop_preflight::{
    LoopPreflightDispositionV1, LoopPreflightRejectV1,
};
use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
use crate::mir::builder::control_flow::joinir::route_entry::registry::selection::select_recipe_first_routes;
use crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts;
use crate::mir::builder::control_flow::plan::facts::LoopFacts;

mod split_scan;
use split_scan::classify_split_scan;
mod bool_predicate_scan;
use bool_predicate_scan::classify_bool_predicate_scan;

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
        LoopRouteId::LoopBreakRecipe => classify_loop_break(facts),
        LoopRouteId::IfPhiJoin => classify_if_phi_join(facts),
        LoopRouteId::LoopContinueOnly => classify_loop_continue_only(facts),
        LoopRouteId::LoopTrueEarlyExit => classify_loop_true_early_exit(facts),
        LoopRouteId::LoopCharMap => classify_loop_char_map(facts),
        LoopRouteId::LoopArrayJoin => classify_loop_array_join(facts),
        LoopRouteId::NestedLoopMinimal => classify_nested_loop_minimal(facts),
        LoopRouteId::ScanWithInit => classify_scan_with_init(facts),
        LoopRouteId::SplitScan => classify_split_scan(facts),
        LoopRouteId::BoolPredicateScan => classify_bool_predicate_scan(facts),
        LoopRouteId::LoopSimpleWhile => classify_simple_while(facts),
        LoopRouteId::AccumConstLoop => classify_accum_const(facts),
        _ => LoopPreflightRejectV1::SourceTopologyUnavailable { route },
    }
}

fn classify_scan_with_init(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(scan) = facts.scan_with_init() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::ScanWithInit,
        };
    };
    let Some(topology) = scan.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::ScanWithInit,
        };
    };
    if topology.has_scope_box_lineage() {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::ScanWithInit,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::ScanWithInit,
    }
}

fn classify_nested_loop_minimal(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(nested) = facts.nested_loop_minimal() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::NestedLoopMinimal,
        };
    };
    let Some(topology) = nested.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::NestedLoopMinimal,
        };
    };
    if topology.has_scope_box_lineage() {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::NestedLoopMinimal,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::NestedLoopMinimal,
    }
}

fn classify_loop_array_join(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(array_join) = facts.loop_array_join() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopArrayJoin,
        };
    };
    let Some(topology) = array_join.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopArrayJoin,
        };
    };
    if topology.has_scope_box_lineage() {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopArrayJoin,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::LoopArrayJoin,
    }
}

fn classify_loop_char_map(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(char_map) = facts.loop_char_map() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopCharMap,
        };
    };
    let Some(topology) = char_map.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopCharMap,
        };
    };
    if topology.has_scope_box_lineage() {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopCharMap,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::LoopCharMap,
    }
}

fn classify_loop_true_early_exit(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(early_exit) = facts.loop_true_early_exit() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopTrueEarlyExit,
        };
    };
    let Some(topology) = early_exit.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopTrueEarlyExit,
        };
    };
    if topology.has_scope_box_lineage() {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopTrueEarlyExit,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::LoopTrueEarlyExit,
    }
}

fn classify_loop_continue_only(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(continue_only) = facts.loop_continue_only() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopContinueOnly,
        };
    };
    let Some(topology) = continue_only.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopContinueOnly,
        };
    };
    if topology.has_scope_box_lineage() {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopContinueOnly,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::LoopContinueOnly,
    }
}

fn classify_if_phi_join(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(if_phi_join) = facts.if_phi_join() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::IfPhiJoin,
        };
    };
    let Some(topology) = if_phi_join.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::IfPhiJoin,
        };
    };
    if !topology.if_else().scope_box_children().is_empty()
        || !topology.step().scope_box_children().is_empty()
    {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::IfPhiJoin,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::IfPhiJoin,
    }
}

fn classify_loop_break(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(loop_break) = facts.loop_break() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopBreakRecipe,
        };
    };
    let Some(topology) = loop_break.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopBreakRecipe,
        };
    };
    if !topology.break_if().scope_box_children().is_empty()
        || !topology.carrier_update().scope_box_children().is_empty()
        || !topology.step().scope_box_children().is_empty()
    {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopBreakRecipe,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::LoopBreakRecipe,
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
mod array_join_tests;
#[cfg(test)]
mod bool_predicate_scan_tests;
#[cfg(test)]
mod split_scan_tests;

#[cfg(test)]
mod char_map_tests;

#[cfg(test)]
mod nested_loop_tests;

#[cfg(test)]
mod scan_with_init_tests;

#[cfg(test)]
mod tests {
    use super::{classify_front, issue_all_route_preflight_v1};
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::loop_preflight::{
        LoopPreflightDispositionV1, LoopPreflightRejectV1,
    };
    use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
    use crate::mir::builder::control_flow::joinir::route_entry::registry::selection::select_recipe_first_routes;
    use crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts;
    use crate::mir::builder::control_flow::plan::facts::{
        try_build_live_loop_facts, try_build_loop_facts,
    };

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

    fn loop_break_fixture(scope_boxed: bool) -> (ASTNode, Vec<ASTNode>) {
        let v = |name: &str| ASTNode::Variable {
            name: name.into(),
            span: Span::unknown(),
        };
        let increment = |name: &str| ASTNode::Assignment {
            target: Box::new(v(name)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v(name)),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
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
        let statements = vec![
            ASTNode::If {
                condition: Box::new(v("stop")),
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

    fn if_phi_fixture(scope_boxed: bool) -> (ASTNode, Vec<ASTNode>) {
        let v = |name: &str| ASTNode::Variable {
            name: name.into(),
            span: Span::unknown(),
        };
        let int = |value| ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        };
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(int(3)),
            span: Span::unknown(),
        };
        let update_sum = |value| ASTNode::Assignment {
            target: Box::new(v("sum")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("sum")),
                right: Box::new(int(value)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let if_else = ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Greater,
                left: Box::new(v("i")),
                right: Box::new(int(0)),
                span: Span::unknown(),
            }),
            then_body: vec![update_sum(1)],
            else_body: Some(vec![update_sum(0)]),
            span: Span::unknown(),
        };
        let step = ASTNode::Assignment {
            target: Box::new(v("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("i")),
                right: Box::new(int(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let statements = vec![if_else, step];
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

    fn assert_loop_break_front(condition: &ASTNode, body: &[ASTNode]) {
        let facts = try_build_loop_facts(condition, body)
            .expect("no freeze")
            .expect("loop facts");
        let canonical = canonicalize_loop_facts(facts);
        assert_eq!(
            select_recipe_first_routes(Some(&canonical))
                .raw_execution_routes()
                .first(),
            Some(&LoopRouteId::LoopBreakRecipe)
        );
    }

    fn assert_if_phi_front(condition: &ASTNode, body: &[ASTNode]) {
        let facts = try_build_loop_facts(condition, body)
            .expect("no freeze")
            .expect("loop facts");
        let canonical = canonicalize_loop_facts(facts);
        assert_eq!(
            select_recipe_first_routes(Some(&canonical))
                .raw_execution_routes()
                .first(),
            Some(&LoopRouteId::IfPhiJoin)
        );
    }

    fn specialized_loop_break_fixture() -> (ASTNode, Vec<ASTNode>) {
        let v = |name: &str| ASTNode::Variable {
            name: name.into(),
            span: Span::unknown(),
        };
        let lit = |value: &str| ASTNode::Literal {
            value: LiteralValue::String(value.into()),
            span: Span::unknown(),
        };
        let local_ch = ASTNode::Local {
            variables: vec!["ch".into()],
            initial_values: vec![Some(Box::new(ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "substring".into(),
                arguments: vec![
                    v("i"),
                    ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(v("i")),
                        right: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(1),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    },
                ],
                span: Span::unknown(),
            }))],
            declared_type_names: Vec::new(),
            span: Span::unknown(),
        };
        let break_if = ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left: Box::new(v("ch")),
                right: Box::new(lit("")),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        };
        let digit_or_break = ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::And,
                left: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::GreaterEqual,
                    left: Box::new(v("ch")),
                    right: Box::new(lit("0")),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::LessEqual,
                    left: Box::new(v("ch")),
                    right: Box::new(lit("9")),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            then_body: vec![
                ASTNode::Assignment {
                    target: Box::new(v("acc")),
                    value: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(v("acc")),
                        right: Box::new(v("ch")),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                },
                ASTNode::Assignment {
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
                },
            ],
            else_body: Some(vec![ASTNode::Break {
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        };
        (
            ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            },
            vec![local_ch, break_if, digit_or_break],
        )
    }

    fn early_exit_fixture(scope_boxed: bool, returns: bool) -> (ASTNode, Vec<ASTNode>) {
        let v = |name: &str| ASTNode::Variable {
            name: name.into(),
            span: Span::unknown(),
        };
        let increment = |name: &str| ASTNode::Assignment {
            target: Box::new(v(name)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v(name)),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let exit = if returns {
            ASTNode::Return {
                value: Some(Box::new(v("sum"))),
                span: Span::unknown(),
            }
        } else {
            ASTNode::Break {
                span: Span::unknown(),
            }
        };
        let mut statements = vec![ASTNode::If {
            condition: Box::new(v("done")),
            then_body: vec![exit],
            else_body: None,
            span: Span::unknown(),
        }];
        if !returns {
            statements.push(increment("sum"));
        }
        statements.push(increment("i"));
        let body = if scope_boxed {
            vec![ASTNode::ScopeBox {
                body: statements,
                span: Span::unknown(),
            }]
        } else {
            statements
        };
        (
            ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            },
            body,
        )
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

    #[test]
    fn direct_if_phi_is_policy_rejected_after_raw_front_check() {
        let (condition, body) = if_phi_fixture(false);
        assert_if_phi_front(&condition, &body);
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            issue_all_route_preflight_v1(live),
            LoopPreflightDispositionV1::Rejected(
                LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
                    route: LoopRouteId::IfPhiJoin
                }
            )
        ));
    }

    #[test]
    fn scope_box_if_phi_is_rejected_before_policy() {
        let (condition, body) = if_phi_fixture(true);
        assert_if_phi_front(&condition, &body);
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            issue_all_route_preflight_v1(live),
            LoopPreflightDispositionV1::Rejected(
                LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
                    route: LoopRouteId::IfPhiJoin
                }
            )
        ));
    }

    #[test]
    fn topology_absent_if_phi_is_source_rejected_after_raw_front_check() {
        let (condition, body) = if_phi_fixture(false);
        assert_if_phi_front(&condition, &body);
        let mut facts = try_build_loop_facts(&condition, &body).unwrap().unwrap();
        facts
            .if_phi_join
            .as_mut()
            .expect("IfPhiJoin facts")
            .source_topology = None;
        assert!(matches!(
            classify_front(&facts, LoopRouteId::IfPhiJoin),
            LoopPreflightRejectV1::SourceTopologyUnavailable {
                route: LoopRouteId::IfPhiJoin
            }
        ));
    }

    #[test]
    fn direct_generic_loop_break_is_policy_rejected_after_raw_front_check() {
        let (condition, body) = loop_break_fixture(false);
        assert_loop_break_front(&condition, &body);
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            issue_all_route_preflight_v1(live),
            LoopPreflightDispositionV1::Rejected(
                LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
                    route: LoopRouteId::LoopBreakRecipe
                }
            )
        ));
    }

    #[test]
    fn scope_box_generic_loop_break_is_rejected_before_policy() {
        let (condition, body) = loop_break_fixture(true);
        assert_loop_break_front(&condition, &body);
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            issue_all_route_preflight_v1(live),
            LoopPreflightDispositionV1::Rejected(
                LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
                    route: LoopRouteId::LoopBreakRecipe
                }
            )
        ));
    }

    #[test]
    fn specialized_loop_break_remains_source_topology_unavailable() {
        let (condition, body) = specialized_loop_break_fixture();
        assert_loop_break_front(&condition, &body);
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            issue_all_route_preflight_v1(live),
            LoopPreflightDispositionV1::Rejected(
                LoopPreflightRejectV1::SourceTopologyUnavailable {
                    route: LoopRouteId::LoopBreakRecipe
                }
            )
        ));
    }

    #[test]
    fn direct_early_exit_return_and_break_are_policy_rejected() {
        for returns in [true, false] {
            let (condition, body) = early_exit_fixture(false, returns);
            let live = try_build_live_loop_facts(&condition, &body)
                .unwrap()
                .unwrap();
            assert!(matches!(
                issue_all_route_preflight_v1(live),
                LoopPreflightDispositionV1::Rejected(
                    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
                        route: LoopRouteId::LoopTrueEarlyExit
                    }
                )
            ));
        }
    }

    #[test]
    fn scope_box_early_exit_is_rejected_before_policy() {
        let (condition, body) = early_exit_fixture(true, false);
        let live = try_build_live_loop_facts(&condition, &body)
            .unwrap()
            .unwrap();
        assert!(matches!(
            issue_all_route_preflight_v1(live),
            LoopPreflightDispositionV1::Rejected(
                LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
                    route: LoopRouteId::LoopTrueEarlyExit
                }
            )
        ));
    }
}
