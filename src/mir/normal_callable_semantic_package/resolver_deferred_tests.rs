use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveFunctionErrorV1, ResolveOwnerForestErrorV1,
    ScriptResolverDeferredCauseV1, ScriptResolverDeferredSiteV1, ShadowResolveErrorV0,
    SourceResolverDeferredV1,
};
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::{
    issue_normal_callable_semantic_package_v1,
    issue_normal_callable_semantic_package_with_brand_catalog_v1,
    NormalCallableSemanticPackageIssueV1,
};

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

#[test]
fn constructor_resolver_deferred_keeps_the_exact_parser_source_id() {
    let source = final_source("box Holder { init() { return missing_constructor_value } }");
    let source_id = source
        .with_constructor_semantic_syntax(|loan| {
            loan.rows()
                .first()
                .expect("constructor row")
                .source_id()
                .clone()
        })
        .expect("constructor identity loan");
    let mut resolver = FunctionSemanticResolverSessionV1::new(94).unwrap();
    let deferred = match issue_normal_callable_semantic_package_v1(&mut resolver, source) {
        Err(NormalCallableSemanticPackageIssueV1::InstanceConstructors(
            super::instance_constructor_semantic::InstanceConstructorSemanticBatchIssueV1::ResolverDeferred(
                deferred,
            ),
        )) => deferred,
        other => panic!("expected constructor resolver deferral, got {other:?}"),
    };

    assert_eq!(deferred.len(), 1);
    assert!(deferred
        .first()
        .source()
        .constructor_source_id()
        .expect("constructor source id")
        .same_as(&source_id));
    match deferred.first().observation() {
        SourceResolverDeferredV1::Located {
            cause: ScriptResolverDeferredCauseV1::UnresolvedName { name },
            site: ScriptResolverDeferredSiteV1::Expression(_),
        } => assert_eq!(&**name, "missing_constructor_value"),
        other => panic!("expected located constructor unresolved name, got {other:?}"),
    }
}

#[test]
fn constructor_construction_reject_keeps_the_exact_parser_source_id() {
    let source = final_source(
        "brand PageId: i64\n\
         box Holder { init() { return PageId(1, 2) } }",
    );
    let source_id = source
        .with_constructor_semantic_syntax(|loan| {
            loan.rows()
                .first()
                .expect("constructor row")
                .source_id()
                .clone()
        })
        .expect("constructor identity loan");
    let catalog = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(
        source.ast(),
    )
    .expect("brand catalog");
    let mut resolver = FunctionSemanticResolverSessionV1::new(95).unwrap();
    let reject = match issue_normal_callable_semantic_package_with_brand_catalog_v1(
        &mut resolver,
        source,
        Some(&catalog),
    ) {
        Err(NormalCallableSemanticPackageIssueV1::InstanceConstructors(
            super::instance_constructor_semantic::InstanceConstructorSemanticBatchIssueV1::Resolver(
                reject,
            ),
        )) => reject,
        other => panic!("expected constructor construction reject, got {other:?}"),
    };

    assert!(reject
        .source()
        .constructor_source_id()
        .expect("constructor source id")
        .same_as(&source_id));
    assert!(matches!(
        reject.error(),
        ResolveOwnerForestErrorV1::Function(ResolveFunctionErrorV1::Syntax(
            ShadowResolveErrorV0::BrandConstructorArity { actual: 2, .. }
        ))
    ));
}
