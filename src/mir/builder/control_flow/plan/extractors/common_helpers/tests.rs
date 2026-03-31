use super::*;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

fn make_break() -> ASTNode {
    ASTNode::Break {
        span: Span::unknown(),
    }
}

fn make_continue() -> ASTNode {
    ASTNode::Continue {
        span: Span::unknown(),
    }
}

fn make_return() -> ASTNode {
    ASTNode::Return {
        value: None,
        span: Span::unknown(),
    }
}

fn make_if_with_break() -> ASTNode {
    ASTNode::If {
        condition: Box::new(ASTNode::Variable {
            name: "cond".to_string(),
            span: Span::unknown(),
        }),
        then_body: vec![make_break()],
        else_body: None,
        span: Span::unknown(),
    }
}

fn make_nested_loop() -> ASTNode {
    ASTNode::Loop {
        condition: Box::new(ASTNode::Variable {
            name: "cond".to_string(),
            span: Span::unknown(),
        }),
        body: vec![make_break()],
        span: Span::unknown(),
    }
}

#[test]
fn test_count_control_flow_break() {
    let body = vec![make_break()];
    let counts = count_control_flow(&body, ControlFlowDetector::default());
    assert_eq!(counts.break_count, 1);
    assert_eq!(counts.continue_count, 0);
}

#[test]
fn test_has_break_statement() {
    let body = vec![make_if_with_break()];
    assert!(has_break_statement(&body));
}

#[test]
fn test_has_continue_statement() {
    let body = vec![make_continue()];
    assert!(has_continue_statement(&body));
}

#[test]
fn test_has_continue_statement_false() {
    let body = vec![make_break()];
    assert!(!has_continue_statement(&body));
}

#[test]
fn test_has_return_statement() {
    let body = vec![make_return()];
    assert!(has_return_statement(&body));
}

#[test]
fn test_has_return_statement_false() {
    let body = vec![make_break()];
    assert!(!has_return_statement(&body));
}

#[test]
fn test_has_control_flow_statement_break() {
    let body = vec![make_if_with_break()];
    assert!(has_control_flow_statement(&body));
}

#[test]
fn test_has_control_flow_statement_continue() {
    let body = vec![make_continue()];
    assert!(has_control_flow_statement(&body));
}

#[test]
fn test_has_control_flow_statement_false() {
    let body = vec![ASTNode::Variable {
        name: "x".to_string(),
        span: Span::unknown(),
    }];
    assert!(!has_control_flow_statement(&body));
}

#[test]
fn test_count_control_flow_detects_nested_loop_at_top_level() {
    let body = vec![make_nested_loop()];
    let counts = count_control_flow(&body, ControlFlowDetector::default());
    assert!(counts.has_nested_loop);
}

#[test]
fn test_count_control_flow_detects_nested_loop_in_if() {
    let body = vec![ASTNode::If {
        condition: Box::new(ASTNode::Variable {
            name: "cond".to_string(),
            span: Span::unknown(),
        }),
        then_body: vec![make_nested_loop()],
        else_body: None,
        span: Span::unknown(),
    }];
    let counts = count_control_flow(&body, ControlFlowDetector::default());
    assert!(counts.has_nested_loop);
}

#[test]
fn test_has_control_flow_statement_break_in_scopebox() {
    let body = vec![ASTNode::ScopeBox {
        body: vec![make_break()],
        span: Span::unknown(),
    }];
    assert!(has_control_flow_statement(&body));
}

#[test]
fn test_extract_loop_variable_success() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(ASTNode::Variable {
            name: "i".to_string(),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(10),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    assert_eq!(extract_loop_variable(&condition), Some("i".to_string()));
}

#[test]
fn test_extract_loop_variable_not_comparison() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(ASTNode::Variable {
            name: "i".to_string(),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(10),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    assert_eq!(extract_loop_variable(&condition), None);
}

#[test]
fn test_extract_loop_variable_not_variable() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(5),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(10),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    assert_eq!(extract_loop_variable(&condition), None);
}

#[test]
fn test_is_true_literal_success() {
    let condition = ASTNode::Literal {
        value: LiteralValue::Bool(true),
        span: Span::unknown(),
    };

    assert!(is_true_literal(&condition));
}

#[test]
fn test_is_true_literal_false() {
    let condition = ASTNode::Literal {
        value: LiteralValue::Bool(false),
        span: Span::unknown(),
    };

    assert!(!is_true_literal(&condition));
}

#[test]
fn test_is_true_literal_not_literal() {
    let condition = ASTNode::Variable {
        name: "x".to_string(),
        span: Span::unknown(),
    };

    assert!(!is_true_literal(&condition));
}

#[test]
fn test_validate_continue_at_end_success() {
    let body = vec![make_break(), make_continue()];
    assert!(validate_continue_at_end(&body));
}

#[test]
fn test_validate_continue_at_end_false() {
    let body = vec![make_continue(), make_break()];
    assert!(!validate_continue_at_end(&body));
}

#[test]
fn test_validate_break_in_simple_if_success() {
    let body = vec![make_if_with_break()];
    assert!(validate_break_in_simple_if(&body));
}

#[test]
fn test_validate_break_in_simple_if_with_else() {
    let body = vec![ASTNode::If {
        condition: Box::new(ASTNode::Variable {
            name: "done".to_string(),
            span: Span::unknown(),
        }),
        then_body: vec![make_break()],
        else_body: Some(vec![make_continue()]),
        span: Span::unknown(),
    }];
    assert!(!validate_break_in_simple_if(&body));
}

#[test]
fn test_validate_break_in_simple_if_multiple_statements() {
    let body = vec![ASTNode::If {
        condition: Box::new(ASTNode::Variable {
            name: "done".to_string(),
            span: Span::unknown(),
        }),
        then_body: vec![make_break(), make_continue()],
        else_body: None,
        span: Span::unknown(),
    }];
    assert!(!validate_break_in_simple_if(&body));
}

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn lit_i(v: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(v),
        span: Span::unknown(),
    }
}

fn assign(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(var(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

#[test]
fn test_extract_loop_increment_plan_uses_tail_assignment_fallback() {
    let body = vec![
        ASTNode::MethodCall {
            object: Box::new(var("arr")),
            method: "push".to_string(),
            arguments: vec![var("i")],
            span: Span::unknown(),
        },
        assign(
            "i",
            ASTNode::MethodCall {
                object: Box::new(var("arr")),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            },
        ),
    ];

    let inc = extract_loop_increment_plan(&body, "i")
        .expect("no error")
        .expect("must extract fallback step");
    assert!(matches!(inc, ASTNode::MethodCall { .. }));
}

#[test]
fn test_extract_loop_increment_plan_ignores_non_tail_assignment() {
    let body = vec![
        assign("i", lit_i(1)),
        ASTNode::MethodCall {
            object: Box::new(var("arr")),
            method: "push".to_string(),
            arguments: vec![var("i")],
            span: Span::unknown(),
        },
    ];

    let inc = extract_loop_increment_plan(&body, "i").expect("no error");
    assert!(inc.is_none());
}

#[test]
fn test_extract_loop_increment_plan_ignores_tail_other_var() {
    let body = vec![
        ASTNode::MethodCall {
            object: Box::new(var("arr")),
            method: "push".to_string(),
            arguments: vec![var("i")],
            span: Span::unknown(),
        },
        assign("j", lit_i(1)),
    ];

    let inc = extract_loop_increment_plan(&body, "i").expect("no error");
    assert!(inc.is_none());
}

