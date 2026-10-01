use super::{NormalDefaultProgramRootConsumptionV1, RejectedNormalDefaultRootOwnerV1};
use crate::ast::{ASTNode, Span};
use crate::mir::builder::{
    BuilderInvocationConfigV1, CallableMainMaterializationPolicyV1, MirBuilder,
    ModuleBuilderInvocationSessionV1, NormalDefaultRootCatalogLifecycleStageV1,
    NormalRuntimeInputSnapshotV1, PreparedNormalDefaultProgramRootV1,
};
use crate::parser::{NyashParser, ParserBuildConfig};
use hakorune_mir_defs::CanonicalGlobalTargetV1;

pub(super) fn callable_source(
    source: &str,
    config: ParserBuildConfig,
) -> PreparedNormalDefaultProgramRootV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(source, config)
        .expect("normal callable source");
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform")
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("fixture must remain source-backed")
    };
    PreparedNormalDefaultProgramRootV1::from_callable_source(source)
}

pub(super) fn session() -> ModuleBuilderInvocationSessionV1 {
    let current = MirBuilder::new();
    let config = BuilderInvocationConfigV1::snapshot_for_raw(&current, None);
    ModuleBuilderInvocationSessionV1::open(&current, config)
}

#[test]
fn artifact_validation_rejects_uncovered_sibling_and_empty_birth() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    use crate::mir::{
        BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction,
        MirType, ValueId,
    };
    for (empty_birth, exact_read) in [(false, false), (true, false), (false, true)] {
        let source = callable_source("print(42)", ParserBuildConfig::default());
        let completed = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                source,
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .unwrap();
        let (_, mut module, validate) = completed.into_artifact_parts();
        let key =
            hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Unowned", 0);
        let name = if empty_birth {
            key.mir_symbol_projection()
        } else {
            "uncovered".into()
        };
        let entry = BasicBlockId::new(0);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: name.clone(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            entry,
        );
        let mut block = BasicBlock::new(entry);
        if exact_read {
            let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap();
            let field = hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(object, 0)
                .unwrap();
            block.instructions.push(MirInstruction::ObjectFieldGet {
                dst: ValueId(1),
                base: ValueId(0),
                field,
            });
            block.instruction_spans.push(Span::unknown());
        }
        block.set_terminator(if empty_birth {
            crate::mir::MirInstruction::Return { value: None }
        } else if exact_read {
            MirInstruction::Return {
                value: Some(ValueId(1)),
            }
        } else {
            MirInstruction::ReturnFault {
                fault_frame: ValueId::new(0),
            }
        });
        function.add_block(block);
        module.add_function(function);
        if empty_birth {
            module.canonical_callable_definitions.insert(key, name);
        }
        let error = validate(&module).unwrap_err();
        assert!(
            error.contains(if exact_read {
                "unowned-exact-field-read"
            } else if empty_birth {
                "uncovered-birth-definition"
            } else {
                "uncovered-lifecycle-function"
            }),
            "{error}"
        );
    }
}

