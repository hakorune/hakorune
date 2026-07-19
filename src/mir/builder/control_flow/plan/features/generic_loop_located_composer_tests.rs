use crate::mir::builder::control_flow::plan::generic_loop::facts::extract::test_support::with_default_and_strict_modes;
use crate::mir::builder::control_flow::plan::generic_loop::located_representation::VerifiedLocatedGenericLoopBodyRepresentationV1;
use crate::mir::builder::control_flow::plan::{
    visit_core_call_sources_v1, CoreCallSourceV1, LocatedLoopPlanExpressionPortV1,
};
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::builder::MirBuilder;
use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, VerifiedCallableResultLegacySourceViewV1,
};
use crate::mir::MirType;

use super::compose_located_generic_loop_v1;

#[test]
fn actual_strict_loop_composes_and_final_seals_in_one_call() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    with_default_and_strict_modes(|_mode| {
        let activation = actual_parser_add_fixture::plan();
        let caller = actual_parser_add_fixture::caller(&activation);
        let view = VerifiedCallableResultLegacySourceViewV1::verify(&activation, &caller)
            .expect("actual source view");
        let root_body = view.root_body();
        let loop_root = view
            .body_stmt(&root_body, 4)
            .expect("actual Loop remains Body(4)");
        let port = LocatedLoopPlanExpressionPortV1::new(view);
        let representation =
            VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(&port, loop_root)
                .expect("actual O0 representation");

        let mut builder = seeded_builder();
        let variable_map_before = builder.variable_ctx.variable_map.clone();
        let value_types_before = builder.type_ctx.value_types.clone();
        let _scope = LexicalScopeGuard::new(&mut builder);
        let result = compose_located_generic_loop_v1(
            &mut builder,
            representation,
            &port,
            &activation,
            &caller,
        );

        let located = result.expect("both located loop modes compose");
        assert!(located.plan_is_loop());
        let schedule = located
            .schedule()
            .sites_in_source_order()
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(schedule.len(), 9);
        let mut traversal = Vec::new();
        visit_core_call_sources_v1(located.plan_for_tests(), &mut |source| {
            if let CoreCallSourceV1::LocatedMethodCall(site) = source {
                traversal.push(
                    schedule
                        .iter()
                        .position(|candidate| candidate == site)
                        .expect("every plan site belongs to the final schedule"),
                );
            }
        });
        assert_eq!(traversal, vec![3, 4, 5, 6, 8, 7, 0, 1, 2]);
        assert_eq!(builder.variable_ctx.variable_map, variable_map_before);
        assert_ne!(builder.type_ctx.value_types, value_types_before);
    });
}

fn seeded_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("located_generic_loop_l0/0".to_string());
    seed(&mut builder, "text", MirType::String);
    seed(&mut builder, "pos", MirType::Integer);
    seed(&mut builder, "value", MirType::Integer);
    seed(&mut builder, "me", MirType::Box("ParserBox".to_string()));
    seed(
        &mut builder,
        "ParserStringUtilsBox",
        MirType::Box("ParserStringUtilsBox".to_string()),
    );
    builder
}

fn seed(builder: &mut MirBuilder, name: &str, ty: MirType) {
    let value = builder.alloc_typed(ty);
    builder
        .variable_ctx
        .variable_map
        .insert(name.to_string(), value);
}
