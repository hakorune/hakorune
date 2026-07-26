use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::{MirBuilder, MirInstruction};

use super::input_view::{
    RawLegacyBodyInputV1, RawLegacyExpressionInputV1, RawLegacyStatementInputV1,
};
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use crate::mir::builder::stmts::block_stmt::{
    build_block, build_block_input_view_with_port_v1, build_statement,
    build_statement_input_view_with_port_v1,
};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
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

fn instructions(builder: &MirBuilder) -> Vec<MirInstruction> {
    builder
        .function_state
        .current_function
        .as_ref()
        .expect("current function")
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter().cloned())
        .collect()
}

#[test]
fn legacy_input_view_and_legacy_facade_share_one_matcher_behavior() {
    let expression = add(integer(1), add(integer(2), integer(3)));

    let mut facade = MirBuilder::new();
    facade.enter_function_for_test("raw_input_view/0".to_string());
    let facade_value = facade.build_expression_impl(expression.clone()).unwrap();

    let mut view = MirBuilder::new();
    view.enter_function_for_test("raw_input_view/0".to_string());
    let mut port = RawLegacyChildLoweringPortV1;
    let view_value = view
        .build_expression_input_view_with_port_v1(
            &mut port,
            RawLegacyExpressionInputV1::new(expression),
        )
        .unwrap();

    assert_eq!(view_value, facade_value);
    assert_eq!(instructions(&view), instructions(&facade));
}

#[test]
fn legacy_body_and_statement_facades_preserve_input_view_parity() {
    let body = vec![integer(1), add(integer(2), integer(3))];

    let mut body_facade = MirBuilder::new();
    body_facade.enter_function_for_test("raw_input_body/0".to_string());
    let facade_body_value = build_block(&mut body_facade, body.clone()).unwrap();

    let mut body_view = MirBuilder::new();
    body_view.enter_function_for_test("raw_input_body/0".to_string());
    let mut body_port = RawLegacyChildLoweringPortV1;
    let view_body_value = build_block_input_view_with_port_v1(
        &mut body_view,
        &mut body_port,
        RawLegacyBodyInputV1::new(body),
    )
    .unwrap();

    assert_eq!(view_body_value, facade_body_value);
    assert_eq!(instructions(&body_view), instructions(&body_facade));

    let statement = add(integer(4), integer(5));
    let mut statement_facade = MirBuilder::new();
    statement_facade.enter_function_for_test("raw_input_statement/0".to_string());
    let facade_statement_value = build_statement(&mut statement_facade, statement.clone()).unwrap();

    let mut statement_view = MirBuilder::new();
    statement_view.enter_function_for_test("raw_input_statement/0".to_string());
    let mut statement_port = RawLegacyChildLoweringPortV1;
    let view_statement_value = build_statement_input_view_with_port_v1(
        &mut statement_view,
        &mut statement_port,
        RawLegacyStatementInputV1::new(statement),
    )
    .unwrap();

    assert_eq!(view_statement_value, facade_statement_value);
    assert_eq!(
        instructions(&statement_view),
        instructions(&statement_facade)
    );
}
