//! Scan-with-init preflight fixtures stay outside the classifier to keep its box small.

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

fn add(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn scan_with_init_fixture(scope_boxed: bool, prefix_sibling: bool) -> (ASTNode, Vec<ASTNode>) {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(variable("i")),
        right: Box::new(ASTNode::MethodCall {
            object: Box::new(variable("s")),
            method: "length".into(),
            arguments: vec![],
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let matched_if_return = ASTNode::If {
        condition: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(ASTNode::MethodCall {
                object: Box::new(variable("s")),
                method: "substring".into(),
                arguments: vec![variable("i"), add(variable("i"), integer(1))],
                span: Span::unknown(),
            }),
            right: Box::new(variable("ch")),
            span: Span::unknown(),
        }),
        then_body: vec![ASTNode::Return {
            value: Some(Box::new(variable("i"))),
            span: Span::unknown(),
        }],
        else_body: None,
        span: Span::unknown(),
    };
    let step = ASTNode::Assignment {
        target: Box::new(variable("i")),
        value: Box::new(add(variable("i"), integer(1))),
        span: Span::unknown(),
    };
    let mut statements = Vec::new();
    if prefix_sibling {
        statements.push(ASTNode::Local {
            variables: vec!["unused".into()],
            initial_values: vec![None],
            declared_type_names: Vec::new(),
            span: Span::unknown(),
        });
    }
    statements.extend([matched_if_return, step]);
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

fn assert_scan_with_init_raw_front(condition: &ASTNode, body: &[ASTNode]) {
    let facts = try_build_loop_facts(condition, body)
        .expect("no freeze")
        .expect("loop facts");
    let canonical = canonicalize_loop_facts(facts);
    assert_eq!(
        select_recipe_first_routes(Some(&canonical))
            .raw_execution_routes()
            .first(),
        Some(&LoopRouteId::ScanWithInit)
    );
}

#[test]
fn direct_scan_with_init_is_policy_rejected_after_raw_front_check() {
    let (condition, body) = scan_with_init_fixture(false, false);
    assert_scan_with_init_raw_front(&condition, &body);
    let live = try_build_live_loop_facts(&condition, &body)
        .unwrap()
        .unwrap();
    assert!(matches!(
        issue_all_route_preflight_v1(live),
        LoopPreflightDispositionV1::Rejected(
            LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
                route: LoopRouteId::ScanWithInit
            }
        )
    ));
}

#[test]
fn prefix_sibling_scan_with_init_is_policy_rejected_after_raw_front_check() {
    let (condition, body) = scan_with_init_fixture(false, true);
    assert_scan_with_init_raw_front(&condition, &body);
    let live = try_build_live_loop_facts(&condition, &body)
        .unwrap()
        .unwrap();
    assert!(matches!(
        issue_all_route_preflight_v1(live),
        LoopPreflightDispositionV1::Rejected(
            LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
                route: LoopRouteId::ScanWithInit
            }
        )
    ));
}

#[test]
fn scope_box_scan_with_init_is_rejected_before_policy() {
    let (condition, body) = scan_with_init_fixture(true, false);
    assert_scan_with_init_raw_front(&condition, &body);
    let live = try_build_live_loop_facts(&condition, &body)
        .unwrap()
        .unwrap();
    assert!(matches!(
        issue_all_route_preflight_v1(live),
        LoopPreflightDispositionV1::Rejected(LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::ScanWithInit
        })
    ));
}

#[test]
fn topology_absent_scan_with_init_is_source_rejected_after_raw_front_check() {
    let (condition, body) = scan_with_init_fixture(false, false);
    assert_scan_with_init_raw_front(&condition, &body);
    let mut facts = try_build_loop_facts(&condition, &body).unwrap().unwrap();
    facts
        .scan_with_init
        .as_mut()
        .expect("ScanWithInit facts")
        .source_topology = None;
    assert!(matches!(
        classify_front(&facts, LoopRouteId::ScanWithInit),
        LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::ScanWithInit
        }
    ));
}