#[test]
fn artifact_validation_rejects_exact_read_drift_and_birth_reentry() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    use crate::mir::{MirInstruction, ValueId};
    for mutation in ["base", "dst", "field", "missing", "duplicate", "birth"] {
        let source = callable_source(
            include_str!("../../../apps/typed-object-birth-min/main.hako"),
            ParserBuildConfig::default(),
        );
        let completed = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                source,
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .unwrap();
        let (_, mut module, validate) = completed.into_artifact_parts();
        let root = module.functions.get_mut("main").unwrap();
        let block =
            root.blocks
                .values_mut()
                .find(|block| {
                    block.instructions.iter().any(|instruction| {
                        matches!(instruction, MirInstruction::ObjectFieldGet { .. })
                    })
                })
                .unwrap();
        let index = block
            .instructions
            .iter()
            .position(|instruction| matches!(instruction, MirInstruction::ObjectFieldGet { .. }))
            .unwrap();
        let original = block.instructions[index].clone();
        match mutation {
            "missing" => {
                block.instructions.remove(index);
                block.instruction_spans.remove(index);
            }
            "duplicate" => {
                block.instructions.push(original.clone());
                block.instruction_spans.push(Span::unknown());
            }
            "birth" => {}
            _ => {
                let MirInstruction::ObjectFieldGet { dst, base, field } =
                    &mut block.instructions[index]
                else {
                    unreachable!()
                };
                match mutation {
                    "base" => *base = ValueId(90001),
                    "dst" => *dst = ValueId(90002),
                    "field" => {
                        *field = hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(
                            field.object(),
                            999,
                        )
                        .unwrap()
                    }
                    _ => unreachable!(),
                }
            }
        }
        if mutation == "birth" {
            let birth = module.functions.get_mut("Pair.birth/2").unwrap();
            let block = birth.blocks.get_mut(&birth.entry_block).unwrap();
            block.instructions.push(original);
            block.instruction_spans.push(Span::unknown());
        }
        let error = validate(&module).unwrap_err();
        assert!(
            error.contains(if mutation == "birth" {
                "unowned-exact-field-read"
            } else {
                "ordinary-field-read/"
            }),
            "{mutation}: {error}"
        );
    }
}

#[test]
fn artifact_validation_rejects_terminal_add_operand_drift() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        include_str!("../../../apps/typed-object-birth-min/main.hako"),
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("source-backed Pair must lower");
    let (_, mut module, validate) = completed.into_artifact_parts();
    let root = module.functions.get_mut("main").expect("main root");
    let add = root
        .blocks
        .values_mut()
        .flat_map(|block| block.instructions.iter_mut())
        .find(|instruction| {
            matches!(
                instruction,
                crate::mir::MirInstruction::BinOp {
                    op: crate::mir::BinaryOp::Add,
                    ..
                }
            )
        })
        .expect("terminal Add");
    let crate::mir::MirInstruction::BinOp { lhs, .. } = add else {
        unreachable!()
    };
    *lhs = crate::mir::ValueId(90001);
    assert!(validate(&module)
        .unwrap_err()
        .contains("ordinary-terminal-result/add-binding-drift"));
}

#[test]
fn verified_expansion_disposition_reaches_script_and_app_root_lowering() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    for (source, expected_app_mode) in [
        ("42", false),
        ("static box Main { main() { return 0 } }", true),
    ] {
        let source = NyashParser::parse_from_string(source).expect("route source");
        let source = PreparedNormalDefaultProgramRootV1::seal(source).expect("Program source");
        let completed = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                source,
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect("verified route must lower");
        let (session, _, _) = completed.into_parts();

        assert_eq!(session.builder().root_is_app_mode, Some(expected_app_mode));
    }
}

