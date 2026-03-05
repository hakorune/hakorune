//! Phase 29ao P30: shadow adopt composer entrypoints (Facts -> LoweredRecipe).

use super::coreloop_gates::coreloop_base_gate;
use super::coreloop_v2_nested_minimal::try_compose_core_loop_v2_nested_minimal;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::config::env::joinir_dev;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::canon::generic_loop::{
    classify_step_placement, StepPlacement,
};
use crate::mir::builder::control_flow::plan::facts::feature_facts::{
    detect_nested_loop, ExitKindFacts,
};
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::normalizer::{
    normalize_generic_loop_v0, normalize_generic_loop_v1,
};
use crate::mir::builder::control_flow::plan::observability::flowbox_tags;
use crate::mir::builder::control_flow::plan::planner::{Freeze, PlanBuildOutcome};
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_pattern_detection::LoopPatternKind;

pub(in crate::mir::builder) struct ShadowAdoptOutcome {
    pub core_plan: LoweredRecipe,
    pub emit_flowbox_adopt_tag: bool,
}

pub(in crate::mir::builder) enum PrePlanShadowOutcome {
    Adopt(ShadowAdoptOutcome),
    GuardError(String),
}

enum GenericLoopAdoptCandidate {
    V0,
    V1,
}

fn pick_generic_loop_adopt_candidate(
    facts: &CanonicalLoopFacts,
) -> Option<GenericLoopAdoptCandidate> {
    if facts.facts.generic_loop_v1.is_some() {
        return Some(GenericLoopAdoptCandidate::V1);
    }
    if facts.facts.generic_loop_v0.is_some() {
        return Some(GenericLoopAdoptCandidate::V0);
    }
    None
}

fn try_adopt_generic_loop_preferred(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    allow_generic_loop: bool,
) -> Result<Option<ShadowAdoptOutcome>, String> {
    if !allow_generic_loop {
        return Ok(None);
    }
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    match pick_generic_loop_adopt_candidate(facts) {
        Some(GenericLoopAdoptCandidate::V1) => try_adopt_generic_loop_v1(builder, ctx, outcome),
        Some(GenericLoopAdoptCandidate::V0) => try_adopt_generic_loop_v0(builder, ctx, outcome),
        None => Ok(None),
    }
}

pub(in crate::mir::builder) fn strict_nested_loop_guard(
    outcome: &PlanBuildOutcome,
    ctx: &LoopRouteContext,
) -> Option<String> {
    if joinir_dev::debug_enabled() {
        let features = flowbox_tags::features_from_facts(outcome.facts.as_ref());
        let features_csv = features.join(",");
        let plan_state = if outcome.plan.is_some() {
            "Some"
        } else {
            "None"
        };
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[plan/trace:nested_loop_guard_entry] ctx=shadow_adopt features={} plan={}",
            features_csv, plan_state
        ));
    }
    let facts_present = outcome.facts.is_some();
    let (v0_present, v1_shape, v1_exit_allowed) =
        outcome
            .facts
            .as_ref()
            .map_or((false, false, false), |facts| {
                let v0 = facts.facts.generic_loop_v0.is_some();
                let v1_shape = facts
                    .facts
                    .generic_loop_v1
                    .as_ref()
                    .and_then(|f| f.shape_id.as_ref())
                    .is_some();
                let v1_exit_allowed = facts
                    .facts
                    .generic_loop_v1
                    .as_ref()
                    .and_then(|f| f.body_exit_allowed.as_ref())
                    .is_some();
                (v0, v1_shape, v1_exit_allowed)
            });
    let allow_generic_loop = outcome.facts.as_ref().map_or(false, |facts| {
        facts.facts.generic_loop_v0.is_some() || facts.facts.generic_loop_v1.is_some()
    });
    let nested_loop = outcome
        .facts
        .as_ref()
        .map(|facts| facts.nested_loop)
        .unwrap_or_else(|| detect_nested_loop(ctx.body));
    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[plan/trace:nested_loop_guard] func={} nested_loop={} facts_present={} allow_generic_loop={} v0={} v1_shape={} v1_exit_allowed={}",
            ctx.func_name,
            nested_loop,
            facts_present,
            allow_generic_loop,
            v0_present,
            v1_shape,
            v1_exit_allowed
        ));
    }
    if !nested_loop {
        return None;
    }
    if allow_generic_loop {
        return None;
    }
    if allow_strict_nested_pattern4_min1(outcome, ctx) {
        return None;
    }

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[plan/freeze:nested_loop_guard] func={} span={} plan={:?} route_kind={} depth={:?}",
            ctx.func_name,
            ctx.condition.span(),
            outcome.plan,
            ctx.route_kind.semantic_label(),
            ctx.step_tree_max_loop_depth
        ));
    }

    let plan_repr_raw = if outcome.plan.is_some() {
        format!("{:?}", outcome.plan)
    } else {
        outcome.facts.as_ref().and_then(|facts| {
            facts.facts.pattern4_continue.as_ref().map(|pattern4| {
                let mut carrier_vars: Vec<String> =
                    pattern4.carrier_updates.keys().cloned().collect();
                carrier_vars.sort();
                format!(
                    "Some(Pattern4Continue(Pattern4ContinuePlan {{ loop_var: {:?}, carrier_vars: {:?}, condition: {:?}, continue_condition: {:?}, carrier_updates: {:?}, loop_increment: {:?} }}))",
                    pattern4.loop_var,
                    carrier_vars,
                    pattern4.condition,
                    pattern4.continue_condition,
                    pattern4.carrier_updates,
                    pattern4.loop_increment
                )
            })
        }).unwrap_or_else(|| format!("{:?}", outcome.plan))
    };
    let plan_repr =
        crate::mir::builder::control_flow::plan::diagnostics::span_format::normalize_span_line_col(
            &plan_repr_raw,
        );
    let freeze = Freeze::unstructured(&format!(
        "nested loop requires plan/composer support: {} not in strict_nested_loop_guard allowlist",
        plan_repr
    ));
    Some(freeze.to_string())
}

