//! S2A: parsed nested-`IfThen` carrier evidence, test-only.
//!
//! The fixture keeps an inner canonical `j` step separate from an additional
//! `IfThen` write. Resolver-issued BindingRefs are the only identity evidence;
//! the Generic observer is used only for stage and witness corroboration.

use super::generic_accepted_plan_reachability_tests::{
    observe_source_under_current_config, CorpusModeV1, EffectOwnerV1, PlanStageV1,
    ReachabilityRowV1,
};
use super::generic_stage_observer_tests::{
    observe_selected_fixture, AttemptTraceV1, GenericStageTraceV1, ObserverModeV1, TerminalTraceV1,
};
use super::route_id::LoopRouteId;
use super::select_recipe_first_routes;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::single_planner::try_build_outcome;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1,
    ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, SourceExprSiteV1, SourceNodeSiteV1,
    SourcePathSegmentV1, SourceStmtSiteV1,
};
use crate::parser::NyashParser;

const SOURCE: &str = r#"
function generic_both_nested_if(i, j) {
    loop(i < 3) {
        loop(j < 3) {
            if i < 2 {
                j = j + 1
            }
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

#[derive(Debug)]
struct NestedIfEvidenceV1 {
    write_binding: BindingRefV1,
    inner_step_binding: BindingRefV1,
    post_read_binding: BindingRefV1,
    strict_ancestor: bool,
    source_frame_identity: bool,
    forest_len: usize,
    raw_schedule: Vec<LoopRouteId>,
    direct: Vec<ReachabilityRowV1>,
    witness: GenericStageTraceV1,
}

fn site(segments: &[SourcePathSegmentV1]) -> SourceNodeSiteV1 {
    SourceNodeSiteV1::from_segments(segments.to_vec())
}

fn expr_site(segments: &[SourcePathSegmentV1]) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(site(segments))
}

fn stmt_site(segments: &[SourcePathSegmentV1]) -> SourceStmtSiteV1 {
    SourceStmtSiteV1::from_node(site(segments))
}

fn parse_function() -> ASTNode {
    let root = NyashParser::parse_from_string(SOURCE).expect("nested If source parses");
    let ASTNode::Program { statements, .. } = root else {
        panic!("nested If source must produce a Program")
    };
    statements
        .into_iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .expect("nested If source must contain a function")
}

fn outer_parts(function: &ASTNode) -> (&ASTNode, &[ASTNode]) {
    let ASTNode::FunctionDeclaration { body, .. } = function else {
        panic!("nested If source must produce a function")
    };
    let ASTNode::Loop {
        condition, body, ..
    } = body.first().expect("outer loop")
    else {
        panic!("function body must start with the outer loop")
    };
    (condition.as_ref(), body.as_slice())
}

fn assignment_binding(
    product: &crate::mir::resolved_semantics::VerifiedResolvedFunctionV1,
    site: &SourceExprSiteV1,
) -> BindingRefV1 {
    match product.assignment_target(site) {
        Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) => *binding,
        other => panic!("expected BindingRebind target, got {other:?}"),
    }
}

fn read_binding(
    product: &crate::mir::resolved_semantics::VerifiedResolvedFunctionV1,
    site: &SourceExprSiteV1,
) -> BindingRefV1 {
    match product.variable_ref(site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        other => panic!("expected Local read, got {other:?}"),
    }
}

fn strict_ancestor(
    product: &crate::mir::resolved_semantics::VerifiedResolvedFunctionV1,
    binding: BindingRefV1,
    site: &SourceExprSiteV1,
) -> bool {
    let Some(owner_scope) = product.binding(binding).map(|record| record.owner_scope()) else {
        return false;
    };
    let Some(mut current) = product.exact_scope_containing(site.node()) else {
        return false;
    };
    while let Some(parent) = product.scope(current).and_then(|scope| scope.parent()) {
        if parent == owner_scope {
            return true;
        }
        current = parent;
    }
    false
}

fn corpus_mode(mode: ObserverModeV1) -> CorpusModeV1 {
    match mode {
        ObserverModeV1::Release => CorpusModeV1::Release,
        ObserverModeV1::Strict => CorpusModeV1::Strict,
        ObserverModeV1::StrictPlannerRequired => CorpusModeV1::StrictPlannerRequired,
    }
}

fn collect(mode: ObserverModeV1) -> NestedIfEvidenceV1 {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let function = parse_function();
    let (condition, body) = outer_parts(&function);
    let outer = stmt_site(&[SourcePathSegmentV1::Body(0)]);
    let inner = stmt_site(&[
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::LoopBody(0),
    ]);
    let write = expr_site(&[
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::LoopBody(0),
        SourcePathSegmentV1::LoopBody(0),
        SourcePathSegmentV1::IfThen(0),
        SourcePathSegmentV1::Target,
    ]);
    let inner_step = expr_site(&[
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::LoopBody(0),
        SourcePathSegmentV1::LoopBody(1),
        SourcePathSegmentV1::Target,
    ]);
    let post_read = expr_site(&[SourcePathSegmentV1::Body(1), SourcePathSegmentV1::Value]);
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver session");
    let product = resolver
        .resolve(FunctionSyntaxViewV1::from_ast(&function).expect("function view"))
        .expect("nested If source resolves");
    let write_binding = assignment_binding(&product, &write);
    let inner_step_binding = assignment_binding(&product, &inner_step);
    let post_read_binding = read_binding(&product, &post_read);
    let forest = product
        .resolved_loop_source_forest(&outer)
        .expect("nested loop forest is sealed");
    assert_eq!(forest.members().len(), 2, "outer and inner loops only");
    assert_eq!(forest.members()[0].source().site(), &outer);
    assert_eq!(forest.members()[1].source().site(), &inner);
    assert_eq!(forest.members()[0].parent_index(), None);
    assert_eq!(forest.members()[1].parent_index(), Some(0));
    let outer_source = forest.members()[0].source();
    let inner_source = forest.members()[1].source();
    let outer_lookup = product
        .resolved_loop_source(&outer)
        .expect("outer source lookup");
    let inner_lookup = product
        .resolved_loop_source(&inner)
        .expect("inner source lookup");
    let source_frame_identity = outer_source.frame_key().matches(&outer_lookup.frame_key())
        && inner_source.frame_key().matches(&inner_lookup.frame_key())
        && outer_source.matches_identity(product.function_origin(), product.source_kind(), &outer)
        && inner_source.matches_identity(product.function_origin(), product.source_kind(), &inner)
        && write_binding.owner() == product.owner()
        && inner_step_binding.owner() == product.owner()
        && post_read_binding.owner() == product.owner();
    let raw_schedule = {
        let _config = mode.config();
        let ctx = LoopRouteContext::new(&condition, body, "generic_nested_if/0", false, false);
        let outcome = try_build_outcome(&ctx).expect("nested If facts");
        let facts = outcome.facts.as_ref().expect("nested If canonical facts");
        select_recipe_first_routes(Some(facts))
            .raw_execution_routes()
            .to_vec()
    };
    let direct = {
        let _config = mode.config();
        observe_source_under_current_config(
            corpus_mode(mode),
            condition.clone(),
            body.to_vec(),
            "generic_nested_if/0",
        )
    };
    let witness = observe_selected_fixture(
        mode,
        condition.clone(),
        body.to_vec(),
        "generic_nested_if/0",
    );
    NestedIfEvidenceV1 {
        write_binding,
        inner_step_binding,
        post_read_binding,
        strict_ancestor: strict_ancestor(&product, write_binding, &write),
        source_frame_identity,
        forest_len: forest.members().len(),
        raw_schedule,
        direct,
        witness,
    }
}

fn assert_positive(mode: ObserverModeV1) {
    let evidence = collect(mode);
    let repeat = collect(mode);
    assert_eq!(evidence.write_binding, evidence.post_read_binding);
    assert_eq!(evidence.inner_step_binding, evidence.post_read_binding);
    assert!(evidence.strict_ancestor);
    assert!(evidence.source_frame_identity);
    assert_eq!(evidence.forest_len, 2);
    assert_eq!(evidence.raw_schedule, repeat.raw_schedule);
    assert_eq!(
        evidence.raw_schedule,
        vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
    );
    assert_eq!(
        evidence.direct, repeat.direct,
        "direct stage drift: {mode:?}"
    );
    assert_eq!(evidence.witness, repeat.witness, "witness drift: {mode:?}");
    assert_eq!(
        evidence
            .direct
            .iter()
            .map(|row| row.route)
            .collect::<Vec<_>>(),
        vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
    );
    assert!(evidence.direct.iter().all(|row| {
        row.stage == PlanStageV1::LowerSome
            && row.first_effect_owner == EffectOwnerV1::GenericComposer
    }));
    assert!(evidence
        .direct
        .iter()
        .all(|row| row.semantic_digest.is_some()));
    assert_ne!(
        evidence.direct[0].semantic_digest, evidence.direct[1].semantic_digest,
        "nested If carrier digest difference remains explicit"
    );
    assert_eq!(evidence.witness.raw_schedule, evidence.raw_schedule);
    assert_eq!(
        evidence.witness.carrier_observation,
        Some(
            crate::mir::builder::control_flow::plan::facts::GenericLoopCarrierObservationV1::CompleteRecursiveCarrier(vec!["j".into()])
        )
    );
    assert_eq!(
        evidence.witness.attempted,
        vec![AttemptTraceV1 {
            route: LoopRouteId::GenericLoopV0,
            cursor: 0,
            suffix: vec![LoopRouteId::GenericLoopV1],
        }]
    );
    assert!(evidence.witness.generic_debts.is_empty());
    assert_eq!(
        evidence.witness.terminal,
        TerminalTraceV1::Succeeded(LoopRouteId::GenericLoopV0)
    );
}

#[test]
fn generic_d2_b4_s2_nested_if() {
    assert_positive(ObserverModeV1::Release);
    assert_positive(ObserverModeV1::Strict);
}
