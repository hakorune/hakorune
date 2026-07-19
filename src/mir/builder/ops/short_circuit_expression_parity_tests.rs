use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::value_kind::MirValueKind;
use crate::mir::{BasicBlockId, MirBuilder, MirInstruction, MirType, ValueId};

use super::super::recursive_child_lowering::with_legacy_expression_recursion_guard_v1;
use super::logical_shortcircuit::build_logical_shortcircuit_pre_sc0_i0_reference_v1;

#[derive(Debug, PartialEq)]
struct ShortCircuitParitySnapshotV1 {
    result: Result<ValueId, String>,
    blocks: Vec<(BasicBlockId, Vec<MirInstruction>, Option<MirInstruction>)>,
    value_types: Vec<(ValueId, MirType)>,
    value_kinds: Vec<(ValueId, MirValueKind)>,
    value_origins: Vec<(ValueId, String)>,
    variable_map: Vec<(String, ValueId)>,
    pin_slots: Vec<(ValueId, String)>,
    current_block: Option<BasicBlockId>,
    next_value_id: u32,
    recursion_depth: usize,
}

fn boolean(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
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

fn lower_pre_i0_reference(
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
        return Err("SC0-P0 reference requires BinaryOp".to_string());
    };
    with_legacy_expression_recursion_guard_v1(builder, node_kind, move |builder| {
        build_logical_shortcircuit_pre_sc0_i0_reference_v1(builder, *left, operator, *right)
    })
}

fn snapshot(builder: &MirBuilder, result: Result<ValueId, String>) -> ShortCircuitParitySnapshotV1 {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .expect("current SC0-P0 function");
    let mut blocks = function
        .blocks
        .iter()
        .map(|(id, block)| (*id, block.instructions.clone(), block.terminator.clone()))
        .collect::<Vec<_>>();
    blocks.sort_by_key(|(id, _, _)| *id);

    let mut value_kinds = builder
        .function_state
        .type_ctx
        .value_kinds
        .iter()
        .map(|(value, kind)| (*value, *kind))
        .collect::<Vec<_>>();
    value_kinds.sort_by_key(|(value, _)| *value);

    let mut pin_slots = builder
        .function_state
        .pin_slot_names
        .iter()
        .map(|(value, name)| (*value, name.clone()))
        .collect::<Vec<_>>();
    pin_slots.sort_by_key(|(value, _)| *value);

    ShortCircuitParitySnapshotV1 {
        result,
        blocks,
        value_types: builder
            .function_state
            .type_ctx
            .value_types
            .iter()
            .map(|(value, ty)| (*value, ty.clone()))
            .collect(),
        value_kinds,
        value_origins: builder
            .function_state
            .type_ctx
            .value_origin_newbox
            .iter()
            .map(|(value, owner)| (*value, owner.clone()))
            .collect(),
        variable_map: builder
            .function_state
            .variable_ctx
            .variable_map
            .iter()
            .map(|(name, value)| (name.clone(), *value))
            .collect(),
        pin_slots,
        current_block: builder.function_state.current_block,
        next_value_id: function.next_value_id,
        recursion_depth: builder.recursion_depth,
    }
}

fn assert_parity(expression: ASTNode) {
    let mut selected = builder("short_circuit_parity/0");
    let selected_result = lower_selected(&mut selected, expression.clone());
    let selected_snapshot = snapshot(&selected, selected_result);

    let mut reference = builder("short_circuit_parity/0");
    let reference_result = lower_pre_i0_reference(&mut reference, expression);
    let reference_snapshot = snapshot(&reference, reference_result);

    assert_eq!(selected_snapshot, reference_snapshot);
}

#[test]
fn and_or_bool_matrix_has_exact_pre_i0_snapshot_parity() {
    for operator in [BinaryOperator::And, BinaryOperator::Or] {
        for left in [false, true] {
            for right in [false, true] {
                assert_parity(binary(operator.clone(), boolean(left), boolean(right)));
            }
        }
    }
}

#[test]
fn nested_and_or_comparison_tree_has_exact_pre_i0_snapshot_parity() {
    assert_parity(binary(
        BinaryOperator::And,
        binary(BinaryOperator::Equal, integer(3), integer(3)),
        binary(
            BinaryOperator::Or,
            binary(BinaryOperator::Less, integer(1), integer(2)),
            binary(BinaryOperator::Greater, integer(4), integer(9)),
        ),
    ));
}

#[test]
fn method_call_children_have_exact_pre_i0_snapshot_parity() {
    assert_parity(binary(
        BinaryOperator::And,
        type_check(integer(1)),
        type_check(integer(2)),
    ));
    assert_parity(binary(
        BinaryOperator::Or,
        type_check(integer(3)),
        type_check(integer(4)),
    ));
}

#[test]
fn child_failures_and_reuse_have_exact_pre_i0_snapshot_parity() {
    for expression in [
        binary(BinaryOperator::And, variable("missing_left"), boolean(true)),
        binary(
            BinaryOperator::Or,
            boolean(false),
            variable("missing_right"),
        ),
    ] {
        let mut selected = builder("short_circuit_parity_failure/0");
        let selected_result = lower_selected(&mut selected, expression.clone());
        let selected_failure = snapshot(&selected, selected_result);

        let mut reference = builder("short_circuit_parity_failure/0");
        let reference_result = lower_pre_i0_reference(&mut reference, expression);
        let reference_failure = snapshot(&reference, reference_result);
        assert_eq!(selected_failure, reference_failure);

        let recovery = binary(BinaryOperator::And, boolean(true), boolean(false));
        let selected_recovery = lower_selected(&mut selected, recovery.clone());
        let reference_recovery = lower_pre_i0_reference(&mut reference, recovery);
        assert_eq!(
            snapshot(&selected, selected_recovery),
            snapshot(&reference, reference_recovery)
        );
    }
}
