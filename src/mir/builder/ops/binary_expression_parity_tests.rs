use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::value_kind::MirValueKind;
use crate::mir::{BasicBlockId, MirBuilder, MirInstruction, MirType, ValueId};

use super::super::recursive_child_lowering::with_legacy_expression_recursion_guard_v1;

#[derive(Debug, PartialEq)]
struct BinaryParitySnapshotV1 {
    result: Result<ValueId, String>,
    blocks: Vec<(BasicBlockId, Vec<MirInstruction>, Option<MirInstruction>)>,
    value_types: Vec<(ValueId, MirType)>,
    value_kinds: Vec<(ValueId, MirValueKind)>,
    value_origins: Vec<(ValueId, String)>,
    next_value_id: u32,
    recursion_depth: usize,
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
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

fn type_check(receiver: ASTNode) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(receiver),
        method: "is".to_string(),
        arguments: vec![ASTNode::Literal {
            value: LiteralValue::String("Integer".to_string()),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn builder(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn lower_selected(builder: &mut MirBuilder, expression: ASTNode) -> Result<ValueId, String> {
    builder.build_expression(expression)
}

fn lower_legacy_reference(
    builder: &mut MirBuilder,
    expression: ASTNode,
) -> Result<ValueId, String> {
    let node_kind = std::mem::discriminant(&expression);
    let ASTNode::BinaryOp {
        left,
        operator,
        right,
        ..
    } = expression
    else {
        return Err("BIN0-P0 reference requires BinaryOp".to_string());
    };
    with_legacy_expression_recursion_guard_v1(builder, node_kind, move |builder| {
        let left = builder.build_expression(*left)?;
        let right = builder.build_expression(*right)?;
        builder.build_binary_op_from_values(operator, left, right)
    })
}

fn snapshot(builder: &MirBuilder, result: Result<ValueId, String>) -> BinaryParitySnapshotV1 {
    let function = builder
        .scope_ctx
        .current_function
        .as_ref()
        .expect("current BIN0-P0 function");
    let mut blocks = function
        .blocks
        .iter()
        .map(|(id, block)| (*id, block.instructions.clone(), block.terminator.clone()))
        .collect::<Vec<_>>();
    blocks.sort_by_key(|(id, _, _)| *id);

    let mut value_kinds = builder
        .type_ctx
        .value_kinds
        .iter()
        .map(|(value, kind)| (*value, *kind))
        .collect::<Vec<_>>();
    value_kinds.sort_by_key(|(value, _)| *value);

    BinaryParitySnapshotV1 {
        result,
        blocks,
        value_types: builder
            .type_ctx
            .value_types
            .iter()
            .map(|(value, ty)| (*value, ty.clone()))
            .collect(),
        value_kinds,
        value_origins: builder
            .type_ctx
            .value_origin_newbox
            .iter()
            .map(|(value, owner)| (*value, owner.clone()))
            .collect(),
        next_value_id: function.next_value_id,
        recursion_depth: builder.recursion_depth,
    }
}

fn assert_parity(expression: ASTNode) {
    let mut selected = builder("binary_parity/0");
    let selected_result = lower_selected(&mut selected, expression.clone());
    let selected_snapshot = snapshot(&selected, selected_result);

    let mut reference = builder("binary_parity/0");
    let reference_result = lower_legacy_reference(&mut reference, expression);
    let reference_snapshot = snapshot(&reference, reference_result);

    assert_eq!(selected_snapshot, reference_snapshot);
}

#[test]
fn ordinary_operator_matrix_has_exact_legacy_snapshot_parity() {
    let operators = [
        BinaryOperator::Add,
        BinaryOperator::Subtract,
        BinaryOperator::Multiply,
        BinaryOperator::Divide,
        BinaryOperator::Modulo,
        BinaryOperator::BitAnd,
        BinaryOperator::BitOr,
        BinaryOperator::BitXor,
        BinaryOperator::Shl,
        BinaryOperator::Shr,
        BinaryOperator::Equal,
        BinaryOperator::NotEqual,
        BinaryOperator::Less,
        BinaryOperator::Greater,
        BinaryOperator::LessEqual,
        BinaryOperator::GreaterEqual,
    ];
    for operator in operators {
        assert_parity(binary(operator, integer(7), integer(3)));
    }
}

#[test]
fn method_call_on_each_side_has_exact_legacy_snapshot_parity() {
    assert_parity(binary(
        BinaryOperator::Equal,
        type_check(integer(1)),
        integer(2),
    ));
    assert_parity(binary(
        BinaryOperator::Equal,
        integer(1),
        type_check(integer(2)),
    ));
}

#[test]
fn nested_binary_depth_two_through_four_has_exact_legacy_snapshot_parity() {
    for depth in 2..=4 {
        let mut expression = integer(9);
        for value in (0..depth).rev() {
            expression = binary(BinaryOperator::Add, integer(value), expression);
        }
        assert_parity(expression);
    }
}

#[test]
fn child_failures_and_reuse_have_exact_legacy_snapshot_parity() {
    for expression in [
        binary(BinaryOperator::Add, variable("missing_left"), integer(3)),
        binary(BinaryOperator::Add, integer(7), variable("missing_right")),
    ] {
        let mut selected = builder("binary_parity_failure/0");
        let selected_result = lower_selected(&mut selected, expression.clone());
        let selected_failure = snapshot(&selected, selected_result);

        let mut reference = builder("binary_parity_failure/0");
        let reference_result = lower_legacy_reference(&mut reference, expression);
        let reference_failure = snapshot(&reference, reference_result);
        assert_eq!(selected_failure, reference_failure);

        let recovery = binary(BinaryOperator::Add, integer(4), integer(5));
        let selected_recovery = lower_selected(&mut selected, recovery.clone());
        let reference_recovery = lower_legacy_reference(&mut reference, recovery);
        assert_eq!(
            snapshot(&selected, selected_recovery),
            snapshot(&reference, reference_recovery)
        );
    }
}
