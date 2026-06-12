use super::try_extract_loop_cond_return_in_body_facts;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn string(value: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(value.to_string()),
        span: Span::unknown(),
    }
}

fn binop(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn assign(target: ASTNode, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(target),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn local(name: &str, init: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_string()],
        initial_values: vec![Some(Box::new(init))],
        declared_type_names: Vec::new(),
        span: Span::unknown(),
    }
}

#[test]
fn return_in_body_brace_balance_shape_matches() {
    std::env::set_var("NYASH_JOINIR_DEV", "1");

    let condition = binop(BinaryOperator::Less, var("i"), var("n"));
    let inner_return = ASTNode::If {
        condition: Box::new(binop(BinaryOperator::Equal, var("depth"), int(0))),
        then_body: vec![ASTNode::Return {
            value: Some(Box::new(int(0))),
            span: Span::unknown(),
        }],
        else_body: None,
        span: Span::unknown(),
    };
    let else_if = ASTNode::If {
        condition: Box::new(binop(BinaryOperator::Equal, var("ch"), string("}"))),
        then_body: vec![assign(var("depth"), int(0)), inner_return],
        else_body: None,
        span: Span::unknown(),
    };
    let body = vec![
        local("depth", int(0)),
        ASTNode::If {
            condition: Box::new(binop(BinaryOperator::Equal, var("ch"), string("{"))),
            then_body: vec![assign(var("depth"), int(1))],
            else_body: Some(vec![else_if]),
            span: Span::unknown(),
        },
        assign(var("i"), binop(BinaryOperator::Add, var("i"), int(1))),
    ];

    let facts = try_extract_loop_cond_return_in_body_facts(&condition, &body)
        .expect("extract ok")
        .expect("facts");

    assert_eq!(facts.recipe.body.body.len(), 3);
}

#[test]
fn return_in_body_rejects_continue() {
    std::env::set_var("NYASH_JOINIR_DEV", "1");

    let condition = binop(BinaryOperator::Less, var("i"), var("n"));
    let body = vec![
        local("depth", int(0)),
        ASTNode::If {
            condition: Box::new(binop(BinaryOperator::Equal, var("ch"), string("{"))),
            then_body: vec![assign(var("depth"), int(1))],
            else_body: Some(vec![ASTNode::Continue {
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        },
        assign(var("i"), binop(BinaryOperator::Add, var("i"), int(1))),
    ];

    let facts = try_extract_loop_cond_return_in_body_facts(&condition, &body).expect("extract ok");
    assert!(facts.is_none());
}

#[test]
fn return_in_body_rejects_nested_loop() {
    std::env::set_var("NYASH_JOINIR_DEV", "1");

    let condition = binop(BinaryOperator::Less, var("i"), var("n"));
    let body = vec![
        local("depth", int(0)),
        ASTNode::Loop {
            condition: Box::new(binop(BinaryOperator::Less, var("j"), var("n"))),
            body: vec![ASTNode::Return {
                value: Some(Box::new(int(0))),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
        assign(var("i"), binop(BinaryOperator::Add, var("i"), int(1))),
    ];

    let facts = try_extract_loop_cond_return_in_body_facts(&condition, &body).expect("extract ok");
    assert!(facts.is_none());
}

#[test]
fn simple_if_return_then_step_shape_detects() {
    let body = vec![
        ASTNode::If {
            condition: Box::new(var("found")),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(var("j"))),
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        },
        assign(var("j"), binop(BinaryOperator::Add, var("j"), int(1))),
    ];

    assert!(super::matches_simple_if_return_then_step_shape(&body).expect("shape check"));
}

#[test]
fn return_in_body_simple_if_return_then_step_matches_in_dev_mode() {
    std::env::set_var("NYASH_JOINIR_DEV", "1");

    let condition = binop(
        BinaryOperator::LessEqual,
        binop(BinaryOperator::Add, var("j"), var("m")),
        var("n"),
    );
    let body = vec![
        ASTNode::If {
            condition: Box::new(var("found")),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(var("j"))),
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        },
        assign(var("j"), binop(BinaryOperator::Add, var("j"), int(1))),
    ];

    let facts = try_extract_loop_cond_return_in_body_facts(&condition, &body)
        .expect("extract ok")
        .expect("facts");
    assert_eq!(facts.recipe.body.body.len(), 2);
}

#[test]
fn return_in_body_simple_if_return_then_step_with_method_call_condition_matches() {
    std::env::set_var("NYASH_JOINIR_DEV", "1");

    let condition = binop(
        BinaryOperator::LessEqual,
        binop(BinaryOperator::Add, var("j"), var("m")),
        var("n"),
    );
    let body = vec![
        ASTNode::If {
            condition: Box::new(ASTNode::MethodCall {
                object: Box::new(var("me")),
                method: "starts_with".to_string(),
                arguments: vec![var("src"), var("j"), var("pat")],
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(var("j"))),
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        },
        assign(var("j"), binop(BinaryOperator::Add, var("j"), int(1))),
    ];

    let facts = try_extract_loop_cond_return_in_body_facts(&condition, &body)
        .expect("extract ok")
        .expect("facts");
    assert_eq!(facts.recipe.body.body.len(), 2);
}

#[test]
fn return_in_body_if_else_if_return_shape_matches() {
    std::env::set_var("NYASH_JOINIR_DEV", "1");

    let condition = binop(BinaryOperator::Less, var("v"), int(1));
    let body = vec![ASTNode::If {
        condition: Box::new(binop(BinaryOperator::Equal, var("v"), int(0))),
        then_body: vec![ASTNode::Return {
            value: Some(Box::new(int(0))),
            span: Span::unknown(),
        }],
        else_body: Some(vec![ASTNode::If {
            condition: Box::new(binop(BinaryOperator::Equal, var("v"), int(1))),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(int(1))),
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        }]),
        span: Span::unknown(),
    }];

    let facts = try_extract_loop_cond_return_in_body_facts(&condition, &body)
        .expect("extract ok")
        .expect("facts");
    assert_eq!(facts.recipe.body.body.len(), 1);
}
