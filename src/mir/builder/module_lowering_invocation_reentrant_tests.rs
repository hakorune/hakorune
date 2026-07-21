//! HEADERPORT0-REENTRANT-TERM0-P0: capture/commit lifetime proofs.
//!
//! These fixtures exercise the disconnected capture-only and commit-only
//! seams.  They do not activate raw production lowering.

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::builder::calls::CanonicalFunctionSessionErrorV1;
use crate::mir::builder::module_lowering_invocation::{
    LegacyChildDraftAdmissionV1, ModuleLoweringInvocationV1, ModuleLoweringPortChildErrorV1,
};
use crate::mir::builder::recursive_child_lowering::{
    RawBoxMethodChildPortV1, RawInvocationChildPortV1,
};
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirBuilder, MirFunction, MirType};
use std::collections::HashMap;

fn draft(symbol: &str) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: symbol.to_owned(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn seeded<'builder>(builder: &'builder mut MirBuilder) -> ModuleLoweringInvocationV1<'builder> {
    builder.enter_function_for_test("reentrant_parent/0".to_owned());
    ModuleLoweringInvocationV1::open(builder)
}

fn function(name: &str, is_static: bool) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params: Vec::new(),
        param_decls: Vec::<ParamDecl>::new(),
        return_type_name: None,
        body: vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn nested_box(name: &str, is_static: bool) -> ASTNode {
    let method = function("run", is_static);
    let ASTNode::FunctionDeclaration {
        name: method_name, ..
    } = &method
    else {
        unreachable!()
    };
    ASTNode::BoxDeclaration {
        name: name.to_owned(),
        fields: Vec::new(),
        field_decls: Vec::new(),
        public_fields: Vec::new(),
        private_fields: Vec::new(),
        methods: HashMap::from([(method_name.clone(), method)]),
        constructors: HashMap::new(),
        init_fields: Vec::new(),
        weak_fields: Vec::new(),
        delegates: Vec::new(),
        invariants: Vec::new(),
        transitions: Vec::new(),
        is_interface: false,
        is_record: false,
        is_static,
        extends: Vec::new(),
        implements: Vec::new(),
        type_parameters: Vec::new(),
        is_sync: false,
        static_init: None,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn nested_box_with_constructor(name: &str) -> ASTNode {
    let mut node = nested_box(name, false);
    let ASTNode::BoxDeclaration { constructors, .. } = &mut node else {
        unreachable!()
    };
    constructors.insert("birth/0".to_owned(), function("birth", false));
    node
}

#[test]
fn pending_capture_ends_before_header_loan_and_commit() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);

    invocation
        .with_module_port(|builder, port| {
            let pending = port
                .capture_legacy_pending(builder, "inner/0", Vec::new(), |_| Ok(draft("inner/0")))
                .unwrap();

            port.with_headers(|headers| {
                assert!(!headers.contains_symbol("inner/0"));
                assert_eq!(headers.symbol_count(), 0);
            });

            port.commit_legacy_pending(
                pending,
                LegacyChildDraftAdmissionV1::legacy_symbol("inner/0".into(), 0),
            )
        })
        .unwrap();

    invocation.with_header_port(|builder, headers| {
        assert!(headers.contains_symbol("inner/0"));
        assert_eq!(headers.symbol_count(), 1);
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .unwrap()
                .signature
                .name,
            "reentrant_parent/0"
        );
    });
}

#[test]
fn rejected_commit_restores_parent_without_collector_delta() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);

    let result = invocation.with_module_port(|builder, port| {
        let pending = port
            .capture_legacy_pending(builder, "inner/0", Vec::new(), |_| Ok(draft("inner/0")))
            .unwrap();
        port.commit_legacy_pending(
            pending,
            LegacyChildDraftAdmissionV1::legacy_symbol("wrong/0".into(), 0),
        )
    });

    assert!(matches!(
        result,
        Err(ModuleLoweringPortChildErrorV1::Admission(_))
    ));
    invocation.with_header_port(|builder, headers| {
        assert_eq!(headers.symbol_count(), 0);
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .unwrap()
                .signature
                .name,
            "reentrant_parent/0"
        );
    });
}

#[test]
fn capture_failure_never_reaches_commit_terminal() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);

    let result = invocation.with_module_port(|builder, port| {
        let captured = port.capture_legacy_pending(
            builder,
            "failed/0",
            Vec::new(),
            move |_| -> Result<MirFunction, String> { Err("nested body failure".to_owned()) },
        );
        captured.map(|pending| {
            drop(pending);
        })
    });

    assert!(matches!(
        result,
        Err(ModuleLoweringPortChildErrorV1::Session(
            CanonicalFunctionSessionErrorV1::Primary(_)
        ))
    ));
    invocation.with_header_port(|_builder, headers| assert_eq!(headers.symbol_count(), 0));
}

