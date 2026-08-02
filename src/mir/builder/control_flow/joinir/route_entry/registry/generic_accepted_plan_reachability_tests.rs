//! M4-D2-A: test-only reachability evidence for accepted Generic plans.
//!
//! This corpus observes the existing facts/selector/composer/verifier/lowerer
//! path on fresh builders.  It is deliberately not a policy oracle and never
//! changes the production scheduler or creates a Recipe/PHI consumer.

use super::generic_selection_matrix_tests::{
    additive_body, additive_condition, both_body, effect_without_local_body, neither_body,
    progression_condition, simple_while_body, true_body, true_condition, v1_only_body,
    v1_only_effect_body,
};
use super::generic_semantic_digest_tests::{core_plan_semantic_digest, CorePlanSemanticDigestV1};
use super::route_id::LoopRouteId;
use super::select_recipe_first_routes;
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::control_flow::joinir::route_entry::router::{
    lower_verified_core_plan, LoopRouteContext,
};
use crate::mir::builder::control_flow::lower::PlanLowerer;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::plan::single_planner::try_build_outcome;
use crate::mir::builder::control_flow::plan::{CoreLoopPlan, CorePlan};
use crate::mir::builder::control_flow::verify::observability::flowbox_tags::FlowboxVia;
use crate::mir::builder::control_flow::verify::PlanVerifier;
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirType};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CorpusModeV1 {
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
pub(super) enum PlanStageV1 {
    ComposerError,
    NonLoopRoot,
    VerifierRejected,
    LowerSome,
    LowerNone,
    LowerError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EffectOwnerV1 {
    None,
    GenericComposer,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct GenericDirectStageEvidenceV1 {
    pub(super) route: LoopRouteId,
    pub(super) stage: PlanStageV1,
    pub(super) first_effect_owner: EffectOwnerV1,
    pub(super) before_compose: CandidateSnapshotV1,
    pub(super) before_lower: CandidateSnapshotV1,
    pub(super) after_lower: CandidateSnapshotV1,
    pub(super) semantic_digest: Option<CorePlanSemanticDigestV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CandidateSnapshotV1 {
    pub(super) current_block: Option<BasicBlockId>,
    pub(super) block_count: usize,
    pub(super) next_value_id: Option<u32>,
    pub(super) variable_count: usize,
    pub(super) typed_value_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct ReachabilityRowV1 {
    mode: CorpusModeV1,
    route: LoopRouteId,
    root_is_loop: bool,
    stage: PlanStageV1,
    first_effect_owner: EffectOwnerV1,
    before_compose: CandidateSnapshotV1,
    before_lower: CandidateSnapshotV1,
    after_lower: CandidateSnapshotV1,
    semantic_digest: Option<CorePlanSemanticDigestV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NestedCarrierSemanticEvidenceV1 {
    route: LoopRouteId,
    outer_final_value_names: Vec<String>,
    outer_phi_tags: Vec<String>,
    nested_final_value_names: Vec<String>,
}

/// Test-only projection of extraction facts; it is not a route or policy authority.
#[derive(Debug, Clone, PartialEq, Eq)]
enum GenericCarrierObservationDispositionV1 {
    CompleteNoRecursiveCarrier,
    CompleteRecursiveCarrier(Vec<String>),
    Unavailable(&'static str),
    Ambiguous(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GenericCarrierObservationV1 {
    disposition: GenericCarrierObservationDispositionV1,
}

// Fixture-scoped probe for D2-B2b. Before promotion, the production extractor
// must cover every accepted container or return typed unavailable/ambiguous;
// silently treating an unsupported container as carrier-free is forbidden.
fn collect_recursive_carrier_targets(
    body: &[ASTNode],
    loop_var: &str,
    nested: bool,
    targets: &mut BTreeSet<String>,
) -> Result<(), &'static str> {
    for stmt in body {
        match stmt {
            ASTNode::Assignment { target, .. } if nested => {
                if let ASTNode::Variable { name, .. } = target.as_ref() {
                    if name != loop_var {
                        targets.insert(name.clone());
                    }
                }
            }
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                collect_recursive_carrier_targets(then_body, loop_var, true, targets)?;
                if let Some(else_body) = else_body {
                    collect_recursive_carrier_targets(else_body, loop_var, true, targets)?;
                }
            }
            ASTNode::Loop { body, .. } => {
                collect_recursive_carrier_targets(body, loop_var, true, targets)?;
            }
            ASTNode::ScopeBox { body, .. } => {
                collect_recursive_carrier_targets(body, loop_var, nested, targets)?;
            }
            ASTNode::Program { statements, .. } => {
                collect_recursive_carrier_targets(statements, loop_var, nested, targets)?;
            }
            ASTNode::LoopRange { .. } => return Err("LoopRange"),
            ASTNode::Lambda { .. } => return Err("Lambda"),
            ASTNode::BlockExpr { .. } => return Err("BlockExpr"),
            ASTNode::TryCatch { .. } => return Err("TryCatch"),
            ASTNode::TaskScope { .. } => return Err("TaskScope"),
            ASTNode::ContextScope { .. } => return Err("ContextScope"),
            ASTNode::FastMemRegion { .. } => return Err("FastMemRegion"),
            ASTNode::BuildGate { .. } => return Err("BuildGate"),
            _ => {}
        }
    }
    Ok(())
}

fn observe_carrier_body(body: &[ASTNode], loop_var: &str) -> GenericCarrierObservationV1 {
    let mut targets = BTreeSet::new();
    let disposition = match collect_recursive_carrier_targets(body, loop_var, false, &mut targets) {
        Ok(()) if targets.is_empty() => {
            GenericCarrierObservationDispositionV1::CompleteNoRecursiveCarrier
        }
        Ok(()) => GenericCarrierObservationDispositionV1::CompleteRecursiveCarrier(
            targets.into_iter().collect(),
        ),
        Err(container) => GenericCarrierObservationDispositionV1::Unavailable(container),
    };
    GenericCarrierObservationV1 { disposition }
}

fn observe_generic_carrier_facts(
    mode: CorpusModeV1,
    name: &str,
) -> (GenericCarrierObservationV1, Vec<LoopRouteId>) {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let _config = crate::test_support::ScopedTestConfig::apply(&[
        ("HAKO_JOINIR_STRICT", mode.env().1),
        ("HAKO_JOINIR_PLANNER_REQUIRED", mode.planner_required()),
        ("NYASH_JOINIR_STRICT", None),
    ]);
    let (condition, body) = fixture(name);
    let ctx = LoopRouteContext::new(&condition, &body, "generic_reachability/0", false, false);
    let outcome = try_build_outcome(&ctx).expect("carrier fixture must build facts");
    let facts = outcome
        .facts
        .as_ref()
        .expect("carrier fixture must retain canonical facts");
    let v1 = facts
        .facts
        .generic_loop_v1()
        .expect("carrier observation requires Generic V1 facts");
    let observation = observe_carrier_body(&v1.body, &v1.loop_var);
    let raw_schedule = select_recipe_first_routes(Some(facts))
        .raw_execution_routes()
        .to_vec();
    (observation, raw_schedule)
}

fn seeded_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("generic_reachability/0".to_string());
    for (name, ty) in [
        ("i", MirType::Integer),
        ("j", MirType::Integer),
        ("m", MirType::Integer),
        ("n", MirType::Integer),
    ] {
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
        "v1-only-effect" => (progression_condition(), v1_only_effect_body()),
        "effect-no-local" => (progression_condition(), effect_without_local_body()),
        "v0-additive" => (additive_condition(), additive_body()),
        "v1-true-body-step" => (true_condition(), true_body()),
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
    // The production-like generic composer expects one active lexical scope
    // for body-local lowering. Keep that setup inside the fresh candidate so
    // a planner-required row is not misclassified as a pre-effect failure
    // merely because this test omitted the normal scope boundary.
    let _scope = LexicalScopeGuard::new(&mut builder);
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
            before_compose,
            before_lower: after_compose.clone(),
            after_lower: after_compose,
            semantic_digest: None,
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
            before_compose,
            before_lower: before_lower.clone(),
            after_lower: before_lower,
            semantic_digest: None,
        };
    }
    if PlanVerifier::verify(&plan).is_err() {
        return ReachabilityRowV1 {
            mode,
            route,
            root_is_loop,
            stage: PlanStageV1::VerifierRejected,
            first_effect_owner,
            before_compose,
            before_lower: before_lower.clone(),
            after_lower: before_lower,
            semantic_digest: None,
        };
    }
    let semantic_digest = Some(core_plan_semantic_digest(&plan));
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
        before_compose,
        before_lower,
        after_lower: snapshot(&builder),
        semantic_digest,
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

fn compose_both_plan(route: LoopRouteId) -> CorePlan {
    let (condition, body) = fixture("both");
    let ctx = LoopRouteContext::new(&condition, &body, "generic_reachability/0", false, false);
    let outcome = try_build_outcome(&ctx).expect("Both fixture must produce Generic facts");
    let facts = outcome
        .facts
        .as_ref()
        .expect("Both fixture must retain canonical facts");
    let mut builder = seeded_builder();
    let _scope = LexicalScopeGuard::new(&mut builder);
    match route {
        LoopRouteId::GenericLoopV0 => {
            RecipeComposer::compose_generic_loop_v0_recipe(&mut builder, facts, &ctx)
        }
        LoopRouteId::GenericLoopV1 => {
            RecipeComposer::compose_generic_loop_v1_recipe(&mut builder, facts, &ctx)
        }
        other => panic!("unexpected non-Generic route in carrier witness: {other:?}"),
    }
    .expect("Both Generic route must compose for the semantic witness")
}

fn nested_loop<'a>(plans: &'a [CorePlan]) -> Option<&'a CoreLoopPlan> {
    for plan in plans {
        match plan {
            CorePlan::Loop(loop_plan) => return Some(loop_plan),
            CorePlan::Seq(items) => {
                if let Some(loop_plan) = nested_loop(items) {
                    return Some(loop_plan);
                }
            }
            CorePlan::If(if_plan) => {
                if let Some(loop_plan) = nested_loop(&if_plan.then_plans) {
                    return Some(loop_plan);
                }
                if let Some(else_plans) = if_plan.else_plans.as_ref() {
                    if let Some(loop_plan) = nested_loop(else_plans) {
                        return Some(loop_plan);
                    }
                }
            }
            CorePlan::BranchN(_) | CorePlan::Effect(_) | CorePlan::Exit(_) => {}
        }
    }
    None
}

fn nested_carrier_evidence(route: LoopRouteId) -> NestedCarrierSemanticEvidenceV1 {
    let plan = compose_both_plan(route);
    let CorePlan::Loop(outer) = plan else {
        panic!("Both Generic route must compose to an outer Loop")
    };
    let nested = nested_loop(&outer.body).expect("Both fixture must retain its inner Loop");
    NestedCarrierSemanticEvidenceV1 {
        route,
        outer_final_value_names: outer
            .final_values
            .iter()
            .map(|(name, _)| name.clone())
            .collect(),
        outer_phi_tags: outer.phis.iter().map(|phi| phi.tag.clone()).collect(),
        nested_final_value_names: nested
            .final_values
            .iter()
            .map(|(name, _)| name.clone())
            .collect(),
    }
}

pub(super) fn observe_both_direct_stage(
    strict_or_dev: bool,
    planner_required: bool,
) -> Vec<GenericDirectStageEvidenceV1> {
    let mode = if planner_required {
        CorpusModeV1::StrictPlannerRequired
    } else if strict_or_dev {
        CorpusModeV1::Strict
    } else {
        CorpusModeV1::Release
    };
    let _config = crate::test_support::ScopedTestConfig::apply(&[
        (
            "HAKO_JOINIR_STRICT",
            if strict_or_dev { Some("1") } else { None },
        ),
        (
            "HAKO_JOINIR_PLANNER_REQUIRED",
            if planner_required { Some("1") } else { None },
        ),
        ("NYASH_JOINIR_STRICT", None),
    ]);
    observe_fixture(mode, "both")
        .into_iter()
        .map(|row| GenericDirectStageEvidenceV1 {
            route: row.route,
            stage: row.stage,
            first_effect_owner: row.first_effect_owner,
            before_compose: row.before_compose,
            before_lower: row.before_lower,
            after_lower: row.after_lower,
            semantic_digest: row.semantic_digest,
        })
        .collect()
}

#[test]
fn generic_both_facts_emit_test_only_recursive_carrier_observation() {
    for mode in [
        CorpusModeV1::Release,
        CorpusModeV1::Strict,
        CorpusModeV1::StrictPlannerRequired,
    ] {
        let (both, raw_schedule) = observe_generic_carrier_facts(mode, "both");
        let (both_repeat, repeat_schedule) = observe_generic_carrier_facts(mode, "both");
        assert_eq!(
            both.disposition,
            GenericCarrierObservationDispositionV1::CompleteRecursiveCarrier(vec!["j".into()])
        );
        assert_eq!(both.disposition, both_repeat.disposition);
        assert_eq!(raw_schedule, repeat_schedule);
        if matches!(mode, CorpusModeV1::StrictPlannerRequired) {
            assert_eq!(raw_schedule, vec![LoopRouteId::GenericLoopV1]);
        } else {
            assert_eq!(
                raw_schedule,
                vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
            );
        }

        let (simple, simple_schedule) = observe_generic_carrier_facts(mode, "simple-while");
        let (simple_repeat, repeat_simple_schedule) =
            observe_generic_carrier_facts(mode, "simple-while");
        assert_eq!(
            simple.disposition,
            GenericCarrierObservationDispositionV1::CompleteNoRecursiveCarrier
        );
        assert_eq!(simple.disposition, simple_repeat.disposition);
        assert_eq!(simple_schedule, repeat_simple_schedule);
        assert!(!simple_schedule.contains(&LoopRouteId::GenericLoopV1));
    }
}

#[test]
fn generic_carrier_observation_marks_preserved_unsupported_container_unavailable() {
    let body = vec![ASTNode::LoopRange {
        var_name: "k".to_string(),
        start: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(0),
            span: Span::unknown(),
        }),
        end: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }),
        body: Vec::new(),
        span: Span::unknown(),
    }];
    let observation = observe_carrier_body(&body, "i");
    assert_eq!(
        observation.disposition,
        GenericCarrierObservationDispositionV1::Unavailable("LoopRange")
    );
    assert_ne!(
        observation.disposition,
        GenericCarrierObservationDispositionV1::CompleteNoRecursiveCarrier
    );
}

