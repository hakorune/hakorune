use crate::mir::callable_semantic_batch::{
    issue_resolved_callable_semantic_batch_v1, issue_s6c_typed_input_relation_v1,
    VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::{
    issue_core_method_manifest_row_ref_v1, CORE_METHOD_MANIFEST_BRAND_V1,
};
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::{
    CoreMethodInstanceTargetIssuerV1, FunctionSemanticResolverSessionV1,
};
use crate::mir::source_call_target::issue_source_bound_s6c_call_relation_v1;
use crate::parser::{NyashParser, ParserBuildConfig};

use super::{
    issue_s6c_exit_tail_source_coseal_v1, issue_s6c_scan_with_init_facts_v1, S6CExitRoleV1,
    S6CExitTailSourceCoSealRejectV1, S6CScanWithInitFactsRejectV1,
    VerifiedS6CExitTailSourceCoSealV1,
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

fn issue(
    source: &str,
    ordinal: u32,
) -> Result<VerifiedS6CExitTailSourceCoSealV1, S6CExitTailSourceCoSealRejectV1> {
    let batch = batch(source, ordinal);
    let completion = batch
        .with_lowering_input(0, verify_function_completion_v1)
        .unwrap()
        .unwrap();
    batch
        .with_declaration_semantics(|view| {
            let row = &view.declarations()[0];
            let loop_site = row.function().only_loop_site().unwrap();
            let typed = issue_s6c_typed_input_relation_v1(row, &loop_site).unwrap();
            let mut targets =
                CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V1)
                    .unwrap();
            let length = targets
                .issue(issue_core_method_manifest_row_ref_v1(CoreMethodOp::StringLen, 0).unwrap())
                .unwrap();
            let substring = targets
                .issue(
                    issue_core_method_manifest_row_ref_v1(CoreMethodOp::StringSubstring, 2)
                        .unwrap(),
                )
                .unwrap();
            row.with_source_ledger(|ledger| {
                let calls =
                    issue_source_bound_s6c_call_relation_v1(&ledger, typed, length, substring)
                        .unwrap();
                issue_s6c_exit_tail_source_coseal_v1(&ledger, calls, completion)
            })
            .unwrap()
        })
        .unwrap()
}

#[test]
fn exact_inner_return_and_outer_tail_are_cosealed_without_source_order() {
    let coseal = issue(FIXTURE, 201).expect("exact S6C Exit/Tail source co-seal");
    coseal.with_coseal(|view| {
        assert_eq!(view.completion().explicit_sites().len(), 2);
        assert!(view
            .completion()
            .explicit_sites()
            .contains(view.loop_return_site()));
        assert!(view
            .completion()
            .explicit_sites()
            .contains(view.tail_site()));
        assert_ne!(view.loop_return_value(), view.tail_value());
        assert_ne!(view.if_site(), view.tail_site());
        assert_eq!(
            view.calls().typed().membership().frame(),
            view.calls().length().frame()
        );
    });
}

#[test]
fn inner_minus_one_cannot_be_reclassified_as_callable_tail() {
    let source = FIXTURE.replacen("return i", "return -1", 1);
    assert_eq!(
        issue(&source, 202).unwrap_err(),
        S6CExitTailSourceCoSealRejectV1::WrongExitRegion(S6CExitRoleV1::CallableTail)
    );
}

#[test]
fn outer_index_cannot_be_reclassified_as_loop_return() {
    let source = FIXTURE.rsplit_once("return -1").unwrap();
    let source = format!("{}return i{}", source.0, source.1);
    assert_eq!(
        issue(&source, 203).unwrap_err(),
        S6CExitTailSourceCoSealRejectV1::WrongExitRegion(S6CExitRoleV1::LoopReturn)
    );
}

#[test]
fn outer_not_one_cannot_be_reclassified_as_minus_one_tail() {
    let source = FIXTURE.rsplit_once("return -1").unwrap();
    let source = format!("{}return !1{}", source.0, source.1);
    assert_eq!(
        issue(&source, 206).unwrap_err(),
        S6CExitTailSourceCoSealRejectV1::WrongExitValue(S6CExitRoleV1::CallableTail)
    );
}

#[test]
fn foreign_ledger_rejects_before_exit_tail_product() {
    let first = batch(FIXTURE, 204);
    let completion = first
        .with_lowering_input(0, verify_function_completion_v1)
        .unwrap()
        .unwrap();
    let calls = first
        .with_declaration_semantics(|view| {
            let row = &view.declarations()[0];
            let loop_site = row.function().only_loop_site().unwrap();
            let typed = issue_s6c_typed_input_relation_v1(row, &loop_site).unwrap();
            let mut targets =
                CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V1)
                    .unwrap();
            let length = targets
                .issue(issue_core_method_manifest_row_ref_v1(CoreMethodOp::StringLen, 0).unwrap())
                .unwrap();
            let substring = targets
                .issue(
                    issue_core_method_manifest_row_ref_v1(CoreMethodOp::StringSubstring, 2)
                        .unwrap(),
                )
                .unwrap();
            row.with_source_ledger(|ledger| {
                issue_source_bound_s6c_call_relation_v1(&ledger, typed, length, substring).unwrap()
            })
            .unwrap()
        })
        .unwrap();

    let second = batch(FIXTURE, 205);
    let rejected = second
        .with_declaration_semantics(|view| {
            view.declarations()[0]
                .with_source_ledger(|ledger| {
                    issue_s6c_exit_tail_source_coseal_v1(&ledger, calls, completion)
                })
                .unwrap()
        })
        .unwrap()
        .unwrap_err();
    assert_eq!(rejected, S6CExitTailSourceCoSealRejectV1::ForeignOwner);
}

#[test]
fn complete_scan_with_init_facts_seal_the_exact_source_surface() {
    let coseal = issue(FIXTURE, 207).expect("exact S6C Exit/Tail source co-seal");
    let facts =
        issue_s6c_scan_with_init_facts_v1(coseal).expect("exact S6C source closure and Facts seal");
    facts.with_facts(|view| {
        assert_eq!(view.source().completion().explicit_sites().len(), 2);
        assert_ne!(view.source().tail_value(), view.source().tail_operand());
    });
}

#[test]
fn extra_body_statement_rejects_source_closure() {
    let source = FIXTURE.replace(
        "            i = i + 1",
        "            42\n            i = i + 1",
    );
    let coseal = issue(&source, 208).expect("existing source relations remain valid");
    assert_eq!(
        issue_s6c_scan_with_init_facts_v1(coseal).unwrap_err(),
        S6CScanWithInitFactsRejectV1::StatementCoverage
    );
}
