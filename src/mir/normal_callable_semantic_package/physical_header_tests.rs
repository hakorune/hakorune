use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, CompilationContext, SelectedNormalCallableKeyV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::{issue_normal_callable_semantic_package_v1, SelectedCallableSemanticRefV1};

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
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

fn issue(
    source: &str,
) -> Result<
    super::VerifiedNormalCallableSemanticPackageV1,
    super::NormalCallableSemanticPackageIssueV1,
> {
    let mut resolver = FunctionSemanticResolverSessionV1::new(194).unwrap();
    issue_normal_callable_semantic_package_v1(&mut resolver, final_source(source))
}

#[test]
fn explicit_result_annotation_lends_one_complete_header_view() {
    let package = issue("static box Api { run(value: i64): i64 { return value } }")
        .expect("explicit source/header cohort");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Api", "run", 1),
    );
    let mut port = installed.begin_lowering(&context).expect("same catalog");
    port.with_selected_lowering_input(&key, |input| {
        let header = input.physical_header().expect("header cohort row");
        assert_eq!(header.owner(), input.source().owner());
        assert_eq!(
            header.result(),
            crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1::I64
        );
        assert_eq!(header.completion_owner(), input.source().owner());
        assert!(header.completion_returns_value());
        assert_eq!(header.completion_explicit_site_count(), 1);
        assert!(header.completion_cleanup_is_empty());
        assert!(matches!(
            input.semantic(),
            SelectedCallableSemanticRefV1::Ordinary
        ));
    })
    .expect("header loan");
}

#[test]
fn absent_result_annotation_keeps_ordinary_package_without_physical_header() {
    let package = issue("static box Api { run(value: i64) { return value } }")
        .expect("ordinary package remains source-compatible");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Api", "run", 1),
    );
    let mut port = installed.begin_lowering(&context).expect("same catalog");
    port.with_selected_lowering_input(&key, |input| {
        assert!(input.physical_header().is_none());
    })
    .expect("ordinary loan");
}

#[test]
fn mixed_package_lends_only_the_eligible_physical_header_row() {
    let package = issue(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .expect("mixed parser scan package");
    let selected_keys = package.selected.keys().cloned().collect::<Vec<_>>();
    assert_eq!(selected_keys.len(), 4);
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let mut port = installed.begin_lowering(&context).expect("same catalog");
    for key in selected_keys {
        let SelectedNormalCallableKeyV1::Cataloged(source_key) = &key else {
            panic!("parser scan rows must stay cataloged")
        };
        let method = source_key.name();
        let expected_header = method == "skip_while";
        port.with_selected_lowering_input(&key, |input| {
            let header = input.physical_header();
            assert_eq!(header.is_some(), expected_header, "{method}");
            if let Some(header) = header {
                assert_eq!(
                    header.result(),
                    crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1::I64
                );
                assert!(header.completion_returns_value());
                assert_eq!(header.completion_explicit_site_count(), 2);
            }
        })
        .expect("selected row loan");
    }
}

#[test]
fn non_i64_result_annotation_is_not_reclassified_as_a_physical_header() {
    let issue = issue("static box Api { run(value: i64): i32 { return value } }");
    assert!(matches!(
        issue,
        Err(super::NormalCallableSemanticPackageIssueV1::PhysicalHeader(
            _
        ))
    ));
}
