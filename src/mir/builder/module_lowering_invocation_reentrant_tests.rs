//! HEADERPORT0-REENTRANT-TERM0-P0: capture/commit lifetime proofs.
//!
//! These fixtures exercise the disconnected capture-only and commit-only
//! seams.  They do not activate raw production lowering.

use crate::ast::{
    ASTNode, BinaryOperator, DeclarationAttrs, EnumMatchArm, LiteralValue, ParamDecl, Span,
};
use crate::mir::builder::calls::CanonicalFunctionSessionErrorV1;
use crate::mir::builder::module_lowering_invocation::{
    LegacyChildDraftAdmissionV1, ModuleLoweringInvocationV1, ModuleLoweringPortChildErrorV1,
};
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawBoxMethodChildPortV1, RawInvocationChildPortV1,
    RawLegacyChildLoweringPortV1,
};
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1,
    RawInvocationSourceTransportV1, RawSourceTransportPortV1,
};
use crate::mir::builder::RawSourceLocatorV1;
use crate::mir::{
    BasicBlockId, EffectMask, FunctionSignature, MirBuilder, MirFunction, MirInstruction, MirType,
};
use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};

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
    function_with_body(
        name,
        is_static,
        vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }],
    )
}

fn function_with_body(name: &str, is_static: bool, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params: Vec::new(),
        param_decls: Vec::<ParamDecl>::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn nested_box_with_body(name: &str, is_static: bool, body: Vec<ASTNode>) -> ASTNode {
    let method = function_with_body("run", is_static, body);
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

fn nested_box(name: &str, is_static: bool) -> ASTNode {
    nested_box_with_body(name, is_static, vec![function_return_value(1)])
}

fn function_return_value(value: i64) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        })),
        span: Span::unknown(),
    }
}

fn outer_source() -> RawInvocationSourceTransportV1<()> {
    RawInvocationSourceTransportV1::root(
        (),
        RawInvocationRootLineageV1::Main(RawSourceLocatorV1::for_test(
            0,
            "Main",
            "run",
            "Outer.run/0",
            0,
        )),
    )
}

fn lower_located_loop(
    builder: &mut MirBuilder,
    port: &mut RawInvocationChildPortV1<'_, '_>,
    node: ASTNode,
) -> Result<crate::mir::ValueId, String> {
    let (_, root) = RawInvocationSourceContextV1::from_transport(
        RawInvocationSourceTransportV1::root((), RawInvocationRootLineageV1::ScriptRoot),
    );
    let transport = root.body_statement(node, 0);
    port.with_source_transport_v1(transport, |port, node| {
        drive_legacy_expression_v1(builder, port, node)
    })
}

