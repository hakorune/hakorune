use crate::parser::{NyashParser, ParserBuildConfig};

use super::{
    issue_resolved_callable_semantic_batch_v1, ResolvedCallableSemanticBatchIssueV1,
    VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::resolved_semantics::{
    DeclaredInstanceCallEffectSourceDispositionV1, DeclaredInstanceCallSemanticEffectV1,
    FunctionSemanticResolverSessionV1,
};

fn batch(source: &str) -> VerifiedResolvedCallableSemanticBatchV1 {
    batch_result(source).expect("resolved callable semantic batch")
}

fn batch_result(
    source: &str,
) -> Result<VerifiedResolvedCallableSemanticBatchV1, ResolvedCallableSemanticBatchIssueV1> {
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
        let mut resolver = FunctionSemanticResolverSessionV1::new(271).unwrap();
        issue_resolved_callable_semantic_batch_v1(&mut resolver, source)
    })
}

#[test]
fn unannotated_declared_instance_target_is_opaque_observable() {
    let batch = batch("box Counter { call() { return me.value() } value() { return 1 } }");
    let DeclaredInstanceCallEffectSourceDispositionV1::Published(catalog) =
        batch.declared_instance_call_effect_source()
    else {
        panic!("expected declared-instance effect catalog");
    };
    assert_eq!(catalog.len(), 1);
    assert_eq!(
        catalog.rows()[0].effect(),
        DeclaredInstanceCallSemanticEffectV1::OpaqueObservable
    );
}

#[test]
fn query_contract_has_precedence_over_opaque_default() {
    let batch = batch(
        "box Counter { call() { return me.value() } @rune CallableContract(query) value(): i64 { return 1 } }",
    );
    let DeclaredInstanceCallEffectSourceDispositionV1::Published(catalog) =
        batch.declared_instance_call_effect_source()
    else {
        panic!("expected declared-instance effect catalog");
    };
    assert_eq!(catalog.len(), 1);
    assert_eq!(
        catalog.rows()[0].effect(),
        DeclaredInstanceCallSemanticEffectV1::DeclaredQuery { rune_ordinal: 0 }
    );
}

#[test]
fn static_current_owner_has_no_declared_instance_effect_rows() {
    let batch = batch("static box Api { call() { return me.value() } value() { return 1 } }");
    assert!(matches!(
        batch.declared_instance_call_effect_source(),
        DeclaredInstanceCallEffectSourceDispositionV1::NoRootDeclaredInstanceCall
    ));
}

#[test]
fn wrong_declared_instance_arity_rejects_before_effect_issuer() {
    let result = batch_result(
        "box Counter { call() { return me.value() } value(argument) { return argument } }",
    );
    assert!(matches!(
        result,
        Err(ResolvedCallableSemanticBatchIssueV1::DeclaredInstanceCallRelation { .. })
    ));
}
