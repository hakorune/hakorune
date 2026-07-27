//! Shared test-only factory for the exact pre-loop source association.
//!
//! The factory owns the structural source descent once. Individual proof
//! modules remain responsible for their configured Builder and route effects.

use crate::ast::ASTNode;
use crate::mir::callable_result_representation::actual_parser_add_fixture;
use crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1;
use crate::mir::resolved_semantics::{ExprChildRoleV1, SourceExprSiteV1};
use crate::mir::source_call_target::{
    VerifiedRawCallableSourceViewV1, VerifiedSourceMethodCallSiteV1,
};
use crate::mir::source_instance_result_contract::{
    prepare_preloop_located_argument_v1, prepare_preloop_nested_result_association_v1,
    seal_nested_instance_result_contract, PreparedPreloopLocatedArgumentV1,
    VerifiedCurrentOwnerInstanceResultTargetV1,
};

use super::method_call_descent::RawLegacyMethodCallInputV1;

pub(super) fn with_prepared_preloop<R>(
    f: impl for<'site, 'view, 'catalog> FnOnce(
        PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
        RawLegacyMethodCallInputV1,
        ASTNode,
        String,
        Vec<ASTNode>,
        SourceExprSiteV1,
    ) -> R,
) -> R {
    actual_parser_add_fixture::with_instance_result_contract_inputs(
        |catalog, caller, sites, _targets, results| {
            let call = VerifiedSourceMethodCallSiteV1::verify(catalog, caller, sites[0].clone())
                .expect("selected pre-loop source MethodCall");
            let target = VerifiedCurrentOwnerInstanceResultTargetV1::seal(&call)
                .expect("selected pre-loop target");
            let proof = results
                .issue_unannotated_body_proof(target.target())
                .expect("selected pre-loop Integer proof");
            let contract = seal_nested_instance_result_contract(target, proof)
                .expect("selected pre-loop Integer contract");

            let view = VerifiedRawCallableSourceViewV1::verify(catalog, caller)
                .expect("catalog-backed Raw source view");
            let body = view.root_body();
            let statement = view.body_stmt(&body, 3).expect("Body(3)");
            let outer = view
                .child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentValue)
                .expect("Body(3).Value");
            let inner = view
                .child_expr_from_expr(&outer, ExprChildRoleV1::CallArgument(1))
                .expect("Body(3).Value.Argument(1)");
            let association = prepare_preloop_nested_result_association_v1(
                contract,
                view.method_call_input(&inner)
                    .expect("located inner MethodCall"),
            )
            .expect("exact pre-loop association");
            let outer_call = view
                .method_call_input(&outer)
                .expect("located outer MethodCall");
            let outer_receiver = outer_call.receiver().clone();
            let outer_method = outer_call.method().to_string();
            let outer_arguments = outer_call.arguments().to_vec();
            let outer_input = RawLegacyMethodCallInputV1::new(
                outer_receiver.clone(),
                outer_method.clone(),
                outer_arguments.clone(),
            );
            let selected = view
                .method_call_argument(outer_call, 1)
                .expect("structural Argument(1)");
            let prepared = prepare_preloop_located_argument_v1(selected, association)
                .expect("exact outer/inner relation");

            f(
                prepared,
                outer_input,
                outer_receiver,
                outer_method,
                outer_arguments,
                sites[0].clone(),
            )
        },
    )
}

pub(super) fn with_prepared_stageb_correspondence<R>(
    f: impl for<'site, 'view, 'catalog, 'result> FnOnce(
        PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
        RawLegacyMethodCallInputV1,
        ASTNode,
        String,
        Vec<ASTNode>,
        SourceExprSiteV1,
        &'result VerifiedCallableResultDispositionV1,
    ) -> R,
) -> R {
    actual_parser_add_fixture::with_stageb_carrier_correspondence_inputs(
        |catalog, caller, outer_site, inner_sites, targets, results| {
            assert!(targets.is_branded_by(catalog));
            assert!(results.is_branded_by(catalog, targets));
            let outer_target = targets
                .target(caller, outer_site)
                .expect("selected outer static target");
            let outer_result = results
                .disposition(outer_target.target())
                .expect("selected outer static result");
            assert!(
                results.call_result(caller, outer_site).is_none(),
                "the general solver must not synthesize the nested call row"
            );

            let call =
                VerifiedSourceMethodCallSiteV1::verify(catalog, caller, inner_sites[0].clone())
                    .expect("selected pre-loop source MethodCall");
            let target = VerifiedCurrentOwnerInstanceResultTargetV1::seal(&call)
                .expect("selected pre-loop target");
            let proof = results
                .issue_unannotated_body_proof(target.target())
                .expect("selected pre-loop Integer proof");
            let contract = seal_nested_instance_result_contract(target, proof)
                .expect("selected pre-loop Integer contract");

            let view = VerifiedRawCallableSourceViewV1::verify(catalog, caller)
                .expect("catalog-backed Raw source view");
            let body = view.root_body();
            let statement = view.body_stmt(&body, 3).expect("Body(3)");
            let outer = view
                .child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentValue)
                .expect("Body(3).Value");
            assert_eq!(outer.site(), outer_site);
            let inner = view
                .child_expr_from_expr(&outer, ExprChildRoleV1::CallArgument(1))
                .expect("Body(3).Value.Argument(1)");
            let association = prepare_preloop_nested_result_association_v1(
                contract,
                view.method_call_input(&inner)
                    .expect("located inner MethodCall"),
            )
            .expect("exact pre-loop association");
            let outer_call = view
                .method_call_input(&outer)
                .expect("located outer MethodCall");
            let outer_receiver = outer_call.receiver().clone();
            let outer_method = outer_call.method().to_string();
            let outer_arguments = outer_call.arguments().to_vec();
            let outer_input = RawLegacyMethodCallInputV1::new(
                outer_receiver.clone(),
                outer_method.clone(),
                outer_arguments.clone(),
            );
            let selected = view
                .method_call_argument(outer_call, 1)
                .expect("structural Argument(1)");
            let prepared = prepare_preloop_located_argument_v1(selected, association)
                .expect("exact outer/inner relation");

            f(
                prepared,
                outer_input,
                outer_receiver,
                outer_method,
                outer_arguments,
                inner_sites[0].clone(),
                outer_result,
            )
        },
    )
}