#[test]
fn source_backed_print_producer_publishes_typed_builtin_row() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let print_source = callable_source("print(42)", ParserBuildConfig::default());
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            print_source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("source-backed Print producer must lower");
    let (_, module, _) = completed.into_parts();
    let calls = module
        .functions
        .iter()
        .flat_map(|(_, function)| function.blocks.values())
        .flat_map(|block| block.all_instructions())
        .filter_map(|instruction| match instruction {
            crate::mir::MirInstruction::Call(call) => Some((
                call.dst,
                crate::mir::ValueId::INVALID,
                Some(call.callee.clone()),
                call.args.len(),
            )),
            crate::mir::MirInstruction::LegacyCallV0 {
                dst,
                func,
                callee,
                args,
                ..
            } => Some((*dst, *func, callee.clone(), args.len())),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(calls.len(), 1, "Print producer must publish one call");
    let (dst, func, callee, arg_len) = calls.into_iter().next().expect("Print call");
    assert_eq!(dst, None, "builtin Print has no destination");
    assert_eq!(func, crate::mir::ValueId::INVALID);
    assert_eq!(
        callee,
        Some(crate::mir::Callee::Global(
            CanonicalGlobalTargetV1::builtin_print(),
        ))
    );
    assert_eq!(arg_len, 1, "Print keeps one source argument");

    let backend_view = crate::mir::function::PublishedMirBackendView::try_new(&module)
        .expect("published Print producer view");
    assert_eq!(
        backend_view.route(),
        crate::mir::function::PublishedStaticMethodRouteV1::CanonicalTyped
    );
    assert_eq!(backend_view.builtin_print_calls().len(), 1);
    assert!(backend_view.static_method_calls().is_empty());
    assert!(backend_view.free_function_calls().is_empty());

    for (source, expected) in [
        (
            "print(1)",
            crate::mir::builder::AdmittedNormalRootExecutionModeV1::ProgramRuntime,
        ),
        (
            "static box Main { main() { return 0 } }",
            crate::mir::builder::AdmittedNormalRootExecutionModeV1::App,
        ),
    ] {
        match callable_source(source, ParserBuildConfig::default())
            .consume_source_backed_root_once()
        {
            NormalDefaultProgramRootConsumptionV1::SourceBacked(Ok(consumed)) => {
                assert_eq!(consumed.consume_at_named_test_terminal(), expected);
            }
            NormalDefaultProgramRootConsumptionV1::SourceBacked(Err(rejected)) => {
                rejected.discard_at_named_root_execution_terminal();
                panic!("source-backed facade unexpectedly rejected")
            }
            NormalDefaultProgramRootConsumptionV1::Compatibility(source) => {
                RejectedNormalDefaultRootOwnerV1::Compatibility(source)
                    .discard_at_named_lifecycle_terminal();
                panic!("source-backed fixture entered compatibility")
            }
        }
    }
}

#[test]
fn source_backed_app_main_root_uses_cataloged_scope() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        "static box Main { main() { return 0 } }",
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("source-backed App Main root must lower through its package scope");
    let (_, module, _) = completed.into_parts();
    assert!(module
        .functions
        .iter()
        .any(|(_, function)| function.signature.name == "main"));
}

#[test]
fn source_backed_app_main_direct_call_consumes_affine_loan() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        "static box Main { main() { return helper(2) } helper(value: i64): i64 { return value } }",
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("source-backed App Main direct call must consume its loan");
    let (_, module, _) = completed.into_parts();
    let main = module
        .functions
        .iter()
        .find(|(_, function)| function.signature.name == "main")
        .map(|(_, function)| function)
        .expect("lowered Main function");
    let calls = main
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter(|instruction| {
            matches!(
                instruction,
                crate::mir::MirInstruction::Call(_)
                    | crate::mir::MirInstruction::LegacyCallV0 { .. }
                    | crate::mir::MirInstruction::Invoke {
                        operation: crate::mir::instruction::InvokeOperation::Call { .. },
                        ..
                    }
            )
        })
        .count();
    assert_eq!(calls, 1);
    let callee = main
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .find_map(|instruction| match instruction {
            crate::mir::MirInstruction::Call(call) => Some(call.callee.clone()),
            crate::mir::MirInstruction::LegacyCallV0 { callee, .. } => callee.clone(),
            crate::mir::MirInstruction::Invoke {
                operation:
                    crate::mir::instruction::InvokeOperation::Call {
                        call, ..
                    },
                ..
            } => Some(call.callee.clone()),
            _ => None,
        })
        .expect("direct call callee");
    assert_eq!(
        callee,
        crate::mir::Callee::Global(
            hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::test_static_box_method(
                "Main", "helper", 1,
            )
            .canonical_global_target_v1()
            .expect("static helper target"),
        )
    );
    assert_eq!(module.canonical_callable_definition_count(), 1);
    let backend_view = crate::mir::function::PublishedMirBackendView::try_new(&module)
        .expect("published App Main direct-call view");
    // Lifecycle-bearing direct calls lower through Invoke, which the
    // published view classifies honestly: the call family has no
    // selected-C corridor row, so the module reports
    // UnsupportedBeforeObject rather than a wrong CanonicalTyped.
    assert_eq!(
        backend_view.route(),
        crate::mir::function::PublishedStaticMethodRouteV1::UnsupportedBeforeObject
    );
    assert!(backend_view.static_method_calls().is_empty());
}

