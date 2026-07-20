use super::MirBuilder;
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::{ConstValue, MirInstruction, MirType};

fn literal(value: LiteralValue) -> ASTNode {
    ASTNode::Literal {
        value,
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

    let value = builder
        .build_unary_op("-".to_string(), literal(LiteralValue::Integer(7)))
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
    assert!(builder
        .build_unary_op("-".to_string(), literal(LiteralValue::Integer(7)))
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
fn finalization_snapshots_the_canonical_literal_fact_without_a_late_repair() {
    let mut builder = MirBuilder::new();
    let module = builder
        .build_module(literal(LiteralValue::String("text".to_string())))
        .unwrap();
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
