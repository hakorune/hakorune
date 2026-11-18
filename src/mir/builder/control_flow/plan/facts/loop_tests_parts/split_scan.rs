use super::super::try_build_loop_facts;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

fn v(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}


#[test]
fn loopfacts_ok_some_for_loop_cond_break_continue_when_split_scan_missing_else() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::LessEqual,
        left: Box::new(v("i")),
        right: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Subtract,
            left: Box::new(ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(v("separator")),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let if_stmt = ASTNode::If {
        condition: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "substring".to_string(),
                arguments: vec![
                    v("i"),
                    ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(v("i")),
                        right: Box::new(ASTNode::MethodCall {
                            object: Box::new(v("separator")),
                            method: "length".to_string(),
                            arguments: vec![],
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    },
                ],
                span: Span::unknown(),
            }),
            right: Box::new(v("separator")),
            span: Span::unknown(),
        }),
        then_body: vec![ASTNode::MethodCall {
            object: Box::new(v("result")),
            method: "push".to_string(),
            arguments: vec![ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "substring".to_string(),
                arguments: vec![v("start"), v("i")],
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }],
        else_body: None,
        span: Span::unknown(),
    };

    let facts = try_build_loop_facts(&condition, &[if_stmt]).expect("Ok");
    let facts = facts.expect("Some");
    assert!(facts.loop_cond_break_continue.is_some());
    assert!(facts.scan_with_init.is_none());
    assert!(facts.split_scan.is_none());
}
