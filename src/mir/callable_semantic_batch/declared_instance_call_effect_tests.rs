use crate::parser::{NyashParser, ParserBuildConfig};

use super::{
    issue_resolved_callable_semantic_batch_v1, ResolvedCallableSemanticBatchIssueV1,
    VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::resolved_semantics::{
    DeclaredInstanceCallEffectSourceDispositionV1, DeclaredInstanceCallSemanticEffectV1,
    DeclaredInstanceCallEffectIssuerV1, DeclaredInstanceCallEffectIssueV1,
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
    for (body, count) in [
        ("return me.value()", 1),
        ("local first = me.value() local second = me.value() return me.value()", 3),
    ] {
        let batch = batch(&format!("box Counter {{ call() {{ {body} }} value() {{ return 1 }} }}"));
        let DeclaredInstanceCallEffectSourceDispositionV1::Published(catalog) =
            batch.declared_instance_call_effect_source()
        else { panic!("expected declared-instance effect catalog"); };
        assert_eq!(catalog.len(), count);
        assert_eq!(catalog.rows().iter().map(|row| row.call_site()).collect::<std::collections::BTreeSet<_>>().len(), count);
        for row in catalog.rows() {
            assert_eq!(row.effect(), DeclaredInstanceCallSemanticEffectV1::OpaqueObservable);
            assert_eq!(row.target_method_identity(), catalog.rows()[0].target_method_identity());
        }
        // Removing the target syntax is still a per-use source refusal, even
        // when a larger call count itself is valid.
        assert!(matches!(DeclaredInstanceCallEffectIssuerV1::issue(
            batch.declared_instance_call_source(), &[]),
            Err(DeclaredInstanceCallEffectIssueV1::TargetSyntaxMissing)));
    }
}

#[test]
fn query_contract_has_precedence_over_opaque_default() {
    for (body, count) in [
        ("return me.value()", 1),
        ("local first = me.value() local second = me.value() return me.value()", 3),
    ] {
        let batch = batch(&format!(
            "box Counter {{ call() {{ {body} }} @rune CallableContract(query) value(): i64 {{ return 1 }} }}",
        ));
        let DeclaredInstanceCallEffectSourceDispositionV1::Published(catalog) =
            batch.declared_instance_call_effect_source()
        else { panic!("expected declared-instance effect catalog"); };
        assert_eq!(catalog.len(), count);
        for row in catalog.rows() {
            assert_eq!(row.effect(), DeclaredInstanceCallSemanticEffectV1::DeclaredQuery { rune_ordinal: 0 });
            assert_eq!(row.target_method_identity(), catalog.rows()[0].target_method_identity());
        }
    }
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
