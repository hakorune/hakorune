//! M4-D2-A: test-only reachability evidence for accepted Generic plans.
//!
//! This corpus observes the existing facts/selector/composer/verifier/lowerer
//! path on fresh builders.  It is deliberately not a policy oracle and never
//! changes the production scheduler or creates a Recipe/PHI consumer.

use super::generic_selection_matrix_tests::{
    both_body, neither_body, progression_condition, simple_while_body, v1_only_body,
};
use super::route_id::LoopRouteId;
use super::select_recipe_first_routes;
use crate::mir::builder::control_flow::joinir::route_entry::router::{
    lower_verified_core_plan, LoopRouteContext,
};
use crate::mir::builder::control_flow::lower::PlanLowerer;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::plan::single_planner::try_build_outcome;
use crate::mir::builder::control_flow::plan::CorePlan;
use crate::mir::builder::control_flow::verify::observability::flowbox_tags::FlowboxVia;
use crate::mir::builder::control_flow::verify::PlanVerifier;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CorpusModeV1 {
    Release,
    Strict,
    StrictPlannerRequired,
}

impl CorpusModeV1 {
    fn env(self) -> (&'static str, Option<&'static str>) {
        match self {
            Self::Release => ("HAKO_JOINIR_STRICT", None),
            Self::Strict => ("HAKO_JOINIR_STRICT", Some("1")),
            Self::StrictPlannerRequired => ("HAKO_JOINIR_STRICT", Some("1")),
        }
    }