#[test]
fn source_backed_app_main_qualified_static_call_uses_canonical_owner() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        "static box Helpers { run(value: i64): i64 { return value } } static box Main { main() { return Helpers.run(2) } }",
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("source-backed qualified Main call must use the canonical owner");
    let (_, module, _) = completed.into_parts();
    let main = module
        .functions
        .iter()
        .find(|(_, function)| function.signature.name == "main")
        .map(|(_, function)| function)
        .expect("lowered Main function");
    let call = main
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .find_map(|instruction| match instruction {
            crate::mir::MirInstruction::Call(call) => Some(call),
            crate::mir::MirInstruction::LegacyCallV0 { .. } => None,
            _ => None,
        })
        .expect("qualified call physical terminal");
    assert_eq!(
        call.callee,
        crate::mir::Callee::Global(
            hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::test_static_box_method(
                "Helpers", "run", 1,
            )
            .canonical_global_target_v1()
            .expect("qualified static target"),
        )
    );
    assert_eq!(module.canonical_callable_definition_count(), 1);
}

#[test]
fn source_backed_declared_instance_me_method_emits_mandatory_receiver_call() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        "box Probe { wrap(value) { return value } run() { return me.wrap(7) } } static box Main { main() { return 0 } }",
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("source-backed declared instance method must lower");
    let (_, module, _) = completed.into_parts();
    let run = module
        .functions
        .iter()
        .find(|(_, function)| function.signature.name == "Probe.run/0")
        .map(|(_, function)| function)
        .expect("lowered Probe.run function");
    let call = run
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .find_map(|instruction| match instruction {
            crate::mir::MirInstruction::Call(call) => {
                Some((Some(call.callee.clone()), call.args.clone()))
            }
            crate::mir::MirInstruction::LegacyCallV0 { callee, args, .. } => {
                Some((callee.clone(), args.clone()))
            }
            _ => None,
        })
        .expect("declared instance call");
    assert_eq!(call.1.len(), 1, "receiver must stay outside source args");
    assert!(matches!(
        call.0,
        Some(crate::mir::Callee::SameModuleInstance { ref key, receiver })
            if key.namespace() == hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
                && key.owner() == "Probe"
                && key.name() == "wrap"
                && key.arity() == 1
                && receiver == crate::mir::ValueId::new(0)
    ));
    let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method(
        "Probe", "wrap", 1,
    );
    assert_eq!(
        module.canonical_callable_definition_symbol(&key),
        Some("Probe.wrap/1")
    );
    assert_eq!(module.canonical_callable_definition_count(), 2);
}

#[test]
fn root_expansion_failure_precedes_prepare_and_retains_source() {
    let source = NyashParser::parse_from_string(
        r#"
                static box Main { main() { return 0 } }
                static box Main { main() { return 1 } }
            "#,
    )
    .expect("duplicate Main source");
    let source = PreparedNormalDefaultProgramRootV1::seal(source).expect("Program source");
    let rejected = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect_err("duplicate Main must reject before prepare");

    assert_eq!(
        rejected.stage(),
        NormalDefaultRootCatalogLifecycleStageV1::RootExpansion
    );
    assert!(rejected.session.builder().current_module.is_none());
    assert!(matches!(
        rejected
            ._source
            .as_ref()
            .expect("preflight rejection retains compatibility source")
            .source_ast(),
        crate::ast::ASTNode::Program { .. }
    ));
    rejected.discard();
}

