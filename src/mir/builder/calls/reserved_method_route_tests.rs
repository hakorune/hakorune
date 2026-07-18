use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::instruction::MemOpKind;
use crate::mir::{Callee, MirBuilder, MirInstruction};

use super::super::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use super::method_call_descent::RawLegacyMethodCallInputV1;
use super::reserved_method_route::{build_reserved_method_call_v1, ReservedMethodCallOutcomeV1};

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn string(value: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(value.into()),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn method(receiver: &str, name: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(variable(receiver)),
        method: name.into(),
        arguments,
        span: Span::unknown(),
    }
}

fn instructions(builder: &MirBuilder) -> impl Iterator<Item = &MirInstruction> {
    builder
        .scope_ctx
        .current_function
        .as_ref()
        .expect("current function")
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
}

#[test]
fn selected_mir_debug_route_preserves_debug_payload() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("mir_debug/0".into());
    let result = builder
        .build_expression(method("__mir__", "log", vec![string("value"), integer(7)]))
        .unwrap();

    assert!(instructions(&builder).any(|instruction| {
        matches!(
            instruction,
            MirInstruction::Debug { value, message }
                if *value != result && message == "value"
        )
    }));
    assert!(!instructions(&builder).any(|instruction| matches!(
        instruction,
        MirInstruction::Const {
            value: crate::mir::ConstValue::String(value),
            ..
        } if value == "value"
    )));
}

#[test]
fn selected_mir_mark_evaluates_neither_label_nor_extra_arguments() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("mir_mark_syntax_only/0".into());
    builder
        .build_expression(method(
            "__mir__",
            "mark",
            vec![string("marker"), variable("must_not_be_evaluated")],
        ))
        .unwrap();

    assert!(instructions(&builder).any(|instruction| matches!(
        instruction,
        MirInstruction::Debug { message, .. } if message == "marker"
    )));
}

#[test]
fn selected_mir_log_stops_at_first_failed_suffix_and_builder_is_reusable() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("mir_log_failure/0".into());
    let error = builder
        .build_expression(method(
            "__mir__",
            "log",
            vec![string("value"), integer(1), variable("missing"), integer(3)],
        ))
        .unwrap_err();
    assert!(error.contains("Undefined variable: missing"));
    let integers = instructions(&builder)
        .filter_map(|instruction| match instruction {
            MirInstruction::Const {
                value: crate::mir::ConstValue::Integer(value),
                ..
            } => Some(*value),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(integers, vec![1]);

    builder.build_expression(integer(9)).unwrap();
    assert!(instructions(&builder).any(|instruction| matches!(
        instruction,
        MirInstruction::Const {
            value: crate::mir::ConstValue::Integer(9),
            ..
        }
    )));
}

#[test]
fn ordinary_reserved_decision_descends_no_children() {
    let input = RawLegacyMethodCallInputV1::new(
        variable("__mir__"),
        "log".to_string(),
        vec![variable("non_literal_label")],
    );
    let mut port = RawLegacyChildLoweringPortV1;
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("reserved_ordinary/0".into());

    assert!(matches!(
        build_reserved_method_call_v1(&mut builder, &mut port, &input).unwrap(),
        ReservedMethodCallOutcomeV1::Ordinary
    ));
    assert_eq!(instructions(&builder).count(), 0);
}

#[test]
fn selected_mir_debug_zero_argument_failure_is_stable() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("mir_debug_fail/0".into());
    assert_eq!(
        builder
            .build_expression(method("__mir__", "mark", vec![]))
            .unwrap_err(),
        "__mir__.log/__mir__.mark requires at least a label argument"
    );
    assert!(!instructions(&builder)
        .any(|instruction| matches!(instruction, MirInstruction::Debug { .. })));
}

#[test]
fn selected_repl_route_preserves_extern_call() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("repl/0".into());
    builder
        .build_expression(method("__repl", "get", vec![string("name")]))
        .unwrap();

    assert!(instructions(&builder).any(|instruction| {
        matches!(
            instruction,
            MirInstruction::Call {
                callee: Some(Callee::Extern(name)),
                ..
            } if name == "__repl.get"
        )
    }));
}

#[test]
fn selected_repl_unsupported_method_failure_is_stable() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("repl_fail/0".into());
    assert_eq!(
        builder
            .build_expression(method(
                "__repl",
                "other",
                vec![variable("must_not_be_evaluated")],
            ))
            .unwrap_err(),
        "__repl.other is not supported. Only __repl.get and __repl.set are allowed."
    );
    assert_eq!(instructions(&builder).count(), 0);
}

#[test]
fn selected_fastmem_method_route_preserves_memop_lowering() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_method/0".into());
    let body = vec![ASTNode::FastMemRegion {
        contract: "PageMapV0".into(),
        body: vec![ASTNode::Local {
            variables: vec!["address".into()],
            initial_values: vec![Some(Box::new(method("mem", "addr", vec![integer(4096)])))],
            declared_type_names: Vec::new(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }];
    crate::mir::builder::stmts::block_stmt::build_block(&mut builder, body).unwrap();

    assert!(instructions(&builder).any(|instruction| {
        matches!(
            instruction,
            MirInstruction::MemOp {
                kind: MemOpKind::AddrOf,
                ..
            }
        )
    }));
}

#[test]
fn selected_fastmem_arity_failure_precedes_argument_effects() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_method_arity/0".into());
    let body = vec![ASTNode::FastMemRegion {
        contract: "PageMapV0".into(),
        body: vec![method("mem", "addr", vec![integer(1), integer(2)])],
        span: Span::unknown(),
    }];
    let error =
        crate::mir::builder::stmts::block_stmt::build_block(&mut builder, body).unwrap_err();
    assert!(error.contains("[freeze:contract][fastmem/arity]"));
    assert!(!instructions(&builder).any(|instruction| matches!(
        instruction,
        MirInstruction::Const {
            value: crate::mir::ConstValue::Integer(1 | 2),
            ..
        }
    )));
}

#[test]
fn selected_fastmem_table_id_preflight_precedes_argument_effects() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_method_table_id/0".into());
    let body = vec![ASTNode::FastMemRegion {
        contract: "PageMapV0".into(),
        body: vec![method(
            "mem",
            "assumeTableLength",
            vec![integer(7), integer(4)],
        )],
        span: Span::unknown(),
    }];
    let error =
        crate::mir::builder::stmts::block_stmt::build_block(&mut builder, body).unwrap_err();
    assert!(error.contains("[freeze:contract][fastmem/table_length_requires_table_variable]"));
    assert!(!instructions(&builder).any(|instruction| matches!(
        instruction,
        MirInstruction::Const {
            value: crate::mir::ConstValue::Integer(7 | 4),
            ..
        }
    )));
}

#[test]
fn selected_fastmem_positive_upper_preflight_precedes_argument_effects() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_method_positive_upper/0".into());
    let body = vec![ASTNode::FastMemRegion {
        contract: "PageMapV0".into(),
        body: vec![method(
            "mem",
            "assumeIndexInRange",
            vec![integer(7), integer(0)],
        )],
        span: Span::unknown(),
    }];
    let error =
        crate::mir::builder::stmts::block_stmt::build_block(&mut builder, body).unwrap_err();
    assert!(error.contains("[freeze:contract][fastmem/table_length_requires_positive_usize]"));
    assert!(!instructions(&builder).any(|instruction| matches!(
        instruction,
        MirInstruction::Const {
            value: crate::mir::ConstValue::Integer(7 | 0),
            ..
        }
    )));
}
