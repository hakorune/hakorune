use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, CompilationContext, NormalCatalogedBoxMethodDraftAdmissionV1,
    NormalRootExecutionConsumerRejectV1, SelectedNormalCallableKeyV1,
};
use crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{
    BuildMode, NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1,
};

use super::{
    issue_normal_callable_semantic_package_v1, NormalCallableDynamicProjectionRefV1,
    NormalCallableSemanticPackageInstallIssueV1, NormalCallableSemanticPackageIssueV1,
    SelectedCallableSemanticRefV1,
};

use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;

pub(super) fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    final_source_with_config(source, ParserBuildConfig::default())
}

fn final_source_with_config(
    source: &str,
    config: ParserBuildConfig,
) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(source, config)
        .expect("normal callable source");
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    })
}

fn issue_with_config(
    source: &str,
    config: ParserBuildConfig,
) -> Result<super::VerifiedNormalCallableSemanticPackageV1, NormalCallableSemanticPackageIssueV1> {
    let mut resolver = FunctionSemanticResolverSessionV1::new(92).unwrap();
    issue_normal_callable_semantic_package_v1(
        &mut resolver,
        final_source_with_config(source, config),
    )
}

fn issue(
    source: &str,
) -> Result<super::VerifiedNormalCallableSemanticPackageV1, NormalCallableSemanticPackageIssueV1> {
    let mut resolver = FunctionSemanticResolverSessionV1::new(91).unwrap();
    issue_normal_callable_semantic_package_v1(&mut resolver, final_source(source))
}

#[test]
fn selected_dynamic_loan_carries_the_package_source_seed() {
    let package = issue(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .expect("exact Dynamic package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "ParserScanLoopBox",
            "skip_while",
            4,
        ),
    );
    let mut port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    port.with_selected_lowering_input(&key, |input| match input.semantic() {
        SelectedCallableSemanticRefV1::Dynamic { source, .. } => {
            assert_eq!(source.owner(), input.source().owner());
        }
        SelectedCallableSemanticRefV1::Ordinary => panic!("Dynamic row lost package seed"),
    })
    .expect("selected Dynamic loan");
}

#[test]
fn foreign_catalog_admission_rejects_before_selected_loan_callback() {
    let package = issue(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .expect("exact Dynamic package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let foreign = NormalCatalogedBoxMethodDraftAdmissionV1::seal(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("ForeignBox", "skip_while", 4),
    )
    .expect("foreign admission shape");
    let mut port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    let mut called = false;
    let result = port.with_selected_cataloged_lowering_input(foreign, |_| {
        called = true;
    });
    assert_eq!(
        result,
        Err(NormalCallableSemanticPackageInstallIssueV1::SelectedKeyUnavailable)
    );
    assert!(!called);
}

#[test]
fn package_scoped_loan_retains_exact_parameter_contract() {
    let package = issue("static box Api { run(source, pos: i64, end: i64, tail) { return pos } }")
        .expect("typed parameter package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Api", "run", 4),
    );
    let mut port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    port.with_selected_lowering_input(&key, |input| {
        let kinds = input
            .parameter_contracts()
            .map(|(_, _, kind)| kind)
            .collect::<Vec<_>>();
        assert_eq!(
            kinds,
            [
                CallableParameterContractKindV1::OpaqueHandle,
                CallableParameterContractKindV1::ExactTrivial(
                    crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64,
                ),
                CallableParameterContractKindV1::ExactTrivial(
                    crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64,
                ),
                CallableParameterContractKindV1::OpaqueHandle,
            ]
        );
    })
    .expect("exact contract loan");
    port.take_object_definitions(&context)
        .expect("explicit definition transfer for semantic-only test");
    port.complete().expect("selected contract consumed");
}

#[test]
fn package_scoped_loan_retains_exact_text_parameter_contract() {
    let package =
        issue("static box Api { run(source: StringBox, needle: StringBox) { return 0 } }")
            .expect("ordinary StringBox package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Api", "run", 2),
    );
    let mut port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    port.with_selected_lowering_input(&key, |input| {
        let rows = input.parameter_contracts().collect::<Vec<_>>();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, 0);
        assert_eq!(rows[1].0, 1);
        assert!(rows.iter().all(|(_, binding, kind)| {
            binding.owner() == input.source().owner()
                && matches!(
                    kind,
                    CallableParameterContractKindV1::ExactText(
                        crate::mir::exact_text_parameter_abi::ExactTextFormalAbiV1::STRING_BOX
                    )
                )
        }));
        assert_ne!(rows[0].1, rows[1].1);
    })
    .expect("exact text contract loan");
    port.take_object_definitions(&context)
        .expect("explicit definition transfer for semantic-only test");
    port.complete().expect("selected contract consumed");
}

#[test]
fn static_sibling_and_dynamic_candidate_share_one_complete_package_batch() {
    let dynamic = include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako");
    let source = format!("static box Helpers {{ helper(value) {{ return value }} }}\n{dynamic}");
    let package = issue(&source).expect("mixed static and Dynamic package");

    assert_eq!(package.batch().declarations().len(), 5);
    assert_eq!(package.parameter_declaration_count(), 5);
    assert_eq!(package.parameter_count(), 16);
    assert!(matches!(
        package.dynamic_projection(),
        NormalCallableDynamicProjectionRefV1::Selected { .. }
    ));
    assert_eq!(
        package
            .batch()
            .declarations()
            .next()
            .expect("static sibling remains first")
            .mode(),
        crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1::StaticBoxMethod
    );
}