fn allow_strict_nested_pattern4_min1(outcome: &PlanBuildOutcome, ctx: &LoopRouteContext) -> bool {
    if outcome.plan.is_some() {
        return false;
    }
    if ctx.route_kind != LoopPatternKind::Pattern4Continue {
        return false;
    }
    let Some(facts) = outcome.facts.as_ref() else {
        return false;
    };
    if !facts.nested_loop {
        return false;
    }
    if !facts.exit_usage.has_continue || facts.exit_usage.has_break || facts.exit_usage.has_return {
        return false;
    }

    let Some(pattern4) = facts.facts.pattern4_continue.as_ref() else {
        return false;
    };
    if pattern4.carrier_updates.len() != 1 {
        return false;
    }

    let Some(loop_upper_bound) = extract_lt_var_int(&pattern4.condition, &pattern4.loop_var) else {
        return false;
    };
    let Some(continue_lower_bound) =
        extract_ge_var_int(&pattern4.continue_condition, &pattern4.loop_var)
    else {
        return false;
    };

    continue_lower_bound > loop_upper_bound
        && is_add_one_of_var(&pattern4.loop_increment, &pattern4.loop_var)
}

fn extract_lt_var_int(node: &ASTNode, var_name: &str) -> Option<i64> {
    match node {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left,
            right,
            ..
        } if matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == var_name) => {
            match right.as_ref() {
                ASTNode::Literal {
                    value: LiteralValue::Integer(n),
                    ..
                } => Some(*n),
                _ => None,
            }
        }
        _ => None,
    }
}

fn extract_ge_var_int(node: &ASTNode, var_name: &str) -> Option<i64> {
    match node {
        ASTNode::BinaryOp {
            operator: BinaryOperator::GreaterEqual,
            left,
            right,
            ..
        } if matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == var_name) => {
            match right.as_ref() {
                ASTNode::Literal {
                    value: LiteralValue::Integer(n),
                    ..
                } => Some(*n),
                _ => None,
            }
        }
        _ => None,
    }
}

fn is_add_one_of_var(node: &ASTNode, var_name: &str) -> bool {
    matches!(
        node,
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left,
            right,
            ..
        } if matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == var_name)
            && matches!(
                right.as_ref(),
                ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    ..
                }
            )
    )
}

