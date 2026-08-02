use super::module_draft_collector::{
    DraftPublicationPolicyV1, FunctionDraftKeyV1, ModuleDraftCollectorV1,
};
use super::module_lowering_invocation::ModuleLoweringInvocationV1;
use super::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceTransportV1, RawSourceTransportPortV1,
};
use super::recursive_child_lowering::{
    drive_legacy_body_v1, drive_legacy_expression_v1, RawInvocationChildPortV1,
};
use crate::ast::{ASTNode, BinaryOperator, CatchClause, CheckItem, FieldDecl, LiteralValue, Span};
use crate::mir::{
    BasicBlockId, BindingId, Effect, EffectMask, FunctionSignature, MirBuilder, MirFunction,
    MirInstruction, MirType,
};
use crate::parser::NyashParser;
pub(super) fn int(value: i64) -> ASTNode {
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
pub(super) fn new_expr(class: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::New {
        class: class.to_owned(),
        arguments,
        type_arguments: Vec::new(),
        field_initializers: Vec::new(),
        span: Span::unknown(),
    }
}
fn string(value: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(value.to_string()),
        span: Span::unknown(),
    }
}
fn boolean(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
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
pub(super) fn instructions(builder: &MirBuilder) -> Vec<MirInstruction> {
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
pub(super) fn collector() -> ModuleDraftCollectorV1 {
    collector_with_return_type(MirType::Void)
}
pub(super) fn collector_with_return_type(return_type: MirType) -> ModuleDraftCollectorV1 {
    let mut collector = ModuleDraftCollectorV1::default();
    let function = MirFunction::new(
        FunctionSignature {
            name: "Prefix.f/1".to_string(),
            params: vec![MirType::Integer],
            return_type,
            effects: EffectMask::READ.add(Effect::ReadHeap),
        },
        BasicBlockId(0),
    );
    collector
        .prepare_admission(
            FunctionDraftKeyV1::LegacySymbol("Prefix.f/1".to_string()),
            "Prefix.f/1".to_string(),
            1,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(function)
        .unwrap()
        .collect();
    collector
}

pub(super) fn birth_collector() -> ModuleDraftCollectorV1 {
    let mut collector = ModuleDraftCollectorV1::default();
    let function = MirFunction::new(
        FunctionSignature {
            name: "Prefix.birth/1".to_owned(),
            params: vec![MirType::Integer],
            return_type: MirType::Void,
            effects: EffectMask::READ.add(Effect::ReadHeap),
        },
        BasicBlockId(0),
    );
    collector
        .prepare_admission(
            FunctionDraftKeyV1::LegacySymbol("Prefix.birth/1".to_owned()),
            "Prefix.birth/1".to_owned(),
            1,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(function)
        .unwrap()
        .collect();
    collector
}

macro_rules! with_port {
    ($builder:ident, $port:ident, $body:block) => {{
        let mut invocation = ModuleLoweringInvocationV1::with_collector(&mut $builder, collector());
        invocation.with_module_port(|$builder, module_port| {
            let mut $port = RawInvocationChildPortV1::new(module_port);
            $body
        });
    }};
}

fn check(expression: ASTNode) -> ASTNode {
    ASTNode::CheckExpr {
        name: None,
        items: vec![CheckItem {
            label: None,
            expression,
        }],
        span: Span::unknown(),
    }
}

fn field(object: ASTNode, name: &str) -> ASTNode {
    ASTNode::FieldAccess {
        object: Box::new(object),
        field: name.to_string(),
        span: Span::unknown(),
    }
}

fn function_call(name: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.to_string(),
        arguments,
        span: Span::unknown(),
    }
}

fn record(fields: Vec<(&str, ASTNode)>) -> ASTNode {
    ASTNode::RecordLiteral {
        record_type_name: "Pair".to_string(),
        fields: fields
            .into_iter()
            .map(|(name, value)| (name.to_string(), value))
            .collect(),
        span: Span::unknown(),
    }
}

fn pair(builder: &mut MirBuilder) {
    builder.comp_ctx.register_record_decl(
        "Pair".to_string(),
        Vec::new(),
        &[FieldDecl {
            name: "value".to_string(),
            declared_type_name: None,
            is_weak: false,
            default_value: None,
        }],
    );
}

fn parsed_box(source: &str) -> ASTNode {
    let ASTNode::Program { mut statements, .. } = NyashParser::parse_from_string(source).unwrap()
    else {
        panic!("expected Program");
    };
    assert_eq!(statements.len(), 1);
    statements.remove(0)
}

#[test]
fn raw_invocation_port_collects_static_and_instance_box_methods() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_port_boxes/0".to_owned());
    let mut invocation = ModuleLoweringInvocationV1::open(&mut builder);

    invocation.with_module_port(|builder, module_port| {
        let mut port = RawInvocationChildPortV1::new(module_port);
        for source in [
            "static box RawStatic { run() { return 7 } }",
            "box RawInstance { run() { return 8 } }",
        ] {
            drive_legacy_expression_v1(builder, &mut port, parsed_box(source)).unwrap();
        }
    });

    invocation.with_header_port(|_builder, headers| {
        assert!(headers.contains_symbol("RawStatic.run/0"));
        assert!(headers.contains_symbol("RawInstance.run/0"));
        assert_eq!(headers.symbol_count(), 2);
    });
}

#[test]
fn raw_invocation_port_preserves_binary_and_unary_children() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_port_binary_unary/0".to_string());
    with_port!(builder, port, {
        let binary =
            drive_legacy_expression_v1(builder, &mut port, add(int(1), add(int(2), int(3))))
                .unwrap();
        let unary = drive_legacy_expression_v1(
            builder,
            &mut port,
            ASTNode::UnaryOp {
                operator: crate::ast::UnaryOperator::Minus,
                operand: Box::new(add(int(4), int(5))),
                span: Span::unknown(),
            },
        )
        .unwrap();
        assert!(instructions(builder).iter().any(|row| matches!(row, MirInstruction::BinOp { dst, .. } if *dst == binary || *dst == unary)));
        port.with_headers(|headers| assert_eq!(headers.symbol_count(), 1));
    });
}

#[test]
fn raw_invocation_port_preserves_async_qmark_check_collection_and_print_children() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_port_misc/0".to_string());
    with_port!(builder, port, {
        for node in [
            ASTNode::Nowait {
                variable: "pending".to_string(),
                expression: Box::new(add(int(1), int(2))),
                span: Span::unknown(),
            },
            ASTNode::AwaitExpression {
                expression: Box::new(int(3)),
                span: Span::unknown(),
            },
            ASTNode::QMarkPropagate {
                expression: Box::new(int(4)),
                span: Span::unknown(),
            },
            check(add(int(5), int(6))),
            ASTNode::ArrayLiteral {
                elements: vec![add(int(7), int(8))],
                span: Span::unknown(),
            },
            ASTNode::MapLiteral {
                entries: vec![("key".to_string(), add(int(9), int(10)))],
                span: Span::unknown(),
            },
            ASTNode::Print {
                expression: Box::new(add(int(11), int(12))),
                span: Span::unknown(),
            },
        ] {
            drive_legacy_expression_v1(builder, &mut port, node).unwrap();
        }
        assert!(instructions(builder)
            .iter()
            .any(|row| matches!(row, MirInstruction::Select { .. })));
        port.with_headers(|headers| assert_eq!(headers.symbol_count(), 1));
    });
}
#[test]
fn raw_invocation_port_preserves_assignment_and_compound_children() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_port_assignment/0".to_string());
    let old = crate::mir::builder::emission::constant::emit_integer(&mut builder, 1).unwrap();
    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert("x".into(), old);
    builder
        .function_state
        .binding_ctx
        .insert("x".into(), BindingId::new(0));
    with_port!(builder, port, {
        let ordinary = ASTNode::Assignment {
            target: Box::new(variable("x")),
            value: Box::new(add(int(1), int(2))),
            span: Span::unknown(),
        };
        port.with_source_transport_v1(
            RawInvocationSourceTransportV1::root(ordinary, RawInvocationRootLineageV1::ScriptRoot),
            |port, ordinary| drive_legacy_expression_v1(builder, port, ordinary),
        )
        .unwrap();
        let grouped = ASTNode::GroupedAssignmentExpr {
            lhs: "x".to_string(),
            rhs: Box::new(add(int(2), int(3))),
            span: Span::unknown(),
        };
        port.with_source_transport_v1(
            RawInvocationSourceTransportV1::root(grouped, RawInvocationRootLineageV1::ScriptRoot),
            |port, grouped| drive_legacy_expression_v1(builder, port, grouped),
        )
        .unwrap();
        let compound = ASTNode::CompoundAssignment {
            target: Box::new(variable("x")),
            operator: BinaryOperator::Add,
            value: Box::new(add(int(4), int(5))),
            span: Span::unknown(),
        };
        port.with_source_transport_v1(
            RawInvocationSourceTransportV1::root(compound, RawInvocationRootLineageV1::ScriptRoot),
            |port, compound| drive_legacy_expression_v1(builder, port, compound),
        )
        .unwrap();
        let field_assignment = ASTNode::Assignment {
            target: Box::new(field(variable("x"), "slot")),
            value: Box::new(add(int(6), int(7))),
            span: Span::unknown(),
        };
        port.with_source_transport_v1(
            RawInvocationSourceTransportV1::root(
                field_assignment,
                RawInvocationRootLineageV1::ScriptRoot,
            ),
            |port, assignment| drive_legacy_expression_v1(builder, port, assignment),
        )
        .unwrap();
        let rows = instructions(builder);
        assert!(rows
            .iter()
            .any(|row| matches!(row, MirInstruction::FieldSet { .. })));
        port.with_headers(|headers| assert_eq!(headers.symbol_count(), 1));
    });
}
#[test]
fn raw_invocation_port_preserves_call_and_from_children() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_port_calls/0".to_string());
    with_port!(builder, port, {
        let indirect = ASTNode::Call {
            callee: Box::new(add(int(1), int(2))),
            arguments: vec![add(int(3), int(4))],
            span: Span::unknown(),
        };
        drive_legacy_expression_v1(builder, &mut port, indirect).unwrap();
        let from = ASTNode::FromCall {
            parent: "Parent".to_string(),
            method: "build".to_string(),
            arguments: vec![add(int(5), int(6))],
            span: Span::unknown(),
        };
        drive_legacy_expression_v1(builder, &mut port, from).unwrap();
        assert!(instructions(builder)
            .iter()
            .any(|row| matches!(row, MirInstruction::Call { .. })));
        port.with_headers(|headers| assert_eq!(headers.symbol_count(), 1));
    });
}

