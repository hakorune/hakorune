use super::normal_default_root_catalog_lifecycle_tests::{callable_source, session};
use super::RejectedNormalDefaultRootOwnerV1;
use crate::mir::builder::{
    CallableMainMaterializationPolicyV1, NormalDefaultRootCatalogLifecycleStageV1,
    NormalRuntimeInputSnapshotV1, PreparedNormalDefaultProgramRootV1,
};
use crate::parser::{BuildMode, NyashParser, ParserBuildConfig};

#[test]
fn artifact_validation_rejects_selected_ordinary_child_symbol_drift() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source("box Page {} static box Main { main() { return me.helper(2) } helper(value: i64): i64 { return value } }", ParserBuildConfig::default());
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("ordinary child source must lower");
    let (_, mut module, validate) = completed.into_artifact_parts();
    let (_, helper) = module
        .functions
        .iter_mut()
        .find(|(_, function)| function.signature.name.contains("helper"))
        .expect("ordinary helper definition");
    helper.signature.name.push_str("_drift");
    let error = validate(&module).expect_err("child symbol drift must reject");
    assert!(error.contains("child-definition-symbol-drift"), "{error}");
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
        NormalDefaultRootCatalogLifecycleStageV1::RootExpansion,
        "unexpected stage for error: {}",
        rejected.error()
    );
    // Root-execution consumption rejects the selected-gate program
    // before semantic seal and retains the rejected owner so the
    // failing source stays inspectable — same preflight-retains-source
    // contract as the compatibility arm.
    assert!(
        rejected
            .error()
            .to_string()
            .contains("[mir/normal-root/consume]"),
        "{}",
        rejected.error()
    );
    assert!(rejected.session.builder().current_module.is_none());
    assert!(matches!(
        rejected._source,
        Some(RejectedNormalDefaultRootOwnerV1::RootExecution(_))
    ));
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
        let rejected = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                source,
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect_err("compatibility static box fate is retired");
        let error = rejected.error().to_string();
        // A Compatibility root is exactly the raw lane; its static-box fate
        // terminal retired with 7167aee18e, so the honest receipt is the
        // retired freeze, not a silent fallback lowering.
        assert!(
            error.contains("[freeze:contract][raw-compat/runtime-box-fate-retired/static]"),
            "{error}"
        );
        rejected.discard();
    });
}

#[test]
fn map_lifecycle_stop_precedes_catalog_install_and_body_allocation() {
    for body in [
        "local m: i64 = %{} return 30",
        "local a = new Page() local m: i64 = %{\"a\" => a} return 30",
    ] {
        let source = callable_source(
            &format!("box Page {{}} static box Main {{ main() {{ {body} }} }}"),
            ParserBuildConfig::default(),
        );
        let rejected = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                source,
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect_err("Map annotation rejects before body effects");
        assert_eq!(
            rejected.stage(),
            NormalDefaultRootCatalogLifecycleStageV1::CatalogInstall
        );
        assert!(rejected.error().to_string().contains("MapLocalAnnotation"));
        assert!(rejected
            .session
            .builder()
            .comp_ctx
            .callable_declaration_catalog_vacant());
        if let Some(module) = &rejected.session.builder().current_module {
            assert!(module.functions.values().all(|function| function
                .blocks
                .values()
                .all(|block| block.all_instructions().next().is_none())));
        }
        assert!(rejected._source.is_none(), "no compatibility retry source");
        rejected.discard();
    }
}