pub(in crate::mir::builder) fn try_shadow_adopt_pre_plan(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    allow_generic_loop: bool,
) -> Result<Option<PrePlanShadowOutcome>, String> {
    let strict_or_dev = joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
    let planner_required = strict_or_dev && joinir_dev::planner_required_enabled();
    if planner_required {
        let freeze = Freeze::contract(
            "shadow_adopt pre-plan is disallowed in planner_required (verified recipe boundary)",
        )
        .to_string();
        return Ok(Some(PrePlanShadowOutcome::GuardError(freeze)));
    }
    if let Some(adopt) = try_adopt_nested_minimal(builder, ctx, outcome)? {
        return Ok(Some(PrePlanShadowOutcome::Adopt(adopt)));
    }

    if let Some(err) = strict_nested_loop_guard(outcome, ctx) {
        return Ok(Some(PrePlanShadowOutcome::GuardError(err)));
    }

    if let Some(adopt) = try_adopt_generic_loop_preferred(
        builder,
        ctx,
        outcome,
        allow_generic_loop,
    )? {
        return Ok(Some(PrePlanShadowOutcome::Adopt(adopt)));
    }

    Ok(None)
}

fn try_adopt_nested_minimal(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<ShadowAdoptOutcome>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    if facts.facts.pattern6_nested_minimal.is_none() {
        return Ok(None);
    }

    let core_plan = try_compose_core_loop_v2_nested_minimal(builder, facts, ctx)?
        .ok_or_else(|| {
            "pattern6 nested minimal strict/dev adopt failed: compose rejected".to_string()
        })?;
    Ok(Some(ShadowAdoptOutcome {
        core_plan,
        emit_flowbox_adopt_tag: facts.nested_loop,
    }))
}

fn generic_loop_v0_gate(facts: &CanonicalLoopFacts) -> bool {
    if !coreloop_base_gate(facts) || facts.value_join_needed {
        return false;
    }
    if facts.facts.pattern2_loopbodylocal.is_some() {
        return false;
    }
    facts.exit_kinds_present.iter().all(|kind| {
        matches!(
            kind,
            ExitKindFacts::Return | ExitKindFacts::Break | ExitKindFacts::Continue
        )
    })
}

fn emit_flowbox_adopt_tag_for_generic(
    facts: &CanonicalLoopFacts,
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    facts.exit_usage.has_continue
        || matches!(
            classify_step_placement(body, loop_var, loop_increment).placement,
            Some(StepPlacement::InBody(_))
        )
}

fn try_adopt_generic_loop_v0(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<ShadowAdoptOutcome>, String> {
    if outcome.plan.is_some() {
        return Ok(None);
    }
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    let Some(generic) = facts.facts.generic_loop_v0.as_ref() else {
        return Ok(None);
    };
    if !generic_loop_v0_gate(facts) {
        return Ok(None);
    }

    let core_plan = normalize_generic_loop_v0(builder, generic, ctx)
        .map_err(|e| format!("generic_loop_v0 strict/dev adopt failed: {}", e))?;
    let emit_flowbox_adopt_tag = emit_flowbox_adopt_tag_for_generic(
        facts,
        generic.body.as_ref(),
        &generic.loop_var,
        &generic.loop_increment,
    );
    Ok(Some(ShadowAdoptOutcome {
        core_plan,
        emit_flowbox_adopt_tag,
    }))
}

fn try_adopt_generic_loop_v1(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<ShadowAdoptOutcome>, String> {
    if outcome.plan.is_some() {
        return Ok(None);
    }
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    let Some(generic) = facts.facts.generic_loop_v1.as_ref() else {
        return Ok(None);
    };
    if !generic_loop_v0_gate(facts) {
        return Ok(None);
    }

    let core_plan = normalize_generic_loop_v1(builder, generic, ctx)
        .map_err(|e| format!("generic_loop_v1 strict/dev adopt failed: {}", e))?;
    let emit_flowbox_adopt_tag = emit_flowbox_adopt_tag_for_generic(
        facts,
        generic.body.as_ref(),
        &generic.loop_var,
        &generic.loop_increment,
    );
    Ok(Some(ShadowAdoptOutcome {
        core_plan,
        emit_flowbox_adopt_tag,
    }))
}
