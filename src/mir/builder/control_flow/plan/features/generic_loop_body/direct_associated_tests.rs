use super::direct_associated::lower_direct_body_input;
use crate::ast::{ASTNode, Span};
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::control_flow::plan::RawLoopPlanExpressionPortV1;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

#[test]
fn direct_core_accepts_empty_raw_prefix() {
    let port = RawLoopPlanExpressionPortV1::new();
    let body: &[ASTNode] = &[];
    let mut builder = MirBuilder::new();
    let mut bindings = BTreeMap::new();
    let plans = lower_direct_body_input(
        &mut builder,
        &mut bindings,
        &port,
        body,
        &BTreeMap::new(),
        "i",
        "[test/direct-associated]",
    )
    .expect("empty direct prefix is accepted");
    assert!(plans.is_empty());
}

#[test]
fn direct_core_rejects_raw_only_tail_before_later_input() {
    let port = RawLoopPlanExpressionPortV1::new();
    let body = vec![
        ASTNode::Print {
            expression: Box::new(ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        },
        ASTNode::Return {
            value: None,
            span: Span::unknown(),
        },
    ];
    let mut builder = MirBuilder::new();
    let mut bindings = BTreeMap::new();
    let error = lower_direct_body_input(
        &mut builder,
        &mut bindings,
        &port,
        body.as_slice(),
        &BTreeMap::new(),
        "i",
        "[test/direct-associated]",
    )
    .expect_err("raw-only Print stays outside the associated core");
    assert!(error.contains("unsupported associated direct statement"));
}

#[test]
fn direct_core_stops_after_return() {
    let port = RawLoopPlanExpressionPortV1::new();
    let body = vec![
        ASTNode::Return {
            value: None,
            span: Span::unknown(),
        },
        ASTNode::Print {
            expression: Box::new(ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(2),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        },
    ];
    let mut builder = MirBuilder::new();
    let mut bindings = BTreeMap::new();
    let plans = lower_direct_body_input(
        &mut builder,
        &mut bindings,
        &port,
        body.as_slice(),
        &BTreeMap::new(),
        "i",
        "[test/direct-associated]",
    )
    .expect("return is a supported terminal statement");
    assert!(matches!(plans.as_slice(), [LoweredRecipe::Exit(_)]));
}
