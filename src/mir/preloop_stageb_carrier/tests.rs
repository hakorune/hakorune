use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, project_static_exact_i64_requirement_v1,
};
use crate::mir::resolved_semantics::{
    ExprChildRoleV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
};
use crate::mir::source_call_target::{
    VerifiedRawCallableSourceViewV1, VerifiedSourceMethodCallSiteV1,
};
use crate::mir::source_instance_result_contract::{
    prepare_preloop_located_argument_v1, prepare_preloop_nested_result_association_v1,
    seal_nested_instance_result_contract, VerifiedCurrentOwnerInstanceResultTargetV1,
};

use super::activation::{
    prepare_preloop_stageb_carrier_rows_v1, PreloopStageBCarrierActivationErrorV1,
    PreloopStageBCarrierActivationStageV1, VerifiedPreloopStageBCarrierActivationPlanV1,
};
use super::{
    seal_preloop_outer_carrier_result_v1, PreloopOuterCarrierResultContractErrorV1,
    PreloopOuterCarrierResultContractStageV1,
};

macro_rules! bind_actual_preloop {
    (
        $call:ident,
        $view:ident,
        $prepared:ident,
        $catalog:expr,
        $caller:expr,
        $inner_site:expr,
        $results:expr
    ) => {
        let $call = VerifiedSourceMethodCallSiteV1::verify($catalog, $caller, $inner_site.clone())
            .expect("selected pre-loop source MethodCall");
        let inner_target = VerifiedCurrentOwnerInstanceResultTargetV1::seal(&$call)
            .expect("selected pre-loop target");
        let inner_proof = $results
            .issue_unannotated_body_proof(inner_target.target())
            .expect("selected pre-loop Integer proof");
        let inner_contract = seal_nested_instance_result_contract(inner_target, inner_proof)
            .expect("selected pre-loop Integer contract");
        let $view = VerifiedRawCallableSourceViewV1::verify($catalog, $caller)
            .expect("catalog-backed Raw source view");
        let body = $view.root_body();
        let statement = $view.body_stmt(&body, 3).expect("Body(3)");
        let outer = $view
            .child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentValue)
            .expect("Body(3).Value");
        let inner = $view
            .child_expr_from_expr(&outer, ExprChildRoleV1::CallArgument(1))
            .expect("Body(3).Value.Argument(1)");
        let association = prepare_preloop_nested_result_association_v1(
            inner_contract,
            $view
                .method_call_input(&inner)
                .expect("located inner MethodCall"),
        )
        .expect("exact pre-loop association");
        let outer_call = $view
            .method_call_input(&outer)
            .expect("located outer MethodCall");
        let selected = $view
            .method_call_argument(outer_call, 1)
            .expect("structural Argument(1)");
        let $prepared = prepare_preloop_located_argument_v1(selected, association)
            .expect("exact outer/inner relation");
    };
}

fn loop_refresh_outer_site() -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(4),
        SourcePathSegmentV1::LoopBody(5),
        SourcePathSegmentV1::Value,
    ]))
}

#[test]
fn actual_preloop_outer_integer_contract_seals_without_general_call_result() {
    actual_parser_add_fixture::with_stageb_carrier_correspondence_inputs(
        |catalog, caller, outer_site, inner_sites, targets, results| {
            assert!(results.call_result(caller, outer_site).is_none());
            let requirement = project_static_exact_i64_requirement_v1(
                catalog, caller, outer_site, targets, results,
            )
            .expect("bounded exact static requirement");
            assert_eq!(requirement.required_i64_arguments(), [1]);
            bind_actual_preloop!(
                call,
                view,
                prepared,
                catalog,
                caller,
                &inner_sites[0],
                results
            );

            let contract = seal_preloop_outer_carrier_result_v1(requirement, prepared)
                .expect("exact outer carrier Integer contract");
            assert!(contract.is_branded_by(catalog));
            assert_eq!(contract.caller(), caller);
            assert_eq!(contract.outer_site(), outer_site);
            assert_eq!(contract.selected_argument_index(), 1);
            assert_eq!(contract.inner_site(), &inner_sites[0]);
            assert_eq!(contract.target().owner(), "ParserStringUtilsBox");
            assert_eq!(contract.target().name(), "skip_ws");
            assert!(contract.result_is_integer());
            contract.discard();
            let _ = (call, view);
        },
    );
}

