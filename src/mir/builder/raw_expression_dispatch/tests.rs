use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::{MirBuilder, MirInstruction};

use super::input_view::RawLegacyExpressionInputV1;
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;

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