#[test]
fn port_aware_static_body_collects_nested_static_child_before_outer_commit() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);
    let body = vec![nested_box("NestedStatic", true)];

    invocation
        .with_module_port(|builder, module_port| {
            let pending = {
                let mut raw_port = RawInvocationChildPortV1::new(module_port);
                raw_port.capture_static_box_method_pending_v1(
                    builder,
                    "Outer.run/0".into(),
                    Vec::new(),
                    Vec::new(),
                    None,
                    body,
                    Vec::new(),
                    DeclarationAttrs::default(),
                )?
            };
            module_port.commit_legacy_pending(
                pending,
                LegacyChildDraftAdmissionV1::legacy_symbol("Outer.run/0".into(), 0),
            )
        })
        .unwrap();

    invocation.with_header_port(|_builder, headers| {
        assert!(headers.contains_symbol("NestedStatic.run/0"));
        assert!(headers.contains_symbol("Outer.run/0"));
        assert_eq!(headers.symbol_count(), 2);
    });
}

#[test]
fn port_aware_static_body_collects_nested_instance_child_before_outer_commit() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);
    let body = vec![nested_box("NestedInstance", false)];

    invocation
        .with_module_port(|builder, module_port| {
            let pending = {
                let mut raw_port = RawInvocationChildPortV1::new(module_port);
                raw_port.capture_static_box_method_pending_v1(
                    builder,
                    "Outer.run/0".into(),
                    Vec::new(),
                    Vec::new(),
                    None,
                    body,
                    Vec::new(),
                    DeclarationAttrs::default(),
                )?
            };
            module_port.commit_legacy_pending(
                pending,
                LegacyChildDraftAdmissionV1::legacy_symbol("Outer.run/0".into(), 0),
            )
        })
        .unwrap();

    invocation.with_header_port(|_builder, headers| {
        assert!(headers.contains_symbol("NestedInstance.run/0"));
        assert!(headers.contains_symbol("Outer.run/0"));
        assert_eq!(headers.symbol_count(), 2);
    });
}

#[test]
fn port_aware_nested_instance_constructor_uses_the_same_child_terminal() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);
    let body = vec![nested_box_with_constructor("NestedCtor")];

    invocation
        .with_module_port(|builder, module_port| {
            let pending = {
                let mut raw_port = RawInvocationChildPortV1::new(module_port);
                raw_port.capture_static_box_method_pending_v1(
                    builder,
                    "Outer.run/0".into(),
                    Vec::new(),
                    Vec::new(),
                    None,
                    body,
                    Vec::new(),
                    DeclarationAttrs::default(),
                )?
            };
            module_port.commit_legacy_pending(
                pending,
                LegacyChildDraftAdmissionV1::legacy_symbol("Outer.run/0".into(), 0),
            )
        })
        .unwrap();

    invocation.with_header_port(|_builder, headers| {
        assert!(headers.contains_symbol("NestedCtor.birth/0"));
        assert!(headers.contains_symbol("NestedCtor.run/0"));
        assert!(headers.contains_symbol("Outer.run/0"));
        assert_eq!(headers.symbol_count(), 3);
    });
}

#[test]
fn invocation_main_box_is_rejected_before_root_effects() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);
    let main_box = nested_box("Main", true);

    let ASTNode::BoxDeclaration { name, methods, .. } = main_box else {
        unreachable!()
    };

    let before_instruction_count = invocation.with_header_port(|builder, _headers| {
        builder
            .function_state
            .current_function
            .as_ref()
            .and_then(|function| function.blocks.get(&BasicBlockId::new(0)))
            .map(|block| block.instructions.len())
            .unwrap_or(0)
    });

    let result = invocation.with_module_port(|builder, port| {
        let mut raw_port = RawInvocationChildPortV1::new(port);
        raw_port.lower_static_main_box(builder, name, methods)
    });

    assert!(result
        .expect_err("nested Main must be rejected before effects")
        .contains("root-only Main box"));
    invocation.with_header_port(|builder, headers| {
        assert_eq!(headers.symbol_count(), 0);
        let after_instruction_count = builder
            .function_state
            .current_function
            .as_ref()
            .and_then(|function| function.blocks.get(&BasicBlockId::new(0)))
            .map(|block| block.instructions.len())
            .unwrap_or(0);
        assert_eq!(after_instruction_count, before_instruction_count);
    });
}

#[test]
fn port_aware_capture_failure_restores_parent_without_collection() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);
    let body = vec![ASTNode::ContextScope {
        name: "unsupported".to_owned(),
        declared_type_name: None,
        value: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }),
        body: Vec::new(),
        source_keyword: "context".to_owned(),
        span: Span::unknown(),
    }];

    let result = invocation.with_module_port(|builder, module_port| {
        let mut raw_port = RawInvocationChildPortV1::new(module_port);
        raw_port
            .capture_static_box_method_pending_v1(
                builder,
                "Outer.failed/0".to_owned(),
                Vec::new(),
                Vec::new(),
                None,
                body,
                Vec::new(),
                DeclarationAttrs::default(),
            )
            .map(|pending| {
                drop(pending);
            })
    });

    assert!(result.is_err());
    invocation.with_header_port(|builder, headers| {
        assert_eq!(headers.symbol_count(), 0);
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .unwrap()
                .signature
                .name,
            "reentrant_parent/0"
        );
    });
}