#[test]
fn raw_invocation_port_preserves_function_preflight_children() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_port_function_preflight/0".to_string());
    with_port!(builder, port, {
        let typeop = drive_legacy_expression_v1(
            builder,
            &mut port,
            function_call("isType", vec![add(int(1), int(2)), string("Integer")]),
        )
        .unwrap();
        let externcall = drive_legacy_expression_v1(
            builder,
            &mut port,
            function_call("externcall", vec![string("io.print"), add(int(3), int(4))]),
        )
        .unwrap();
        assert!(instructions(builder).iter().any(|row| matches!(
            row,
            MirInstruction::TypeOp { dst, .. } if *dst == typeop
        )));
        assert!(instructions(builder).iter().any(|row| matches!(
            row,
            MirInstruction::Call {
                dst: Some(dst),
                callee: Some(crate::mir::Callee::Extern(_)),
                ..
            } if *dst == externcall
        )));
        port.with_headers(|headers| assert_eq!(headers.symbol_count(), 1));
    });
}

#[test]
fn raw_invocation_port_preserves_if_children() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_port_if/0".to_string());
    with_port!(builder, port, {
        let if_node = ASTNode::If {
            condition: Box::new(boolean(true)),
            then_body: vec![add(int(1), int(2))],
            else_body: Some(vec![add(int(3), int(4))]),
            span: Span::unknown(),
        };
        let output = drive_legacy_expression_v1(builder, &mut port, if_node).unwrap();
        assert!(instructions(builder).iter().any(|row| matches!(
            row,
            MirInstruction::Phi { dst, .. } if *dst == output
        )));
        port.with_headers(|headers| assert_eq!(headers.symbol_count(), 1));
    });
}

