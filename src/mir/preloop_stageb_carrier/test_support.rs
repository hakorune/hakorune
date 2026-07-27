//! Shared test-only owner for the actual Parser Stage-B activation plan.

use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, project_static_exact_i64_requirement_v1,
};
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::source_call_target::{
    VerifiedRawCallableSourceViewV1, VerifiedSourceMethodCallSiteV1,
};
use crate::mir::source_instance_result_contract::{
    prepare_preloop_located_argument_v1, prepare_preloop_nested_result_association_v1,
    seal_nested_instance_result_contract, VerifiedCurrentOwnerInstanceResultTargetV1,
};

use super::{
    prepare_preloop_stageb_carrier_rows_v1, seal_preloop_outer_carrier_result_v1,
    VerifiedPreloopStageBCarrierActivationPlanV1,
};

pub(crate) fn actual_parser_activation_plan() -> VerifiedPreloopStageBCarrierActivationPlanV1 {
    let (catalog, rows) =
        actual_parser_add_fixture::with_owned_stageb_carrier_correspondence_inputs(
            |catalog, caller, outer_site, inner_sites, targets, results| {
                let requirement = project_static_exact_i64_requirement_v1(
                    catalog, caller, outer_site, targets, results,
                )
                .expect("actual Parser exact outer requirement");
                let source_call =
                    VerifiedSourceMethodCallSiteV1::verify(catalog, caller, inner_sites[0].clone())
                        .expect("actual Parser selected inner call");
                let inner_target = VerifiedCurrentOwnerInstanceResultTargetV1::seal(&source_call)
                    .expect("actual Parser selected inner target");
                let inner_proof = results
                    .issue_unannotated_body_proof(inner_target.target())
                    .expect("actual Parser selected inner result");
                let inner_contract =
                    seal_nested_instance_result_contract(inner_target, inner_proof)
                        .expect("actual Parser inner Integer contract");
                let view = VerifiedRawCallableSourceViewV1::verify(catalog, caller)
                    .expect("actual Parser Raw source view");
                let body = view.root_body();
                let statement = view.body_stmt(&body, 3).expect("actual Parser Body(3)");
                let outer = view
                    .child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentValue)
                    .expect("actual Parser Body(3).Value");
                let inner = view
                    .child_expr_from_expr(&outer, ExprChildRoleV1::CallArgument(1))
                    .expect("actual Parser selected argument");
                let association = prepare_preloop_nested_result_association_v1(
                    inner_contract,
                    view.method_call_input(&inner)
                        .expect("actual Parser located inner call"),
                )
                .expect("actual Parser exact inner association");
                let outer_call = view
                    .method_call_input(&outer)
                    .expect("actual Parser located outer call");
                let selected = view
                    .method_call_argument(outer_call, 1)
                    .expect("actual Parser structural Argument(1)");
                let prepared = prepare_preloop_located_argument_v1(selected, association)
                    .expect("actual Parser exact outer/inner relation");
                let contract = seal_preloop_outer_carrier_result_v1(requirement, prepared)
                    .expect("actual Parser outer Integer contract");
                prepare_preloop_stageb_carrier_rows_v1(contract)
                    .expect("actual Parser owned activation row")
            },
        );
    VerifiedPreloopStageBCarrierActivationPlanV1::seal(catalog, rows)
        .expect("actual Parser same-allocation activation")
}
