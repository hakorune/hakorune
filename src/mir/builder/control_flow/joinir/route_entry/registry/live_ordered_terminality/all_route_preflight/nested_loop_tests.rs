//! Nested-loop preflight fixtures stay outside the classifier to keep its box small.

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

fn less(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(left),
        right: Box::new(right),
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

fn assignment(target: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(target)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn increment(name: &str) -> ASTNode {
    assignment(name, add(variable(name), integer(1)))
}

fn inner_loop() -> ASTNode {
    ASTNode::Loop {
        condition: Box::new(less(variable("j"), integer(3))),
        body: vec![
            assignment("sum", add(variable("sum"), integer(1))),
            increment("j"),
        ],
        span: Span::unknown(),
    }
}

fn variable_outer_schedule() -> Vec<ASTNode> {
    vec![
        ASTNode::Local {
            variables: vec!["j".into()],
            initial_values: vec![None],
            declared_type_names: Vec::new(),
            span: Span::unknown(),
        },
        ASTNode::Local {
            variables: vec!["j".into()],
            initial_values: vec![None],
            declared_type_names: Vec::new(),
            span: Span::unknown(),
        },
        assignment("j", integer(0)),
        assignment("i", integer(0)),
        inner_loop(),
        increment("i"),
    ]
}

fn nested_loop_fixture(scope_boxed: bool) -> (ASTNode, Vec<ASTNode>) {
    let statements = variable_outer_schedule();
    let body = if scope_boxed {
        vec![ASTNode::ScopeBox {
            body: statements,
            span: Span::unknown(),
        }]
    } else {
        statements
    };
    (less(variable("i"), integer(3)), body)
}

fn assert_nested_loop_raw_front(condition: &ASTNode, body: &[ASTNode]) {
    let facts = try_build_loop_facts(condition, body)
        .expect("no freeze")
        .expect("loop facts");
    let canonical = canonicalize_loop_facts(facts);
    assert_eq!(
        select_recipe_first_routes(Some(&canonical))
            .raw_execution_routes()
            .first(),
        Some(&LoopRouteId::NestedLoopMinimal)
    );
}

#[test]
fn direct_variable_schedule_is_policy_rejected_after_raw_front_check() {
    let (condition, body) = nested_loop_fixture(false);
    assert_nested_loop_raw_front(&condition, &body);
    let live = try_build_live_loop_facts(&condition, &body)
        .unwrap()
        .unwrap();
    assert!(matches!(
        issue_all_route_preflight_v1(live),
        LoopPreflightDispositionV1::Rejected(
            LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
                route: LoopRouteId::NestedLoopMinimal
            }
        )
    ));
}

#[test]
fn scope_box_variable_schedule_is_rejected_before_policy() {
    let (condition, body) = nested_loop_fixture(true);
    assert_nested_loop_raw_front(&condition, &body);
    let live = try_build_live_loop_facts(&condition, &body)
        .unwrap()
        .unwrap();
    assert!(matches!(
        issue_all_route_preflight_v1(live),
        LoopPreflightDispositionV1::Rejected(LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::NestedLoopMinimal
        })
    ));
}

#[test]
fn topology_absent_variable_schedule_is_source_rejected_after_raw_front_check() {
    let (condition, body) = nested_loop_fixture(false);
    assert_nested_loop_raw_front(&condition, &body);
    let mut facts = try_build_loop_facts(&condition, &body).unwrap().unwrap();
    facts
        .nested_loop_minimal
        .as_mut()
        .expect("NestedLoopMinimal facts")
        .source_topology = None;
    assert!(matches!(
        classify_front(&facts, LoopRouteId::NestedLoopMinimal),
        LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::NestedLoopMinimal
        }
    ));
}
