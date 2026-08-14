use crate::mir::callable_semantic_batch::{
    issue_resolved_callable_semantic_batch_v1, issue_s6c_typed_input_relation_v1,
};
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::{
    issue_core_method_manifest_row_ref_v1, CORE_METHOD_MANIFEST_BRAND_V1,
};
use crate::mir::loop_structural_facts::{
    issue_s6c_exit_tail_source_coseal_v1, issue_s6c_scan_with_init_facts_v1,
};
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::{
    CoreMethodInstanceTargetIssuerV1, FunctionSemanticResolverSessionV1,
};
use crate::mir::source_call_target::issue_source_bound_s6c_call_relation_v1;
use crate::parser::{NyashParser, ParserBuildConfig};

use super::produce_s6c_scan_with_init_recipe_v2;
use super::s6c_scan_with_init_joinir::with_s6c_scan_with_init_logical_join_input;
use super::s6c_scan_with_init_rows::{
    S6CRecipeExitRowRefV2, S6CRecipeIfRowRefV2, S6CRecipeOperationRowRefV2,
};
use super::{
    consume_s6c_scan_with_init_logical_output_v1, issue_s6c_scan_with_init_logical_output_v1,
    S6CLogicalConsumerResultV1,
};

const FIXTURE: &str = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako");

pub(super) fn issue_facts(
    source: &str,
    ordinal: u32,
) -> crate::mir::loop_structural_facts::VerifiedS6CScanWithInitFactsV1 {
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
    let batch = issue_resolved_callable_semantic_batch_v1(&mut resolver, source).unwrap();
    let completion = batch
        .with_lowering_input(0, verify_function_completion_v1)
        .unwrap()
        .unwrap();
    let coseal = batch
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
        .unwrap();
    issue_s6c_scan_with_init_facts_v1(coseal.expect("Exit/Tail source co-seal"))
        .expect("closed S6C Facts")
}

#[test]
fn producer_seals_exact_recipe_and_join_facade() {
    let product = produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 901))
        .expect("exact S6C Recipe product");
    product.with_product(|view| {
        assert_eq!(view.recipe().root_loop().raw(), 0);
        assert_eq!(view.recipe().loop_count(), 1);
        assert_eq!(view.recipe().block_count(), 3);
        assert_eq!(view.recipe().item_count(), 15);
        assert_eq!(view.recipe().value_count(), 15);
        assert_eq!(view.roles().text_equal_if().raw(), 8);
        assert_eq!(view.roles().step_write().item().raw(), 14);
        assert_eq!(view.roles().step_write().value().raw(), 14);
        let rows = view.recipe_rows();
        assert_eq!(rows.root_loop().expect("root row").body.raw(), 1);
        assert_eq!(
            rows.condition_block().expect("condition block").items.len(),
            3
        );
        assert!(matches!(
            rows.operation(view.roles().length_call()),
            Some(S6CRecipeOperationRowRefV2::CallSlot { .. })
        ));
        assert!(rows.subject_input().is_some());
        assert!(rows.needle_input().is_some());
        assert!(rows.index_input().is_some());
        assert!(rows.write(view.roles().step_write()).is_some());
        assert!(matches!(
            rows.text_equal_if(),
            Some(S6CRecipeIfRowRefV2 { .. })
        ));
        assert!(matches!(
            rows.loop_exit(view.roles().loop_return()),
            Some(S6CRecipeExitRowRefV2 { .. })
        ));
        assert_eq!(view.logical_transfer().branches().len(), 1);
        assert_eq!(view.logical_transfer().summary_transfers().len(), 1);
        assert_eq!(view.join_role_seal().backedge_count(), 1);
    });
}

#[test]
fn logical_join_input_facade_co_seals_calls_and_transfer() {
    let product = produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 902))
        .expect("exact S6C Recipe product");
    with_s6c_scan_with_init_logical_join_input(&product, |input| {
        assert_eq!(input.rows().loop_count(), 1);
        assert_eq!(input.rows().block_count(), 3);
        assert_eq!(input.rows().binding_count(), 1);
        assert_eq!(input.rows().input_count(), 3);
        assert_eq!(input.rows().value_count(), 15);
        assert_eq!(input.rows().item_count(), 15);
        assert_eq!(input.rows().carrier_count(), 1);
        assert_eq!(input.rows().exit_count(), 1);
        assert_eq!(input.length().operation(), CoreMethodOp::StringLen);
        assert_eq!(input.length().arity(), 0);
        assert_eq!(
            input.length().placement(),
            crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Condition
        );
        assert_eq!(input.substring().operation(), CoreMethodOp::StringSubstring);
        assert_eq!(input.substring().arity(), 2);
        assert_eq!(
            input.substring().placement(),
            crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body
        );
        assert_eq!(input.rows().item_count(), 15);
        assert_eq!(input.logical_transfer().branches().len(), 1);
        assert_eq!(input.logical_transfer().summary_transfers().len(), 1);
        assert_eq!(
            input
                .logical_transfer()
                .boundaries()
                .iter()
                .filter(|row| row.role
                    == crate::mir::loop_recipe_contract::LoopJoinEdgeRoleV1::Backedge)
                .count(),
            1
        );
        assert!(matches!(
            input.length().recipe_row(),
            S6CRecipeOperationRowRefV2::CallSlot {
                receiver: Some(_),
                args,
                result: Some(_),
            } if args.is_empty()
        ));
    })
    .expect("logical JOINIR input parity");
}

#[test]
fn logical_output_product_owns_fixed_rows_and_borrows_join_transfer() {
    let output = issue_s6c_scan_with_init_logical_output_v1(
        produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 903))
            .expect("exact S6C Recipe product"),
    )
    .expect("logical output rows");
    output
        .try_with_output(|view| {
            assert!(view.domains().is_exact_s6c());
            assert_eq!(view.rows().values().len(), 15);
            assert_eq!(view.rows().blocks().len(), 3);
            assert_eq!(view.rows().items().len(), 15);
            assert_eq!(view.calls().len(), 2);
            assert_eq!(
                view.calls().length().role(),
                super::S6CLogicalCallRoleV1::Length
            );
            assert_eq!(
                view.calls().substring().role(),
                super::S6CLogicalCallRoleV1::Substring
            );
            assert_eq!(
                view.calls().length().source().operation(),
                CoreMethodOp::StringLen
            );
            assert_eq!(
                view.calls().substring().source().operation(),
                CoreMethodOp::StringSubstring
            );
            assert_eq!(view.logical_transfer().branches().len(), 1);
            assert_eq!(view.logical_transfer().summary_transfers().len(), 1);
            Ok(())
        })
        .expect("logical output parity");
}

#[test]
fn logical_consumer_returns_typed_consumed_terminal() {
    let output = issue_s6c_scan_with_init_logical_output_v1(
        produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 904))
            .expect("exact S6C Recipe product"),
    )
    .expect("logical output rows");
    assert_eq!(
        consume_s6c_scan_with_init_logical_output_v1(&output),
        Ok(S6CLogicalConsumerResultV1::Consumed)
    );
}
