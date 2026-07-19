use crate::mir::builder::control_flow::plan::generic_loop::facts::extract::test_support::{
    with_default_and_strict_modes, GenericLoopTestModeV1,
};
use crate::mir::builder::control_flow::plan::generic_loop::located_representation::{
    VerifiedLocatedGenericLoopBodyRepresentationV1, VerifiedLocatedGenericLoopDirectPreflightV1,
    VerifiedLocatedGenericLoopLoweringModeV1,
};
use crate::mir::builder::control_flow::plan::LocatedLoopPlanExpressionPortV1;
use crate::mir::builder::MirBuilder;
use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, VerifiedCallableResultLegacySourceViewV1,
};

#[test]
fn actual_default_direct_preflight_is_builder_free_and_collects_value_target() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    with_default_and_strict_modes(|mode| {
        if !matches!(mode, GenericLoopTestModeV1::Default) {
            return;
        }
        let activation = actual_parser_add_fixture::plan();
        let caller = actual_parser_add_fixture::caller(&activation);
        let view = VerifiedCallableResultLegacySourceViewV1::verify(&activation, &caller)
            .expect("actual source view");
        let loop_root = view
            .body_stmt(&view.root_body(), 4)
            .expect("actual loop root");
        let port = LocatedLoopPlanExpressionPortV1::new(view);
        let representation =
            VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(&port, loop_root)
                .expect("default direct representation");
        let lowering = representation
            .bind_lowering_port(&port)
            .expect("bound direct lowering view");

        let mut builder = MirBuilder::new();
        let variable_map = builder.function_state.variable_ctx.variable_map.clone();
        let value_types = builder.function_state.type_ctx.value_types.clone();
        let origin = builder.function_state.type_ctx.value_origin_newbox.clone();
        let execution = VerifiedLocatedGenericLoopDirectPreflightV1::verify(&lowering)
            .expect("direct preflight");
        let (_lowering, targets) = execution.into_execution().into_components();

        assert_eq!(targets.as_ref(), ["value"]);
        assert_eq!(
            builder.function_state.variable_ctx.variable_map,
            variable_map
        );
        assert_eq!(builder.function_state.type_ctx.value_types, value_types);
        assert_eq!(builder.function_state.type_ctx.value_origin_newbox, origin);
    });
}

#[test]
fn strict_mode_rejects_direct_preflight_before_builder_effects() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    with_default_and_strict_modes(|mode| {
        if !matches!(mode, GenericLoopTestModeV1::StrictPlannerRequired) {
            return;
        }
        let activation = actual_parser_add_fixture::plan();
        let caller = actual_parser_add_fixture::caller(&activation);
        let view = VerifiedCallableResultLegacySourceViewV1::verify(&activation, &caller)
            .expect("actual source view");
        let loop_root = view
            .body_stmt(&view.root_body(), 4)
            .expect("actual loop root");
        let port = LocatedLoopPlanExpressionPortV1::new(view);
        let representation =
            VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(&port, loop_root)
                .expect("strict representation");
        let lowering = representation
            .bind_lowering_port(&port)
            .expect("bound strict lowering view");
        assert!(matches!(
            lowering.mode(),
            VerifiedLocatedGenericLoopLoweringModeV1::ExitAllowedRecipe { .. }
        ));
        let Err(error) = VerifiedLocatedGenericLoopDirectPreflightV1::verify(&lowering) else {
            panic!("strict mode is not DirectRecipeOnly");
        };
        assert!(matches!(
            error,
            super::LocatedGenericLoopDirectPreflightErrorV1::WrongLoweringMode
        ));
    });
}
