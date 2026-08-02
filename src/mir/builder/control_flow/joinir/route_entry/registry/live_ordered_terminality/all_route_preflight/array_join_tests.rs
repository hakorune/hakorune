//! Array-join preflight fixtures stay outside the classifier to keep its box small.

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

fn assignment(target: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(target)),
        value: Box::new(value),
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

fn array_join_fixture(scope_boxed: bool) -> (ASTNode, Vec<ASTNode>) {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(variable("i")),
        right: Box::new(method_call(variable("arr"), "length", vec![])),
        span: Span::unknown(),
    };
    let separator_guard = ASTNode::If {
        condition: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Greater,
            left: Box::new(variable("i")),
            right: Box::new(integer(0)),
            span: Span::unknown(),
        }),
        then_body: vec![assignment(
            "result",
            add(variable("result"), variable("sep")),
        )],
        else_body: None,
        span: Span::unknown(),
    };
    let array_append = assignment(
        "result",
        add(
            variable("result"),
            method_call(variable("arr"), "get", vec![variable("i")]),
        ),
    );
    let step = assignment("i", add(variable("i"), integer(1)));
    let statements = vec![separator_guard, array_append, step];
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

fn assert_array_join_raw_front(condition: &ASTNode, body: &[ASTNode]) {
    let facts = try_build_loop_facts(condition, body)
        .expect("no freeze")
        .expect("loop facts");
    let canonical = canonicalize_loop_facts(facts);
    assert_eq!(
        select_recipe_first_routes(Some(&canonical))
            .raw_execution_routes()
            .first(),
        Some(&LoopRouteId::LoopArrayJoin)
    );
}

#[test]
fn direct_array_join_is_policy_rejected_after_raw_front_check() {
    let (condition, body) = array_join_fixture(false);
    assert_array_join_raw_front(&condition, &body);
    let live = try_build_live_loop_facts(&condition, &body)
        .unwrap()
        .unwrap();
    assert!(matches!(
        issue_all_route_preflight_v1(live),
        LoopPreflightDispositionV1::Rejected(
            LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
                route: LoopRouteId::LoopArrayJoin
            }
        )
    ));
}

#[test]
fn scope_box_array_join_is_rejected_before_policy() {
    let (condition, body) = array_join_fixture(true);
    assert_array_join_raw_front(&condition, &body);
    let live = try_build_live_loop_facts(&condition, &body)
        .unwrap()
        .unwrap();
    assert!(matches!(
        issue_all_route_preflight_v1(live),
        LoopPreflightDispositionV1::Rejected(LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopArrayJoin
        })
    ));
}

#[test]
fn topology_absent_array_join_is_source_rejected_after_raw_front_check() {
    let (condition, body) = array_join_fixture(false);
    assert_array_join_raw_front(&condition, &body);
    let mut facts = try_build_loop_facts(&condition, &body).unwrap().unwrap();
    facts
        .loop_array_join
        .as_mut()
        .expect("LoopArrayJoin facts")
        .source_topology = None;
    assert!(matches!(
        classify_front(&facts, LoopRouteId::LoopArrayJoin),
        LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopArrayJoin
        }
    ));
}
