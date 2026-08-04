//! D2-B4-S2: test-only, shadowing-safe Generic nested-carrier evidence.
//!
//! The source fixture is parsed normally, resolved through BindingRefV1, and
//! then sent through the existing Generic observer. No production policy or
//! Builder/MIR caller consumes the witness.

use super::generic_accepted_plan_reachability_tests::{
    nested_carrier_evidence, EffectOwnerV1, NestedCarrierSemanticEvidenceV1, PlanStageV1,
};
use super::generic_stage_observer_tests::{
    observe_selected_fixture, GenericStageTraceV1, ObserverModeV1,
};
use super::route_id::LoopRouteId;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::lower_verified_core_plan;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::PlanLowerer;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::plan::single_planner::try_build_outcome;
use crate::mir::builder::control_flow::plan::CorePlan;
use crate::mir::builder::control_flow::verify::observability::flowbox_tags::FlowboxVia;
use crate::mir::builder::control_flow::verify::PlanVerifier;
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1,
    ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, SourceExprSiteV1, SourceNodeSiteV1,
    SourcePathSegmentV1, SourceStmtSiteV1, VerifiedResolvedFunctionV1,
};
use crate::mir::MirType;
use crate::parser::NyashParser;

const SOURCE: &str = r#"
function generic_both(i, j) {
    loop(i < 3) {
        loop(j < 3) {
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

const SHADOWING_SOURCE: &str = r#"
function generic_both_shadowing(i, j) {
    loop(i < 3) {
        loop(j < 3) {
            local j = 0
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DispositionV1 {
    V1ForResolvedOuterCarrier,
    UnresolvedStop,
}

#[derive(Debug, PartialEq, Eq)]
enum LegacyCarrierProjectionV1 {
    Observed {
        v0_outer_has_carrier: bool,
        v1_outer_has_carrier: bool,
    },
    SuppressedByPlannerRequired,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedGenericNestedCarrierDisjointnessV1 {
    function_origin: crate::mir::resolved_semantics::FunctionOriginV1,
    outer_loop_site: SourceStmtSiteV1,
    inner_loop_site: SourceStmtSiteV1,
    write_binding: BindingRefV1,
    post_loop_read_binding: BindingRefV1,
    write_is_in_strict_ancestor: bool,
    frame_identity_matches: bool,
    raw_schedule: Vec<LoopRouteId>,
    carrier_observation:
        Option<crate::mir::builder::control_flow::plan::facts::GenericLoopCarrierObservationV1>,
    v1_stage: Option<(PlanStageV1, EffectOwnerV1)>,
    trace: GenericStageTraceV1,
    fresh_repeat_stable: bool,
    legacy_carrier_projection: LegacyCarrierProjectionV1,
}

fn parse_function(source: &str) -> ASTNode {
    let root = NyashParser::parse_from_string(source).expect("nested carrier source parses");
    let ASTNode::Program { statements, .. } = root else {
        panic!("nested carrier source must parse to Program")
    };
    statements
        .into_iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .expect("nested carrier source must contain a function")
}

fn outer_loop_parts(function: &ASTNode) -> (&ASTNode, &[ASTNode]) {
    let ASTNode::FunctionDeclaration { body, .. } = function else {
        panic!("nested carrier fixture must be a function")
    };
    let ASTNode::Loop {
        condition, body, ..
    } = body.first().expect("outer loop")
    else {
        panic!("function body index 0 must be the outer loop")
    };
    (condition.as_ref(), body.as_slice())
}

fn outer_loop_site() -> SourceStmtSiteV1 {
    stmt(&[SourcePathSegmentV1::Body(0)])
}

fn inner_loop_site() -> SourceStmtSiteV1 {
    stmt(&[
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::LoopBody(0),
    ])
}

fn write_site(shadowing: bool) -> SourceExprSiteV1 {
    expr(&[
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::LoopBody(0),
        SourcePathSegmentV1::LoopBody(if shadowing { 1 } else { 0 }),
        SourcePathSegmentV1::Target,
    ])
}

fn post_loop_read_site() -> SourceExprSiteV1 {
    expr(&[SourcePathSegmentV1::Body(1), SourcePathSegmentV1::Value])
}

fn stmt(segments: &[SourcePathSegmentV1]) -> SourceStmtSiteV1 {
    SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(segments.to_vec()))
}

fn expr(segments: &[SourcePathSegmentV1]) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments.to_vec()))
}

fn strict_ancestor_binding(
    product: &VerifiedResolvedFunctionV1,
    binding: BindingRefV1,
    site: &SourceExprSiteV1,
) -> bool {
    let Some(ancestor_scope) = product.binding(binding).map(|record| record.owner_scope()) else {
        return false;
    };
    let Some(mut current) = product.exact_scope_containing(site.node()) else {
        return false;
    };
    while let Some(parent) = product.scope(current).and_then(|scope| scope.parent()) {
        if parent == ancestor_scope {
            return true;
        }
        current = parent;
    }
    false
}

fn resolved_binding(product: &VerifiedResolvedFunctionV1, site: &SourceExprSiteV1) -> BindingRefV1 {
    let Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) = product.assignment_target(site)
    else {
        panic!("nested carrier assignment must resolve to BindingRebind")
    };
    *binding
}

fn read_binding(product: &VerifiedResolvedFunctionV1, site: &SourceExprSiteV1) -> BindingRefV1 {
    let Some(ResolvedLexicalRefV1::Local(binding)) = product.variable_ref(site) else {
        panic!("post-loop j read must resolve to Local")
    };
    binding
}

fn stage_for_v1(
    mode: ObserverModeV1,
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<(PlanStageV1, EffectOwnerV1)> {
    let _config = mode.config();
    let ctx = LoopRouteContext::new(condition, body, "generic_bindingref/0", false, false);
    let outcome = try_build_outcome(&ctx).ok()?;
    let facts = outcome.facts.as_ref()?;
    let routes = super::selection::select_recipe_first_routes(Some(facts));
    if !routes
        .raw_execution_routes()
        .iter()
        .any(|route| *route == LoopRouteId::GenericLoopV1)
    {
        return None;
    }
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("generic_bindingref/0".to_string());
    for name in ["i", "j"] {
        let value = builder.alloc_typed(MirType::Integer);
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert(name.to_string(), value);
    }
    let _scope = LexicalScopeGuard::new(&mut builder);
    let before_compose = builder
        .function_state
        .current_function
        .as_ref()
        .map(|f| f.next_value_id);
    let plan = RecipeComposer::compose_generic_loop_v1_recipe(&mut builder, facts, &ctx).ok();
    let first_effect_owner = if builder
        .function_state
        .current_function
        .as_ref()
        .map(|f| f.next_value_id)
        != before_compose
    {
        EffectOwnerV1::GenericComposer
    } else {
        EffectOwnerV1::None
    };
    let Some(plan) = plan else {
        return Some((PlanStageV1::ComposerError, first_effect_owner));
    };
    if !matches!(&plan, CorePlan::Loop(_)) {
        return Some((PlanStageV1::NonLoopRoot, first_effect_owner));
    }
    if PlanVerifier::verify(&plan).is_err() {
        return Some((PlanStageV1::VerifierRejected, first_effect_owner));
    }
    let stage = if mode.strict_or_dev() {
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
    Some((
        match stage {
            Ok(Some(_)) => PlanStageV1::LowerSome,
            Ok(None) => PlanStageV1::LowerNone,
            Err(_) => PlanStageV1::LowerError,
        },
        first_effect_owner,
    ))
}

fn has_carrier_for(evidence: &NestedCarrierSemanticEvidenceV1, label: &str) -> bool {
    let carrier = format!("loop_carrier_{label}");
    let step_in = format!("loop_step_in_{label}");
    evidence
        .outer_final_value_names
        .iter()
        .any(|name| name == label)
        && evidence.outer_phi_tags.iter().any(|tag| tag == &carrier)
        && evidence.outer_phi_tags.iter().any(|tag| tag == &step_in)
}

fn build_witness(source: &str, mode: ObserverModeV1) -> VerifiedGenericNestedCarrierDisjointnessV1 {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let function = parse_function(source);
    let (condition, body) = outer_loop_parts(&function);
    let ctx = LoopRouteContext::new(condition, body, "generic_bindingref/0", false, false);
    let outcome = {
        let _config = mode.config();
        try_build_outcome(&ctx).expect("parsed nested carrier source must produce facts")
    };
    let canonical = outcome
        .facts
        .as_ref()
        .expect("parsed source must retain facts");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver session");
    let product = resolver
        .resolve(FunctionSyntaxViewV1::from_ast(&function).expect("function view"))
        .expect("parsed nested carrier source resolves");
    let outer = outer_loop_site();
    let inner = inner_loop_site();
    let write = write_site(source == SHADOWING_SOURCE);
    let read = post_loop_read_site();
    let write_binding = resolved_binding(&product, &write);
    let post_loop_read_binding = read_binding(&product, &read);
    let forest = product
        .resolved_loop_source_forest(&outer)
        .expect("sealed loop forest");
    assert_eq!(forest.members().len(), 2, "outer and inner loops only");
    assert_eq!(forest.members()[0].source().site(), &outer);
    assert_eq!(forest.members()[1].source().site(), &inner);
    assert_eq!(forest.members()[0].parent_index(), None);
    assert_eq!(forest.members()[1].parent_index(), Some(0));
    let outer_source = forest.members()[0].source();
    let inner_source = forest.members()[1].source();
    let outer_lookup = product
        .resolved_loop_source(&outer)
        .expect("outer loop source remains sealed");
    let outer_frame_matches = outer_source.frame_key().matches(&outer_lookup.frame_key());
    let source_identity_matches =
        outer_source.matches_identity(product.function_origin(), product.source_kind(), &outer)
            && inner_source.matches_identity(
                product.function_origin(),
                product.source_kind(),
                &inner,
            );
    let binding_owner_matches = write_binding.owner() == product.owner()
        && post_loop_read_binding.owner() == product.owner();

    let trace = observe_selected_fixture(
        mode,
        condition.clone(),
        body.to_vec(),
        "generic_bindingref/0",
    );
    let repeat_function = parse_function(source);
    let (repeat_condition, repeat_body) = outer_loop_parts(&repeat_function);
    let repeat_trace = observe_selected_fixture(
        mode,
        repeat_condition.clone(),
        repeat_body.to_vec(),
        "generic_bindingref/0",
    );
    let fresh_repeat_stable = repeat_trace == trace;
    let legacy_carrier_projection = if source == SOURCE && !mode.planner_required() {
        let _config = mode.config();
        let v0 = nested_carrier_evidence(LoopRouteId::GenericLoopV0);
        let v1 = nested_carrier_evidence(LoopRouteId::GenericLoopV1);
        LegacyCarrierProjectionV1::Observed {
            v0_outer_has_carrier: has_carrier_for(&v0, "j"),
            v1_outer_has_carrier: has_carrier_for(&v1, "j"),
        }
    } else if mode.planner_required() {
        LegacyCarrierProjectionV1::SuppressedByPlannerRequired
    } else {
        LegacyCarrierProjectionV1::Observed {
            v0_outer_has_carrier: false,
            v1_outer_has_carrier: false,
        }
    };
    VerifiedGenericNestedCarrierDisjointnessV1 {
        function_origin: product.function_origin(),
        outer_loop_site: outer,
        inner_loop_site: inner,
        write_binding,
        post_loop_read_binding,
        write_is_in_strict_ancestor: strict_ancestor_binding(&product, write_binding, &write),
        frame_identity_matches: outer_frame_matches
            && source_identity_matches
            && binding_owner_matches,
        raw_schedule: super::selection::select_recipe_first_routes(Some(canonical))
            .raw_execution_routes()
            .to_vec(),
        carrier_observation: canonical
            .facts
            .generic_loop_v1()
            .map(|facts| facts.carrier_observation.clone()),
        v1_stage: stage_for_v1(mode, condition, body),
        trace,
        fresh_repeat_stable,
        legacy_carrier_projection,
    }
}

fn evaluate(
    evidence: &VerifiedGenericNestedCarrierDisjointnessV1,
    expected_same_binding: bool,
) -> DispositionV1 {
    let recursive = matches!(
        evidence.carrier_observation,
        Some(
            crate::mir::builder::control_flow::plan::facts::GenericLoopCarrierObservationV1::CompleteRecursiveCarrier(_)
        )
    );
    if expected_same_binding
        && evidence.write_binding == evidence.post_loop_read_binding
        && evidence.write_is_in_strict_ancestor
        && evidence.frame_identity_matches
        && evidence.raw_schedule.as_slice()
            == [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
        && recursive
        && evidence.v1_stage == Some((PlanStageV1::LowerSome, EffectOwnerV1::GenericComposer))
        && evidence.trace.raw_schedule == evidence.raw_schedule
        && evidence.trace.carrier_observation == evidence.carrier_observation
        && evidence.fresh_repeat_stable
    {
        DispositionV1::V1ForResolvedOuterCarrier
    } else {
        DispositionV1::UnresolvedStop
    }
}

#[test]
fn generic_d2_b4_s2_parsed_outer_binding_is_shadowing_safe() {
    for mode in [ObserverModeV1::Release, ObserverModeV1::Strict] {
        let evidence = build_witness(SOURCE, mode);
        assert_eq!(
            evaluate(&evidence, true),
            DispositionV1::V1ForResolvedOuterCarrier,
            "natural parsed {mode:?} Both row must issue only a test witness"
        );
        assert_eq!(
            evidence.raw_schedule,
            vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
        );
        assert_eq!(evidence.write_binding, evidence.post_loop_read_binding);
        assert_eq!(
            evidence.legacy_carrier_projection,
            LegacyCarrierProjectionV1::Observed {
                v0_outer_has_carrier: false,
                v1_outer_has_carrier: true,
            }
        );
    }
}

#[test]
fn generic_d2_b4_s2_shadowing_local_does_not_issue_disjointness() {
    let evidence = build_witness(SHADOWING_SOURCE, ObserverModeV1::Strict);
    assert_ne!(evidence.write_binding, evidence.post_loop_read_binding);
    assert_eq!(
        evaluate(&evidence, false),
        DispositionV1::UnresolvedStop,
        "shadowing must not issue an outer-carrier witness"
    );
}

#[test]
fn generic_d2_b4_s2_planner_required_remains_unresolved() {
    let evidence = build_witness(SOURCE, ObserverModeV1::StrictPlannerRequired);
    assert_eq!(evidence.raw_schedule, vec![LoopRouteId::GenericLoopV1]);
    assert_eq!(
        evidence.legacy_carrier_projection,
        LegacyCarrierProjectionV1::SuppressedByPlannerRequired
    );
    assert_eq!(evaluate(&evidence, true), DispositionV1::UnresolvedStop);
}
