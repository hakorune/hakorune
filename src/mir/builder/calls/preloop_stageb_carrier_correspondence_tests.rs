//! Read-only/code-facing proof for the bounded Stage-B carrier handoff.
//!
//! This fixture stops before production activation and type publication. It
//! proves that the selected inner Call, outer Call, assignment carrier, and
//! exact outer result requirement are distinct authorities that correspond.

use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, VerifiedCallableResultDispositionV1,
};
use crate::mir::{Callee, MirBuilder, MirInstruction, MirModule, MirType, ValueId};

use super::member_route::MemberCallRoutePlan;
use super::preloop_located_argument_port::PreloopLocatedArgumentPortV1;
use super::preloop_nested_result_test_support::with_prepared_stageb_correspondence;

#[derive(Debug)]
struct PreloopStageBCarrierCorrespondenceProbeV1 {
    inner_destination: ValueId,
    outer_destination: ValueId,
    assignment_rhs: ValueId,
    assigned_destination: ValueId,
    required_i64_arguments: Box<[u32]>,
}

#[test]
fn actual_preloop_stageb_carrier_correspondence_is_exact() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_prepared_stageb_correspondence(
            |prepared,
             outer_input,
             outer_receiver,
             outer_method,
             _outer_arguments,
             _selected_site,
             outer_result| {
                let required_i64_arguments = match outer_result {
                    VerifiedCallableResultDispositionV1::ExactI64 {
                        required_i64_arguments,
                    } => required_i64_arguments.clone(),
                    other => panic!("outer carrier result must be exact Integer: {other:?}"),
                };
                assert_eq!(
                    required_i64_arguments.as_ref(),
                    [1],
                    "the bounded outer result must require structural Argument(1)"
                );

                let mut builder = MirBuilder::new();
                builder.current_module =
                    Some(MirModule::new("preloop-stageb-correspondence".to_string()));
                builder
                    .comp_ctx
                    .install_callable_declaration_catalog(
                        actual_parser_add_fixture::declaration_catalog_for_lowering(),
                    )
                    .expect("fixture lowering catalog");
                builder
                    .lower_instance_method_prefix_for_test(
                        "ParserBox",
                        actual_parser_add_fixture::method_declaration_for_lowering(),
                        3,
                        |builder, suffix| {
                            assert!(matches!(suffix.first(), Some(ASTNode::Assignment { .. })));
                            let route = builder
                                .plan_member_call_route(&outer_receiver, &outer_method)
                                .expect("existing outer route plan");
                            assert!(matches!(route, MemberCallRoutePlan::StaticReceiver { .. }));

                            let mut port = PreloopLocatedArgumentPortV1::new(
                                RawLegacyChildLoweringPortV1,
                                prepared,
                            );
                            let outer_destination = builder
                                .execute_prepared_member_call_route_v1(
                                    &mut port,
                                    &outer_input,
                                    route,
                                )
                                .expect("configured outer route");
                            let nested = port
                                .into_emitted_nested_result()
                                .expect("outer success commits inner receipt");
                            let inner_destination = nested.final_destination();
                            assert_ne!(
                                inner_destination, outer_destination,
                                "inner and outer Calls must retain distinct destinations"
                            );

                            let outer_call_destinations = builder
                                .current_function_instructions()
                                .iter()
                                .filter_map(|instruction| match instruction {
                                    MirInstruction::Call {
                                        dst: Some(dst),
                                        callee: Some(Callee::Global(symbol)),
                                        ..
                                    } if symbol == "ParserStringUtilsBox.skip_ws/2" => Some(*dst),
                                    _ => None,
                                })
                                .collect::<Vec<_>>();
                            assert_eq!(outer_call_destinations, [outer_destination]);

                            let assignment_rhs = outer_destination;
                            let assigned_destination = builder
                                .build_assignment_from_value("pos".to_string(), assignment_rhs)
                                .expect("existing assignment completion");
                            assert_eq!(
                                assigned_destination, outer_destination,
                                "the current untyped pos row must not project the Call result"
                            );
                            assert_eq!(
                                builder.function_state.variable_ctx.variable_map["pos"],
                                assigned_destination,
                                "the assignment-published carrier must be the later loop input"
                            );
                            assert_ne!(
                                builder
                                    .function_state
                                    .type_ctx
                                    .get_type(assigned_destination),
                                Some(&MirType::Integer),
                                "correspondence proof must not publish the outer Integer fact"
                            );

                            nested.discard();
                            Ok((
                                assigned_destination,
                                PreloopStageBCarrierCorrespondenceProbeV1 {
                                    inner_destination,
                                    outer_destination,
                                    assignment_rhs,
                                    assigned_destination,
                                    required_i64_arguments,
                                },
                            ))
                        },
                    )
                    .map(|probe| {
                        assert_eq!(probe.outer_destination, probe.assignment_rhs);
                        assert_eq!(probe.outer_destination, probe.assigned_destination);
                        assert_ne!(probe.inner_destination, probe.outer_destination);
                        assert_eq!(probe.required_i64_arguments.as_ref(), [1]);
                    })
                    .expect("production-shaped correspondence fixture");
            },
        );
    });
}