    fn planner_required(self) -> Option<&'static str> {
        match self {
            Self::StrictPlannerRequired => Some("1"),
            Self::Release | Self::Strict => None,
        }
    }

    fn strict_or_dev(self) -> bool {
        !matches!(self, Self::Release)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlanStageV1 {
    ComposerError,
    NonLoopRoot,
    VerifierRejected,
    LowerSome,
    LowerNone,
    LowerError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EffectOwnerV1 {
    None,
    GenericComposer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CandidateSnapshotV1 {
    current_block: Option<BasicBlockId>,
    block_count: usize,
    next_value_id: Option<u32>,
    variable_count: usize,
    typed_value_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReachabilityRowV1 {
    mode: CorpusModeV1,
    route: LoopRouteId,
    root_is_loop: bool,
    stage: PlanStageV1,
    first_effect_owner: EffectOwnerV1,
    before_lower: CandidateSnapshotV1,
    after_lower: CandidateSnapshotV1,
}

fn seeded_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("generic_reachability/0".to_string());
    for (name, ty) in [("i", MirType::Integer), ("j", MirType::Integer)] {
        let value = builder.alloc_typed(ty);
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert(name.to_string(), value);
    }
    builder
}

fn snapshot(builder: &MirBuilder) -> CandidateSnapshotV1 {
    CandidateSnapshotV1 {
        current_block: builder.function_state.current_block,
        block_count: builder
            .function_state
            .current_function
            .as_ref()
            .map(|function| function.blocks.len())
            .unwrap_or_default(),
        next_value_id: builder
            .function_state
            .current_function
            .as_ref()
            .map(|function| function.next_value_id),
        variable_count: builder.function_state.variable_ctx.variable_map.len(),
        typed_value_count: builder.function_state.type_ctx.value_types.len(),
    }
}

fn fixture(name: &str) -> (crate::ast::ASTNode, Vec<crate::ast::ASTNode>) {
    match name {
        "v1-only" => (progression_condition(), v1_only_body()),
        "both" => (progression_condition(), both_body()),
        "simple-while" => (progression_condition(), simple_while_body()),
        "neither" => (progression_condition(), neither_body()),
        _ => panic!("unknown Generic reachability fixture: {name}"),
    }
}

fn observe_row(
    mode: CorpusModeV1,
    route: LoopRouteId,
    facts: &crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts,
    ctx: &LoopRouteContext<'_>,
) -> ReachabilityRowV1 {
    let mut builder = seeded_builder();
    let before_compose = snapshot(&builder);
    let composed = match route {
        LoopRouteId::GenericLoopV0 => {
            RecipeComposer::compose_generic_loop_v0_recipe(&mut builder, facts, ctx)
        }
        LoopRouteId::GenericLoopV1 => {
            RecipeComposer::compose_generic_loop_v1_recipe(&mut builder, facts, ctx)
        }
        other => panic!("unexpected non-Generic route in reachability corpus: {other:?}"),
    };
    let Ok(plan) = composed else {
        let after_compose = snapshot(&builder);
        return ReachabilityRowV1 {
            mode,
            route,
            root_is_loop: false,
            stage: PlanStageV1::ComposerError,
            first_effect_owner: if before_compose != after_compose {
                EffectOwnerV1::GenericComposer
            } else {
                EffectOwnerV1::None
            },
            before_lower: after_compose.clone(),
            after_lower: after_compose,
        };
    };
    let root_is_loop = matches!(&plan, CorePlan::Loop(_));
    let before_lower = snapshot(&builder);
    let first_effect_owner = if before_compose != before_lower {
        EffectOwnerV1::GenericComposer
    } else {
        EffectOwnerV1::None
    };
    if !root_is_loop {
        return ReachabilityRowV1 {
            mode,
            route,
            root_is_loop,
            stage: PlanStageV1::NonLoopRoot,
            first_effect_owner,
            before_lower: before_lower.clone(),
            after_lower: before_lower,
        };
    }
    if PlanVerifier::verify(&plan).is_err() {
        return ReachabilityRowV1 {
            mode,
            route,
            root_is_loop,
            stage: PlanStageV1::VerifierRejected,
            first_effect_owner,
            before_lower: before_lower.clone(),
            after_lower: before_lower,
        };
    }
    let lower_result = if mode.strict_or_dev() {
        lower_verified_core_plan(
            &mut builder,
            ctx,
            true,
            Some(facts),
            plan,
            FlowboxVia::Shadow,
        )
    } else {
        PlanLowerer::lower(&mut builder, plan, ctx)
    };
    let stage = match lower_result {
        Ok(Some(_value)) => PlanStageV1::LowerSome,
        Ok(None) => PlanStageV1::LowerNone,
        Err(_error) => PlanStageV1::LowerError,
    };
    ReachabilityRowV1 {
        mode,
        route,
        root_is_loop,
        stage,
        first_effect_owner,
        before_lower,
        after_lower: snapshot(&builder),
    }
}

fn observe_fixture(mode: CorpusModeV1, name: &str) -> Vec<ReachabilityRowV1> {
    let (condition, body) = fixture(name);
    let ctx = LoopRouteContext::new(&condition, &body, "generic_reachability/0", false, false);
    let outcome = match try_build_outcome(&ctx) {
        Ok(outcome) => outcome,
        Err(_error) => return Vec::new(),
    };
    let Some(facts) = outcome.facts.as_ref() else {
        return Vec::new();
    };
    select_recipe_first_routes(Some(facts))
        .raw_execution_routes()
        .iter()
        .copied()
        .filter(|route| {
            matches!(
                route,
                LoopRouteId::GenericLoopV0 | LoopRouteId::GenericLoopV1
            )
        })
        .map(|route| observe_row(mode, route, facts, &ctx))
        .collect()
}

#[test]
fn generic_accepted_plan_reachability_corpus_is_test_only_and_repeatable() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let mut accepted = 0usize;
    for mode in [
        CorpusModeV1::Release,
        CorpusModeV1::Strict,
        CorpusModeV1::StrictPlannerRequired,
    ] {
        let _config = crate::test_support::ScopedTestConfig::apply(&[
            ("HAKO_JOINIR_STRICT", mode.env().1),
            ("HAKO_JOINIR_PLANNER_REQUIRED", mode.planner_required()),
            ("NYASH_JOINIR_STRICT", None),
        ]);
        for name in ["v1-only", "both", "simple-while", "neither"] {
            let rows = observe_fixture(mode, name);
            for row in rows {
                assert!(
                    row.root_is_loop,
                    "Generic composer root must be Loop: {row:?}"
                );
                assert!(
                    row.first_effect_owner == EffectOwnerV1::GenericComposer,
                    "accepted Generic composition must leave candidate evidence: {row:?}"
                );
                let repeat = observe_fixture(mode, name)
                    .into_iter()
                    .find(|candidate| candidate.route == row.route)
                    .expect("repeat fixture must retain selected Generic route");
                assert_eq!(
                    row.stage, repeat.stage,
                    "fresh candidate stage drift: {row:?}"
                );
                assert_eq!(row.before_lower, repeat.before_lower);
                assert_eq!(row.after_lower, repeat.after_lower);
                if row.stage == PlanStageV1::LowerSome {
                    accepted += 1;
                }
            }
        }
    }
    assert!(
        accepted >= 3,
        "known Generic corpus must reach lower success in at least three rows"
    );
}