#[test]
fn source_backed_non_static_main_rejects_with_policy_before_builder_effects() {
    let source = callable_source(
        "box Main { main() { return 0 } }",
        ParserBuildConfig::default(),
    );
    let rejected = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect_err("non-static Main must reject at source policy");

    assert_eq!(
        rejected.stage(),
        NormalDefaultRootCatalogLifecycleStageV1::RootExpansion
    );
    assert!(rejected.session.builder().current_module.is_none());
    assert!(rejected
        .error()
        .to_string()
        .contains("SourcePolicy(MainMustBeStatic)"));
    assert!(rejected._source.is_some());
    rejected.discard();
}

#[test]
fn catalog_failure_follows_prepare_and_retains_source() {
    let ASTNode::Program { mut statements, .. } =
        NyashParser::parse_from_string("box Duplicate { first() { return 0 } }")
            .expect("first Box source")
    else {
        unreachable!()
    };
    let ASTNode::Program {
        statements: second, ..
    } = NyashParser::parse_from_string("box Duplicate { second() { return 1 } }")
        .expect("second Box source")
    else {
        unreachable!()
    };
    statements.extend(second);
    let source = ASTNode::Program {
        statements,
        span: Span::unknown(),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(source).expect("Program source");
    let rejected = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect_err("duplicate Box owner must reject during catalog seal");

    assert_eq!(
        rejected.stage(),
        NormalDefaultRootCatalogLifecycleStageV1::CatalogSeal
    );
    assert!(rejected.session.builder().current_module.is_some());
    assert!(matches!(
        rejected
            ._source
            .as_ref()
            .expect("catalog rejection retains compatibility source")
            .source_ast(),
        crate::ast::ASTNode::Program { .. }
    ));
    rejected.discard();
}

#[test]
fn source_bound_static_result_owner_reaches_the_raw_terminal() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        let source = NyashParser::parse_from_string(
            r#"
                static box StringHelpers {
                    int_to_str(n) {
                        local value = me.to_i64("x")
                        return value
                    }
                    to_i64(x) { return x + 1 }
                }
                "#,
        )
        .expect("source-bound static fixture");
        let source = PreparedNormalDefaultProgramRootV1::seal(source).expect("Program source");
        let rejected = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                source,
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect_err("compatibility static box fate is retired");
        let error = rejected.error().to_string();
        // Reaching the raw terminal is now the retired freeze itself —
        // 7167aee18e retired the raw runtime-box-fate lane this fixture
        // deliberately drives.
        assert!(
            error.contains("[freeze:contract][raw-compat/runtime-box-fate-retired/static]"),
            "{error}"
        );
        rejected.discard();
    });
}

