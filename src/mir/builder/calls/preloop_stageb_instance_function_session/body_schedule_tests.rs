//! Focused F6-2 proofs for the one-driver Stage-B body schedule.

use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::calls::lowering::{
    mir_method_param_decls_from_source, normalize_instance_method_param_decls,
    normalize_instance_method_params,
};
use crate::mir::builder::module_draft_collector::ModuleDraftCollectorV1;
use crate::mir::builder::module_lowering_invocation::ModuleLoweringInvocationV1;
use crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1;
use crate::mir::builder::stmts::block_driver::LegacyBlockDescentPortV1;
use crate::mir::callable_result_representation::actual_parser_add_fixture;
use crate::mir::{MirBuilder, MirInstruction, MirModule, MirType, ValueId};

use super::body_schedule::{
    drive_preloop_stageb_body_schedule_v1, PreloopStageBBodySchedulePortV1,
};
use super::rejection::{PreloopStageBBodyScheduleCauseV1, PreloopStageBBodyScheduleStageV1};
use crate::mir::builder::calls::preloop_nested_result_test_support::with_actual_parser_stageb_ingress;

fn configure_actual_parser_function(builder: &mut MirBuilder) {
    let ASTNode::FunctionDeclaration {
        name,
        params,
        param_decls,
        return_type_name,
        body,
        uses,
        attrs,
        is_static,
        ..
    } = actual_parser_add_fixture::method_declaration_for_lowering()
    else {
        panic!("actual Parser fixture must be a function declaration");
    };
    assert!(!is_static);
    let box_name = "ParserBox";
    let function_name = format!("{box_name}.{name}/{}", params.len());
    let params = normalize_instance_method_params(&function_name, params);
    let param_decls = normalize_instance_method_param_decls(&function_name, param_decls);
    builder.current_module = Some(MirModule::new("preloop-stageb-body-schedule".to_owned()));
    builder
        .create_method_skeleton(function_name, box_name, &params, &body)
        .expect("actual Parser instance skeleton");
    builder.set_current_function_declared_signature(
        mir_method_param_decls_from_source(box_name, &params, &param_decls),
        return_type_name,
    );
    builder.set_current_function_runes(&attrs);
    builder.set_current_function_declared_capability_uses(&uses);
    builder
        .setup_method_params(box_name, &params)
        .expect("actual Parser parameters");
}

fn with_actual_schedule<R>(
    shadow_outer_static_box: bool,
    f: impl FnOnce(
        &mut MirBuilder,
        Result<
            super::body_schedule::CompletedPreloopStageBBodyScheduleV1,
            super::rejection::RejectedPreloopStageBBodyScheduleV1,
        >,
    ) -> R,
) -> R {
    with_actual_parser_stageb_ingress(|mut builder, ingress| {
        configure_actual_parser_function(&mut builder);
        if shadow_outer_static_box {
            let shadow = builder
                .build_expression(ASTNode::Literal {
                    value: LiteralValue::Integer(99),
                    span: Span::unknown(),
                })
                .expect("shadow static receiver");
            builder.bind_variable_for_test("ParserStringUtilsBox", shadow);
        }
        let result = {
            let mut invocation = ModuleLoweringInvocationV1::with_collector(
                &mut builder,
                ModuleDraftCollectorV1::default(),
            );
            invocation.with_module_port(|builder, module_port| {
                drive_preloop_stageb_body_schedule_v1(
                    builder,
                    RawInvocationChildPortV1::new(module_port),
                    ingress,
                )
            })
        };
        f(&mut builder, result)
    })
}

fn instruction_count(builder: &MirBuilder) -> usize {
    builder
        .current_function_instructions()
        .iter()
        .filter(|instruction| matches!(instruction, MirInstruction::Call { .. }))
        .count()
}

