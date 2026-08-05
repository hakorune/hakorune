//! D3-S1-S2: parsed source to candidate-stage plan projection.
//!
//! This is a cfg(test)-only bridge.  Resolver BindingRef evidence and actual
//! V0/V1 CorePlan projections are observed together, but the name-backed
//! projection is not a typed BindingRef-to-ValueId relation and never selects
//! a production route.

use super::generic_accepted_plan_reachability_tests::{
    evaluate_nested_carrier_policy_probe, observe_source_under_current_config, CandidateSnapshotV1,
    CorpusModeV1, EffectOwnerV1, GenericCarrierPolicyDispositionV1, GenericCarrierPolicyFrameV1,
    PlanStageV1, ReachabilityRowV1,
};
use super::generic_nested_carrier_bindingref_tests::{
    inner_loop_site, outer_loop_site, parse_function, post_loop_read_site, read_binding,
    resolved_binding, write_site, SOURCE,
};
use super::generic_semantic_digest_tests::{core_plan_semantic_digest, CorePlanSemanticDigestV1};
use super::generic_stage_observer_tests::{
    observe_selected_fixture, GenericStageTraceV1, ObserverModeV1, TerminalTraceV1,
};
use super::route_id::LoopRouteId;
use crate::ast::ASTNode;
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
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, SourcePathSegmentV1,
    SourceStmtSiteV1, VerifiedResolvedFunctionV1,
};
use crate::mir::{MirType, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlanProjectionV1 {
    outer_final_names: Vec<String>,
    outer_phi_tags: Vec<String>,
    nested_final_names: Vec<String>,
    outer_final_j: Option<ValueId>,
    outer_carrier_phi_j: Option<ValueId>,
}

#[derive(Debug, Clone, PartialEq)]
struct CandidateObservationV1 {
    route: LoopRouteId,
    stage: PlanStageV1,
    first_effect_owner: EffectOwnerV1,
    before_compose: CandidateSnapshotV1,
    before_lower: CandidateSnapshotV1,
    after_lower: CandidateSnapshotV1,
    projection: Option<PlanProjectionV1>,
    semantic_digest: Option<CorePlanSemanticDigestV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResolverObligationV1 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    write_binding: BindingRefV1,
    post_loop_read_binding: BindingRefV1,
    diagnostic_name: String,
    forest_len: usize,
    frame_matches: bool,
}

fn source_parts() -> (ASTNode, Vec<ASTNode>) {
    let function = parse_function(SOURCE);
    let ASTNode::FunctionDeclaration { body, .. } = &function else {
        panic!("natural Both source must be a function")
    };
    let ASTNode::Loop {
        condition, body, ..
    } = body
        .first()
        .expect("natural Both source must start with a loop")
    else {
        panic!("natural Both source root must be a loop")
    };
    (condition.as_ref().clone(), body.clone())
}

fn seeded_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("generic_source_bridge/0".to_string());
    for name in ["i", "j", "m", "n"] {
        let value = builder.alloc_typed(MirType::Integer);
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

fn collect_loops<'a>(plan: &'a CorePlan, loops: &mut Vec<&'a CoreLoopPlan>) {
    match plan {
        CorePlan::Loop(loop_plan) => {
            loops.push(loop_plan);
            for child in &loop_plan.body {
                collect_loops(child, loops);
            }
        }
        CorePlan::Seq(items) => {
            for item in items {
                collect_loops(item, loops);
            }
        }
        CorePlan::If(if_plan) => {
            for item in &if_plan.then_plans {
                collect_loops(item, loops);
            }
            if let Some(items) = &if_plan.else_plans {
                for item in items {
                    collect_loops(item, loops);
                }
            }
        }
        CorePlan::BranchN(_) | CorePlan::Effect(_) | CorePlan::Exit(_) => {}
    }
}

fn project_plan(plan: &CorePlan) -> PlanProjectionV1 {
    let mut loops = Vec::new();
    collect_loops(plan, &mut loops);
    let outer = loops
        .first()
        .expect("candidate plan must contain an outer loop");
    let nested = loops
        .get(1)
        .expect("natural Both must retain an inner loop");
    PlanProjectionV1 {
        outer_final_names: outer
            .final_values
            .iter()
            .map(|(name, _)| name.clone())
            .collect(),
        outer_phi_tags: outer.phis.iter().map(|phi| phi.tag.clone()).collect(),
        nested_final_names: nested
            .final_values
            .iter()
            .map(|(name, _)| name.clone())
            .collect(),
        outer_final_j: outer
            .final_values
            .iter()
            .find(|(name, _)| name == "j")
            .map(|(_, value)| *value),
        outer_carrier_phi_j: outer
            .phis
            .iter()
            .find(|phi| phi.tag == "loop_carrier_j")
            .map(|phi| phi.dst),
    }
}

fn compose_candidate(
    mode: ObserverModeV1,
    route: LoopRouteId,
    condition: &ASTNode,
    body: &[ASTNode],
) -> CandidateObservationV1 {
    let _config = mode.config();
    let ctx = LoopRouteContext::new(condition, body, "generic_source_bridge/0", false, false);
    let outcome = try_build_outcome(&ctx).expect("parsed natural Both facts must build");
    let facts = outcome
        .facts
        .as_ref()
        .expect("parsed natural Both facts must be retained");
    let mut builder = seeded_builder();
    let _scope = LexicalScopeGuard::new(&mut builder);
    let before_compose = snapshot(&builder);
    let composed = match route {
        LoopRouteId::GenericLoopV0 => {
            RecipeComposer::compose_generic_loop_v0_recipe(&mut builder, facts, &ctx)
        }
        LoopRouteId::GenericLoopV1 => {
            RecipeComposer::compose_generic_loop_v1_recipe(&mut builder, facts, &ctx)
        }
        other => panic!("unexpected route in candidate bridge: {other:?}"),
    };
    let Ok(plan) = composed else {
        let after = snapshot(&builder);
        return CandidateObservationV1 {
            route,
            stage: PlanStageV1::ComposerError,
            first_effect_owner: if after != before_compose {
                EffectOwnerV1::GenericComposer
            } else {
                EffectOwnerV1::None
            },
            before_compose,
            before_lower: after.clone(),
            after_lower: after,
            projection: None,
            semantic_digest: None,
        };
    };
    let before_lower = snapshot(&builder);
    let first_effect_owner = if before_lower != before_compose {
        EffectOwnerV1::GenericComposer
    } else {
        EffectOwnerV1::None
    };
    let projection = Some(project_plan(&plan));
    let semantic_digest = Some(core_plan_semantic_digest(&plan));
    let stage = if !matches!(&plan, CorePlan::Loop(_)) {
        PlanStageV1::NonLoopRoot
    } else if PlanVerifier::verify(&plan).is_err() {
        PlanStageV1::VerifierRejected
    } else {
        let result = if mode.strict_or_dev() {
            lower_verified_core_plan(
                &mut builder,
                &ctx,
                true,
                Some(facts),
                plan,
                FlowboxVia::Shadow,
            )
        } else {
            PlanLowerer::lower(&mut builder, plan, &ctx)
        };
        match result {
            Ok(Some(_)) => PlanStageV1::LowerSome,
            Ok(None) => PlanStageV1::LowerNone,
            Err(_) => PlanStageV1::LowerError,
        }
    };
    CandidateObservationV1 {
        route,
        stage,
        first_effect_owner,
        before_compose,
        before_lower,
        after_lower: snapshot(&builder),
        projection,
        semantic_digest,
    }
}

fn corpus_mode(mode: ObserverModeV1) -> CorpusModeV1 {
    match mode {
        ObserverModeV1::Release => CorpusModeV1::Release,
        ObserverModeV1::Strict => CorpusModeV1::Strict,
        ObserverModeV1::StrictPlannerRequired => CorpusModeV1::StrictPlannerRequired,
    }
}

fn resolver_obligation() -> ResolverObligationV1 {
    let function = parse_function(SOURCE);
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver session");
    let product = resolver
        .resolve(FunctionSyntaxViewV1::from_ast(&function).expect("function syntax view"))
        .expect("natural Both source resolves");
    let write_site = write_site(false);
    let read_site = post_loop_read_site();
    let write_binding = resolved_binding(&product, &write_site);
    let post_loop_read_binding = read_binding(&product, &read_site);
    let forest = product
        .resolved_loop_source_forest(&outer_loop_site())
        .expect("natural Both forest");
    let outer_source = forest
        .members()
        .first()
        .expect("outer forest member")
        .source();
    let outer_lookup = product
        .resolved_loop_source(&outer_loop_site())
        .expect("outer source lookup");
    let inner_source = forest
        .members()
        .get(1)
        .expect("inner forest member")
        .source();
    assert!(outer_source.frame_key().matches(&outer_lookup.frame_key()));
    assert!(inner_source.matches_identity(
        product.function_origin(),
        product.source_kind(),
        &inner_loop_site()
    ));
    assert_eq!(write_binding, post_loop_read_binding);
    let diagnostic_name = product
        .binding(write_binding)
        .expect("write binding record")
        .diagnostic_name()
        .to_string();
    ResolverObligationV1 {
        owner: product.owner(),
        write_binding,
        post_loop_read_binding,
        diagnostic_name,
        forest_len: forest.members().len(),
        frame_matches: true,
    }
}

fn parsed_rows(
    mode: ObserverModeV1,
    condition: &ASTNode,
    body: &[ASTNode],
) -> Vec<ReachabilityRowV1> {
    let _config = mode.config();
    observe_source_under_current_config(
        corpus_mode(mode),
        condition.clone(),
        body.to_vec(),
        "generic_source_bridge/0",
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LegacyDtoDispositionV1 {
    ObservedLegacyV0,
    UnresolvedStop,
}

fn evaluate_legacy_dto(debt_count: usize, terminal: &TerminalTraceV1) -> LegacyDtoDispositionV1 {
    if debt_count == 0 && *terminal == TerminalTraceV1::Succeeded(LoopRouteId::GenericLoopV0) {
        LegacyDtoDispositionV1::ObservedLegacyV0
    } else {
        LegacyDtoDispositionV1::UnresolvedStop
    }
}

#[test]
fn generic_d3_s1_s2_parsed_candidate_stage_bridge_is_observation_only() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    for mode in [ObserverModeV1::Release, ObserverModeV1::Strict] {
        let (condition, body) = source_parts();
        let obligation = resolver_obligation();
        let repeat_obligation = resolver_obligation();
        let rows = parsed_rows(mode, &condition, &body);
        assert_eq!(obligation.forest_len, 2);
        assert_eq!(obligation.write_binding, obligation.post_loop_read_binding);
        assert_ne!(obligation.owner, repeat_obligation.owner);
        assert_eq!(
            obligation.write_binding.binding(),
            repeat_obligation.write_binding.binding()
        );
        assert_eq!(
            obligation.diagnostic_name,
            repeat_obligation.diagnostic_name
        );
        assert_eq!(obligation.frame_matches, repeat_obligation.frame_matches);
        assert_eq!(obligation.forest_len, repeat_obligation.forest_len);
        assert_eq!(obligation.diagnostic_name, "j");
        assert!(obligation.frame_matches);
        assert_eq!(
            rows.iter().map(|row| row.route).collect::<Vec<_>>(),
            vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
        );

        let forward = [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
            .into_iter()
            .map(|route| compose_candidate(mode, route, &condition, &body))
            .collect::<Vec<_>>();
        let reverse = [LoopRouteId::GenericLoopV1, LoopRouteId::GenericLoopV0]
            .into_iter()
            .map(|route| compose_candidate(mode, route, &condition, &body))
            .collect::<Vec<_>>();
        for route in [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1] {
            let lhs = forward.iter().find(|row| row.route == route).unwrap();
            let rhs = reverse.iter().find(|row| row.route == route).unwrap();
            assert_eq!(lhs, rhs, "route observation must be order-independent");
            let parsed = rows.iter().find(|row| row.route == route).unwrap();
            assert_eq!(lhs.stage, parsed.stage);
            assert_eq!(lhs.first_effect_owner, parsed.first_effect_owner);
            assert_eq!(lhs.before_compose, parsed.before_compose);
            assert_eq!(lhs.before_lower, parsed.before_lower);
            assert_eq!(lhs.after_lower, parsed.after_lower);
        }
        let v0 = forward
            .iter()
            .find(|row| row.route == LoopRouteId::GenericLoopV0)
            .unwrap();
        let v1 = forward
            .iter()
            .find(|row| row.route == LoopRouteId::GenericLoopV1)
            .unwrap();
        assert_eq!(v0.stage, PlanStageV1::LowerSome);
        assert_eq!(v1.stage, PlanStageV1::LowerSome);
        assert_eq!(v0.first_effect_owner, EffectOwnerV1::GenericComposer);
        assert_eq!(v1.first_effect_owner, EffectOwnerV1::GenericComposer);
        let v0_projection = v0.projection.as_ref().unwrap();
        let v1_projection = v1.projection.as_ref().unwrap();
        assert!(!v0_projection
            .outer_final_names
            .iter()
            .any(|name| name == "j"));
        assert!(v0_projection
            .nested_final_names
            .iter()
            .any(|name| name == "j"));
        assert!(v1_projection
            .outer_final_names
            .iter()
            .any(|name| name == "j"));
        assert!(v1_projection
            .outer_phi_tags
            .iter()
            .any(|tag| tag == "loop_carrier_j"));
        assert!(v1_projection
            .outer_phi_tags
            .iter()
            .any(|tag| tag == "loop_step_in_j"));
        assert_eq!(
            v1_projection.outer_final_j, v1_projection.outer_carrier_phi_j,
            "plan-local final/PHI projection must agree; this is not BindingRef provenance"
        );

        let trace: GenericStageTraceV1 = observe_selected_fixture(
            mode,
            condition.clone(),
            body.clone(),
            "generic_source_bridge/legacy/0",
        );
        assert_eq!(
            trace.raw_schedule,
            vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
        );
        assert_eq!(
            trace.terminal,
            TerminalTraceV1::Succeeded(LoopRouteId::GenericLoopV0)
        );
        assert!(trace.generic_debts.is_empty());
        assert_eq!(
            evaluate_legacy_dto(trace.generic_debts.len(), &trace.terminal),
            LegacyDtoDispositionV1::ObservedLegacyV0
        );
        assert_eq!(
            evaluate_legacy_dto(1, &TerminalTraceV1::Succeeded(LoopRouteId::GenericLoopV1)),
            LegacyDtoDispositionV1::UnresolvedStop,
            "synthetic debt/V1-terminal mutation is evaluator-only negative evidence"
        );
    }
}

#[test]
fn generic_d3_s1_s2_planner_required_remains_unresolved() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let mode = ObserverModeV1::StrictPlannerRequired;
    let (condition, body) = source_parts();
    let rows = parsed_rows(mode, &condition, &body);
    assert_eq!(
        rows.iter().map(|row| row.route).collect::<Vec<_>>(),
        vec![LoopRouteId::GenericLoopV1]
    );
    assert_eq!(rows[0].stage, PlanStageV1::LowerSome);
    let trace = observe_selected_fixture(
        mode,
        condition,
        body,
        "generic_source_bridge/planner-required/0",
    );
    let observation = trace
        .carrier_observation
        .as_ref()
        .expect("planner-required parsed source retains carrier observation");
    assert_eq!(
        evaluate_nested_carrier_policy_probe(
            observation,
            GenericCarrierPolicyFrameV1 {
                has_overlap: false,
                strict_or_dev: trace.frame.strict_or_dev,
                planner_required: trace.frame.planner_required,
                contract_present: trace.frame.recipe_contract_present,
                v1_stage_accepted: true,
            },
        ),
        GenericCarrierPolicyDispositionV1::UnresolvedStop
    );
    assert_eq!(
        trace.terminal,
        TerminalTraceV1::Succeeded(LoopRouteId::GenericLoopV1)
    );
}