fn sorted_instruction_debug(builder: &MirBuilder) -> Vec<String> {
    let mut rows = builder
        .function_state
        .current_function
        .as_ref()
        .expect("current function")
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .map(|row| format!("{row:?}"))
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

fn nested_box_with_constructor(name: &str) -> ASTNode {
    let mut node = nested_box(name, false);
    let ASTNode::BoxDeclaration { constructors, .. } = &mut node else {
        unreachable!()
    };
    constructors.insert("birth/0".to_owned(), function("birth", false));
    node
}

fn collect_seed(invocation: &mut ModuleLoweringInvocationV1<'_>, symbol: &str) {
    invocation
        .with_module_port(|builder, port| {
            let pending =
                port.capture_legacy_pending(builder, symbol, Vec::new(), |_| Ok(draft(symbol)))?;
            port.commit_legacy_pending(
                pending,
                LegacyChildDraftAdmissionV1::legacy_symbol(symbol.into(), 0),
            )
        })
        .unwrap();
}

fn assert_parent_and_prefix(
    invocation: &mut ModuleLoweringInvocationV1<'_>,
    expected_symbols: &[&str],
) {
    invocation.with_header_port(|builder, headers| {
        assert_eq!(headers.symbol_count(), expected_symbols.len());
        for symbol in expected_symbols {
            assert!(headers.contains_symbol(symbol));
        }
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
        assert_eq!(builder.recursion_depth, 0);
    });
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
    collect_seed(&mut invocation, "prefix/0");

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
    assert_parent_and_prefix(&mut invocation, &["prefix/0"]);
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
                raw_port.with_source_transport_v1(outer_source(), |port, ()| {
                    port.capture_static_box_method_pending_v1(
                        builder,
                        "Outer.run/0".into(),
                        Vec::new(),
                        Vec::new(),
                        None,
                        body,
                        Vec::new(),
                        DeclarationAttrs::default(),
                    )
                })?
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
                raw_port.with_source_transport_v1(outer_source(), |port, ()| {
                    port.capture_static_box_method_pending_v1(
                        builder,
                        "Outer.run/0".into(),
                        Vec::new(),
                        Vec::new(),
                        None,
                        body,
                        Vec::new(),
                        DeclarationAttrs::default(),
                    )
                })?
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
                raw_port.with_source_transport_v1(outer_source(), |port, ()| {
                    port.capture_static_box_method_pending_v1(
                        builder,
                        "Outer.run/0".into(),
                        Vec::new(),
                        Vec::new(),
                        None,
                        body,
                        Vec::new(),
                        DeclarationAttrs::default(),
                    )
                })?
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
fn raw_capture_commit_reaches_static_instance_constructor_depth_three() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);
    let leaf = nested_box_with_constructor("Leaf");
    let middle = nested_box_with_body("Middle", false, vec![leaf, function_return_value(2)]);
    let body = vec![middle, function_return_value(3)];

    invocation
        .with_module_port(|builder, module_port| {
            let pending = {
                let mut raw_port = RawInvocationChildPortV1::new(module_port);
                raw_port.with_source_transport_v1(outer_source(), |port, ()| {
                    port.capture_static_box_method_pending_v1(
                        builder,
                        "Outer.run/0".into(),
                        Vec::new(),
                        Vec::new(),
                        None,
                        body,
                        Vec::new(),
                        DeclarationAttrs::default(),
                    )
                })?
            };
            module_port.commit_legacy_pending(
                pending,
                LegacyChildDraftAdmissionV1::legacy_symbol("Outer.run/0".into(), 0),
            )
        })
        .unwrap();

    assert_parent_and_prefix(
        &mut invocation,
        &["Leaf.birth/0", "Leaf.run/0", "Middle.run/0", "Outer.run/0"],
    );
}

#[test]
fn raw_capture_commit_failure_matrix_preserves_prefix_and_reuse() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);
    collect_seed(&mut invocation, "prefix/0");

    let primary = invocation.with_module_port(|builder, port| {
        port.capture_legacy_pending(builder, "primary/0", Vec::new(), |_| {
            Err("raw primary".to_owned())
        })
        .map(drop)
    });
    assert!(matches!(
        primary,
        Err(ModuleLoweringPortChildErrorV1::Session(
            CanonicalFunctionSessionErrorV1::Primary(_)
        ))
    ));
    assert_parent_and_prefix(&mut invocation, &["prefix/0"]);

    let cleanup = invocation.with_module_port(|builder, port| {
        port.capture_legacy_pending(builder, "cleanup/0", Vec::new(), |builder| {
            builder.recursion_depth = 1;
            Ok(draft("cleanup/0"))
        })
        .map(drop)
    });
    assert!(matches!(
        cleanup,
        Err(ModuleLoweringPortChildErrorV1::Session(
            CanonicalFunctionSessionErrorV1::Cleanup(_)
        ))
    ));
    assert_parent_and_prefix(&mut invocation, &["prefix/0"]);

    let admission = invocation.with_module_port(|builder, port| {
        let pending = port.capture_legacy_pending(builder, "admission/0", Vec::new(), |_| {
            Ok(draft("admission/0"))
        })?;
        port.commit_legacy_pending(
            pending,
            LegacyChildDraftAdmissionV1::legacy_symbol("wrong/0".into(), 0),
        )
    });
    assert!(matches!(
        admission,
        Err(ModuleLoweringPortChildErrorV1::Admission(_))
    ));
    assert_parent_and_prefix(&mut invocation, &["prefix/0"]);

    let panic = catch_unwind(AssertUnwindSafe(|| {
        let _ = invocation.with_module_port(|builder, port| {
            port.capture_legacy_pending(builder, "panic/0", Vec::new(), |_| {
                panic!("raw child panic")
            })
            .map(drop)
        });
    }));
    assert!(panic.is_err());
    assert_parent_and_prefix(&mut invocation, &["prefix/0"]);

    collect_seed(&mut invocation, "after/0");
    assert_parent_and_prefix(&mut invocation, &["prefix/0", "after/0"]);
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

