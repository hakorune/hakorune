use super::super::try_build_loop_facts_with_ctx;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::planner::PlannerContext;
use crate::mir::loop_route_detection::LoopRouteKind;

fn v(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

#[test]
fn loopfacts_ctx_keeps_simple_while_route_even_when_kind_mismatch() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(v("i")),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(3),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let step = ASTNode::Assignment {
        target: Box::new(v("i")),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let ctx = PlannerContext {
        route_kind: Some(LoopRouteKind::LoopBreak),
        in_static_box: false,
        debug: false,
    };

    let facts = try_build_loop_facts_with_ctx(&ctx, &condition, &[step]).expect("Ok");
    let facts = facts.expect("Some");
    assert!(facts.loop_simple_while.is_some());
}

#[test]
fn loopfacts_ctx_allows_simple_while_route_when_kind_matches() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(v("i")),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(3),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let step = ASTNode::Assignment {
        target: Box::new(v("i")),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let ctx = PlannerContext {
        route_kind: Some(LoopRouteKind::LoopSimpleWhile),
        in_static_box: false,
        debug: false,
    };

    let facts = try_build_loop_facts_with_ctx(&ctx, &condition, &[step]).expect("Ok");
    let facts = facts.expect("Some");
    assert!(facts.loop_simple_while.is_some());
}

#[test]
fn loopfacts_ctx_allows_bool_predicate_scan_route_in_static_box() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(v("i")),
        right: Box::new(ASTNode::MethodCall {
            object: Box::new(v("s")),
            method: "length".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let predicate_if = ASTNode::If {
        condition: Box::new(ASTNode::UnaryOp {
            operator: crate::ast::UnaryOperator::Not,
            operand: Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::This {
                    span: Span::unknown(),
                }),
                method: "is_digit".to_string(),
                arguments: vec![ASTNode::MethodCall {
                    object: Box::new(v("s")),
                    method: "substring".to_string(),
                    arguments: vec![
                        v("i"),
                        ASTNode::BinaryOp {
                            operator: BinaryOperator::Add,
                            left: Box::new(v("i")),
                            right: Box::new(ASTNode::Literal {
                                value: LiteralValue::Integer(1),
                                span: Span::unknown(),
                            }),
                            span: Span::unknown(),
                        },
                    ],
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
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let body = vec![predicate_if, step];
    let allow_ctx = PlannerContext {
        route_kind: None,
        in_static_box: false,
        debug: false,
    };
    let block_ctx = PlannerContext {
        route_kind: None,
        in_static_box: true,
        debug: false,
    };

    let allow = try_build_loop_facts_with_ctx(&allow_ctx, &condition, &body).expect("Ok");
    assert!(allow
        .as_ref()
        .and_then(|facts| facts.bool_predicate_scan.as_ref())
        .is_some());

    let allow_static = try_build_loop_facts_with_ctx(&block_ctx, &condition, &body).expect("Ok");
    assert!(allow_static
        .as_ref()
        .and_then(|facts| facts.bool_predicate_scan.as_ref())
        .is_some());
}

#[test]
fn loopfacts_ok_none_when_condition_not_supported() {
    let condition = v("i"); // not `i < n`
    let facts = try_build_loop_facts_with_ctx(
        &PlannerContext {
            route_kind: None,
            in_static_box: false,
            debug: false,
        },
        &condition,
        &[],
    )
    .expect("Ok");
    assert!(facts.is_none());
}

#[test]
fn loopfacts_ok_none_when_step_var_differs_from_condition_var() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(v("i")),
        right: Box::new(ASTNode::MethodCall {
            object: Box::new(v("s")),
            method: "length".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let step = ASTNode::Assignment {
        target: Box::new(v("j")),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("j")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let facts = try_build_loop_facts_with_ctx(
        &PlannerContext {
            route_kind: None,
            in_static_box: false,
            debug: false,
        },
        &condition,
        &[step],
    )
    .expect("Ok");
    assert!(facts.is_none());
}