#[test]
fn raw_invocation_port_preserves_try_body_children() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_port_trycatch/0".to_string());
    with_port!(builder, port, {
        let try_node = ASTNode::TryCatch {
            try_body: vec![add(int(1), int(2))],
            catch_clauses: vec![
                CatchClause {
                    exception_type: Some("Error".into()),
                    variable_name: None,
                    body: vec![add(int(3), int(4))],
                    span: Span::unknown(),
                },
                CatchClause {
                    exception_type: Some("Ignored".into()),
                    variable_name: None,
                    body: vec![add(int(99), int(100))],
                    span: Span::unknown(),
                },
            ],
            finally_body: Some(vec![add(int(5), int(6))]),
            span: Span::unknown(),
        };
        port.with_source_transport_v1(
            RawInvocationSourceTransportV1::root(try_node, RawInvocationRootLineageV1::ScriptRoot),
            |port, try_node| drive_legacy_expression_v1(builder, port, try_node),
        )
        .unwrap();
        assert_eq!(
            instructions(builder)
                .iter()
                .filter(|row| matches!(row, MirInstruction::BinOp { .. }))
                .count(),
            3,
            "try, first catch, and cleanup must each descend once"
        );
        port.with_headers(|headers| assert_eq!(headers.symbol_count(), 1));
    });
}