#[test]
fn generic_both_nested_carrier_semantic_witness_is_not_alpha_noise() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let v0 = nested_carrier_evidence(LoopRouteId::GenericLoopV0);
    let v1 = nested_carrier_evidence(LoopRouteId::GenericLoopV1);

    // Source-level meaning: the inner loop writes the outer `j`, so the
    // binding must remain observable after the outer loop.  V1 carries that
    // binding through the outer loop; V0's plan does not.
    assert!(v0.nested_final_value_names.iter().any(|name| name == "j"));
    assert!(!v0.outer_final_value_names.iter().any(|name| name == "j"));
    assert!(v1.outer_final_value_names.iter().any(|name| name == "j"));
    assert!(v1.outer_phi_tags.iter().any(|tag| tag == "loop_carrier_j"));
    assert!(v1.outer_phi_tags.iter().any(|tag| tag == "loop_step_in_j"));
    assert_ne!(v0, v1, "carrier meaning must remain visible in evidence");

    for strict_or_dev in [false, true] {
        let stages = observe_both_direct_stage(strict_or_dev, false);
        assert_eq!(
            stages.len(),
            2,
            "release/strict Both must exercise both fresh Generic candidates"
        );
        for stage in stages {
            assert_eq!(stage.stage, PlanStageV1::LowerSome, "{stage:?}");
        }
    }
}

