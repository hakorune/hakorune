//! Production-prefix proof for receipt-backed pre-loop Integer publication.
//!
//! Each cell owns and discards its configured Builder. The selected source
//! association and physical receipt come from the existing proof path; only
//! the final type publication terminal is new here.

use hakorune_mir_builder::lowering_facts::TypeFactDecisionErrorV1;

use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use crate::mir::callable_result_representation::actual_parser_add_fixture;
use crate::mir::{MirBuilder, MirModule, MirType, ValueId};

use super::member_route::MemberCallRoutePlan;
use super::preloop_located_argument_port::PreloopLocatedArgumentPortV1;
use super::preloop_nested_result_test_support::with_prepared_preloop;
use super::preloop_nested_result_type::publish_preloop_nested_integer_result_v1;

fn run_success_cell(existing: Option<MirType>) -> MirType {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_prepared_preloop(
            |prepared, outer_input, outer_receiver, outer_method, _, _| {
                let mut builder = MirBuilder::new();
                builder.current_module =
                    Some(MirModule::new("preloop-type-p0-success".to_string()));
                builder
                    .comp_ctx
                    .install_callable_declaration_catalog(
                        actual_parser_add_fixture::declaration_catalog_for_lowering(),
                    )
                    .expect("production prefix lowering catalog");

                builder
                    .lower_instance_method_prefix_for_test(
                        "ParserBox",
                        actual_parser_add_fixture::method_declaration_for_lowering(),
                        3,
                        |builder, suffix| {
                            assert!(matches!(suffix.first(), Some(ASTNode::Assignment { .. })));
                            let route = builder
                                .plan_member_call_route(&outer_receiver, &outer_method)
                                .expect("existing outer route");
                            assert!(matches!(route, MemberCallRoutePlan::StaticReceiver { .. }));
                            let mut port = PreloopLocatedArgumentPortV1::new(
                                RawLegacyChildLoweringPortV1,
                                prepared,
                            );
                            let outer_value = builder
                                .execute_prepared_member_call_route_v1(
                                    &mut port,
                                    &outer_input,
                                    route,
                                )
                                .expect("production-shaped outer route");
                            let receipt = port
                                .into_emitted_nested_result()
                                .expect("outer success emits the nested receipt");
                            let destination = receipt.final_destination();

                            match existing {
                                Some(existing) => builder
                                    .function_state
                                    .type_ctx
                                    .set_type(destination, existing),
                                None => {
                                    builder
                                        .function_state
                                        .type_ctx
                                        .value_types
                                        .remove(&destination);
                                }
                            }

                            publish_preloop_nested_integer_result_v1(
                                receipt,
                                &mut builder.function_state.type_ctx,
                            )
                            .expect("missing, Unknown, and matching Integer must publish");
                            let published = builder
                                .function_state
                                .type_ctx
                                .get_type(destination)
                                .cloned()
                                .expect("exact nested Integer fact");
                            Ok((outer_value, published))
                        },
                    )
                    .expect("production-prefix publication fixture")
            },
        )
    })
}

fn run_conflict_cell() -> TypeFactDecisionErrorV1 {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_prepared_preloop(
            |prepared, outer_input, outer_receiver, outer_method, _, _| {
                let mut builder = MirBuilder::new();
                builder.current_module =
                    Some(MirModule::new("preloop-type-p0-conflict".to_string()));
                builder
                    .comp_ctx
                    .install_callable_declaration_catalog(
                        actual_parser_add_fixture::declaration_catalog_for_lowering(),
                    )
                    .expect("production prefix lowering catalog");
                let mut retained = None;

                let error = builder
                    .lower_instance_method_prefix_for_test(
                        "ParserBox",
                        actual_parser_add_fixture::method_declaration_for_lowering(),
                        3,
                        |builder, _suffix| {
                            let route = builder
                                .plan_member_call_route(&outer_receiver, &outer_method)
                                .expect("existing outer route");
                            let mut port = PreloopLocatedArgumentPortV1::new(
                                RawLegacyChildLoweringPortV1,
                                prepared,
                            );
                            let outer_value = builder
                                .execute_prepared_member_call_route_v1(
                                    &mut port,
                                    &outer_input,
                                    route,
                                )
                                .expect("production-shaped outer route");
                            let receipt = port
                                .into_emitted_nested_result()
                                .expect("outer success emits the nested receipt");
                            let destination = receipt.final_destination();
                            builder
                                .function_state
                                .type_ctx
                                .set_type(destination, MirType::Bool);

                            let rejected = publish_preloop_nested_integer_result_v1(
                                receipt,
                                &mut builder.function_state.type_ctx,
                            )
                            .expect_err("concrete Bool must reject Integer publication");
                            assert_eq!(rejected.destination(), destination);
                            assert_eq!(
                                builder.function_state.type_ctx.get_type(destination),
                                Some(&MirType::Bool),
                                "typed conflict must not overwrite the concrete fact"
                            );
                            retained = Some(rejected.cause().clone());
                            rejected.discard();
                            Err::<(ValueId, ()), _>(
                                "[preloop-type-p0/concrete-conflict]".to_string(),
                            )
                        },
                    )
                    .expect_err("type conflict must discard the fixture candidate");
                assert!(error.contains("concrete-conflict"));
                retained.expect("typed conflict cause retained before candidate discard")
            },
        )
    })
}

#[test]
fn production_prefix_publishes_none_unknown_and_matching_integer() {
    for existing in [None, Some(MirType::Unknown), Some(MirType::Integer)] {
        assert_eq!(run_success_cell(existing), MirType::Integer);
    }
}

#[test]
fn production_prefix_conflict_preserves_fact_then_fresh_fixture_succeeds() {
    assert_eq!(
        run_conflict_cell(),
        TypeFactDecisionErrorV1::ConcreteFactConflict {
            existing: MirType::Bool,
            proposed: MirType::Integer,
        }
    );
    assert_eq!(run_success_cell(None), MirType::Integer);
}
