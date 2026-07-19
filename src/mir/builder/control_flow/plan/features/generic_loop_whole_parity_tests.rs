//! P0 proof for the complete raw/located GenericLoopV1 composer.
//!
//! Each process mode compares only raw against located. Default and strict
//! modes are independent semantic profiles; they are never compared with one
//! another. The typed plan snapshot erases only call-source provenance.

use std::collections::BTreeMap;

use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::generic_loop::facts::extract::test_support::{
    with_default_and_strict_modes, GenericLoopTestModeV1,
};
use crate::mir::builder::control_flow::plan::parity_snapshot_test_support::{
    collect_call_sources, normalized_semantic_plans,
};
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::plan::LoopPlanExpressionPortV1;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan};
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::builder::MirBuilder;
use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, VerifiedCallableResultActivationPlanV1,
    VerifiedCallableResultLegacySourceViewV1, VerifiedCallableResultLoopClaimScheduleV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::value_kind::MirValueKind;
use crate::mir::{MirModule, MirType, ValueId};

use super::generic_loop_located_composer::compose_located_generic_loop_v1;
use crate::mir::builder::control_flow::plan::generic_loop::located_representation::VerifiedLocatedGenericLoopBodyRepresentationV1;
use crate::mir::builder::control_flow::plan::LocatedLoopPlanExpressionPortV1;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct WholeLoopBuilderSnapshotV1 {
    variable_map: BTreeMap<String, ValueId>,
    value_types: BTreeMap<ValueId, MirType>,
    value_kinds: Vec<(ValueId, MirValueKind)>,
    value_origins: BTreeMap<ValueId, String>,
    string_literals: BTreeMap<ValueId, String>,
}

pub(super) struct WholeLoopRunV1 {
    pub(super) plan: CorePlan,
    pub(super) builder: WholeLoopBuilderSnapshotV1,
    pub(super) call_sources: Vec<crate::mir::builder::control_flow::plan::CoreCallSourceV1>,
    pub(super) schedule: Vec<SourceExprSiteV1>,
}

#[test]
fn actual_default_and_strict_raw_and_located_whole_loops_match_after_source_only_normalization() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    with_default_and_strict_modes(|mode| {
        let activation = actual_parser_add_fixture::plan();
        let caller = actual_parser_add_fixture::caller(&activation);
        let (port, loop_root) = located_loop(&activation, &caller);

        let raw = run_raw(mode, &port, &loop_root);
        let located = run_located(&activation, &caller, port, loop_root);

        assert_eq!(
            normalized_semantic_plans(std::slice::from_ref(&raw.plan)),
            normalized_semantic_plans(std::slice::from_ref(&located.plan)),
            "raw and located plans differ beyond call-source provenance"
        );
        assert_eq!(raw.builder, located.builder);
        assert_call_source_and_schedule(&activation, &caller, &raw, &located);
        assert_mode_golden(mode, &raw.plan);
    });
}

#[test]
fn actual_method_prefix_uses_canonical_parameters_and_keeps_one_live_scope_for_loop_handoff() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let mut builder = MirBuilder::new();
    builder.current_module = Some(MirModule::new("raw-prefix-harness".to_string()));
    builder
        .comp_ctx
        .install_callable_declaration_catalog(
            actual_parser_add_fixture::declaration_catalog_for_lowering(),
        )
        .expect("actual raw-prefix callable catalog");

    let observed = builder
        .lower_instance_method_prefix_for_test(
            "ParserBox",
            actual_parser_add_fixture::method_declaration_for_lowering(),
            4,
            |builder, suffix| {
                let function = builder
                    .scope_ctx
                    .current_function
                    .as_ref()
                    .expect("canonical method skeleton");
                assert_eq!(
                    function.signature.name,
                    "ParserBox.static_const_parse_add/2"
                );
                assert_eq!(function.params.len(), 3);
                for index in 0..3 {
                    let value = function.params[index];
                    assert_eq!(
                        builder.get_value_kind(value),
                        Some(MirValueKind::Parameter(index as u32))
                    );
                }
                assert_eq!(builder.variable_ctx.variable_map["me"], function.params[0]);
                assert_eq!(
                    builder.variable_ctx.variable_map["text"],
                    function.params[1]
                );
                assert_ne!(builder.variable_ctx.variable_map["pos"], function.params[2]);
                assert_eq!(
                    builder
                        .type_ctx
                        .value_origin_newbox
                        .get(&function.params[0]),
                    Some(&"ParserBox".to_string())
                );
                assert!(builder.variable_ctx.variable_map.contains_key("ret"));
                assert!(builder.variable_ctx.variable_map.contains_key("value"));
                assert!(!builder
                    .variable_ctx
                    .variable_map
                    .contains_key("ParserStringUtilsBox"));
                assert_eq!(builder.scope_ctx.lexical_scope_stack.len(), 1);
                assert_eq!(suffix.len(), 2);
                assert!(matches!(suffix[0], crate::ast::ASTNode::Loop { .. }));
                assert!(matches!(suffix[1], crate::ast::ASTNode::Return { .. }));

                let value = builder.variable_ctx.variable_map["value"];
                Ok((
                    value,
                    (
                        function.params.clone(),
                        value,
                        builder.scope_ctx.lexical_scope_stack.len(),
                    ),
                ))
            },
        )
        .expect("canonical method entry and raw prefix");

    assert_eq!(observed.0.len(), 3);
    assert_eq!(observed.2, 1);
    assert!(builder.scope_ctx.current_function.is_none());
    assert!(builder.current_module.as_ref().is_some_and(|module| {
        module
            .get_function("ParserBox.static_const_parse_add/2")
            .is_some()
    }));
}