#[test]
fn generic_accepted_plan_reachability_corpus_is_test_only_and_repeatable() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let mut accepted = 0usize;
    let mut both_lower_some = 0usize;
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
        for name in [
            "v1-only",
            "v1-only-effect",
            "effect-no-local",
            "v0-additive",
            "v1-true-body-step",
            "both",
            "simple-while",
            "neither",
        ] {
            let rows = observe_fixture(mode, name);
            for row in rows {
                if row.stage == PlanStageV1::ComposerError {
                    if matches!(name, "v1-only-effect" | "effect-no-local") {
                        assert_eq!(
                            row.route,
                            LoopRouteId::GenericLoopV1,
                            "effect-call boundary must remain V1-only: {row:?}"
                        );
                        assert_eq!(
                            row.first_effect_owner,
                            EffectOwnerV1::GenericComposer,
                            "effect-call boundary must classify composer failure as effectful: {row:?}"
                        );
                        assert_eq!(
                            row.stage,
                            PlanStageV1::ComposerError,
                            "effect-call boundary must stop at the actual composer error: {row:?}"
                        );
                    }
                    // Composer errors are split by the observed candidate
                    // owner.  `None` is a precondition stop; a Generic owner
                    // means the composer entered its pipeline and this row
                    // remains an effectful unresolved stop.
                    let repeat = observe_fixture(mode, name)
                        .into_iter()
                        .find(|candidate| candidate.route == row.route)
                        .expect("repeat fixture must retain selected Generic route");
                    assert_eq!(
                        row.stage, repeat.stage,
                        "fresh candidate stage drift: {row:?}"
                    );
                    continue;
                }
                assert!(
                    row.root_is_loop,
                    "Generic composer root must be Loop: {row:?}"
                );
                assert!(
                    row.first_effect_owner == EffectOwnerV1::GenericComposer,
                    "accepted Generic composition must leave candidate evidence: {row:?}"
                );
                if name == "v0-additive" {
                    if !matches!(mode, CorpusModeV1::StrictPlannerRequired) {
                        assert_eq!(
                            row.stage,
                            PlanStageV1::LowerSome,
                            "additive V0 row must reach a terminal lower success outside planner-required mode: {row:?}"
                        );
                    }
                }
                if name == "v1-true-body-step" {
                    if !matches!(mode, CorpusModeV1::StrictPlannerRequired) {
                        assert_eq!(
                            row.stage,
                            PlanStageV1::LowerSome,
                            "true-condition V1 row must reach a terminal lower success outside planner-required mode: {row:?}"
                        );
                    }
                }
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
                    if name == "both" {
                        both_lower_some += 1;
                    }
                }
            }
        }
    }
    assert!(
        accepted >= 3,
        "known Generic corpus must reach lower success in at least three rows"
    );
    assert!(
        both_lower_some >= 2,
        "Both fixture must observe V0/V1 lower success in at least two mode rows"
    );
}
