//! Configured proof for the bounded pre-loop located ingress.
//!
//! The fixture owns and discards its whole Builder. Emitted Calls are proof
//! effects only: this row issues no typed physical receipt, final-destination
//! authority, nested-result receipt, or nested Integer publication.

use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use crate::mir::callable_result_representation::actual_parser_add_fixture;
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::source_call_target::{
    VerifiedRawCallableSourceViewV1, VerifiedSourceMethodCallSiteV1,
};
use crate::mir::source_instance_result_contract::{
    prepare_preloop_located_argument_v1, prepare_preloop_nested_result_association_v1,
    seal_nested_instance_result_contract, VerifiedCurrentOwnerInstanceResultTargetV1,
};
use crate::mir::{Callee, MirBuilder, MirInstruction};

use super::member_route::MemberCallRoutePlan;
use super::method_call_descent::RawLegacyMethodCallInputV1;
use super::preloop_located_argument_port::{
    PreloopLocatedArgumentPortV1, PreloopSelectedArgumentStateV1,
};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

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
                builder
                    .comp_ctx
                    .install_callable_declaration_catalog(
                        actual_parser_add_fixture::declaration_catalog_for_lowering(),
                    )
                    .expect("fixture lowering catalog");
                builder.enter_function_for_test("ParserBox.static_const_parse_add/2".to_string());

                let text = builder.build_expression(integer(11)).expect("text value");
                let pos = builder.build_expression(integer(12)).expect("pos value");
                let ret = builder.build_expression(integer(13)).expect("ret value");
                let me = builder.build_expression(integer(14)).expect("me value");
                builder.bind_function_parameter_for_test("text", text);
                builder.bind_function_parameter_for_test("pos", pos);
                builder.bind_variable_for_test("ret", ret);
                builder.bind_variable_for_test("me", me);

                let route = builder
                    .plan_member_call_route(&outer_receiver, &outer_method)
                    .expect("existing outer route plan");
                assert!(
                    matches!(route, MemberCallRoutePlan::StaticReceiver { .. }),
                    "candidate fixture accepts only the existing StaticReceiver plan"
                );

                let mut port =
                    PreloopLocatedArgumentPortV1::new(RawLegacyChildLoweringPortV1, prepared);
                builder
                    .execute_prepared_member_call_route_v1(&mut port, &outer_input, route)
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
                let inner_destinations = instructions
                    .iter()
                    .filter_map(|instruction| match instruction {
                        MirInstruction::Call {
                            dst: Some(dst),
                            callee: Some(Callee::Method { method, .. }),
                            ..
                        } if method == "static_const_eval_pos" => Some(*dst),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                let outer_arguments = instructions
                    .iter()
                    .filter_map(|instruction| match instruction {
                        MirInstruction::Call {
                            callee: Some(Callee::Global(name)),
                            args,
                            ..
                        } if name == "ParserStringUtilsBox.skip_ws/2" => Some(args.as_slice()),
                        _ => None,
                    })
                    .collect::<Vec<_>>();

                assert_eq!(inner_destinations, [requested_destination]);
                assert_eq!(outer_arguments.len(), 1);
                assert!(
                    outer_arguments[0]
                        .iter()
                        .any(|argument| ordinary_copy_root(&instructions, *argument)
                            == requested_destination),
                    "outer terminal must consume the selected inner terminal value"
                );
                port.discard();
            },
        );
    });
}
