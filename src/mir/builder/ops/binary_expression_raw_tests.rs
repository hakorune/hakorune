use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::{BinaryOp, CompareOp, MirBuilder, MirInstruction, MirType, TypeOpKind};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn boolean(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
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

fn instructions(builder: &MirBuilder) -> Vec<MirInstruction> {
    builder
        .function_state
        .current_function
        .as_ref()
        .expect("current BIN0-I0 function")
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter().cloned())
        .collect()
}

#[test]
fn raw_ordinary_binary_entry_preserves_left_right_and_existing_terminal() {
    let mut builder = builder("binary_raw_add/0");

    let output = builder
        .build_expression(binary(BinaryOperator::Add, integer(7), integer(3)))
        .unwrap();

    let rows = instructions(&builder);
    let constants = rows
        .iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Const {
                value: crate::mir::ConstValue::Integer(value),
                ..
            } => Some(*value),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(
        constants.starts_with(&[7, 3]),
        "source children must materialize left before right: {constants:?}"
    );
    assert!(rows.iter().any(|instruction| matches!(
        instruction,
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            ..
        } if *dst == output
    )));
}

#[test]
fn raw_ordinary_binary_accepts_method_calls_on_both_sides() {
    let mut builder = builder("binary_raw_method_sides/0");

    let output = builder
        .build_expression(binary(
            BinaryOperator::Equal,
            type_check(integer(1)),
            type_check(integer(2)),
        ))
        .unwrap();

    let rows = instructions(&builder);
    assert_eq!(
        rows.iter()
            .filter(|instruction| matches!(
                instruction,
                MirInstruction::TypeOp {
                    op: TypeOpKind::Check,
                    ..
                }
            ))
            .count(),
        2
    );
    assert!(rows.iter().any(|instruction| matches!(
        instruction,
        MirInstruction::Compare {
            dst,
            op: CompareOp::Eq,
            ..
        } if *dst == output
    )));
    assert_eq!(
        builder.function_state.type_ctx.value_types.get(&output),
        Some(&MirType::Bool)
    );
}

#[test]
fn nested_raw_ordinary_binary_restores_depth_and_allows_reuse() {
    let mut expression = integer(4);
    for value in (0..4).rev() {
        expression = binary(BinaryOperator::Add, integer(value), expression);
    }
    let mut builder = builder("binary_raw_nested/0");

    builder.build_expression(expression).unwrap();
    assert_eq!(builder.recursion_depth, 0);

    assert!(builder
        .build_expression(binary(
            BinaryOperator::Add,
            variable("missing"),
            integer(99),
        ))
        .is_err());
    assert_eq!(builder.recursion_depth, 0);

    builder
        .build_expression(binary(BinaryOperator::Add, integer(5), integer(6)))
        .unwrap();
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn raw_ordinary_binary_failure_stops_later_child_or_terminal() {
    let mut left_failure = builder("binary_raw_left_failure/0");
    assert!(left_failure
        .build_expression(binary(
            BinaryOperator::Add,
            variable("missing_left"),
            integer(91),
        ))
        .is_err());
    assert!(instructions(&left_failure).is_empty());

    let mut right_failure = builder("binary_raw_right_failure/0");
    assert!(right_failure
        .build_expression(binary(
            BinaryOperator::Add,
            integer(17),
            variable("missing_right"),
        ))
        .is_err());
    let rows = instructions(&right_failure);
    assert_eq!(
        rows.iter()
            .filter(|instruction| matches!(instruction, MirInstruction::Const { .. }))
            .count(),
        1
    );
    assert!(!rows
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::BinOp { .. })));
}

#[test]
fn logical_operators_remain_on_existing_short_circuit_owner() {
    for operator in [BinaryOperator::And, BinaryOperator::Or] {
        let mut builder = builder("binary_raw_logical_owner/0");
        let output = builder
            .build_expression(binary(operator, boolean(false), boolean(true)))
            .unwrap();

        assert_eq!(
            builder.function_state.type_ctx.value_types.get(&output),
            Some(&MirType::Bool)
        );
        assert!(instructions(&builder).iter().any(
            |instruction| matches!(instruction, MirInstruction::Phi { dst, .. } if *dst == output)
        ));
    }
}

#[test]
fn raw_binary_child_depth_failure_restores_parent_depth() {
    let _ = std::panic::catch_unwind(|| {
        crate::runtime::ring0::init_global_ring0(crate::runtime::ring0::default_ring0())
    });
    let mut builder = builder("binary_raw_depth_failure/0");
    builder.recursion_depth = 199;

    let error = builder
        .build_expression(binary(BinaryOperator::Add, integer(1), integer(2)))
        .unwrap_err();
    assert!(error.contains("Recursion depth exceeded: 201"));
    assert_eq!(builder.recursion_depth, 199);

    builder.recursion_depth = 0;
    builder
        .build_expression(binary(BinaryOperator::Add, integer(3), integer(4)))
        .unwrap();
    assert_eq!(builder.recursion_depth, 0);
}