pub(super) fn run_raw(
    mode: GenericLoopTestModeV1,
    port: &LocatedLoopPlanExpressionPortV1<'_>,
    loop_root: &crate::mir::callable_result_representation::LegacyStmtInputV1<'_>,
) -> WholeLoopRunV1 {
    let input = port.borrowed_stmt(&loop_root);
    let crate::ast::ASTNode::Loop {
        condition, body, ..
    } = port.stmt_syntax(&input)
    else {
        panic!("actual fixture root must be a Loop")
    };
    let ctx = LoopRouteContext::new(&condition, &body, "whole_loop_p0/0", false, false);
    let outcome = crate::mir::builder::control_flow::plan::single_planner::try_build_outcome(&ctx)
        .expect("raw GenericLoop extraction");
    let facts = outcome.facts.expect("raw GenericLoop facts");
    assert!(facts.facts.generic_loop_v1().is_some());
    let mut builder = seeded_builder();
    let plan = {
        let _scope = LexicalScopeGuard::new(&mut builder);
        RecipeComposer::compose_generic_loop_v1_recipe(&mut builder, &facts, &ctx)
            .expect("raw GenericLoop composition")
    };
    let _ = mode;
    WholeLoopRunV1 {
        call_sources: collect_call_sources(std::slice::from_ref(&plan)),
        plan,
        builder: snapshot_builder(&builder),
        schedule: Vec::new(),
    }
}

pub(super) fn run_located(
    activation: &VerifiedCallableResultActivationPlanV1,
    caller: &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    port: LocatedLoopPlanExpressionPortV1<'_>,
    loop_root: crate::mir::callable_result_representation::LegacyStmtInputV1<'_>,
) -> WholeLoopRunV1 {
    let representation =
        VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(&port, loop_root)
            .expect("located GenericLoop representation");
    let mut builder = seeded_builder();
    let (plan, schedule) = {
        let _scope = LexicalScopeGuard::new(&mut builder);
        let located = compose_located_generic_loop_v1(
            &mut builder,
            representation,
            &port,
            activation,
            caller,
        )
        .expect("located GenericLoop composition");
        let schedule = located
            .schedule()
            .sites_in_source_order()
            .cloned()
            .collect::<Vec<_>>();
        (located.plan_for_tests().clone(), schedule)
    };
    WholeLoopRunV1 {
        call_sources: collect_call_sources(std::slice::from_ref(&plan)),
        plan,
        builder: snapshot_builder(&builder),
        schedule,
    }
}

pub(super) fn located_loop<'plan>(
    plan: &'plan VerifiedCallableResultActivationPlanV1,
    caller: &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
) -> (
    LocatedLoopPlanExpressionPortV1<'plan>,
    crate::mir::callable_result_representation::LegacyStmtInputV1<'plan>,
) {
    let view =
        VerifiedCallableResultLegacySourceViewV1::verify(plan, caller).expect("actual source view");
    let root = view.root_body();
    let loop_root = view.body_stmt(&root, 4).expect("actual Loop Body(4)");
    (LocatedLoopPlanExpressionPortV1::new(view), loop_root)
}

