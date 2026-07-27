//! F3 proof for the exact located outer static completion.

use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use crate::mir::callable_result_representation::actual_parser_add_fixture;
use crate::mir::{Callee, MirBuilder, MirInstruction, MirModule, MirType, ValueId};

use super::preloop_located_outer_completion::{
    complete_preloop_located_outer_request_v1, PreloopLocatedOuterCompletionErrorV1,
    PreloopLocatedOuterCompletionStageV1, PreloopLocatedOuterObservedRouteV1,
};
use super::preloop_nested_result_test_support::with_prepared_located_outer;

fn configured_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.current_module = Some(MirModule::new("preloop-located-outer".to_owned()));
    builder
        .comp_ctx
        .install_callable_declaration_catalog(
            actual_parser_add_fixture::declaration_catalog_for_lowering(),
        )
        .expect("fixture lowering catalog");
    builder
}

fn call_destinations(builder: &MirBuilder) -> Vec<(String, ValueId)> {
    builder
        .current_function_instructions()
        .iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Call {
                dst: Some(dst),
                callee: Some(Callee::Global(symbol)),
                ..
            } => Some((symbol.clone(), *dst)),
            MirInstruction::Call {
                dst: Some(dst),
                callee: Some(Callee::Method { method, .. }),
                ..
            } => Some((method.clone(), *dst)),
            _ => None,
        })
        .collect()
}

#[test]
fn actual_parser_located_outer_uses_existing_static_completion() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_prepared_located_outer(|prepared, _selected_site| {
            let mut builder = configured_builder();
            builder
                .lower_instance_method_prefix_for_test(
                    "ParserBox",
                    actual_parser_add_fixture::method_declaration_for_lowering(),
                    3,
                    |builder, suffix| {
                        assert!(matches!(suffix.first(), Some(ASTNode::Assignment { .. })));
                        let text_value = builder.function_state.variable_ctx.variable_map["text"];
                        let completed = complete_preloop_located_outer_request_v1(
                            builder,
                            RawLegacyChildLoweringPortV1,
                            prepared,
                        )
                        .expect("exact located outer completion");
                        assert_ne!(
                            completed.inner_destination(),
                            completed.requested_destination()
                        );

                        let calls = call_destinations(builder);
                        assert!(calls.iter().any(|(symbol, destination)| {
                            symbol == "static_const_eval_pos"
                                && *destination == completed.inner_destination()
                        }));
                        assert!(calls.iter().any(|(symbol, destination)| {
                            symbol == "ParserStringUtilsBox.skip_ws/2"
                                && *destination == completed.requested_destination()
                        }));
                        let instructions = builder.current_function_instructions();
                        let outer_arguments = instructions
                            .iter()
                            .find_map(|instruction| match instruction {
                                MirInstruction::Call {
                                    callee: Some(Callee::Global(symbol)),
                                    args,
                                    ..
                                } if symbol == "ParserStringUtilsBox.skip_ws/2" => {
                                    Some(args.as_slice())
                                }
                                _ => None,
                            })
                            .expect("exact outer static Call");
                        assert!(instructions.iter().any(|instruction| matches!(
                            instruction,
                            MirInstruction::Copy { dst, src }
                                if *dst == outer_arguments[0] && *src == text_value
                        )));
                        assert!(instructions.iter().any(|instruction| matches!(
                            instruction,
                            MirInstruction::Copy { dst, src }
                                if *dst == outer_arguments[1]
                                    && *src == completed.inner_destination()
                        )));
                        assert_ne!(
                            builder
                                .function_state
                                .type_ctx
                                .get_type(completed.requested_destination()),
                            Some(&MirType::Integer),
                            "F3 does not publish the outer Integer fact"
                        );
                        completed.discard();
                        Ok((ValueId::new(0), ()))
                    },
                )
                .expect("configured candidate fixture");
        });
    });
}

#[test]
fn located_outer_rejects_alternate_route_before_argument_descent() {
    with_prepared_located_outer(|prepared, _| {
        let mut builder = configured_builder();
        builder
            .lower_instance_method_prefix_for_test(
                "ParserBox",
                actual_parser_add_fixture::method_declaration_for_lowering(),
                3,
                |builder, _suffix| {
                    builder
                        .function_state
                        .variable_ctx
                        .insert("ParserStringUtilsBox".to_owned(), ValueId::new(991));
                    let before = builder.current_function_instructions().len();
                    let rejected = complete_preloop_located_outer_request_v1(
                        builder,
                        RawLegacyChildLoweringPortV1,
                        prepared,
                    )
                    .expect_err("bound receiver must not use StaticReceiver");
                    assert_eq!(
                        rejected.stage(),
                        PreloopLocatedOuterCompletionStageV1::RouteSelection
                    );
                    assert_eq!(
                        rejected.cause(),
                        &PreloopLocatedOuterCompletionErrorV1::AlternateRoute(
                            PreloopLocatedOuterObservedRouteV1::Standard
                        )
                    );
                    assert_eq!(builder.current_function_instructions().len(), before);
                    rejected.discard();
                    Ok((ValueId::new(0), ()))
                },
            )
            .expect("alternate route fixture");
    });
}
