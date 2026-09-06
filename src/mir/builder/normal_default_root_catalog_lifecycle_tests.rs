use super::{NormalDefaultProgramRootConsumptionV1, RejectedNormalDefaultRootOwnerV1};
use crate::ast::{ASTNode, Span};
use crate::mir::builder::{
    BuilderInvocationConfigV1, CallableMainMaterializationPolicyV1, MirBuilder,
    ModuleBuilderInvocationSessionV1, NormalDefaultRootCatalogLifecycleStageV1,
    NormalRuntimeInputSnapshotV1, PreparedNormalDefaultProgramRootV1,
};
use crate::parser::{BuildMode, NyashParser, ParserBuildConfig};
use hakorune_mir_defs::CanonicalGlobalTargetV1;

fn callable_source(source: &str, config: ParserBuildConfig) -> PreparedNormalDefaultProgramRootV1 {
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

fn session() -> ModuleBuilderInvocationSessionV1 {
    let current = MirBuilder::new();
    let config = BuilderInvocationConfigV1::snapshot_for_raw(&current, None);
    ModuleBuilderInvocationSessionV1::open(&current, config)
}

#[test]
fn artifact_validation_rejects_uncovered_sibling_and_empty_birth() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    use crate::mir::{BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirType, ValueId};
    for (empty_birth, exact_read) in [(false, false), (true, false), (false, true)] {
        let source = callable_source("print(42)", ParserBuildConfig::default());
        let completed = session().complete_normal_default_program_root_catalog_lifecycle(
            source, CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty()).unwrap();
        let (_, mut module, validate) = completed.into_artifact_parts();
        let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Unowned", 0);
        let name = if empty_birth { key.mir_symbol_projection() } else { "uncovered".into() };
        let entry = BasicBlockId::new(0);
        let mut function = MirFunction::new(FunctionSignature {
            name: name.clone(), params: vec![], return_type: MirType::Void,
            effects: EffectMask::PURE,
        }, entry);
        let mut block = BasicBlock::new(entry);
        if exact_read {
            let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap();
            let field = hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(object, 0).unwrap();
            block.instructions.push(MirInstruction::ObjectFieldGet {
                dst: ValueId(1), base: ValueId(0), field,
            });
            block.instruction_spans.push(Span::unknown());
        }
        block.set_terminator(if empty_birth { MirInstruction::Return { value: None } }
            else if exact_read { MirInstruction::Return { value: Some(ValueId(1)) } }
            else { MirInstruction::ReturnFault { fault_frame: ValueId::new(0) } });
        function.add_block(block);
        module.add_function(function);
        if empty_birth { module.canonical_callable_definitions.insert(key, name); }
        let error = validate(&module).unwrap_err();
        assert!(error.contains(if exact_read { "unowned-exact-field-read" }
            else if empty_birth { "uncovered-birth-definition" }
            else { "uncovered-lifecycle-function" }), "{error}");
    }
}

#[test]
fn artifact_validation_rejects_exact_read_drift_and_birth_reentry() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    use crate::mir::{MirInstruction, ValueId};
    for mutation in ["base", "dst", "field", "missing", "duplicate", "birth"] {
        let source = callable_source(include_str!("../../../apps/typed-object-birth-min/main.hako"),
            ParserBuildConfig::default());
        let completed = session().complete_normal_default_program_root_catalog_lifecycle(
            source, CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty()).unwrap();
        let (_, mut module, validate) = completed.into_artifact_parts();
        let root = module.functions.get_mut("main").unwrap();
        let block = root.blocks.values_mut().find(|block| block.instructions.iter()
            .any(|instruction| matches!(instruction, MirInstruction::ObjectFieldGet { .. }))).unwrap();
        let index = block.instructions.iter().position(|instruction|
            matches!(instruction, MirInstruction::ObjectFieldGet { .. })).unwrap();
        let original = block.instructions[index].clone();
        match mutation {
            "missing" => { block.instructions.remove(index); block.instruction_spans.remove(index); }
            "duplicate" => { block.instructions.push(original.clone()); block.instruction_spans.push(Span::unknown()); }
            "birth" => {}
            _ => {
                let MirInstruction::ObjectFieldGet { dst, base, field } = &mut block.instructions[index]
                    else { unreachable!() };
                match mutation {
                    "base" => *base = ValueId(90001),
                    "dst" => *dst = ValueId(90002),
                    "field" => *field = hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(
                        field.object(), 999).unwrap(),
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
        assert!(error.contains(if mutation == "birth" { "unowned-exact-field-read" }
            else { "ordinary-field-read/" }), "{mutation}: {error}");
    }
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
            crate::mir::MirInstruction::Call(call) => {
                Some((call.dst, crate::mir::ValueId::INVALID, Some(call.callee.clone()), call.args.len()))
            }
            crate::mir::MirInstruction::LegacyCallV0 {
                dst, func, callee, args, ..
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
            matches!(instruction, crate::mir::MirInstruction::Call(_) | crate::mir::MirInstruction::LegacyCallV0 { .. })
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
    assert_eq!(
        backend_view.route(),
        crate::mir::function::PublishedStaticMethodRouteV1::CanonicalTyped
    );
    assert_eq!(backend_view.static_method_calls().len(), 1);
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
        let completed = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                source,
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect("source-bound static row must lower");
        let (_, module, _) = completed.into_parts();
        assert!(module
            .functions
            .iter()
            .any(|(_, function)| function.signature.name == "StringHelpers.int_to_str/1"));
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
            .contains("static-result-ingress/target-unavailable"),
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

#[test]
fn source_backed_package_failure_is_terminal_before_builder_effects() {
    let source = callable_source(
        r#"
gate Build.test {
  static box ParserScanLoopBox {
    skip_while(src, pos, end, pred_chars) {
      local i = pos
      loop(i < end) {
        local ch = src.substring(i, i + 1)
        if pred_chars.indexOf(ch) < 0 { return i }
        i = i + 1
      }
      return i
    }
  }
}
"#,
        ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        },
    );
    let rejected = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect_err("missing selected-gate parameter authority must reject");

    assert_eq!(
        rejected.stage(),
        NormalDefaultRootCatalogLifecycleStageV1::CallableSemanticSeal
    );
    assert!(rejected.session.builder().current_module.is_none());
    assert!(rejected._source.is_none());
    rejected.discard();
}

#[test]
fn actual_string_helpers_general_result_row_reaches_its_first_loop_carrier() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        let source = NyashParser::parse_from_string(include_str!(concat!(
            "../../../lang/src/shared/common/",
            "string_helpers.hako"
        )))
        .expect("actual StringHelpers source");
        let source = PreparedNormalDefaultProgramRootV1::seal(source).expect("Program source");
        let completed = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                source,
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect("actual StringHelpers exact result must reach GenericLoop");
        let (_, module, _) = completed.into_parts();
        assert!(module
            .functions
            .iter()
            .any(|(_, function)| function.signature.name == "StringHelpers.int_to_str/1"));
    });
}