#[test]
fn source_backed_selected_callable_uses_the_installed_package_port() {
    let source = callable_source(
        "static box Scan { run(value) { return value } }",
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("source-backed package must lower");
    let (_, module, _) = completed.into_parts();

    assert!(module
        .functions
        .iter()
        .any(|(_, function)| function.signature.name == "Scan.run/1"));
}

#[test]
fn parser_scan_package_passes_callable_source_handoff_without_fallback() {
    let source = callable_source(
        include_str!(concat!(
            "../../../lang/src/compiler/parser/scan/",
            "parser_scan_loop_box.hako"
        )),
        ParserBuildConfig::default(),
    );
    let rejected = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect_err("Dynamic physical consumption is not claimed by this cutover");

    assert_eq!(
        rejected.stage(),
        NormalDefaultRootCatalogLifecycleStageV1::RootLower
    );
    assert!(
        rejected
            .error()
            .to_string()
            .contains("static-result-ingress/no-exact-static-target"),
        "unexpected next blocker: {}",
        rejected.error()
    );
    assert!(!rejected
        .error()
        .to_string()
        .contains("callable-semantic-lowering/missing-variable-site"));
    assert!(rejected._source.is_none());
    rejected.discard();
}

/// `local h = me.fetch(..)` where `fetch` claims `NullableObject` lowers
/// through the nullable lifecycle lane: the call site emits the
/// `NullableHandle` invoke result and the caller's exit chain owes the
/// received value a checked `HomeReleaseIfLive` — never an unconditional
/// `HomeRelease` on a value that may carry the `Void` sentinel.
#[test]
fn nullable_receiver_call_emits_lifecycle_invoke_and_checked_release() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        r#"
box Probe {
    init { v }
    birth(v) { me.v = v }
    fetch(flag) {
        if flag == 0 {
            return null
        }
        return new Probe(7)
    }
    run(flag: i64) {
        local h = me.fetch(flag)
        return 0
    }
}
static box Main {
    main() {
        local p = new Probe(1)
        return p.run(0)
    }
}
"#,
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("nullable receiver call must lower");
    let (_, module, _) = completed.into_parts();
    let run = module
        .functions
        .iter()
        .find(|(_, function)| function.signature.name == "Probe.run/1")
        .map(|(_, function)| function)
        .expect("lowered Probe.run function");
    let invokes: Vec<_> = run
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter_map(|instruction| match instruction {
            crate::mir::MirInstruction::Invoke { operation, .. } => Some(operation),
            _ => None,
        })
        .collect();
    assert!(
        invokes.iter().any(|operation| matches!(
            operation,
            crate::mir::instruction::InvokeOperation::Call {
                result: crate::mir::instruction::InvokeCallResultKind::NullableHandle,
                ..
            }
        )),
        "nullable receiver call must emit the NullableHandle invoke: {invokes:?}"
    );
    assert!(
        invokes.iter().any(|operation| matches!(
            operation,
            crate::mir::instruction::InvokeOperation::HomeReleaseIfLive { .. }
        )),
        "the received nullable owes a checked release at exit: {invokes:?}"
    );
    assert!(
        invokes.iter().all(|operation| !matches!(
            operation,
            crate::mir::instruction::InvokeOperation::HomeRelease { .. }
        )),
        "a nullable result must never emit an unconditional release: {invokes:?}"
    );
}

/// An unannotated parameter argument rides the scalar call edge under
/// the sealed untyped admission (`check_call_edge`): `flag` records
/// `MirType::Unknown` and the nullable invoke still emits — the wire
/// carries the binding's slot, never a guessed carrier.
#[test]
fn nullable_receiver_call_admits_untyped_parameter_argument() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        r#"
box Probe {
    init { v }
    birth(v) { me.v = v }
    fetch(flag) {
        if flag == 0 {
            return null
        }
        return new Probe(7)
    }
    run(flag) {
        local h = me.fetch(flag)
        return 0
    }
}
static box Main {
    main() {
        local p = new Probe(1)
        return p.run(0)
    }
}
"#,
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("untyped parameter argument must ride the scalar edge");
    let (_, module, _) = completed.into_parts();
    let run = module
        .functions
        .iter()
        .find(|(_, function)| function.signature.name == "Probe.run/1")
        .map(|(_, function)| function)
        .expect("lowered Probe.run function");
    let invokes: Vec<_> = run
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter_map(|instruction| match instruction {
            crate::mir::MirInstruction::Invoke { operation, .. } => Some(operation),
            _ => None,
        })
        .collect();
    assert!(
        invokes.iter().any(|operation| matches!(
            operation,
            crate::mir::instruction::InvokeOperation::Call {
                result: crate::mir::instruction::InvokeCallResultKind::NullableHandle,
                ..
            }
        )),
        "the untyped parameter argument must emit the NullableHandle invoke: {invokes:?}"
    );
}