#[test]
fn equal_looking_foreign_catalog_rejects_before_result_closure() {
    actual_parser_add_fixture::with_stageb_carrier_correspondence_inputs(
        |outer_catalog, outer_caller, outer_site, _inner_sites, outer_targets, outer_results| {
            let requirement = project_static_exact_i64_requirement_v1(
                outer_catalog,
                outer_caller,
                outer_site,
                outer_targets,
                outer_results,
            )
            .expect("outer allocation requirement");

            actual_parser_add_fixture::with_stageb_carrier_correspondence_inputs(
                |inner_catalog,
                 inner_caller,
                 _outer_site,
                 inner_sites,
                 _inner_targets,
                 inner_results| {
                    bind_actual_preloop!(
                        call,
                        view,
                        prepared,
                        inner_catalog,
                        inner_caller,
                        &inner_sites[0],
                        inner_results
                    );
                    let rejected = seal_preloop_outer_carrier_result_v1(requirement, prepared)
                        .expect_err("equal-looking foreign allocation must reject");
                    assert_eq!(
                        rejected.stage(),
                        PreloopOuterCarrierResultContractStageV1::CatalogAllocation
                    );
                    assert_eq!(
                        rejected.cause(),
                        &PreloopOuterCarrierResultContractErrorV1::ForeignCatalog
                    );
                    rejected.discard();
                    let _ = (call, view);
                },
            );
        },
    );
}

#[test]
fn loop_refresh_outer_site_cannot_borrow_the_preloop_relation() {
    actual_parser_add_fixture::with_stageb_carrier_correspondence_inputs(
        |catalog, caller, _outer_site, inner_sites, targets, results| {
            let refresh_site = loop_refresh_outer_site();
            let requirement = project_static_exact_i64_requirement_v1(
                catalog,
                caller,
                &refresh_site,
                targets,
                results,
            )
            .expect("parked loop-refresh static requirement");
            bind_actual_preloop!(
                call,
                view,
                prepared,
                catalog,
                caller,
                &inner_sites[0],
                results
            );

            let rejected = seal_preloop_outer_carrier_result_v1(requirement, prepared)
                .expect_err("loop-refresh outer site must not pair with pre-loop source");
            assert_eq!(
                rejected.stage(),
                PreloopOuterCarrierResultContractStageV1::OuterSite
            );
            assert_eq!(
                rejected.cause(),
                &PreloopOuterCarrierResultContractErrorV1::OuterSiteMismatch
            );
            rejected.discard();
            let _ = (call, view);
        },
    );
}

#[test]
fn required_argument_set_must_be_exactly_the_selected_argument() {
    actual_parser_add_fixture::with_stageb_carrier_correspondence_inputs(
        |catalog, caller, outer_site, inner_sites, targets, results| {
            let requirement = project_static_exact_i64_requirement_v1(
                catalog, caller, outer_site, targets, results,
            )
            .expect("bounded exact static requirement")
            .with_required_i64_arguments_for_test(&[0]);
            bind_actual_preloop!(
                call,
                view,
                prepared,
                catalog,
                caller,
                &inner_sites[0],
                results
            );

            let rejected = seal_preloop_outer_carrier_result_v1(requirement, prepared)
                .expect_err("Argument(0) requirement cannot justify selected Argument(1)");
            assert_eq!(
                rejected.stage(),
                PreloopOuterCarrierResultContractStageV1::RequiredArguments
            );
            assert_eq!(
                rejected.cause(),
                &PreloopOuterCarrierResultContractErrorV1::RequiredArgumentsMismatch {
                    selected: 1,
                    actual: Box::new([0]),
                }
            );
            rejected.discard();
            let _ = (call, view);
        },
    );
}

