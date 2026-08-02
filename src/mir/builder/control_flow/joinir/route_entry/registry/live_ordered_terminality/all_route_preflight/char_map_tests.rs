//! Char-map preflight fixtures stay outside the classifier to keep its box small.

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

fn method_call(object: ASTNode, method: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(object),
        method: method.into(),
        arguments,
        span: Span::unknown(),
    }
}

fn char_map_fixture(scope_boxed: bool) -> (ASTNode, Vec<ASTNode>) {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(variable("i")),
        right: Box::new(method_call(variable("s"), "length", vec![])),
        span: Span::unknown(),
    };
    let substring_local = ASTNode::Local {
        variables: vec!["ch".into()],
        initial_values: vec![Some(Box::new(method_call(
            variable("s"),
            "substring",
            vec![
                variable("i"),
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(variable("i")),
                    right: Box::new(integer(1)),
                    span: Span::unknown(),
                },
            ],
        )))],
        declared_type_names: Vec::new(),
        span: Span::unknown(),
    };
    let result_update = ASTNode::Assignment {
        target: Box::new(variable("result")),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(variable("result")),
            right: Box::new(method_call(
                ASTNode::This {
                    span: Span::unknown(),
                },
                "char_to_lower",
                vec![variable("ch")],
            )),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let step = ASTNode::Assignment {
        target: Box::new(variable("i")),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(variable("i")),
            right: Box::new(integer(1)),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let statements = vec![substring_local, result_update, step];
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

fn assert_char_map_raw_front(condition: &ASTNode, body: &[ASTNode]) {
    let facts = try_build_loop_facts(condition, body)
        .expect("no freeze")
        .expect("loop facts");
    let canonical = canonicalize_loop_facts(facts);
    assert_eq!(
        select_recipe_first_routes(Some(&canonical))
            .raw_execution_routes()
            .first(),
        Some(&LoopRouteId::LoopCharMap)
    );
}

#[test]
fn direct_char_map_is_policy_rejected_after_raw_front_check() {
    let (condition, body) = char_map_fixture(false);
    assert_char_map_raw_front(&condition, &body);
    let live = try_build_live_loop_facts(&condition, &body)
        .unwrap()
        .unwrap();
    assert!(matches!(
        issue_all_route_preflight_v1(live),
        LoopPreflightDispositionV1::Rejected(
            LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
                route: LoopRouteId::LoopCharMap
            }
        )
    ));
}

#[test]
fn scope_box_char_map_is_rejected_before_policy() {
    let (condition, body) = char_map_fixture(true);
    assert_char_map_raw_front(&condition, &body);
    let live = try_build_live_loop_facts(&condition, &body)
        .unwrap()
        .unwrap();
    assert!(matches!(
        issue_all_route_preflight_v1(live),
        LoopPreflightDispositionV1::Rejected(LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopCharMap
        })
    ));
}

#[test]
fn topology_absent_char_map_is_source_rejected_after_raw_front_check() {
    let (condition, body) = char_map_fixture(false);
    assert_char_map_raw_front(&condition, &body);
    let mut facts = try_build_loop_facts(&condition, &body).unwrap().unwrap();
    facts
        .loop_char_map
        .as_mut()
        .expect("LoopCharMap facts")
        .source_topology = None;
    assert!(matches!(
        classify_front(&facts, LoopRouteId::LoopCharMap),
        LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopCharMap
        }
    ));
}