fn seeded_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("whole_loop_p0/0".to_string());
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

fn snapshot_builder(builder: &MirBuilder) -> WholeLoopBuilderSnapshotV1 {
    WholeLoopBuilderSnapshotV1 {
        variable_map: builder.variable_ctx.variable_map.clone(),
        value_types: builder.type_ctx.value_types.clone(),
        value_kinds: {
            let mut values = builder
                .type_ctx
                .value_kinds
                .iter()
                .map(|(value, kind)| (*value, *kind))
                .collect::<Vec<_>>();
            values.sort_by_key(|(value, _)| *value);
            values
        },
        value_origins: builder.type_ctx.value_origin_newbox.clone(),
        string_literals: builder.type_ctx.string_literals.clone(),
    }
}

fn assert_call_source_and_schedule(
    activation: &VerifiedCallableResultActivationPlanV1,
    caller: &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    raw: &WholeLoopRunV1,
    located: &WholeLoopRunV1,
) {
    assert_eq!(raw.call_sources.len(), 9);
    assert!(raw.call_sources.iter().all(|source| matches!(
        source,
        crate::mir::builder::control_flow::plan::CoreCallSourceV1::Unlocated
    )));
    let (_, loop_root) = located_loop(activation, caller);
    let expected_schedule =
        VerifiedCallableResultLoopClaimScheduleV1::verify(activation, caller, loop_root)
            .expect("actual located loop schedule");
    let expected = expected_schedule
        .sites_in_source_order()
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(located.schedule.len(), 9);
    assert_eq!(expected, located.schedule);
    let traversal = located
        .call_sources
        .iter()
        .map(|source| {
            let site = match source {
                crate::mir::builder::control_flow::plan::CoreCallSourceV1::LocatedMethodCall(
                    site,
                ) => site,
                crate::mir::builder::control_flow::plan::CoreCallSourceV1::Unlocated => {
                    unreachable!()
                }
            };
            located
                .schedule
                .iter()
                .position(|candidate| candidate == site)
                .unwrap()
        })
        .collect::<Vec<_>>();
    assert_eq!(traversal, vec![3, 4, 5, 6, 8, 7, 0, 1, 2]);
}

fn assert_mode_golden(mode: GenericLoopTestModeV1, plan: &CorePlan) {
    let (selects, joins) = plan_shape(plan);
    match mode {
        GenericLoopTestModeV1::Default => assert!(selects > 0, "default must retain Select"),
        GenericLoopTestModeV1::StrictPlannerRequired => {
            assert!(joins > 0, "strict must retain Join-bearing If")
        }
    }
    assert!(matches!(plan, CorePlan::Loop(_)));
}

fn plan_shape(plan: &CorePlan) -> (usize, usize) {
    match plan {
        CorePlan::Seq(children) => children.iter().map(plan_shape).fold((0, 0), add_shape),
        CorePlan::Loop(loop_plan) => loop_plan
            .body
            .iter()
            .map(plan_shape)
            .chain(
                loop_plan
                    .block_effects
                    .iter()
                    .flat_map(|(_, effects)| effects.iter().map(effect_shape)),
            )
            .fold((0, 0), add_shape),
        CorePlan::If(if_plan) => {
            let mut shape = if_plan
                .then_plans
                .iter()
                .map(plan_shape)
                .fold((0, 1usize), add_shape);
            if let Some(else_plans) = &if_plan.else_plans {
                shape = else_plans.iter().map(plan_shape).fold(shape, add_shape);
            }
            shape
        }
        CorePlan::BranchN(branch) => branch
            .arms
            .iter()
            .flat_map(|arm| arm.plans.iter().map(plan_shape))
            .fold((0, 0), add_shape),
        CorePlan::Effect(effect) => effect_shape(effect),
        CorePlan::Exit(_) => (0, 0),
    }
}

fn effect_shape(effect: &CoreEffectPlan) -> (usize, usize) {
    match effect {
        CoreEffectPlan::Select { .. } => (1, 0),
        CoreEffectPlan::IfEffect {
            then_effects,
            else_effects,
            ..
        } => then_effects
            .iter()
            .chain(else_effects.iter().flat_map(|effects| effects.iter()))
            .map(effect_shape)
            .fold((0, 0), add_shape),
        _ => (0, 0),
    }
}

fn add_shape(left: (usize, usize), right: (usize, usize)) -> (usize, usize) {
    (left.0 + right.0, left.1 + right.1)
}
