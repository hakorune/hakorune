use super::super::{try_build_loop_facts, LoopFacts};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::facts::skeleton_facts::SkeletonKind;

fn v(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn lit_int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

#[test]
fn loop_facts_require_skeleton_and_features_when_present() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(v("i")),
        right: Box::new(lit_int(3)),
        span: Span::unknown(),
    };
    let body = vec![ASTNode::Assignment {
        target: Box::new(v("i")),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }];

    let facts = try_build_loop_facts(&condition, &body)
        .expect("Ok")
        .expect("Some");
    assert_eq!(facts.skeleton.kind, SkeletonKind::Loop);
    assert!(!facts.features.exit_usage.has_break);
    assert!(!facts.features.exit_usage.has_continue);
    assert!(!facts.features.exit_usage.has_return);
    let _: LoopFacts = facts;
}
