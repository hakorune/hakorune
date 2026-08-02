//! M1 candidate-abort probe for a Loop failure after route entry.

use super::{MirCompiler, NormalCompileRequestV1};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use std::collections::HashMap;

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}
fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}
fn binary(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}
fn program(statements: Vec<ASTNode>) -> ASTNode {
    ASTNode::Program {
        statements,
        span: Span::unknown(),
    }
}
fn normal_request(ast: ASTNode, source_file: &str) -> NormalCompileRequestV1 {
    NormalCompileRequestV1::for_mir_mode(ast, Some(source_file), HashMap::new()).unwrap()
}
#[test]
fn loop_effect_then_later_failure_discards_candidate_and_reuses_live_compiler() {
    crate::mir::builder::reset_loop_physical_effect_probe();
    let failing = program(vec![
        ASTNode::Local {
            variables: vec!["i".into()],
            initial_values: vec![Some(Box::new(literal(0)))],
            declared_type_names: vec![None],
            span: Span::unknown(),
        },
        ASTNode::Loop {
            condition: Box::new(binary(BinaryOperator::Less, variable("i"), literal(2))),
            body: vec![ASTNode::Assignment {
                target: Box::new(variable("i")),
                value: Box::new(binary(BinaryOperator::Add, variable("i"), literal(1))),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
        ASTNode::Print {
            expression: Box::new(variable("missing")),
            span: Span::unknown(),
        },
    ]);
    let succeeding = program(vec![ASTNode::Local {
        variables: vec!["i".into()],
        initial_values: vec![Some(Box::new(literal(0)))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }]);
    let mut compiler = MirCompiler::with_options(false);
    compiler.builder.set_source_file_hint("live-before.hako");
    compiler.builder.next_value_id();
    compiler.builder.next_block_id();
    let before = compiler.builder.loop_candidate_test_fingerprint();
    let error = compiler
        .compile_normal(normal_request(failing, "loop-failure.hako"))
        .unwrap_err();
    assert!(error.contains("Undefined variable: missing"), "{error}");
    assert!(
        crate::mir::builder::take_loop_physical_effect_probe() > 0,
        "failure fixture must reach Loop physical frame before the later error"
    );
    assert_eq!(compiler.builder.loop_candidate_test_fingerprint(), before);
    let result = compiler
        .compile_normal(normal_request(succeeding, "loop-reused.hako"))
        .unwrap();
    assert!(result.module.functions.contains_key("main"));
    assert_eq!(
        compiler.builder.current_source_file().as_deref(),
        Some("loop-reused.hako")
    );
}
