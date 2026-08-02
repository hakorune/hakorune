//! M4-D0 test-only observation of Generic V0/V1 facts and raw selection.
//!
//! This module stops before handlers/composers. It records the actual
//! AST -> facts -> canonical facts -> raw schedule boundary; it does not
//! decide V0/V1 precedence or claim winner equivalence.

use super::route_id::LoopRouteId;
use super::selection::select_recipe_first_routes;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts;
use crate::mir::builder::control_flow::plan::facts::try_build_loop_facts;

#[derive(Debug, PartialEq, Eq)]
struct GenericSelectionObservation {
    v0_facts: bool,
    v1_facts: bool,
    raw_routes: Vec<LoopRouteId>,
}

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

fn assign(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn field(object: ASTNode, name: &str) -> ASTNode {
    ASTNode::FieldAccess {
        object: Box::new(object),
        field: name.into(),
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

fn observe(condition: &ASTNode, body: &[ASTNode]) -> GenericSelectionObservation {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let facts = try_build_loop_facts(condition, body)
        .expect("Generic selection observation must not freeze");
    let Some(facts) = facts else {
        return GenericSelectionObservation {
            v0_facts: false,
            v1_facts: false,
            raw_routes: Vec::new(),
        };
    };
    let v0_facts = facts.generic_loop_v0().is_some();
    let v1_facts = facts.generic_loop_v1().is_some();
    let canonical = canonicalize_loop_facts(facts);
    let raw_routes = select_recipe_first_routes(Some(&canonical))
        .raw_execution_routes()
        .to_vec();
    GenericSelectionObservation {
        v0_facts,
        v1_facts,
        raw_routes,
    }
}

fn observe_release(condition: &ASTNode, body: &[ASTNode]) -> GenericSelectionObservation {
    let _config = crate::test_support::ScopedTestConfig::apply(&[
        ("HAKO_JOINIR_STRICT", None),
        ("HAKO_JOINIR_PLANNER_REQUIRED", None),
        ("NYASH_JOINIR_STRICT", None),
    ]);
    observe(condition, body)
}

pub(super) fn progression_condition() -> ASTNode {
    less(variable("i"), integer(3))
}

pub(super) fn additive_condition() -> ASTNode {
    less(add(variable("j"), variable("m")), variable("n"))
}

pub(super) fn additive_body() -> Vec<ASTNode> {
    vec![assign("j", add(variable("j"), integer(1)))]
}

pub(super) fn true_condition() -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(true),
        span: Span::unknown(),
    }
}

pub(super) fn true_body() -> Vec<ASTNode> {
    vec![assign("i", add(variable("i"), integer(1)))]
}

fn progression_step() -> ASTNode {
    assign("i", add(variable("i"), integer(1)))
}

pub(super) fn simple_while_body() -> Vec<ASTNode> {
    vec![progression_step()]
}

pub(super) fn v1_only_body() -> Vec<ASTNode> {
    vec![
        ASTNode::Local {
            variables: vec!["tmp".into()],
            initial_values: vec![Some(Box::new(integer(0)))],
            declared_type_names: Vec::new(),
            span: Span::unknown(),
        },
        progression_step(),
    ]
}

pub(super) fn v1_only_effect_body() -> Vec<ASTNode> {
    vec![
        ASTNode::Local {
            variables: vec!["tmp".into()],
            initial_values: vec![Some(Box::new(integer(0)))],
            declared_type_names: Vec::new(),
            span: Span::unknown(),
        },
        method_call(
            field(variable("env"), "console"),
            "error",
            vec![variable("i")],
        ),
        progression_step(),
    ]
}

pub(super) fn effect_without_local_body() -> Vec<ASTNode> {
    vec![
        method_call(
            field(variable("env"), "console"),
            "error",
            vec![variable("i")],
        ),
        progression_step(),
    ]
}

pub(super) fn both_body() -> Vec<ASTNode> {
    let inner = ASTNode::Loop {
        condition: Box::new(less(variable("j"), integer(3))),
        body: vec![assign("j", add(variable("j"), integer(1)))],
        span: Span::unknown(),
    };
    vec![inner, progression_step()]
}

pub(super) fn neither_body() -> Vec<ASTNode> {
    vec![
        ASTNode::ImportStatement {
            path: "unsupported".into(),
            alias: None,
            span: Span::unknown(),
        },
        progression_step(),
    ]
}

#[test]
fn generic_v1_only_fixture_records_source_to_raw_selection() {
    let observation = observe_release(&progression_condition(), &v1_only_body());
    assert_eq!(observation.v0_facts, false);
    assert_eq!(observation.v1_facts, true);
    assert_eq!(observation.raw_routes, vec![LoopRouteId::GenericLoopV1]);
}

#[test]
fn generic_v1_only_effect_fixture_records_v1_ownership() {
    let observation = observe_release(&progression_condition(), &v1_only_effect_body());
    assert!(!observation.v0_facts);
    assert!(observation.v1_facts);
    assert_eq!(observation.raw_routes, vec![LoopRouteId::GenericLoopV1]);
}

#[test]
fn generic_effect_without_local_fixture_records_v0_boundary() {
    let observation = observe_release(&progression_condition(), &effect_without_local_body());
    assert!(!observation.v0_facts);
    assert!(observation.v1_facts);
    assert_eq!(observation.raw_routes, vec![LoopRouteId::GenericLoopV1]);
}

#[test]
fn generic_v0_additive_condition_records_source_to_raw_selection() {
    let observation = observe_release(&additive_condition(), &additive_body());
    assert!(observation.v0_facts);
    assert!(observation.raw_routes.contains(&LoopRouteId::GenericLoopV0));
}

#[test]
fn generic_true_condition_body_step_records_real_route_ownership() {
    let observation = observe_release(&true_condition(), &true_body());
    assert!(observation.v1_facts);
    assert!(
        observation.raw_routes.contains(&LoopRouteId::GenericLoopV1),
        "true-condition body-derived step must retain a Generic V1 route: {observation:?}"
    );
}

#[test]
fn generic_v0_is_observed_only_as_a_simple_while_tail() {
    let observation = observe_release(&progression_condition(), &simple_while_body());
    assert!(observation.v0_facts);
    assert!(observation.v1_facts);
    assert_eq!(
        observation.raw_routes,
        vec![LoopRouteId::LoopSimpleWhile, LoopRouteId::GenericLoopV0]
    );
}

#[test]
fn generic_both_fixture_records_overlap_without_deciding_precedence() {
    let observation = observe_release(&progression_condition(), &both_body());
    assert_eq!(observation.v0_facts, true);
    assert_eq!(observation.v1_facts, true);
    assert_eq!(
        observation.raw_routes,
        vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
    );
}

#[test]
fn generic_neither_fixture_records_empty_facts_and_schedule() {
    let observation = observe_release(&progression_condition(), &neither_body());
    assert_eq!(
        observation,
        GenericSelectionObservation {
            v0_facts: false,
            v1_facts: false,
            raw_routes: Vec::new(),
        }
    );
}

#[test]
fn generic_v0_is_suppressed_by_planner_required_before_selection() {
    let _config = crate::test_support::ScopedTestConfig::apply(&[
        ("HAKO_JOINIR_STRICT", Some("1")),
        ("HAKO_JOINIR_PLANNER_REQUIRED", Some("1")),
    ]);
    let observation = observe(&progression_condition(), &[progression_step()]);
    assert!(!observation.v0_facts);
    assert!(!observation.raw_routes.contains(&LoopRouteId::GenericLoopV0));
}