#[test]
fn main0_loop_variants_publish_one_main_through_collector() {
    for (case_name, source_text, description) in [
        (
            "main0_continue_canonical_root_publishes_one_main_through_collector",
            include_str!("../../../apps/tests/phase29ca_generic_loop_continue_min.hako"),
            "selected Main0 continue fixture lowers through the canonical root route",
        ),
        (
            "main0_in_body_step_canonical_root_publishes_one_main_through_collector",
            include_str!("../../../apps/tests/phase29cb_generic_loop_in_body_step_min.hako"),
            "selected Main0 in-body-step fixture lowers through the canonical root route",
        ),
        (
            "main0_derived_predicate_canonical_root_publishes_one_main_through_collector",
            include_str!("../../../apps/tests/generic_loop_carrier_type_v0_numeric_min.hako"),
            "selected Main0 derived-predicate fixture lowers through the canonical root route",
        ),
    ] {
        println!("case: {case_name}");
        crate::runtime::ring0::ensure_global_ring0_initialized();
        let source = callable_source(source_text, ParserBuildConfig::default());
        let completed = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                source,
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect(description);
        let (_, module, validate) = completed.into_parts();
        let main = module.get_function("main").expect("canonical root main");
        // One physical Main: exactly one function owns the root symbol and
        // exactly one function is marked as the module entry point.
        assert_eq!(
            module
                .functions
                .keys()
                .filter(|name| name.as_str() == "main")
                .count(),
            1
        );
        assert_eq!(
            module
                .functions
                .values()
                .filter(|function| function.metadata.is_entry_point)
                .count(),
            1
        );
        // Root finalization must not append a second value Return.
        let returns = main
            .blocks
            .values()
            .filter(|block| {
                matches!(
                    &block.terminator,
                    Some(crate::mir::MirInstruction::Return { value: Some(_) })
                )
            })
            .count();
        assert_eq!(returns, 1, "canonical main owns exactly one value Return");
        // Current loop lowering must retain a real backedge PHI.
        assert!(main.blocks.values().any(|block| {
            block
                .instructions
                .iter()
                .any(|instruction| matches!(instruction, crate::mir::MirInstruction::Phi { .. }))
        }));
        validate(&module).expect("canonical main passes final root validation");
    }
}

#[test]
fn non_profile_app_main_declines_selection_and_keeps_legacy_wrapper() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    // Observation-only selection must leave the one-shot loan intact for the
    // legacy wrapper when the bounded If/Continue profile does not match.
    let source = callable_source(
        "static box Main { main() { return 7 } }",
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("declined App Main still lowers through the legacy wrapper route");
    let (_, module, validate) = completed.into_parts();
    let main = module.get_function("main").expect("legacy wrapper main");
    assert_eq!(
        main.blocks
            .values()
            .filter(|block| {
                matches!(
                    &block.terminator,
                    Some(crate::mir::MirInstruction::Return { value: Some(_) })
                )
            })
            .count(),
        1
    );
    validate(&module).expect("declined App Main passes final root validation");
}

#[test]
fn main0_bound_variants_stay_on_the_canonical_route() {
    for (case_name, source_text) in [
        (
            "main0_in_body_step_bound_variant_stays_on_the_canonical_route",
            r#"static box Main {
    main() {
        local i = 0
        local tmp = 0
        loop(i < 2) {
            i = i + 1
            tmp = 1
        }
        return i
    }
}"#,
        ),
        (
            "main0_derived_predicate_bound_variant_stays_on_the_canonical_route",
            r#"static box Main {
    main() {
        local j = 0
        local m = 0
        local n = 2
        loop(j + m <= n) {
            j = j + 1
        }
        return j
    }
}"#,
        ),
    ] {
        println!("case: {case_name}");
        crate::runtime::ring0::ensure_global_ring0_initialized();
        // A different bound literal keeps the same source/PHI guarantees.
        let source = callable_source(source_text, ParserBuildConfig::default());
        let completed = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                source,
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect("bound-2 variant keeps the canonical root route");
        let (_, module, validate) = completed.into_parts();
        let main = module.get_function("main").expect("canonical root main");
        assert!(main.blocks.values().any(|block| {
            block
                .instructions
                .iter()
                .any(|instruction| matches!(instruction, crate::mir::MirInstruction::Phi { .. }))
        }));
        validate(&module).expect("bound variant passes final root validation");
    }
}