#[test]
fn actual_parser_body_schedule_publishes_before_real_suffix_frontier() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_actual_schedule(false, |builder, result| {
            let rejected = result.expect_err("the actual suffix frontier must remain explicit");
            assert_eq!(
                rejected.stage(),
                PreloopStageBBodyScheduleStageV1::Suffix,
                "{}",
                rejected.bounded_report()
            );
            assert!(matches!(
                rejected.cause(),
                PreloopStageBBodyScheduleCauseV1::OrdinaryDescent { index: 4, .. }
            ));
            let carrier = rejected
                .retained_published_carrier_for_test()
                .expect("suffix failure retains the complete published carrier");
            assert_ne!(carrier.inner_destination(), carrier.outer_destination());
            assert_eq!(carrier.assigned_destination(), carrier.outer_destination());
            assert_eq!(
                builder
                    .function_state
                    .type_ctx
                    .get_type(carrier.outer_destination()),
                Some(&MirType::Integer)
            );
            assert!(
                instruction_count(builder) >= 2,
                "inner and outer Calls must precede the real suffix frontier"
            );
            rejected.discard();
        });
    });
}

#[test]
fn prefix_failure_retains_pending_source_and_blocks_selected_and_suffix() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "0", || {
        with_actual_schedule(false, |builder, result| {
            let rejected = result.expect_err("ordinary prefix must fail before selection");
            assert_eq!(
                rejected.stage(),
                PreloopStageBBodyScheduleStageV1::Prefix,
                "{}",
                rejected.bounded_report()
            );
            assert!(matches!(
                rejected.cause(),
                PreloopStageBBodyScheduleCauseV1::OrdinaryDescent { index: 0, .. }
            ));
            assert_eq!(instruction_count(builder), 0);
            assert!(rejected.retained_published_carrier_for_test().is_none());
            rejected.discard();
        });
    });
}

#[test]
fn selected_barrier_clips_prefix_suffix_input_and_requires_one_shot_completion() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    with_actual_parser_stageb_ingress(|mut builder, ingress| {
        configure_actual_parser_function(&mut builder);
        let mut invocation = ModuleLoweringInvocationV1::with_collector(
            &mut builder,
            ModuleDraftCollectorV1::default(),
        );
        invocation.with_module_port(|_builder, module_port| {
            let rejected = ingress
                .with_prepared_located_argument(|source, recipe| {
                    let port = PreloopStageBBodySchedulePortV1::prepare(
                        RawInvocationChildPortV1::new(module_port),
                        source,
                        recipe,
                    )
                    .expect("exact schedule preflight");
                    assert_eq!(
                        port.suffix_route_input(0)
                            .expect("prefix route")
                            .expect("prefix slice")
                            .len(),
                        3
                    );
                    assert!(port
                        .suffix_route_input(3)
                        .expect("selected route")
                        .is_none());
                    port.finish(ValueId::new(0))
                        .expect_err("selected row was intentionally not reached")
                })
                .expect("source ingress");
            assert_eq!(
                rejected.stage(),
                PreloopStageBBodyScheduleStageV1::Completion
            );
            assert_eq!(
                rejected.cause(),
                &PreloopStageBBodyScheduleCauseV1::SelectedNotReached
            );
            rejected.discard();
        });
    });
}

#[test]
fn selected_route_drift_retains_rejection_and_fresh_fixture_reuses() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_actual_schedule(true, |builder, result| {
            let rejected = result.expect_err("selected static route drift must fail closed");
            assert_eq!(
                rejected.stage(),
                PreloopStageBBodyScheduleStageV1::Selected,
                "{}",
                rejected.bounded_report()
            );
            assert!(matches!(
                rejected.cause(),
                PreloopStageBBodyScheduleCauseV1::SelectedTransaction { .. }
            ));
            assert!(!builder
                .current_function_instructions()
                .iter()
                .any(|instruction| matches!(
                    instruction,
                    MirInstruction::Call {
                        callee: Some(crate::mir::Callee::Global(symbol)),
                        ..
                    } if symbol == "ParserStringUtilsBox.skip_ws/2"
                )));
            rejected.discard();
        });
    });

    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_actual_schedule(false, |builder, result| {
            assert!(
                instruction_count(builder) >= 2,
                "fresh fixture must reach inner and outer Calls"
            );
            let rejected = result.expect_err("fresh fixture must reach the real suffix frontier");
            assert_eq!(rejected.stage(), PreloopStageBBodyScheduleStageV1::Suffix);
            rejected.discard();
        });
    });
}
