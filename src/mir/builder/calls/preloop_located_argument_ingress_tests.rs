//! Configured proof for the bounded pre-loop located ingress.
//!
//! The fixture owns and discards its whole Builder. Emitted Calls are proof
//! effects only: this row issues no typed physical receipt, final-destination
//! authority, nested-result receipt, or nested Integer publication.

use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use crate::mir::callable_result_representation::actual_parser_add_fixture;
use crate::mir::definitions::call_unified::TypeCertainty;
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::source_call_target::{
    VerifiedRawCallableSourceViewV1, VerifiedSourceMethodCallSiteV1,
};
use crate::mir::source_instance_result_contract::{
    prepare_preloop_located_argument_v1, prepare_preloop_nested_result_association_v1,
    seal_nested_instance_result_contract, VerifiedCurrentOwnerInstanceResultTargetV1,
};
use crate::mir::value_kind::MirValueKind;
use crate::mir::{Callee, MirBuilder, MirInstruction, MirModule, MirType};

use super::member_route::MemberCallRoutePlan;
use super::method_call_descent::RawLegacyMethodCallInputV1;
use super::preloop_located_argument_port::{
    PreloopLocatedArgumentPortV1, PreloopSelectedArgumentStateV1,
};

fn ordinary_copy_root(
    instructions: &[MirInstruction],
    mut value: crate::mir::ValueId,
) -> crate::mir::ValueId {
    let mut remaining = instructions.len();
    while remaining > 0 {
        let Some(source) = instructions
            .iter()
            .find_map(|instruction| match instruction {
                MirInstruction::Copy { dst, src } if *dst == value => Some(*src),
                _ => None,
            })
        else {
            break;
        };
        value = source;
        remaining -= 1;
    }
    value
}

#[test]
fn configured_preloop_ingress_reaches_existing_inner_and_outer_call_terminals() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        actual_parser_add_fixture::with_instance_result_contract_inputs(
            |catalog, caller, sites, _targets, results| {
                let call =
                    VerifiedSourceMethodCallSiteV1::verify(catalog, caller, sites[0].clone())
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
                let selected = view
                    .method_call_argument(outer_call, 1)
                    .expect("structural Argument(1)");
                let prepared = prepare_preloop_located_argument_v1(selected, association)
                    .expect("exact outer/inner relation");

                // The outer compatibility transport is permitted. The selected
                // inner MethodCall itself remains located and never re-enters the
                // Raw dispatcher.
                let outer_input = RawLegacyMethodCallInputV1::new(
                    outer_receiver.clone(),
                    outer_method.clone(),
                    outer_arguments,
                );

                let mut builder = MirBuilder::new();
                builder.current_module =
                    Some(MirModule::new("preloop-production-prefix".to_string()));
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
                            assert_eq!(
                                std::env::var("NYASH_MIR_UNIFIED_CALL").as_deref(),
                                Ok("1"),
                                "production fixture must select unified Call"
                            );
                            assert!(matches!(suffix.first(), Some(ASTNode::Assignment { .. })));

                            let function = builder
                                .function_state
                                .current_function
                                .as_ref()
                                .expect("production-shaped method skeleton");
                            let me = function.params[0];
                            assert_eq!(
                                builder.get_value_kind(me),
                                Some(MirValueKind::Parameter(0))
                            );
                            assert_eq!(builder.function_state.variable_ctx.variable_map["me"], me);
                            assert_eq!(
                                builder.function_state.type_ctx.get_type(me),
                                Some(&MirType::Box("ParserBox".to_string()))
                            );
                            assert_eq!(
                                builder.function_state.type_ctx.value_origin_newbox.get(&me),
                                Some(&"ParserBox".to_string())
                            );
                            assert!(builder
                                .function_state
                                .variable_ctx
                                .variable_map
                                .contains_key("text"));
                            assert!(builder
                                .function_state
                                .variable_ctx
                                .variable_map
                                .contains_key("pos"));
                            assert!(builder
                                .function_state
                                .variable_ctx
                                .variable_map
                                .contains_key("ret"));
                            assert_eq!(builder.function_state.scope.lexical_scope_stack.len(), 1);

                            let call_count_before = builder
                                .current_function_instructions()
                                .iter()
                                .filter(|instruction| {
                                    matches!(instruction, MirInstruction::Call { .. })
                                })
                                .count();
                            let route = builder
                                .plan_member_call_route(&outer_receiver, &outer_method)
                                .expect("existing outer route plan");
                            assert!(
                                matches!(route, MemberCallRoutePlan::StaticReceiver { .. }),
                                "candidate fixture accepts only the existing StaticReceiver plan"
                            );

                            let mut port = PreloopLocatedArgumentPortV1::new(
                                RawLegacyChildLoweringPortV1,
                                prepared,
                            );
                            builder
                                .execute_prepared_member_call_route_v1(
                                    &mut port,
                                    &outer_input,
                                    route,
                                )
                                .expect("configured outer route");

                            let requested_destination = match port.selected_state() {
                                PreloopSelectedArgumentStateV1::Reached(reached) => {
                                    assert_eq!(reached.selected_index(), 1);
                                    assert_eq!(reached.selected_site(), &sites[0]);
                                    reached.requested_destination()
                                }
                                other => panic!("expected retained reached owner, got {other:?}"),
                            };

                            let instructions = builder.current_function_instructions();
                            let emitted = instructions
                                .iter()
                                .filter(|instruction| {
                                    matches!(instruction, MirInstruction::Call { .. })
                                })
                                .skip(call_count_before)
                                .collect::<Vec<_>>();
                            let inner_destinations = emitted
                                .iter()
                                .filter_map(|instruction| match instruction {
                                    MirInstruction::Call {
                                        dst: Some(dst),
                                        callee:
                                            Some(Callee::Method {
                                                box_name,
                                                method,
                                                receiver,
                                                certainty,
                                                ..
                                            }),
                                        ..
                                    } if box_name == "ParserBox"
                                        && method == "static_const_eval_pos"
                                        && receiver.is_some_and(|receiver| {
                                            ordinary_copy_root(&instructions, receiver) == me
                                        })
                                        && *certainty == TypeCertainty::Known =>
                                    {
                                        Some(*dst)
                                    }
                                    _ => None,
                                })
                                .collect::<Vec<_>>();
                            let outer_arguments = emitted
                                .iter()
                                .filter_map(|instruction| match instruction {
                                    MirInstruction::Call {
                                        callee: Some(Callee::Global(name)),
                                        args,
                                        ..
                                    } if name == "ParserStringUtilsBox.skip_ws/2" => {
                                        Some(args.as_slice())
                                    }
                                    _ => None,
                                })
                                .collect::<Vec<_>>();

                            assert_eq!(emitted.len(), 2, "selected inner and outer Call only");
                            assert_eq!(inner_destinations, [requested_destination]);
                            assert_eq!(outer_arguments.len(), 1);
                            assert!(
                                outer_arguments[0].iter().any(|argument| ordinary_copy_root(
                                    &instructions,
                                    *argument
                                )
                                    == requested_destination),
                                "outer terminal must consume the selected inner terminal value"
                            );
                            port.discard();
                            Ok((requested_destination, ()))
                        },
                    )
                    .expect("production-shaped configured outer route");
            },
        );
    });
}
