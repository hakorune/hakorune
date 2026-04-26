use super::*;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

#[test]
fn test_update_kind_name() {
    assert_eq!(UpdateKind::CounterLike.name(), "CounterLike");
    assert_eq!(UpdateKind::AccumulationLike.name(), "AccumulationLike");
    assert_eq!(UpdateKind::Other.name(), "Other");
}

fn span() -> Span {
    Span::unknown()
}

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: span(),
    }
}

fn lit_i(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: span(),
    }
}

fn add(lhs: ASTNode, rhs: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(lhs),
        right: Box::new(rhs),
        span: span(),
    }
}

fn assign(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(var(name)),
        value: Box::new(value),
        span: span(),
    }
}

fn if_with_updates(
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> ASTNode {
    ASTNode::If {
        condition: Box::new(condition),
        then_body,
        else_body,
        span: span(),
    }
}

fn loop_with_body(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::Loop {
        condition: Box::new(var("cond")),
        body,
        span: span(),
    }
}

#[test]
fn test_analyze_single_counter_from_ast() {
    let names = vec!["i".to_string()];
    let loop_body = vec![assign("i", add(var("i"), lit_i(1)))];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert!(summary.has_single_counter());
    assert!(!summary.has_accumulation());
    assert_eq!(summary.counter_count(), 1);
    assert_eq!(summary.accumulation_count(), 0);
}

#[test]
fn test_analyze_accumulation_from_ast() {
    let names = vec!["sum".to_string()];
    let loop_body = vec![assign("sum", add(var("sum"), lit_i(1)))];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert!(!summary.has_single_counter());
    assert!(summary.has_accumulation());
    assert_eq!(summary.counter_count(), 0);
    assert_eq!(summary.accumulation_count(), 1);
}

#[test]
fn test_analyze_mixed_from_ast() {
    let names = vec!["i".to_string(), "sum".to_string()];
    let loop_body = vec![
        assign("sum", add(var("sum"), var("i"))),
        assign("i", add(var("i"), lit_i(1))),
    ];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert!(!summary.has_single_counter());
    assert!(summary.has_accumulation());
    assert_eq!(summary.counter_count(), 1);
    assert_eq!(summary.accumulation_count(), 1);
}

#[test]
fn test_is_if_phi_join_signature_basic_ast() {
    let names = vec!["i".to_string(), "sum".to_string()];
    let loop_body = vec![
        if_with_updates(
            var("cond"),
            vec![assign("sum", add(var("sum"), var("i")))],
            None,
        ),
        assign("i", add(var("i"), lit_i(1))),
    ];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert!(summary.is_simple_if_sum_pattern());
    assert_eq!(summary.counter_count(), 1);
    assert_eq!(summary.accumulation_count(), 1);
}

#[test]
fn test_is_if_phi_join_signature_with_count_ast() {
    let names = vec!["i".to_string(), "sum".to_string(), "count".to_string()];
    let loop_body = vec![
        if_with_updates(
            var("cond"),
            vec![
                assign("sum", add(var("sum"), var("i"))),
                assign("count", add(var("count"), lit_i(1))),
            ],
            None,
        ),
        assign("i", add(var("i"), lit_i(1))),
    ];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert!(summary.is_simple_if_sum_pattern());
    assert_eq!(summary.counter_count(), 1);
    assert_eq!(summary.accumulation_count(), 2);
}

#[test]
fn test_is_if_phi_join_signature_no_accumulator_ast() {
    let names = vec!["i".to_string()];
    let loop_body = vec![assign("i", add(var("i"), lit_i(1)))];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert!(!summary.is_simple_if_sum_pattern());
    assert_eq!(summary.counter_count(), 1);
    assert_eq!(summary.accumulation_count(), 0);
}

#[test]
fn test_is_if_phi_join_signature_no_counter_ast() {
    let names = vec!["sum".to_string()];
    let loop_body = vec![assign("sum", add(var("sum"), lit_i(1)))];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert!(!summary.is_simple_if_sum_pattern());
    assert_eq!(summary.counter_count(), 0);
    assert_eq!(summary.accumulation_count(), 1);
}

#[test]
fn test_is_if_phi_join_signature_multiple_counters_ast() {
    let names = vec!["i".to_string(), "j".to_string(), "sum".to_string()];
    let loop_body = vec![
        assign("i", add(var("i"), lit_i(1))),
        assign("j", add(var("j"), lit_i(1))),
        assign("sum", add(var("sum"), var("i"))),
    ];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert!(!summary.is_simple_if_sum_pattern());
    assert_eq!(summary.counter_count(), 2);
    assert_eq!(summary.accumulation_count(), 1);
}

#[test]
fn loop_update_rhs_first_index_name_requires_self_increment() {
    let names = vec!["i".to_string()];
    let loop_body = vec![assign("i", lit_i(0))];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert_eq!(summary.counter_count(), 0);
    assert_eq!(summary.accumulation_count(), 0);
    assert_eq!(summary.carriers[0].kind, UpdateKind::Other);
}

#[test]
fn loop_update_rhs_first_rejects_non_self_reference() {
    let names = vec!["i".to_string()];
    let loop_body = vec![assign("i", add(var("j"), lit_i(1)))];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert_eq!(summary.counter_count(), 0);
    assert_eq!(summary.accumulation_count(), 0);
    assert_eq!(summary.carriers[0].kind, UpdateKind::Other);
}

#[test]
fn loop_update_rhs_first_self_plus_one_uses_name_only_as_tiebreaker() {
    let names = vec!["i".to_string(), "sum".to_string()];
    let loop_body = vec![
        assign("i", add(var("i"), lit_i(1))),
        assign("sum", add(var("sum"), lit_i(1))),
    ];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert_eq!(summary.counter_count(), 1);
    assert_eq!(summary.accumulation_count(), 1);
}

#[test]
fn loop_update_nested_scope_ignores_nested_loop_assignment() {
    let names = vec!["i".to_string()];
    let loop_body = vec![loop_with_body(vec![assign("i", add(var("i"), lit_i(1)))])];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert!(summary.carriers.is_empty());
    assert_eq!(summary.counter_count(), 0);
    assert_eq!(summary.accumulation_count(), 0);
}

#[test]
fn loop_update_nested_scope_keeps_current_if_branch_assignment() {
    let names = vec!["i".to_string()];
    let loop_body = vec![if_with_updates(
        var("cond"),
        vec![assign("i", add(var("i"), lit_i(1)))],
        None,
    )];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert_eq!(summary.counter_count(), 1);
    assert_eq!(summary.accumulation_count(), 0);
}

#[test]
fn loop_update_multi_assignment_rejects_conflicting_updates() {
    let names = vec!["i".to_string()];
    let loop_body = vec![assign("i", lit_i(0)), assign("i", add(var("i"), lit_i(1)))];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert_eq!(summary.carriers.len(), 1);
    assert_eq!(summary.carriers[0].kind, UpdateKind::Other);
    assert_eq!(summary.counter_count(), 0);
    assert_eq!(summary.accumulation_count(), 0);
}

#[test]
fn loop_update_multi_assignment_rejects_mixed_update_kinds() {
    let names = vec!["i".to_string()];
    let loop_body = vec![
        assign("i", add(var("i"), lit_i(1))),
        assign("i", add(var("i"), lit_i(2))),
    ];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert_eq!(summary.carriers.len(), 1);
    assert_eq!(summary.carriers[0].kind, UpdateKind::Other);
    assert_eq!(summary.counter_count(), 0);
    assert_eq!(summary.accumulation_count(), 0);
}

#[test]
fn loop_update_multi_assignment_accepts_agreeing_if_branches() {
    let names = vec!["sum".to_string()];
    let loop_body = vec![if_with_updates(
        var("cond"),
        vec![assign("sum", add(var("sum"), lit_i(1)))],
        Some(vec![assign("sum", add(var("sum"), lit_i(2)))]),
    )];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert_eq!(summary.counter_count(), 0);
    assert_eq!(summary.accumulation_count(), 1);
}

#[test]
fn loop_update_assignment_value_ignores_nested_assignment_expression() {
    let names = vec!["i".to_string()];
    let loop_body = vec![assign("other", assign("i", add(var("i"), lit_i(1))))];

    let summary = analyze_loop_updates_from_ast(&names, &loop_body);

    assert!(summary.carriers.is_empty());
    assert_eq!(summary.counter_count(), 0);
    assert_eq!(summary.accumulation_count(), 0);
}
