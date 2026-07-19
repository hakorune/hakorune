//! HOOK0 disconnected proof for the Builder-free located Parts preflight.

use std::collections::BTreeMap;

use crate::mir::builder::control_flow::plan::generic_loop::facts::extract::test_support::{
    with_default_and_strict_modes, GenericLoopTestModeV1,
};
use crate::mir::builder::control_flow::plan::generic_loop::located_representation::VerifiedLocatedGenericLoopBodyRepresentationV1;
use crate::mir::builder::control_flow::plan::LocatedLoopPlanExpressionPortV1;
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::builder::MirBuilder;
use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, VerifiedCallableResultActivationPlanV1,
    VerifiedCallableResultLegacySourceViewV1,
};
use crate::mir::MirType;

use super::located_lowering::lower_preflighted_located_parts_root_v1;
use super::located_preflight::{
    LocatedPartsPreflightErrorV1, VerifiedLocatedGenericLoopPartsPreflightV1,
};

#[test]
fn actual_strict_root_seals_before_any_builder_exists() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    with_default_and_strict_modes(|mode| {
        let plan = actual_parser_add_fixture::plan();
        let (port, loop_root) = located_loop(&plan);
        let representation =
            VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(&port, loop_root)
                .expect("actual located representation");
        let lowering = representation
            .bind_lowering_port(&port)
            .expect("exact located port");

        let result = VerifiedLocatedGenericLoopPartsPreflightV1::verify(&lowering);
        match mode {
            GenericLoopTestModeV1::Default => assert!(matches!(
                result,
                Err(LocatedPartsPreflightErrorV1::WrongLoweringMode)
            )),
            GenericLoopTestModeV1::StrictPlannerRequired => {
                result.expect("actual strict root must seal");
            }
        }
    });
}

#[test]
fn actual_strict_root_reaches_the_disconnected_located_adapter() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    with_default_and_strict_modes(|mode| {
        if mode != GenericLoopTestModeV1::StrictPlannerRequired {
            return;
        }
        let plan = actual_parser_add_fixture::plan();
        let (port, loop_root) = located_loop(&plan);
        let representation =
            VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(&port, loop_root)
                .expect("actual located representation");
        let lowering = representation
            .bind_lowering_port(&port)
            .expect("exact located port");
        let preflight = VerifiedLocatedGenericLoopPartsPreflightV1::verify(&lowering)
            .expect("strict carrier profile");

        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("located_parts_actual/0".to_string());
        let _scope = LexicalScopeGuard::new(&mut builder);
        let mut bindings = BTreeMap::new();
        seed(&mut builder, &mut bindings, "text", MirType::String);
        seed(&mut builder, &mut bindings, "pos", MirType::Integer);
        seed(&mut builder, &mut bindings, "value", MirType::Integer);
        seed(
            &mut builder,
            &mut bindings,
            "me",
            MirType::Box("ParserBox".to_string()),
        );
        let empty = BTreeMap::new();

        let plans = lower_preflighted_located_parts_root_v1(
            preflight,
            &mut builder,
            &mut bindings,
            &empty,
            &empty,
            "located_parts_actual",
        )
        .expect("disconnected located adapter lowers the strict root");

        assert!(!plans.is_empty());
        for name in ["op", "rhs", "rv", "value"] {
            assert!(bindings.contains_key(name), "missing binding: {name}");
        }
    });
}

fn located_loop<'plan>(
    plan: &'plan VerifiedCallableResultActivationPlanV1,
) -> (
    LocatedLoopPlanExpressionPortV1<'plan>,
    crate::mir::callable_result_representation::LegacyStmtInputV1<'plan>,
) {
    let caller = actual_parser_add_fixture::caller(plan);
    let view =
        VerifiedCallableResultLegacySourceViewV1::verify(plan, &caller).expect("source view");
    let root = view.root_body();
    let loop_root = view
        .body_stmt(&root, 4)
        .expect("actual Loop is function Body(4)");
    (LocatedLoopPlanExpressionPortV1::new(view), loop_root)
}

fn seed(
    builder: &mut MirBuilder,
    bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    name: &str,
    ty: MirType,
) {
    let value = builder.alloc_typed(ty);
    builder
        .variable_ctx
        .variable_map
        .insert(name.to_string(), value);
    bindings.insert(name.to_string(), value);
}
