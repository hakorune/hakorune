//! Shared original-source fixture issuer; selection changes no source authority.
use crate::mir::builder::NormalRootExecutionConsumerV1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig};

pub(in crate::mir) fn issue_with_brand_catalog(
    source: &str,
) -> Result<
    super::super::VerifiedNormalCallableSemanticPackageV1,
    super::super::NormalCallableSemanticPackageIssueV1,
> {
    issue_with_brand_catalog_selection(source, super::super::PackagePhysicalSelectionV1::AllSource)
}

pub(in crate::mir) fn issue_with_brand_catalog_selection(
    source: &str,
    selection: super::super::PackagePhysicalSelectionV1,
) -> Result<super::super::VerifiedNormalCallableSemanticPackageV1, super::super::NormalCallableSemanticPackageIssueV1> {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source");
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform")
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("fixture must remain source-backed")
    };
    let catalog = crate::analysis::brand_program_declaration_catalog::
        issue_brand_program_declaration_catalog_v1(source.ast())
        .expect("brand catalog");
    let source = NormalRootExecutionConsumerV1::consume_once(source)
        .expect("root execution")
        .into_consumed_source();
    let mut resolver = FunctionSemanticResolverSessionV1::new(93).unwrap();
    super::super::issue_normal_callable_semantic_package_with_selection_v1(
        &mut resolver, source, Some(&catalog),
        crate::mir::builder::LoopFactsPolicyFrameV1::from_environment(), &[], selection,
    )
}
