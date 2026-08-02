//! BoolPredicateScan preflight fixtures stay outside the classifier box.

use super::{classify_front, issue_all_route_preflight_v1};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
use crate::mir::builder::control_flow::joinir::route_entry::registry::loop_preflight::{
    LoopPreflightDispositionV1, LoopPreflightRejectV1,
};
use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
use crate::mir::builder::control_flow::joinir::route_entry::registry::selection::select_recipe_first_routes;
use crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts;
use crate::mir::builder::control_flow::plan::facts::{
    try_build_live_loop_facts, try_build_loop_facts,
};

fn v(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn int(value: i64) -> ASTNode {
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

fn fixture(scope_boxed: bool, prefix_sibling: bool) -> (ASTNode, Vec<ASTNode>) {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(v("i")),
        right: Box::new(ASTNode::MethodCall {
            object: Box::new(v("s")),
            method: "length".into(),
            arguments: vec![],
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let predicate_if = ASTNode::If {
        condition: Box::new(ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            operand: Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::This {
                    span: Span::unknown(),
                }),
                method: "is_digit".into(),
                arguments: vec![ASTNode::MethodCall {
                    object: Box::new(v("s")),
                    method: "substring".into(),
                    arguments: vec![v("i"), add(v("i"), int(1))],
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        then_body: vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(false),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }],
        else_body: None,
        span: Span::unknown(),
    };
    let step = ASTNode::Assignment {
        target: Box::new(v("i")),
        value: Box::new(add(v("i"), int(1))),
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
    statements.extend([predicate_if, step]);
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

fn assert_raw_front(condition: &ASTNode, body: &[ASTNode]) {
    let facts = try_build_loop_facts(condition, body)
        .expect("no freeze")
        .expect("loop facts");
    let canonical = canonicalize_loop_facts(facts);
    assert_eq!(
        select_recipe_first_routes(Some(&canonical))
            .raw_execution_routes()
            .first(),
        Some(&LoopRouteId::BoolPredicateScan)
    );
}

fn assert_live_reject(scope_boxed: bool, prefix_sibling: bool, expected: LoopPreflightRejectV1) {
    let (condition, body) = fixture(scope_boxed, prefix_sibling);
    assert_raw_front(&condition, &body);
    let live = try_build_live_loop_facts(&condition, &body)
        .unwrap()
        .unwrap();
    assert!(matches!(
        issue_all_route_preflight_v1(live),
        LoopPreflightDispositionV1::Rejected(reject) if reject == expected
    ));
}

#[test]
fn direct_bool_predicate_scan_is_policy_rejected_after_raw_front_check() {
    assert_live_reject(
        false,
        false,
        LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
            route: LoopRouteId::BoolPredicateScan,
        },
    );
}

#[test]
fn prefix_sibling_bool_predicate_scan_is_policy_rejected_after_raw_front_check() {
    assert_live_reject(
        false,
        true,
        LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
            route: LoopRouteId::BoolPredicateScan,
        },
    );
}

#[test]
fn scope_box_bool_predicate_scan_is_rejected_before_policy() {
    assert_live_reject(
        true,
        false,
        LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::BoolPredicateScan,
        },
    );
}

#[test]
fn topology_absent_bool_predicate_scan_is_source_rejected_after_raw_front_check() {
    let (condition, body) = fixture(false, false);
    assert_raw_front(&condition, &body);
    let mut facts = try_build_loop_facts(&condition, &body).unwrap().unwrap();
    facts
        .bool_predicate_scan
        .as_mut()
        .expect("BoolPredicateScan facts")
        .source_topology = None;
    assert_eq!(
        classify_front(&facts, LoopRouteId::BoolPredicateScan),
        LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::BoolPredicateScan,
        }
    );
}
