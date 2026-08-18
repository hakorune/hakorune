use crate::mir::callable_semantic_batch::{
    issue_resolved_callable_semantic_batch_v1, issue_s6c_typed_input_relation_v1,
    VerifiedResolvedCallableSemanticBatchV1, VerifiedS6CTypedInputRelationV1,
};
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::{
    issue_core_method_manifest_row_ref_v2, CORE_METHOD_MANIFEST_BRAND_V2,
};
use crate::mir::resolved_semantics::{
    CoreMethodInstanceTargetIssuerV1, FunctionSemanticResolverSessionV1,
    ResolverCoreMethodCallableContractRejectV1, VerifiedCoreMethodInstanceTargetV1,
};
use crate::parser::{NyashParser, ParserBuildConfig};

use super::{
    issue_source_bound_s6c_call_relation_v1, S6CSourceBoundCallRelationRejectV1,
    S6CSourceBoundCallRoleV1,
};

const FIXTURE: &str = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako");

fn batch(source: &str, ordinal: u32) -> VerifiedResolvedCallableSemanticBatchV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source");
    let source = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    });
    let mut resolver = FunctionSemanticResolverSessionV1::new(ordinal).unwrap();
    issue_resolved_callable_semantic_batch_v1(&mut resolver, source).unwrap()
}

fn same_session_targets() -> (
    VerifiedCoreMethodInstanceTargetV1,
    VerifiedCoreMethodInstanceTargetV1,
) {
    let mut issuer =
        CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V2).unwrap();
    let length = issuer
        .issue(issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringLen, 0).unwrap())
        .unwrap();
    let substring = issuer
        .issue(issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringSubstring, 2).unwrap())
        .unwrap();
    (length, substring)
}

fn typed(batch: &VerifiedResolvedCallableSemanticBatchV1) -> VerifiedS6CTypedInputRelationV1 {
    batch
        .with_declaration_semantics(|view| {
            let row = &view.declarations()[0];
            let loop_site = row.function().only_loop_site().expect("one Loop source");
            issue_s6c_typed_input_relation_v1(row, &loop_site)
        })
        .expect("resolved declaration semantics")
        .expect("typed S6C input")
}

#[test]
fn source_bound_s6c_relation_consumes_exact_two_calls_and_targets() {
    let batch = batch(FIXTURE, 101);
    let relation = batch
        .with_declaration_semantics(|view| {
            let row = &view.declarations()[0];
            let loop_site = row.function().only_loop_site().expect("one Loop source");
            let typed = issue_s6c_typed_input_relation_v1(row, &loop_site).unwrap();
            let (length, substring) = same_session_targets();
            row.with_source_ledger(|ledger| {
                issue_source_bound_s6c_call_relation_v1(&ledger, typed, length, substring)
            })
            .unwrap()
        })
        .unwrap()
        .unwrap();

    relation.with_relation(|view| {
        assert_eq!(
            view.length().target().row().row().op,
            CoreMethodOp::StringLen
        );
        assert_eq!(
            view.length().placement(),
            crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Condition
        );
        assert_eq!(
            view.substring().target().row().row().op,
            CoreMethodOp::StringSubstring
        );
        assert_eq!(
            view.substring().placement(),
            crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body
        );
        assert_eq!(view.typed().membership().frame(), view.length().frame());
        assert_eq!(view.typed().membership().frame(), view.substring().frame());
    });
}

#[test]
fn source_bound_s6c_relation_rejects_mixed_target_sessions() {
    let batch = batch(FIXTURE, 102);
    let rejected = batch
        .with_declaration_semantics(|view| {
            let row = &view.declarations()[0];
            let loop_site = row.function().only_loop_site().unwrap();
            let typed = issue_s6c_typed_input_relation_v1(row, &loop_site).unwrap();
            let (length, _) = same_session_targets();
            let (_, substring) = same_session_targets();
            row.with_source_ledger(|ledger| {
                issue_source_bound_s6c_call_relation_v1(&ledger, typed, length, substring)
            })
            .unwrap()
        })
        .unwrap()
        .unwrap_err();
    assert_eq!(
        rejected,
        S6CSourceBoundCallRelationRejectV1::MixedRelationBrand
    );
}

#[test]
fn source_bound_s6c_relation_rejects_swapped_target_roles() {
    let batch = batch(FIXTURE, 103);
    let rejected = batch
        .with_declaration_semantics(|view| {
            let row = &view.declarations()[0];
            let loop_site = row.function().only_loop_site().unwrap();
            let typed = issue_s6c_typed_input_relation_v1(row, &loop_site).unwrap();
            let (length, substring) = same_session_targets();
            row.with_source_ledger(|ledger| {
                issue_source_bound_s6c_call_relation_v1(&ledger, typed, substring, length)
            })
            .unwrap()
        })
        .unwrap()
        .unwrap_err();
    assert!(matches!(
        rejected,
        S6CSourceBoundCallRelationRejectV1::WrongTargetRole {
            role: S6CSourceBoundCallRoleV1::Length,
            op: CoreMethodOp::StringSubstring,
            arity: 2,
        }
    ));
}

#[test]
fn source_bound_s6c_relation_rejects_foreign_ledger_owner() {
    let first = batch(FIXTURE, 104);
    let typed = typed(&first);
    let second = batch(FIXTURE, 105);
    let rejected = second
        .with_declaration_semantics(|view| {
            let row = &view.declarations()[0];
            let (length, substring) = same_session_targets();
            row.with_source_ledger(|ledger| {
                issue_source_bound_s6c_call_relation_v1(&ledger, typed, length, substring)
            })
            .unwrap()
        })
        .unwrap()
        .unwrap_err();
    assert!(matches!(
        rejected,
        S6CSourceBoundCallRelationRejectV1::Callable {
            role: S6CSourceBoundCallRoleV1::Length,
            reject: ResolverCoreMethodCallableContractRejectV1::ForeignLoopMembership,
        }
    ));
}
