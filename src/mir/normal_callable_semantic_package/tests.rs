use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{
    BuildMode, NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1,
};

use super::{
    issue_normal_callable_semantic_dynamic_package_v1, NormalCallableSemanticDynamicPackageIssueV1,
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
) -> Result<
    super::VerifiedNormalCallableSemanticDynamicPackageV1,
    NormalCallableSemanticDynamicPackageIssueV1,
> {
    let mut resolver = FunctionSemanticResolverSessionV1::new(92).unwrap();
    issue_normal_callable_semantic_dynamic_package_v1(
        &mut resolver,
        final_source_with_config(source, config),
    )
}

fn issue(
    source: &str,
) -> Result<
    super::VerifiedNormalCallableSemanticDynamicPackageV1,
    NormalCallableSemanticDynamicPackageIssueV1,
> {
    let mut resolver = FunctionSemanticResolverSessionV1::new(91).unwrap();
    issue_normal_callable_semantic_dynamic_package_v1(&mut resolver, final_source(source))
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
    assert_eq!(package.dynamic_batch_slot(), 0);
    assert_eq!(
        package.dynamic_owner(),
        package
            .batch()
            .declarations()
            .next()
            .expect("Dynamic row remains in the owned batch")
            .owner()
    );
    let _ = package.dynamic_recipe();
}

#[test]
fn top_level_and_dynamic_candidate_share_one_complete_package_batch() {
    let dynamic = include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako");
    let source = format!("function helper(value) {{ return value }}\n{dynamic}");
    let package = issue(&source).expect("mixed top-level and Dynamic package");

    assert_eq!(package.batch().declarations().len(), 5);
    assert_eq!(package.parameter_declaration_count(), 4);
    assert_eq!(package.parameter_count(), 15);
    assert_eq!(package.dynamic_batch_slot(), 1);
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
fn zero_dynamic_candidates_reject_without_default_or_name_selection() {
    assert!(matches!(
        issue("static box Api { run(value) { return value } }"),
        Err(NormalCallableSemanticDynamicPackageIssueV1::MissingDynamicCandidate)
    ));
}

#[test]
fn two_exact_dynamic_candidates_reject_without_ordinal_tiebreak() {
    let first = include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako");
    let second = first.replace("ParserScanLoopBox", "ParserScanLoopBoxTwin");
    let source = format!("{first}\n{second}");
    assert!(matches!(
        issue(&source),
        Err(NormalCallableSemanticDynamicPackageIssueV1::DuplicateDynamicCandidate)
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
        Err(NormalCallableSemanticDynamicPackageIssueV1::MissingDynamicParameterDemand)
    ));
}

#[test]
fn package_has_no_clone_or_split_surface() {
    let model = include_str!("model.rs");
    assert!(!model
        .contains("Clone)]\npub(crate) struct VerifiedNormalCallableSemanticDynamicPackageV1"));
    assert!(!model.contains("fn into_parts"));
}
