use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ScriptResolverDeferredCauseV1, ScriptResolverDeferredSiteV1,
    SourceResolverDeferredV1,
};
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::{issue_normal_callable_semantic_package_v1, NormalCallableSemanticPackageIssueV1};

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