#[test]
fn located_enum_match_lowers_only_the_exact_scrutinee_and_keeps_result_shape() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);
    let enum_match = || ASTNode::EnumMatchExpr {
        enum_name: "Option".to_owned(),
        scrutinee: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(2),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        arms: vec![
            EnumMatchArm {
                variant_name: "Some".to_owned(),
                binding_name: None,
                body: ASTNode::Literal {
                    value: LiteralValue::Bool(true),
                    span: Span::unknown(),
                },
            },
            EnumMatchArm {
                variant_name: "None".to_owned(),
                binding_name: None,
                body: ASTNode::Literal {
                    value: LiteralValue::Bool(false),
                    span: Span::unknown(),
                },
            },
        ],
        else_expr: None,
        span: Span::unknown(),
    };

    let result = invocation
        .with_module_port(|builder, module_port| {
            let mut port = RawInvocationChildPortV1::new(module_port);
            port.with_source_transport_v1(
                RawInvocationSourceTransportV1::root(
                    enum_match(),
                    RawInvocationRootLineageV1::ScriptRoot,
                ),
                |port, enum_match| {
                    drive_legacy_expression_v1(builder, port, enum_match)
                },
            )
        })
        .expect("located EnumMatch");

    let selected_instructions = invocation.with_header_port(|builder, _headers| {
        builder
            .function_state
            .current_function
            .as_ref()
            .expect("current function")
            .blocks[&BasicBlockId::new(0)]
            .instructions
            .clone()
    });
    assert!(selected_instructions
        .iter()
        .any(|row| matches!(row, MirInstruction::BinOp { .. })));
    assert!(selected_instructions
        .iter()
        .any(|row| matches!(row, MirInstruction::VariantTag { .. })));
    assert!(selected_instructions
        .iter()
        .any(|row| matches!(row, MirInstruction::Select { dst, .. } if *dst == result)));

    // The selected scope carries exactly one receipt. Success therefore also
    // proves that Enum arm/else syntax was observed only by the unchanged
    // completion owner and never requested as a recursive child.
    let mut legacy_builder = MirBuilder::new();
    legacy_builder.enter_function_for_test("reentrant_parent/0".to_owned());
    let legacy_result = drive_legacy_expression_v1(
        &mut legacy_builder,
        &mut RawLegacyChildLoweringPortV1,
        enum_match(),
    )
    .expect("legacy EnumMatch oracle");
    let legacy_instructions = legacy_builder
        .function_state
        .current_function
        .as_ref()
        .expect("legacy current function")
        .blocks[&BasicBlockId::new(0)]
        .instructions
        .clone();
    assert_eq!(result, legacy_result);
    assert_eq!(selected_instructions, legacy_instructions);
}

#[test]
fn located_loop_rejects_reachable_box_before_joinir_or_collection() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);
    let before = invocation.with_header_port(|builder, _| sorted_instruction_debug(builder));
    let error = invocation
        .with_module_port(|builder, module_port| {
            let node = ASTNode::Loop {
                condition: Box::new(ASTNode::Literal {
                    value: LiteralValue::Bool(true),
                    span: Span::unknown(),
                }),
                body: vec![nested_box("Nested", true)],
                span: Span::unknown(),
            };
            lower_located_loop(
                builder,
                &mut RawInvocationChildPortV1::new(module_port),
                node,
            )
        })
        .unwrap_err();
    assert!(error.contains("[plan/freeze:contract] raw_loop_child_entry"));
    invocation.with_header_port(|builder, headers| {
        assert_eq!(headers.symbol_count(), 0);
        assert_eq!(sorted_instruction_debug(builder), before);
    });
}

#[test]
fn located_loop_preserves_legacy_route_result_and_instructions() {
    let node = ASTNode::Loop {
        condition: Box::new(ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        }),
        body: vec![ASTNode::Break { span: Span::unknown() }],
        span: Span::unknown(),
    };
    let mut legacy = MirBuilder::new();
    legacy.enter_function_for_test("reentrant_parent/0".to_owned());
    let legacy_result =
        drive_legacy_expression_v1(&mut legacy, &mut RawLegacyChildLoweringPortV1, node.clone());

    let mut selected = MirBuilder::new();
    let mut invocation = seeded(&mut selected);
    let selected_result = invocation.with_module_port(|builder, module_port| {
        lower_located_loop(
            builder,
            &mut RawInvocationChildPortV1::new(module_port),
            node,
        )
    });
    let selected_rows =
        invocation.with_header_port(|builder, _| sorted_instruction_debug(builder));
    assert_eq!(selected_result, legacy_result);
    assert_eq!(selected_rows, sorted_instruction_debug(&legacy));
}
