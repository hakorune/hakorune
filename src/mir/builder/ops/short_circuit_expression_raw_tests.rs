use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::loop_api::LoopBuilderApi;
use crate::mir::{BasicBlockId, MirBuilder, MirInstruction, MirType};

use super::super::recursive_child_lowering::drive_raw_legacy_expression_v1;

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

fn builder(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn instructions(builder: &MirBuilder) -> Vec<(BasicBlockId, MirInstruction)> {
    builder
        .function_state
        .current_function
        .as_ref()
        .expect("current SC0-I0 function")
        .blocks
        .iter()
        .flat_map(|(block, data)| {
            data.instructions
                .iter()
                .cloned()
                .map(|instruction| (*block, instruction))
        })
        .collect()
}

#[test]
fn raw_short_circuit_selector_preserves_and_or_completion() {
    for operator in [BinaryOperator::And, BinaryOperator::Or] {
        let mut builder = builder(&format!("sc0_raw_operator/{operator}"));
        let output = drive_raw_legacy_expression_v1(
            &mut builder,
            binary(operator, boolean(false), boolean(true)),
        )
        .unwrap();

        assert_eq!(
            builder.function_state.type_ctx.value_types.get(&output),
            Some(&MirType::Bool)
        );
        assert!(instructions(&builder).iter().any(
            |(_, instruction)| matches!(instruction, MirInstruction::Phi { dst, .. } if *dst == output)
        ));
    }
}

#[test]
fn raw_rhs_is_materialized_only_inside_the_eval_block() {
    let mut builder = builder("sc0_raw_rhs_block/0");
    let entry = builder.current_block().unwrap();

    drive_raw_legacy_expression_v1(
        &mut builder,
        binary(BinaryOperator::And, boolean(true), boolean(false)),
    )
    .unwrap();

    let bool_constants = instructions(&builder)
        .into_iter()
        .filter_map(|(block, instruction)| match instruction {
            MirInstruction::Const {
                value: crate::mir::ConstValue::Bool(value),
                ..
            } => Some((block, value)),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(bool_constants
        .iter()
        .any(|(block, value)| *block == entry && *value));
    assert!(
        bool_constants
            .iter()
            .any(|(block, value)| *block != entry && !*value),
        "raw RHS false literal must be emitted outside the entry block: {bool_constants:?}"
    );
}

#[test]
fn raw_lhs_failure_stops_before_short_circuit_cfg() {
    let mut builder = builder("sc0_raw_lhs_failure/0");
    let before_blocks = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .len();

    let error = drive_raw_legacy_expression_v1(
        &mut builder,
        binary(BinaryOperator::And, variable("missing_lhs"), boolean(true)),
    )
    .unwrap_err();

    assert!(error.contains("Undefined variable: missing_lhs"));
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .len(),
        before_blocks
    );
    assert!(instructions(&builder).is_empty());
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn raw_rhs_failure_occurs_after_entering_eval_block() {
    let mut builder = builder("sc0_raw_rhs_failure/0");

    let error = drive_raw_legacy_expression_v1(
        &mut builder,
        binary(BinaryOperator::Or, boolean(false), variable("missing_rhs")),
    )
    .unwrap_err();

    assert!(error.contains("Undefined variable: missing_rhs"));
    assert!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .len()
            > 1
    );
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn ordinary_binary_remains_on_bin0_after_short_circuit_cutover() {
    let mut builder = builder("sc0_raw_ordinary_control/0");
    let output = drive_raw_legacy_expression_v1(
        &mut builder,
        binary(BinaryOperator::Add, integer(2), integer(3)),
    )
    .unwrap();

    assert!(instructions(&builder).iter().any(
        |(_, instruction)| matches!(instruction, MirInstruction::BinOp { dst, .. } if *dst == output)
    ));
    assert!(!instructions(&builder)
        .iter()
        .any(|(_, instruction)| matches!(instruction, MirInstruction::Phi { .. })));
}

#[test]
fn failed_raw_short_circuit_allows_a_fresh_builder() {
    let mut failed = builder("sc0_raw_failed/0");
    assert!(drive_raw_legacy_expression_v1(
        &mut failed,
        binary(BinaryOperator::And, variable("missing"), boolean(true),),
    )
    .is_err());

    let mut fresh = builder("sc0_raw_fresh/0");
    let output = drive_raw_legacy_expression_v1(
        &mut fresh,
        binary(BinaryOperator::And, boolean(true), boolean(true)),
    )
    .unwrap();
    assert_eq!(
        fresh.function_state.type_ctx.value_types.get(&output),
        Some(&MirType::Bool)
    );
    assert_eq!(fresh.recursion_depth, 0);
}
