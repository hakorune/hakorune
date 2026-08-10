use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, CompilationContext, SelectedNormalCallableKeyV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{
    BuildMode, NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1,
};

use super::{
    issue_normal_callable_semantic_package_v1, NormalCallableDynamicProjectionRefV1,
    NormalCallableSemanticPackageInstallIssueV1, NormalCallableSemanticPackageIssueV1,
    SelectedCallableSemanticRefV1,
};

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
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
fn parser_scan_source_seals_one_dynamic_candidate_and_all_parameter_demands() {
    let package = issue(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .expect("exact parser scan semantic package");

    assert_eq!(package.batch().declarations().len(), 4);
    assert_eq!(package.parameter_declaration_count(), 4);
    assert_eq!(package.parameter_count(), 15);
    let NormalCallableDynamicProjectionRefV1::Selected { program } = package.dynamic_projection()
    else {
        panic!("exact Dynamic row must remain selected")
    };
    assert_eq!(
        program.current(),
        crate::mir::compiler::dynamic_full_body_recipe::DynamicCarrierCurrentDispositionV1::BorrowedIngressNoEnd
    );
}

#[test]
fn top_level_and_dynamic_candidate_share_one_complete_package_batch() {
    let dynamic = include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako");
    let source = format!("function helper(value) {{ return value }}\n{dynamic}");
    let package = issue(&source).expect("mixed top-level and Dynamic package");

    assert_eq!(package.batch().declarations().len(), 5);
    assert_eq!(package.parameter_declaration_count(), 4);
    assert_eq!(package.parameter_count(), 15);
    assert!(matches!(
        package.dynamic_projection(),
        NormalCallableDynamicProjectionRefV1::Selected { .. }
    ));
    assert_eq!(
        package
            .batch()
            .declarations()
            .next()
            .expect("top-level row remains first")
            .mode(),
        crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1::TopLevel
    );
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
        .replace("ParserScanLoopBox", "Main");
    let package = issue(&source).expect("Main remains a valid unselected batch row");

    assert!(matches!(
        package.dynamic_projection(),
        NormalCallableDynamicProjectionRefV1::ValidUnselected
    ));
    assert_eq!(package.batch().declarations().len(), 4);
}

#[test]
fn unselected_main_candidate_does_not_duplicate_one_selected_dynamic_candidate() {
    let selected = include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako");
    let unselected = selected.replace("ParserScanLoopBox", "Main");
    let package = issue(&format!("{unselected}\n{selected}"))
        .expect("only selected-map Dynamic rows participate in production selection");

    assert!(matches!(
        package.dynamic_projection(),
        NormalCallableDynamicProjectionRefV1::Selected { .. }
    ));
}

#[test]
fn selected_gate_dynamic_candidate_rejects_without_parameter_authority() {
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
    assert!(matches!(
        issue_with_config(
            source,
            ParserBuildConfig {
                mode: BuildMode::Test,
                ..ParserBuildConfig::default()
            },
        ),
        Err(NormalCallableSemanticPackageIssueV1::MissingDynamicParameterDemand)
    ));
}

#[test]
fn package_has_no_clone_or_split_surface() {
    let model = include_str!("model.rs");
    assert!(!model.contains("Clone)]\npub(crate) struct VerifiedNormalCallableSemanticPackageV1"));
    assert!(!model.contains("fn into_parts"));
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
        assert_eq!(input.parameter_demands().len(), 1);
        assert!(input.source().callable_header().is_none());
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
