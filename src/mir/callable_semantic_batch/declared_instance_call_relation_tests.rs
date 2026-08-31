use crate::parser::{NyashParser, ParserBuildConfig};

use super::{
    issue_resolved_callable_semantic_batch_v1, ResolvedCallableSemanticBatchIssueV1,
    VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::resolved_semantics::{
    DeclaredInstanceCallRelationIssueV1, DeclaredInstanceCallSourceDispositionV1,
    FunctionSemanticResolverSessionV1,
};

fn final_source(source: &str) -> crate::parser::VerifiedFinalCallableProgramSourceV1 {
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

fn batch(source: &str) -> VerifiedResolvedCallableSemanticBatchV1 {
    let source = final_source(source);
    let mut resolver = FunctionSemanticResolverSessionV1::new(171).unwrap();
    issue_resolved_callable_semantic_batch_v1(&mut resolver, source).unwrap()
}

#[test]
fn declared_instance_relation_publishes_one_exact_me_call() {
    let batch = batch("box Counter { call() { return me.value() } value() { return 1 } }");
    let DeclaredInstanceCallSourceDispositionV1::Published(catalog) =
        batch.declared_instance_call_source()
    else {
        panic!("expected one declared-instance relation");
    };
    assert_eq!(catalog.len(), 1);
    let row = &catalog.rows()[0];
    assert_eq!(row.source_arity(), 0);
    assert_eq!(
        row.caller_owner(),
        batch.declarations().next().unwrap().owner()
    );
    assert_eq!(
        row.target_owner(),
        batch.declarations().nth(1).unwrap().owner()
    );
}

#[test]
fn same_method_name_on_different_boxes_keeps_nominal_relations_separate() {
    let batch = batch(
        "box Left { call() { return me.value() } value() { return 1 } }
         box Right { call() { return me.value() } value() { return 2 } }",
    );
    let DeclaredInstanceCallSourceDispositionV1::Published(catalog) =
        batch.declared_instance_call_source()
    else {
        panic!("expected two declared-instance relations");
    };
    assert_eq!(catalog.len(), 2);
    assert_ne!(
        catalog.rows()[0].target_owner(),
        catalog.rows()[1].target_owner()
    );
}

#[test]
fn static_current_owner_me_call_is_outside_declared_instance_relation() {
    let batch = batch("static box Api { call() { return me.value() } value() { return 1 } }");
    assert!(matches!(
        batch.declared_instance_call_source(),
        DeclaredInstanceCallSourceDispositionV1::NoRootDeclaredInstanceCall
    ));
}

#[test]
fn declared_instance_relation_rejects_target_arity_mismatch_before_lowering() {
    let source = final_source("box Counter { call() { return me.value(1) } value() { return 1 } }");
    let mut resolver = FunctionSemanticResolverSessionV1::new(177).unwrap();
    let error = issue_resolved_callable_semantic_batch_v1(&mut resolver, source)
        .expect_err("wrong receiver-call arity must reject the relation");
    assert!(matches!(
        error,
        ResolvedCallableSemanticBatchIssueV1::DeclaredInstanceCallRelation {
            _error: DeclaredInstanceCallRelationIssueV1::TargetArityMismatch { .. }
        }
    ));
}
