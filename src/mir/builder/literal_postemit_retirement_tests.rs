use super::MirBuilder;
use crate::ast::{ASTNode, LiteralValue, Span, UnaryOperator};
use crate::mir::builder::recursive_child_lowering::drive_raw_legacy_expression_v1;
use crate::mir::{ConstValue, MirInstruction, MirType, WeakRefOp};

fn literal(value: LiteralValue) -> ASTNode {
    ASTNode::Literal {
        value,
        span: Span::unknown(),
    }
}

fn unary(operator: UnaryOperator, operand: ASTNode) -> ASTNode {
    ASTNode::UnaryOp {
        operator,
        operand: Box::new(operand),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

fn emitted_consts(builder: &MirBuilder) -> Vec<(crate::mir::ValueId, ConstValue)> {
    builder
        .function_state
        .current_function
        .as_ref()
        .expect("test function must exist")
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|instruction| match instruction {
            MirInstruction::Const { dst, value } => Some((*dst, value.clone())),
            _ => None,
        })
        .collect()
}

#[test]
fn literal_callers_observe_canonical_const_facts_before_returning() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("literal_postemit/0".to_string());

    let fixtures = [
        (LiteralValue::Integer(7), MirType::Integer),
        (
            LiteralValue::TypedInteger {
                value: 8,
                declared_type_name: "i64".to_string(),
            },
            MirType::Integer,
        ),
        (LiteralValue::Float(1.5), MirType::Float),
        (LiteralValue::Bool(true), MirType::Bool),
        (LiteralValue::String("text".to_string()), MirType::String),
        (LiteralValue::Null, MirType::Void),
        (LiteralValue::Void, MirType::Void),
    ];

    let mut values = Vec::new();
    for (literal, expected) in fixtures {
        let value = builder.build_literal(literal).unwrap();
        assert_eq!(
            builder.function_state.type_ctx.get_type(value),
            Some(&expected)
        );
        values.push(value);
    }

    assert_eq!(
        builder
            .function_state
            .type_ctx
            .string_literals
            .get(&values[4]),
        Some(&"text".to_string())
    );
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .metadata
        .exact_numeric_const_facts
        .contains_key(&values[1]));
    assert_eq!(emitted_consts(&builder).len(), values.len());
}

#[test]
fn literal_emission_failure_reaches_no_caller_postpublication() {
    let fixtures = [
        LiteralValue::Integer(7),
        LiteralValue::TypedInteger {
            value: 8,
            declared_type_name: "i64".to_string(),
        },
        LiteralValue::Float(1.5),
        LiteralValue::Bool(true),
        LiteralValue::String("text".to_string()),
        LiteralValue::Null,
        LiteralValue::Void,
    ];

    for literal in fixtures {
        let mut builder = MirBuilder::new();
        assert!(builder.build_literal(literal).is_err());
        assert!(builder.function_state.type_ctx.value_types.is_empty());
        assert!(builder.function_state.type_ctx.string_literals.is_empty());
        assert!(builder
            .function_state
            .type_ctx
            .value_origin_newbox
            .is_empty());
    }
}

#[test]
fn folded_negative_integer_observes_the_same_canonical_const_fact() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("literal_negative/0".to_string());

    let value = drive_raw_legacy_expression_v1(
        &mut builder,
        unary(UnaryOperator::Minus, literal(LiteralValue::Integer(7))),
    )
    .unwrap();

    assert_eq!(
        builder.function_state.type_ctx.get_type(value),
        Some(&MirType::Integer)
    );
    assert_eq!(
        emitted_consts(&builder),
        vec![(value, ConstValue::Integer(-7))]
    );
}

#[test]
fn folded_negative_integer_failure_has_no_caller_postpublication() {
    let mut builder = MirBuilder::new();
    assert!(drive_raw_legacy_expression_v1(
        &mut builder,
        unary(UnaryOperator::Minus, literal(LiteralValue::Integer(7))),
    )
    .is_err());
    assert!(builder.function_state.type_ctx.value_types.is_empty());
    assert!(builder.function_state.type_ctx.string_literals.is_empty());
    assert!(builder
        .function_state
        .type_ctx
        .value_origin_newbox
        .is_empty());
}

#[test]
fn weak_unary_lowers_operand_once_before_weak_completion() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("weak_unary/0".to_string());

    let result = drive_raw_legacy_expression_v1(
        &mut builder,
        unary(UnaryOperator::Weak, literal(LiteralValue::Integer(9))),
    )
    .unwrap();
    let instructions = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .collect::<Vec<_>>();

    assert!(matches!(
        instructions.as_slice(),
        [
            MirInstruction::Const { dst: value, .. },
            MirInstruction::WeakRef {
                dst,
                op: WeakRefOp::New,
                value: weak_value,
            },
        ] if *dst == result && value == weak_value
    ));
}

#[test]
fn weak_unary_operand_failure_does_not_emit_or_poison_reuse() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("weak_unary_failure/0".to_string());

    let failure = drive_raw_legacy_expression_v1(
        &mut builder,
        unary(UnaryOperator::Weak, variable("missing")),
    )
    .unwrap_err();
    assert!(failure.contains("Undefined variable"));
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .all(|block| block.instructions.is_empty()));
    let recovered =
        drive_raw_legacy_expression_v1(&mut builder, literal(LiteralValue::Integer(3))).unwrap();
    assert_eq!(
        emitted_consts(&builder),
        vec![(recovered, ConstValue::Integer(3))]
    );
}

#[test]
fn finalization_snapshots_the_canonical_literal_fact_without_a_late_repair() {
    let mut builder = MirBuilder::new();
    builder.prepare_module().expect("module shell");
    let result = builder
        .build_literal(LiteralValue::String("text".to_string()))
        .expect("String literal");
    let module = builder
        .finalize_module(result)
        .expect("finalized literal module");
    let function = module
        .functions
        .get("main")
        .expect("literal module must retain its entry function");
    let string_const = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .find_map(|instruction| match instruction {
            MirInstruction::Const {
                dst,
                value: ConstValue::String(value),
            } if value == "text" => Some(*dst),
            _ => None,
        })
        .expect("literal module must emit its String Const");

    assert_eq!(
        function.metadata.value_types.get(&string_const),
        Some(&MirType::String)
    );
}
