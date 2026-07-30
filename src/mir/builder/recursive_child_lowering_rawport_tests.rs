use crate::ast::{ASTNode, BinaryOperator, CatchClause, CheckItem, FieldDecl, LiteralValue, Span};
use crate::mir::{
    BasicBlockId, BindingId, Effect, EffectMask, FunctionSignature, MirBuilder, MirFunction,
    MirInstruction, MirModule, MirType,
};
use crate::parser::NyashParser;

use super::me_call_header_observation::{
    prepare_me_lowered_call_v1, MeCallHeaderObservationPortV1, MeCallHeaderSourceV1,
    PreparedMeReceiverV1,
};
use super::module_draft_collector::{
    DraftPublicationPolicyV1, FunctionDraftKeyV1, ModuleDraftCollectorV1,
};
use super::module_lowering_invocation::ModuleLoweringInvocationV1;
use super::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceTransportV1, RawSourceTransportPortV1,
};
use super::recursive_child_lowering::{
    drive_legacy_body_v1, drive_legacy_expression_v1, RawInvocationChildPortV1,
    RawLegacyChildLoweringPortV1,
};

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn new_expr(class: &str, arguments: Vec<ASTNode>) -> ASTNode {
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

fn collector() -> ModuleDraftCollectorV1 {
    collector_with_return_type(MirType::Void)
}

fn collector_with_return_type(return_type: MirType) -> ModuleDraftCollectorV1 {
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

fn birth_collector() -> ModuleDraftCollectorV1 {
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
fn raw_invocation_port_reborrows_one_collector_backed_header_view() {
    let mut builder = MirBuilder::new();
    with_port!(builder, port, {
        port.with_headers(|headers| {
            assert_eq!(headers.signature("Prefix.f/1").unwrap().params.len(), 1)
        });
        port.reborrow()
            .with_headers(|headers| assert!(headers.contains_symbol("Prefix.f/1")));
    });
}

#[test]
fn headerport_annotation_matches_legacy_module_signature_without_ambient_module() {
    let symbol = "Prefix.f/1";
    let signature = FunctionSignature {
        name: symbol.to_owned(),
        params: vec![MirType::Integer],
        return_type: MirType::Box("Result".to_owned()),
        effects: EffectMask::READ.add(Effect::ReadHeap),
    };

    let mut legacy = MirBuilder::new();
    legacy.enter_function_for_test("headerport_annotation/0".to_owned());
    legacy.current_module = Some(MirModule::new("legacy-header-module".to_owned()));
    legacy
        .current_module
        .as_mut()
        .unwrap()
        .add_function(MirFunction::new(signature, BasicBlockId(0)));
    let mut port_builder = MirBuilder::new();
    port_builder.enter_function_for_test("headerport_annotation/0".to_owned());

    let dst = crate::mir::ValueId(11);
    super::calls::annotation::annotate_call_result_from_func_name(&mut legacy, dst, symbol);
    let mut invocation = ModuleLoweringInvocationV1::with_collector(
        &mut port_builder,
        collector_with_return_type(MirType::Box("Result".to_owned())),
    );
    invocation.with_header_port(|builder, headers| {
        super::calls::annotation::annotate_call_result_from_func_name_with_lookup(
            builder,
            dst,
            symbol,
            Some(headers),
        );
    });

    assert!(port_builder.current_module.is_none());
    assert_eq!(
        legacy.function_state.type_ctx.value_types.get(&dst),
        port_builder.function_state.type_ctx.value_types.get(&dst)
    );
    assert_eq!(
        legacy.function_state.type_ctx.value_origin_newbox.get(&dst),
        port_builder
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&dst)
    );
}

#[test]
fn explicit_header_authority_survives_unified_call_post_success() {
    let symbol = "Prefix.f/1";
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("headerport_call/0".to_owned());
    let stale = MirFunction::new(
        FunctionSignature {
            name: symbol.to_owned(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId(0),
    );
    builder.current_module = Some(MirModule::new("stale-call-module".to_owned()));
    builder.current_module.as_mut().unwrap().add_function(stale);
    let dst = crate::mir::ValueId(12);
    let mut invocation = ModuleLoweringInvocationV1::with_collector(
        &mut builder,
        collector_with_return_type(MirType::Box("Result".to_owned())),
    );
    invocation.with_header_port(|builder, headers| {
        let arg = crate::mir::builder::emission::constant::emit_integer(builder, 1).unwrap();
        builder
            .emit_unified_call_with_lookup(
                Some(dst),
                super::calls::CallTarget::Global(symbol.to_owned()),
                vec![arg],
                Some(headers),
            )
            .unwrap();
    });
    drop(invocation);
    assert_eq!(
        builder.function_state.type_ctx.value_types.get(&dst),
        Some(&MirType::Box("Result".to_owned()))
    );
}

#[test]
fn headerport_birth_presence_matches_legacy_newbox_branch() {
    let birth = MirFunction::new(
        FunctionSignature {
            name: "Prefix.birth/1".to_owned(),
            params: vec![MirType::Integer],
            return_type: MirType::Void,
            effects: EffectMask::READ.add(Effect::ReadHeap),
        },
        BasicBlockId(0),
    );
    let mut legacy = MirBuilder::new();
    legacy.enter_function_for_test("headerport_birth/0".to_owned());
    legacy.current_module = Some(MirModule::new("legacy-birth-module".to_owned()));
    legacy.current_module.as_mut().unwrap().add_function(birth);
    let mut legacy_port = RawLegacyChildLoweringPortV1;
    let legacy_value = drive_legacy_expression_v1(
        &mut legacy,
        &mut legacy_port,
        new_expr("Prefix", vec![int(7)]),
    )
    .unwrap();

    let mut port_builder = MirBuilder::new();
    port_builder.enter_function_for_test("headerport_birth/0".to_owned());
    let invocation_value = {
        let mut invocation =
            ModuleLoweringInvocationV1::with_collector(&mut port_builder, birth_collector());
        let value = invocation.with_module_port(|builder, module_port| {
            let mut port = RawInvocationChildPortV1::new(module_port);
            drive_legacy_expression_v1(builder, &mut port, new_expr("Prefix", vec![int(7)]))
        });
        drop(invocation);
        value.unwrap()
    };

    assert_eq!(legacy_value, invocation_value);
    assert_eq!(instructions(&legacy), instructions(&port_builder));
    assert!(port_builder.current_module.is_none());
}

#[test]
fn raw_invocation_me_header_ignores_stale_module_signature() {
    let mut builder = MirBuilder::new();
    builder.current_module = Some(crate::mir::MirModule::new("stale-me-module".to_string()));
    builder
        .current_module
        .as_mut()
        .unwrap()
        .add_function(MirFunction::new(
            FunctionSignature {
                name: "Prefix.f/1".to_string(),
                params: vec![MirType::Box("Prefix".to_string()), MirType::Integer],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId(0),
        ));

    let mut invocation = ModuleLoweringInvocationV1::with_collector(&mut builder, collector());
    invocation.with_module_port(|builder, module_port| {
        let mut port = RawInvocationChildPortV1::new(module_port);
        let observation = port.observe_me_call_parameters(builder, "Prefix.f/1");
        assert_eq!(
            observation.source(),
            MeCallHeaderSourceV1::InvocationCollector
        );
        let prepared = prepare_me_lowered_call_v1(observation, Some(crate::mir::ValueId(4)))
            .expect("collector header should be present");
        assert_eq!(prepared.receiver(), &PreparedMeReceiverV1::Static);
    });
}

#[test]
fn raw_invocation_header_miss_does_not_retry_stale_current_module() {
    let symbol = "Ghost.m/1";
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_invocation_header_miss/0".to_owned());
    builder.current_module = Some(crate::mir::MirModule::new("stale-miss-module".to_string()));
    builder
        .current_module
        .as_mut()
        .unwrap()
        .add_function(MirFunction::new(
            FunctionSignature {
                name: symbol.to_string(),
                params: vec![MirType::Box("Ghost".to_string()), MirType::Integer],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId(0),
        ));
    let instructions_before = instructions(&builder);
    let next_value_before = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .next_value_id;

    {
        let mut invocation = ModuleLoweringInvocationV1::with_collector(
            &mut builder,
            ModuleDraftCollectorV1::default(),
        );
        invocation.with_module_port(|builder, module_port| {
            let mut port = RawInvocationChildPortV1::new(module_port);
            let observation = port.observe_me_call_parameters(builder, symbol);
            assert_eq!(
                observation.source(),
                MeCallHeaderSourceV1::InvocationCollector
            );
            assert!(matches!(
                &observation,
                super::me_call_header_observation::MeCallParameterObservationV1::Missing { .. }
            ));
            assert!(prepare_me_lowered_call_v1(observation, None).is_none());
        });
    }

    assert_eq!(instructions(&builder), instructions_before);
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .next_value_id,
        next_value_before
    );
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
    builder.function_state.variable_ctx.variable_map.insert("x".into(), old);
    builder.function_state.binding_ctx.insert("x".into(), BindingId::new(0));
    with_port!(builder, port, {
        let ordinary = ASTNode::Assignment {
            target: Box::new(ASTNode::Variable { name: "x".into(), span: Span::unknown() }),
            value: Box::new(add(int(1), int(2))),
            span: Span::unknown(),
        };
        port.with_source_transport_v1(
            RawInvocationSourceTransportV1::root(
                ordinary,
                RawInvocationRootLineageV1::ScriptRoot,
            ),
            |port, ordinary| drive_legacy_expression_v1(builder, port, ordinary),
        ).unwrap();
        let grouped = ASTNode::GroupedAssignmentExpr {
            lhs: "x".to_string(),
            rhs: Box::new(add(int(2), int(3))),
            span: Span::unknown(),
        };
        port.with_source_transport_v1(
            RawInvocationSourceTransportV1::root(
                grouped,
                RawInvocationRootLineageV1::ScriptRoot,
            ),
            |port, grouped| drive_legacy_expression_v1(builder, port, grouped),
        )
        .unwrap();
        let compound = ASTNode::CompoundAssignment {
            target: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            operator: BinaryOperator::Add,
            value: Box::new(add(int(4), int(5))),
            span: Span::unknown(),
        };
        port.with_source_transport_v1(
            RawInvocationSourceTransportV1::root(
                compound,
                RawInvocationRootLineageV1::ScriptRoot,
            ),
            |port, compound| drive_legacy_expression_v1(builder, port, compound),
        )
        .unwrap();
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
