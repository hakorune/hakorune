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
        LoopPreflightDispositionV1::Rejected(LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopSimpleWhile
        })
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
        LoopPreflightDispositionV1::Rejected(LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::IfPhiJoin
        })
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
        LoopPreflightDispositionV1::Rejected(LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopBreakRecipe
        })
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
        LoopPreflightDispositionV1::Rejected(LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopBreakRecipe
        })
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
        LoopPreflightDispositionV1::Rejected(LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopTrueEarlyExit
        })
    ));
}