#[test]
fn raw_invocation_port_preserves_match_children() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_port_match/0".to_string());
    with_port!(builder, port, {
        let match_node = ASTNode::MatchExpr {
            scrutinee: Box::new(add(int(1), int(2))),
            arms: vec![(LiteralValue::Integer(3), add(int(4), int(5)))],
            else_expr: Box::new(add(int(6), int(7))),
            span: Span::unknown(),
        };
        let output = port
            .with_source_transport_v1(
                RawInvocationSourceTransportV1::root(
                    match_node,
                    RawInvocationRootLineageV1::ScriptRoot,
                ),
                |port, match_node| drive_legacy_expression_v1(builder, port, match_node),
            )
            .unwrap();
        assert!(instructions(builder).iter().any(|row| matches!(
            row,
            MirInstruction::Phi { dst, .. } if *dst == output
        )));
        port.with_headers(|headers| assert_eq!(headers.symbol_count(), 1));
    });
}

#[test]
fn raw_invocation_port_preserves_new_and_field_record_children() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_port_new_field_record/0".to_string());
    pair(&mut builder);
    with_port!(builder, port, {
        let new = ASTNode::New {
            class: "RawPortBox".to_string(),
            arguments: vec![add(int(1), int(2))],
            type_arguments: Vec::new(),
            field_initializers: vec![("value".to_string(), add(int(3), int(4)))],
            span: Span::unknown(),
        };
        drive_legacy_expression_v1(builder, &mut port, new).unwrap();
        let record_value = drive_legacy_expression_v1(
            builder,
            &mut port,
            field(record(vec![("value", add(int(5), int(6)))]), "value"),
        )
        .unwrap();
        assert!(instructions(builder).iter().any(|row| matches!(
            row,
            MirInstruction::FieldSet { .. } | MirInstruction::RecordValuePublish { .. }
        )));
        assert!(instructions(builder)
            .iter()
            .any(|row| matches!(row, MirInstruction::BinOp { dst, .. } if *dst == record_value)));
        port.with_headers(|headers| assert_eq!(headers.symbol_count(), 1));
    });
}

#[test]
fn raw_invocation_port_preserves_throw_child() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_port_throw/0".to_string());
    with_port!(builder, port, {
        drive_legacy_expression_v1(
            builder,
            &mut port,
            ASTNode::Throw {
                expression: Box::new(add(int(1), int(2))),
                span: Span::unknown(),
            },
        )
        .unwrap();
        assert!(builder.is_current_block_terminated());
        port.with_headers(|headers| assert_eq!(headers.symbol_count(), 1));
    });
}

#[test]
fn raw_invocation_port_descends_nested_program_body_once() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_port_program/0".to_owned());
    with_port!(builder, port, {
        port.with_source_transport_v1(
            RawInvocationSourceTransportV1::root((), RawInvocationRootLineageV1::ScriptRoot),
            |port, ()| {
                drive_legacy_body_v1(
                    builder,
                    port,
                    vec![ASTNode::Program {
                        statements: vec![add(int(1), int(2)), add(int(3), int(4))],
                        span: Span::unknown(),
                    }],
                )
            },
        )
        .unwrap();
        assert_eq!(
            instructions(builder)
                .iter()
                .filter(|row| matches!(row, MirInstruction::BinOp { .. }))
                .count(),
            2
        );
    });
}
