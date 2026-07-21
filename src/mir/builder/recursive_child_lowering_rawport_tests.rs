use crate::ast::{ASTNode, BinaryOperator, CheckItem, FieldDecl, LiteralValue, Span};
use crate::mir::{
    BasicBlockId, BindingId, Effect, EffectMask, FunctionSignature, MirBuilder, MirFunction,
    MirInstruction, MirType,
};

use super::module_draft_collector::{
    DraftPublicationPolicyV1, FunctionDraftKeyV1, ModuleDraftCollectorV1,
};
use super::module_lowering_invocation::ModuleLoweringInvocationV1;
use super::recursive_child_lowering::{drive_legacy_expression_v1, RawInvocationChildPortV1};

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn string(value: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(value.to_string()),
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
    let mut collector = ModuleDraftCollectorV1::default();
    let function = MirFunction::new(
        FunctionSignature {
            name: "Prefix.f/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Void,
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
        .insert("x".to_string(), old);
    builder
        .function_state
        .binding_ctx
        .insert("x".to_string(), BindingId::new(0));
    with_port!(builder, port, {
        let grouped = ASTNode::GroupedAssignmentExpr {
            lhs: "x".to_string(),
            rhs: Box::new(add(int(2), int(3))),
            span: Span::unknown(),
        };
        drive_legacy_expression_v1(builder, &mut port, grouped).unwrap();
        let compound = ASTNode::CompoundAssignment {
            target: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            operator: BinaryOperator::Add,
            value: Box::new(add(int(4), int(5))),
            span: Span::unknown(),
        };
        drive_legacy_expression_v1(builder, &mut port, compound).unwrap();
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
