//! Capability-boundary tests for the disconnected pre-loop candidate Port.

use crate::mir::builder::me_call_header_observation::MethodCallLoweringPortV1;
use crate::mir::builder::recursive_child_lowering::{
    RawLegacyChildLoweringPortV1, RecursiveChildLoweringPortV1,
};
use crate::mir::callable_result_representation::actual_parser_add_fixture;
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::source_call_target::{
    VerifiedRawCallableSourceViewV1, VerifiedSourceMethodCallSiteV1,
};
use crate::mir::source_instance_result_contract::{
    prepare_preloop_located_argument_v1, prepare_preloop_nested_result_association_v1,
    seal_nested_instance_result_contract, VerifiedCurrentOwnerInstanceResultTargetV1,
};
use crate::mir::MirBuilder;

use super::preloop_located_argument_ingress::{
    PreloopLocatedArgumentIngressErrorV1, PreloopLocatedArgumentIngressStageV1,
    PreloopObservedMeRouteV1,
};
use super::preloop_located_argument_port::{
    PreloopLocatedArgumentPortV1, PreloopLocatedExpressionInputV1, PreloopSelectedArgumentStateV1,
};
use super::CallArgumentDescentPortV1;

fn assert_method_call_lowering_port<Port: MethodCallLoweringPortV1>() {}

#[test]
fn candidate_port_preserves_the_existing_method_call_capability_bundle() {
    assert_method_call_lowering_port::<
        PreloopLocatedArgumentPortV1<'static, 'static, 'static, RawLegacyChildLoweringPortV1>,
    >();
}

#[test]
fn candidate_rejection_retains_the_exact_selected_source_owner() {
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
            let selected = view
                .method_call_argument(
                    view.method_call_input(&outer)
                        .expect("located outer MethodCall"),
                    1,
                )
                .expect("structural Argument(1)");
            let prepared = prepare_preloop_located_argument_v1(selected, association)
                .expect("exact outer/inner relation");
            let arguments = prepared.selected().parent().arguments().to_vec();
            let mut port =
                PreloopLocatedArgumentPortV1::new(RawLegacyChildLoweringPortV1, prepared);

            let ordinary = port
                .argument_expression_input(&arguments, 0)
                .expect("ordinary Argument(0)");
            assert!(matches!(
                ordinary,
                PreloopLocatedExpressionInputV1::Ordinary(_)
            ));
            assert!(matches!(
                port.selected_state(),
                PreloopSelectedArgumentStateV1::Armed(_)
            ));

            let selected_input = port
                .argument_expression_input(&arguments, 1)
                .expect("selected Argument(1)");
            assert!(matches!(
                port.selected_state(),
                PreloopSelectedArgumentStateV1::InFlight(source)
                    if source.selected().child().site() == &sites[0]
            ));

            let duplicate = port
                .argument_expression_input(&arguments, 1)
                .expect_err("selected source owner is one-shot");
            assert!(duplicate.contains("selected-argument-unavailable"));
            assert!(matches!(
                port.selected_state(),
                PreloopSelectedArgumentStateV1::InFlight(source)
                    if source.selected().child().site() == &sites[0]
            ));

            let mut builder = MirBuilder::new();
            let rejected_report = port
                .lower_expression(&mut builder, selected_input)
                .expect_err("unconfigured candidate must reject before child descent");
            assert!(rejected_report.contains("alternate-me-route"));
            match port.selected_state() {
                PreloopSelectedArgumentStateV1::Rejected(rejected) => {
                    assert_eq!(rejected.selected_index(), 1);
                    assert_eq!(rejected.selected_site(), &sites[0]);
                    assert_eq!(
                        rejected.stage(),
                        PreloopLocatedArgumentIngressStageV1::MeRoute
                    );
                    assert_eq!(
                        rejected.cause(),
                        &PreloopLocatedArgumentIngressErrorV1::AlternateMeRoute(
                            PreloopObservedMeRouteV1::NotApplicable
                        )
                    );
                }
                other => panic!("expected payload-retaining rejection, got {other:?}"),
            }

            let duplicate = port
                .argument_expression_input(&arguments, 1)
                .expect_err("terminal rejection remains one-shot");
            assert!(duplicate.contains("selected-argument-unavailable"));
            assert!(matches!(
                port.selected_state(),
                PreloopSelectedArgumentStateV1::Rejected(rejected)
                    if rejected.selected_site() == &sites[0]
            ));
            port.discard();
        },
    );
}