/// A `Local` argument whose recorded wire type is a concrete non-i64
/// carrier stays rejected: the sealed scalar edge corroborates that
/// record as `call-argument-type-drift`, so emission freezes closed
/// rather than letting a non-scalar value ride the i64 corridor. A
/// `Bool` local keeps no Home obligation, so the argument carrier is
/// the first gate the site reaches.
#[test]
fn nullable_receiver_call_rejects_concrete_non_i64_local_argument() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        r#"
box Probe {
    init { v }
    birth(v) { me.v = v }
    fetch(flag) {
        if flag == 0 {
            return null
        }
        return new Probe(7)
    }
    run() {
        local b = true
        local h = me.fetch(b)
        return 0
    }
}
static box Main {
    main() {
        local p = new Probe(1)
        return p.run()
    }
}
"#,
        ParserBuildConfig::default(),
    );
    let rejected = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect_err("a concrete box carrier must not ride the scalar edge");
    assert!(
        rejected
            .error()
            .to_string()
            .contains("nullable-argument-carrier"),
        "unexpected rejection: {}",
        rejected.error()
    );
    rejected.discard();
}

/// A `new` argument naming a live owned binding moves the lease into the
/// constructed object: `return new Holder(h)` consumes `h` on that exit
/// path, so the tail exit emits no `HomeReleaseIfLive`, while the sibling
/// `h == null` exit still owes its checked release. `Holder`'s duplicate
/// birth store stays construction-unsupported, so the `new` rides the raw
/// lane and no fault-unwind operand enters the count — exactly one
/// checked release survives: one per owed exit, never one per function.
#[test]
fn nullable_result_moved_into_new_releases_only_on_the_owed_exit() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        r#"
box OwMoveHolder {
    init { v }
    birth(v) {
        me.v = v
        me.v = v
    }
}
box OwMoveProbe {
    init { v }
    birth(v) { me.v = v }
    fetch(flag) {
        if flag == 0 {
            return null
        }
        return new OwMoveProbe(7)
    }
    run(flag: i64) {
        local h = me.fetch(flag)
        if h == null {
            return new OwMoveHolder(0)
        }
        return new OwMoveHolder(h)
    }
}
static box OwMoveMain {
    main() {
        local p = new OwMoveProbe(1)
        return p.run(0)
    }
}
"#,
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("a moved nullable argument must keep the owed-exit release only");
    let (_, module, _) = completed.into_parts();
    let run = module
        .functions
        .iter()
        .find(|(_, function)| function.signature.name == "OwMoveProbe.run/1")
        .map(|(_, function)| function)
        .expect("lowered OwMoveProbe.run function");
    let checked_releases = run
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter(|instruction| matches!(
            instruction,
            crate::mir::MirInstruction::Invoke {
                operation: crate::mir::instruction::InvokeOperation::HomeReleaseIfLive { .. },
                ..
            }
        ))
        .count();
    assert_eq!(
        checked_releases, 1,
        "the moved `new` argument owes a release only on the sibling exit"
    );
}

/// `local r = pool.allocate(8)` — a claim-local lexical receiver call
/// whose callee returns an exact-`i64` scalar — lowers through the
/// lexical lifecycle lane: the Standard-route gate corroborates the
/// sealed observation with the minted disposition row and emits
/// `Invoke{SameModuleInstance, I64}` — the receiver rides the callee's
/// `me` slot outside the source arguments, the literal `8` materializes
/// inside them, and the bound result is registered `Integer` so the
/// terminal `return r` type-checks.
#[test]
fn lexical_i64_instance_call_emits_lifecycle_invoke() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        r#"
box Pool {
    birth() { }
    allocate(size: i64): i64 { return size }
}
static box Main {
    main() {
        local pool = new Pool()
        local r = pool.allocate(8)
        return r
    }
}
"#,
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("lexical i64 instance call must lower");
    let (_, module, _) = completed.into_parts();
    let main = module
        .functions
        .iter()
        .find(|(_, function)| function.signature.name == "main")
        .map(|(_, function)| function)
        .expect("lowered main function");
    let invokes: Vec<_> = main
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter_map(|instruction| match instruction {
            crate::mir::MirInstruction::Invoke {
                operation:
                    crate::mir::instruction::InvokeOperation::Call { call, result },
                ..
            } => Some((call, *result)),
            _ => None,
        })
        .collect();
    let (call, result) = invokes
        .iter()
        .find(|(call, _)| {
            matches!(
                &call.callee,
                crate::mir::Callee::SameModuleInstance { key, .. }
                    if key.namespace()
                        == hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
                        && key.owner() == "Pool"
                        && key.name() == "allocate"
                        && key.arity() == 1
            )
        })
        .expect("lexical i64 instance invoke");
    assert_eq!(*result, crate::mir::instruction::InvokeCallResultKind::I64);
    assert_eq!(
        call.args.len(),
        1,
        "the literal argument materializes inside the source args"
    );
}

