use crate::ast::{ASTNode, Span};
use crate::mir::builder::{
    BuilderInvocationConfigV1, CallableMainMaterializationPolicyV1, MirBuilder,
    ModuleBuilderInvocationSessionV1, NormalDefaultRootCatalogLifecycleStageV1,
    NormalRuntimeInputSnapshotV1, PreparedNormalDefaultProgramRootV1,
};
use crate::parser::{BuildMode, NyashParser, ParserBuildConfig};

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
        let (session, _) = completed.into_parts();

        assert_eq!(session.builder().root_is_app_mode, Some(expected_app_mode));
    }
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
        let (_, module) = completed.into_parts();
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
    let (_, module) = completed.into_parts();

    assert!(module
        .functions
        .iter()
        .any(|(_, function)| function.signature.name == "Scan.run/1"));
}

#[test]
fn parser_scan_package_reaches_the_existing_physical_blocker_without_fallback() {
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
    assert!(rejected
        .error()
        .to_string()
        .contains("callable-semantic-lowering/incomplete-consumption"));
    assert!(rejected._source.is_none());
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
        let (_, module) = completed.into_parts();
        assert!(module
            .functions
            .iter()
            .any(|(_, function)| function.signature.name == "StringHelpers.int_to_str/1"));
    });
}