#[test]
fn owned_activation_plan_retains_one_exact_root_assignment_schedule() {
    let (catalog, rows) =
        actual_parser_add_fixture::with_owned_stageb_carrier_correspondence_inputs(
            |catalog, caller, outer_site, inner_sites, targets, results| {
                let requirement = project_static_exact_i64_requirement_v1(
                    catalog, caller, outer_site, targets, results,
                )
                .expect("bounded exact static requirement");
                bind_actual_preloop!(
                    call,
                    view,
                    prepared,
                    catalog,
                    caller,
                    &inner_sites[0],
                    results
                );
                let contract = seal_preloop_outer_carrier_result_v1(requirement, prepared)
                    .expect("exact outer carrier Integer contract");
                let rows = prepare_preloop_stageb_carrier_rows_v1(contract)
                    .expect("owned one-row normalization");
                let _ = (call, view);
                rows
            },
        );
    let plan = VerifiedPreloopStageBCarrierActivationPlanV1::seal(catalog, rows)
        .expect("same-allocation owned plan");
    let row = plan.row();
    assert_eq!(row.caller().owner(), "ParserBox");
    assert_eq!(row.caller().name(), "static_const_parse_add");
    assert_eq!(row.assignment_target().name(), "pos");
    assert!(row.uses().is_empty());
    assert!(row.attrs().is_empty());
    assert_eq!(row.outer_call_site().node().segments().len(), 2);
    assert_eq!(row.selected_argument_index(), 1);
    assert_eq!(row.inner_call_site().node().segments().len(), 3);
    assert_eq!(row.nested_result_rebind().caller(), row.caller());
    assert_eq!(row.nested_result_rebind().site(), row.inner_call_site());
    assert_eq!(row.nested_result_rebind().target().owner(), "ParserBox");
    assert_eq!(
        row.nested_result_rebind().target().name(),
        "static_const_eval_pos"
    );
    assert_eq!(row.nested_result_rebind().target().arity(), 1);
    assert_eq!(row.outer_target().owner(), "ParserStringUtilsBox");
    assert_eq!(row.outer_target().name(), "skip_ws");
    assert!(row.result().is_integer());
    assert_eq!(row.body_handoff().prefix_statement_count(), 3);
    assert_eq!(
        row.body_handoff().selected_statement().node().segments(),
        &[SourcePathSegmentV1::Body(3)]
    );
    assert_eq!(row.body_handoff().suffix_statement_start(), 4);
    assert!(row.body_handoff().body_statement_count() > 4);

    assert_eq!(plan.row().caller().arity(), 2);
    assert_eq!(plan.row().outer_target().arity(), 2);
}

#[test]
fn owned_activation_rows_reject_an_equal_looking_foreign_catalog() {
    let (primary, rows) =
        actual_parser_add_fixture::with_owned_stageb_carrier_correspondence_inputs(
            |catalog, caller, outer_site, inner_sites, targets, results| {
                let requirement = project_static_exact_i64_requirement_v1(
                    catalog, caller, outer_site, targets, results,
                )
                .expect("bounded exact static requirement");
                bind_actual_preloop!(
                    call,
                    view,
                    prepared,
                    catalog,
                    caller,
                    &inner_sites[0],
                    results
                );
                let contract = seal_preloop_outer_carrier_result_v1(requirement, prepared)
                    .expect("exact outer carrier Integer contract");
                let rows = prepare_preloop_stageb_carrier_rows_v1(contract)
                    .expect("owned one-row normalization");
                let _ = (call, view);
                rows
            },
        );
    let (foreign, ()) = actual_parser_add_fixture::with_owned_stageb_carrier_correspondence_inputs(
        |_, _, _, _, _, _| (),
    );
    let rejected = VerifiedPreloopStageBCarrierActivationPlanV1::seal(foreign, rows)
        .expect_err("equal-looking foreign catalog must reject");
    assert_eq!(
        rejected.stage(),
        PreloopStageBCarrierActivationStageV1::CatalogAllocation
    );
    assert_eq!(
        rejected.cause(),
        &PreloopStageBCarrierActivationErrorV1::CatalogAllocationMismatch
    );
    rejected.discard();
    drop(primary);
}
