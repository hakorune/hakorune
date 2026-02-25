use super::*;
use crate::ast::{BinaryOperator, LiteralValue, Span};

fn lit_i(i: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(i),
        span: Span::unknown(),
    }
}

fn lit_true() -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(true),
        span: Span::unknown(),
    }
}

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

#[test]
fn shadowing_inner_local_does_not_create_relay_write() {
    // local sum=0 (parent)
    // loop(true) { { local sum=1; sum=sum+1 } break }
    // Expected: loop does NOT relay-write parent sum (inner sum shadows it).
    let condition = lit_true();
    let body = vec![
        ASTNode::Program {
            statements: vec![
                ASTNode::Local {
                    variables: vec!["sum".to_string()],
                    initial_values: vec![Some(Box::new(lit_i(1)))],
                    span: Span::unknown(),
                },
                ASTNode::Assignment {
                    target: Box::new(var("sum")),
                    value: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(var("sum")),
                        right: Box::new(lit_i(1)),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                },
            ],
            span: Span::unknown(),
        },
        ASTNode::Break {
            span: Span::unknown(),
        },
    ];

    let plan = analyze_loop(&condition, &body, &["sum".to_string()]).unwrap();
    assert!(
        !plan.relay_writes.iter().any(|r| r.name == "sum"),
        "inner shadowed sum must not produce relay_writes: {:?}",
        plan.relay_writes
    );
}

#[test]
fn writes_to_parent_are_relayed() {
    // local sum=0 (parent)
    // loop(true) { sum = sum + 1; break }
    let condition = lit_true();
    let body = vec![
        ASTNode::Assignment {
            target: Box::new(var("sum")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(var("sum")),
                right: Box::new(lit_i(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        },
        ASTNode::Break {
            span: Span::unknown(),
        },
    ];

    let plan = analyze_loop(&condition, &body, &["sum".to_string()]).unwrap();
    assert!(
        plan.relay_writes.iter().any(|r| r.name == "sum"),
        "expected relay_writes to contain parent sum: {:?}",
        plan.relay_writes
    );
}

#[test]
fn condition_reads_are_marked() {
    // loop(i < n) { break }
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(var("i")),
        right: Box::new(var("n")),
        span: Span::unknown(),
    };
    let body = vec![ASTNode::Break {
        span: Span::unknown(),
    }];

    let plan = analyze_loop(
        &condition,
        &body,
        &["i".to_string(), "n".to_string()],
    )
    .unwrap();

    let names: Vec<_> = plan
        .condition_captures
        .iter()
        .map(|c| c.name.as_str())
        .collect();
    assert!(names.contains(&"i"));
    assert!(names.contains(&"n"));
}

#[test]
fn loop_local_written_is_owned_var() {
    // loop(true) { local tmp = 0; tmp = tmp + 1; break }
    let condition = lit_true();
    let body = vec![
        ASTNode::Local {
            variables: vec!["tmp".to_string()],
            initial_values: vec![Some(Box::new(lit_i(0)))],
            span: Span::unknown(),
        },
        ASTNode::Assignment {
            target: Box::new(var("tmp")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(var("tmp")),
                right: Box::new(lit_i(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        },
        ASTNode::Break {
            span: Span::unknown(),
        },
    ];

    let plan = analyze_loop(&condition, &body, &[]).unwrap();
    assert!(
        plan.owned_vars.iter().any(|o| o.name == "tmp"),
        "tmp should be owned in loop scope: {:?}",
        plan.owned_vars
    );
}
