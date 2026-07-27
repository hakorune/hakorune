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
use crate::mir::{Callee, MirBuilder, MirInstruction, MirModule, ValueId};

use super::preloop_located_argument_ingress::{
    PreloopLocatedArgumentIngressErrorV1, PreloopLocatedArgumentIngressStageV1,
    PreloopObservedMeRouteV1,
};
use super::preloop_located_argument_port::{
    PreloopLocatedArgumentPortV1, PreloopLocatedExpressionInputV1, PreloopSelectedArgumentStateV1,
};
use super::preloop_nested_result_test_support::with_actual_parser_stageb_ingress;
use super::preloop_nested_result_test_support::with_prepared_located_outer;
use super::{drive_call_arguments_v1, CallArgumentDescentPortV1};

fn with_reached_inner_port<R>(
    f: impl for<'site, 'view, 'catalog> FnOnce(
        &mut MirBuilder,
        PreloopLocatedArgumentPortV1<'site, 'view, 'catalog, RawLegacyChildLoweringPortV1>,
        Vec<ValueId>,
    ) -> R,
) -> R {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    with_actual_parser_stageb_ingress(|mut builder, ingress| {
        builder.current_module = Some(MirModule::new("preloop-outer-port".to_owned()));
        ingress
            .with_prepared_located_argument(|prepared, _recipe| {
                builder
                    .lower_instance_method_prefix_for_test(
                        "ParserBox",
                        actual_parser_add_fixture::method_declaration_for_lowering(),
                        3,
                        |builder, _suffix| {
                            let arguments = prepared.selected().parent().arguments().to_vec();
                            let mut port = PreloopLocatedArgumentPortV1::new(
                                RawLegacyChildLoweringPortV1,
                                prepared,
                            );
                            let values = drive_call_arguments_v1(builder, &mut port, &arguments)
                                .expect("selected inner physical receipt");
                            assert!(matches!(
                                port.selected_state(),
                                PreloopSelectedArgumentStateV1::ReachedPhysical(_)
                            ));
                            Ok((ValueId::new(0), f(builder, port, values)))
                        },
                    )
                    .expect("configured candidate fixture")
            })
            .expect("actual Parser function ingress")
    })
}

fn outer_call_count(builder: &MirBuilder) -> usize {
    builder
        .current_function_instructions()
        .iter()
        .filter(|instruction| {
            matches!(
                instruction,
                MirInstruction::Call {
                    callee: Some(Callee::Global(symbol)),
                    ..
                } if symbol == "ParserStringUtilsBox.skip_ws/2"
            )
        })
        .count()
}

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

#[test]
fn outer_receipt_preflight_emits_no_call_without_the_inner_receipt() {
    with_prepared_located_outer(|prepared, _| {
        let mut port = PreloopLocatedArgumentPortV1::new(RawLegacyChildLoweringPortV1, prepared);
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("outer_receipt_preflight/0".to_owned());
        let before = builder.current_function_instructions().len();

        let error = port
            .finish_outer_static_request_v1(
                &mut builder,
                "ParserStringUtilsBox",
                "skip_ws",
                2,
                vec![],
            )
            .expect_err("outer receipt requires the selected inner receipt");

        assert!(error.contains("inner receipt unavailable"));
        assert_eq!(builder.current_function_instructions().len(), before);
        assert!(builder
            .current_function_instructions()
            .iter()
            .all(|instruction| !matches!(instruction, MirInstruction::Call { .. })));
        assert!(matches!(
            port.selected_state(),
            PreloopSelectedArgumentStateV1::Armed(_)
        ));
        port.discard();
    });
}

#[test]
fn outer_emission_failure_retains_the_inner_receipt_and_publishes_no_type() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_reached_inner_port(|builder, mut port, values| {
            let inner_destination = match port.selected_state() {
                PreloopSelectedArgumentStateV1::ReachedPhysical(reached) => {
                    reached.final_destination()
                }
                other => panic!("expected reached inner receipt, got {other:?}"),
            };
            let current_block = builder.function_state.current_block;
            builder.function_state.current_block = None;
            let error = port
                .finish_outer_static_request_v1(
                    builder,
                    "ParserStringUtilsBox",
                    "skip_ws",
                    2,
                    values,
                )
                .expect_err("outer physical emission failure");
            builder.function_state.current_block = current_block;
            assert!(error.contains("No current basic block"));
            let PreloopSelectedArgumentStateV1::Rejected(rejected) = port.selected_state() else {
                panic!("outer failure must retain a typed rejection")
            };
            assert_eq!(
                rejected.cause(),
                &PreloopLocatedArgumentIngressErrorV1::OuterPhysicalReceipt(
                    super::unified_emitter::UnifiedValueCallReceiptErrorV1::Emission {
                        detail: "No current basic block".into(),
                    }
                )
            );
            assert_eq!(
                rejected.retained_physical_destination(),
                Some(inner_destination)
            );
            assert_eq!(
                builder.function_state.type_ctx.get_type(inner_destination),
                None
            );
            port.discard();
        });
    });
}

#[test]
fn completed_outer_terminal_rejects_duplicate_and_wrong_completion_without_a_second_call() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_reached_inner_port(|builder, mut port, values| {
            let outer_destination = port
                .finish_outer_static_request_v1(
                    builder,
                    "ParserStringUtilsBox",
                    "skip_ws",
                    2,
                    values.clone(),
                )
                .expect("first outer receipt");
            assert_eq!(outer_call_count(builder), 1);
            let duplicate = port
                .finish_outer_static_request_v1(
                    builder,
                    "ParserStringUtilsBox",
                    "skip_ws",
                    2,
                    values,
                )
                .expect_err("outer receipt is one-shot");
            assert!(duplicate.contains("inner receipt unavailable"));
            assert_eq!(outer_call_count(builder), 1);

            let rejected = port
                .into_reached_physical()
                .expect_err("inner-only terminal must reject the outer owner");
            assert_eq!(
                rejected.cause(),
                &PreloopLocatedArgumentIngressErrorV1::WrongCompletionTerminal
            );
            assert_eq!(
                rejected.retained_outer_destination(),
                Some(outer_destination)
            );
            assert_eq!(
                builder.function_state.type_ctx.get_type(outer_destination),
                None
            );
            rejected.discard();
        });
    });
}