/// `local r = pool.give(pool.allocate(8))` — a proven-i64 call nested in
/// direct argument position of another proven-i64 lexical call — lowers
/// as two ordered `Invoke{SameModuleInstance, I64}` instructions: the
/// inner `allocate` result feeds the outer `give` argument slot, and
/// both invocations fold into the outer statement's single recorded
/// binding group (the inner call owns no destination binding).
#[test]
fn lexical_i64_call_result_argument_emits_ordered_invokes() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        r#"
box Pool {
    birth() { }
    allocate(size: i64): i64 { return size }
    give(p: i64): i64 { return p }
}
static box Main {
    main() {
        local pool = new Pool()
        local r = pool.give(pool.allocate(8))
        return r
    }
}
"#,
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("nested call-result argument must lower");
    let (_, module, _) = completed.into_parts();
    let main = module
        .functions
        .iter()
        .find(|(_, function)| function.signature.name == "main")
        .map(|(_, function)| function)
        .expect("lowered main function");
    let invoke_at = |name: &str| {
        main.blocks
            .iter()
            .find_map(|(id, block)| {
                block
                    .all_instructions()
                    .find(|instruction| {
                        matches!(
                            instruction,
                            crate::mir::MirInstruction::Invoke {
                                operation:
                                    crate::mir::instruction::InvokeOperation::Call {
                                        call,
                                        result
                                    },
                                ..
                            } if *result == crate::mir::instruction::InvokeCallResultKind::I64
                                && matches!(
                                    &call.callee,
                                    crate::mir::Callee::SameModuleInstance { key, .. }
                                        if key.namespace()
                                            == hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
                                            && key.owner() == "Pool"
                                            && key.name() == name
                                )
                        )
                    })
                    .map(|instruction| (*id, instruction))
            })
            .unwrap_or_else(|| panic!("{name} i64 instance invoke"))
    };
    let (allocate_block, allocate_invoke) = invoke_at("allocate");
    let (give_block, give_invoke) = invoke_at("give");
    let (
        crate::mir::MirInstruction::Invoke {
            normal_landing: allocate_landing,
            ..
        },
        crate::mir::MirInstruction::Invoke {
            operation: crate::mir::instruction::InvokeOperation::Call { call, .. },
            ..
        },
    ) = (allocate_invoke, give_invoke)
    else {
        panic!("invoke shapes")
    };
    // The outer call emits inside the inner call's normal-landing block —
    // the CFG edge is the ordering authority, not block-map iteration.
    assert_eq!(
        give_block, *allocate_landing,
        "the consuming invoke seats on the argument call's normal edge"
    );
    // The inner projection names the inner result value, which feeds the
    // outer argument slot.
    let inner_result = main
        .blocks
        .get(allocate_landing)
        .and_then(|block| {
            block.all_instructions().find_map(|instruction| match instruction {
                crate::mir::MirInstruction::InvokeNormalResult {
                    invoke_block,
                    dst,
                } if *invoke_block == allocate_block => Some(*dst),
                _ => None,
            })
        })
        .expect("inner invoke normal projection");
    assert!(
        call.args.contains(&inner_result),
        "the inner call result feeds the outer argument slot: {:?}",
        call.args
    );
}