#[test]
fn instance_shaped_scan_loop_stays_ordinary_owned() {
    let source = include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako")
        .replace("static box ParserScanLoopBox", "box ParserScanLoopBox");
    let package = issue(&source).expect("instance-shaped scan loop remains admissible ordinary");

    assert!(matches!(
        package.dynamic_projection(),
        NormalCallableDynamicProjectionRefV1::ValidUnselected
    ));
    assert!(package
        .batch()
        .declarations()
        .any(|declaration| declaration.mode()
            == ResolvedCallableDeclarationModeV1::InstanceBoxMethod));
}

#[test]
fn zero_dynamic_candidates_are_valid_unselected_without_default_or_name_selection() {
    let package = issue("static box Api { run(value) { return value } }")
        .expect("fully observed non-Dynamic package");
    assert!(matches!(
        package.dynamic_projection(),
        NormalCallableDynamicProjectionRefV1::ValidUnselected
    ));
    assert_eq!(package.batch().declarations().len(), 1);
    assert_eq!(package.parameter_count(), 1);
}

#[test]
fn two_exact_dynamic_candidates_reject_without_ordinal_tiebreak() {
    let first = include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako");
    let second = first.replace("ParserScanLoopBox", "ParserScanLoopBoxTwin");
    let source = format!("{first}\n{second}");
    assert!(matches!(
        issue(&source),
        Err(NormalCallableSemanticPackageIssueV1::DuplicateDynamicCandidate)
    ));
}

#[test]
fn unselected_main_dynamic_candidate_cannot_capture_production_selection() {
    let source = include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako")
        .replace("ParserScanLoopBox", "Main")
        .replace(
            "static box Main {",
            "static box Main {\n  main() { return 0 }",
        );
    let package = issue(&source).expect("Main remains a valid unselected batch row");

    assert!(matches!(
        package.dynamic_projection(),
        NormalCallableDynamicProjectionRefV1::ValidUnselected
    ));
    assert_eq!(package.batch().declarations().len(), 5);
}

#[test]
fn unselected_main_candidate_does_not_duplicate_one_selected_dynamic_candidate() {
    let selected = include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako");
    let unselected = selected.replace("ParserScanLoopBox", "Main").replace(
        "static box Main {",
        "static box Main {\n  main() { return 0 }",
    );
    let package = issue(&format!("{unselected}\n{selected}"))
        .expect("only selected-map Dynamic rows participate in production selection");

    assert!(matches!(
        package.dynamic_projection(),
        NormalCallableDynamicProjectionRefV1::Selected { .. }
    ));
}

#[test]
fn selected_gate_without_total_root_authority_rejects_before_dynamic_classification() {
    let source = r#"
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
"#;
    let result = issue_with_config(
        source,
        ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        },
    );
    assert!(
        matches!(
            &result,
            Err(NormalCallableSemanticPackageIssueV1::RootExecution(
                NormalRootExecutionConsumerRejectV1::SourceAuthorityUnavailable
            ))
        ),
        "unexpected semantic package result: {result:?}"
    );
}

#[test]
fn package_has_no_clone_or_split_surface() {
    let model = include_str!("model.rs");
    let install = include_str!("install.rs");
    assert!(!model.contains("Clone)]\npub(crate) struct VerifiedNormalCallableSemanticPackageV1"));
    assert!(!model.contains("fn into_parts"));
    assert!(install.contains("MissingParameterContract"));
    assert!(install.contains("DuplicateParameterContract"));
    assert!(!install.contains("unwrap_or(&[])"));
}

#[test]
fn consuming_install_and_port_enforce_exact_selected_coverage() {
    let package =
        issue("static box Api { run(value) { return value } }").expect("ordinary package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Api", "run", 1),
    );

    let mut port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    port.with_selected_lowering_input(&key, |input| {
        assert_eq!(input.parameter_contracts().len(), 1);
        assert!(input.source().callable_header().is_none());
        let identity = input.source_identity();
        assert_eq!(identity.owner(), input.source().owner());
        assert_eq!(
            identity.mode(),
            ResolvedCallableDeclarationModeV1::StaticBoxMethod
        );
        assert!(identity.method_source_observation().is_some());
        assert!(matches!(
            input.semantic(),
            SelectedCallableSemanticRefV1::Ordinary
        ));
    })
    .expect("exact selected loan");
    assert!(matches!(
        port.with_selected_lowering_input(&key, |_| ()),
        Err(NormalCallableSemanticPackageInstallIssueV1::DuplicateSelectedKey)
    ));
    port.take_object_definitions(&context)
        .expect("explicit definition transfer for semantic-only test");
    port.complete().expect("all selected rows consumed once");

    let foreign = CompilationContext::new();
    assert!(matches!(
        installed.begin_lowering(&foreign),
        Err(NormalCallableSemanticPackageInstallIssueV1::ForeignCatalog)
    ));
}

#[test]
fn install_rejects_occupied_catalog_and_port_rejects_incomplete_coverage() {
    let source = "static box Api { run(value) { return value } stop(value) { return value } }";
    let mut context = CompilationContext::new();
    let installed = issue(source)
        .expect("first package")
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    assert!(issue(source)
        .expect("second package")
        .prepare_install(&mut context)
        .is_err());

    let port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    assert!(matches!(
        port.complete(),
        Err(NormalCallableSemanticPackageInstallIssueV1::IncompleteSelectedCoverage)
    ));
}

#[path = "issuance_order_tests.rs"]
mod issuance_order;
