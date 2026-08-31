use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::{
    declared_instance_locator::DeclaredInstanceCallPackageLocatorDispositionV1,
    issue_normal_callable_semantic_package_v1,
};

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("declared-instance locator source");
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("source-backed transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    })
}

fn issue(source: &str) -> super::VerifiedNormalCallableSemanticPackageV1 {
    issue_result(source).expect("declared-instance package")
}

fn issue_result(
    source: &str,
) -> Result<
    super::VerifiedNormalCallableSemanticPackageV1,
    super::NormalCallableSemanticPackageIssueV1,
> {
    let mut resolver = FunctionSemanticResolverSessionV1::new(1_071).expect("resolver");
    issue_normal_callable_semantic_package_v1(&mut resolver, final_source(source))
}

#[test]
fn package_retains_one_private_locator_for_exact_me_method_site() {
    let package = issue("box Counter { call() { return me.value() } value() { return 1 } }");
    let disposition = package.declared_instance_call_locators();
    let DeclaredInstanceCallPackageLocatorDispositionV1::Published(catalog) = disposition else {
        panic!("the exact me.method source must publish a locator catalog")
    };
    assert_eq!(catalog.len(), 1);
    let row = catalog.rows().first().expect("one locator row");
    assert_eq!(row.caller_batch_slot(), 0);
    assert_eq!(row.target_batch_slot(), 1);
    assert_eq!(row.relation_row_ordinal(), 0);
    assert_eq!(row.effect_row_ordinal(), 0);
}

#[test]
fn package_publishes_explicit_no_root_for_static_only_source() {
    let package = issue("static box Api { run(value) { return value } }");
    assert!(matches!(
        package.declared_instance_call_locators(),
        DeclaredInstanceCallPackageLocatorDispositionV1::NoRootDeclaredInstanceCall
    ));
}

#[test]
fn package_rejects_missing_declared_instance_target_before_locator_publication() {
    let result =
        issue_result("box Counter { call() { return me.missing() } value() { return 1 } }");
    assert!(matches!(
        result,
        Err(super::NormalCallableSemanticPackageIssueV1::Batch { .. })
            | Err(super::NormalCallableSemanticPackageIssueV1::DeclaredInstanceLocator { .. })
    ));
}
