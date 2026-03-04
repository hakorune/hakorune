//! Phase 29ao P30: shadow adopt composer entrypoints (Facts -> LoweredRecipe).

use super::coreloop_gates::coreloop_base_gate;
use super::coreloop_v2_nested_minimal::try_compose_core_loop_v2_nested_minimal;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::config::env::joinir_dev;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::facts::feature_facts::{
    detect_nested_loop, ExitKindFacts,
};
use crate::mir::builder::control_flow::plan::canon::generic_loop::{
    classify_step_placement, StepPlacement,
};
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::normalizer::{
    normalize_escape_map_minimal, normalize_generic_loop_v0, normalize_generic_loop_v1,
    normalize_int_to_str_minimal, normalize_is_integer_minimal,
    normalize_split_lines_minimal, normalize_skip_ws_minimal,
    normalize_starts_with_minimal,
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

type TryPrePlanAdoptFn<T> =
    fn(&mut MirBuilder, &LoopPatternContext, &PlanBuildOutcome) -> Result<Option<T>, String>;

fn try_adopt_pre_plan_sequence<T>(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
    allow_generic_loop: bool,
    try_is_integer: TryPrePlanAdoptFn<T>,
    try_starts_with: TryPrePlanAdoptFn<T>,
    try_int_to_str: TryPrePlanAdoptFn<T>,
    try_escape_map: TryPrePlanAdoptFn<T>,
    try_split_lines: TryPrePlanAdoptFn<T>,
    try_skip_ws: TryPrePlanAdoptFn<T>,
    try_generic_v1: TryPrePlanAdoptFn<T>,
    try_generic_v0: TryPrePlanAdoptFn<T>,
) -> Result<Option<T>, String> {
    if let Some(adopt) = try_is_integer(builder, ctx, outcome)? {
        return Ok(Some(adopt));
    }
    if let Some(adopt) = try_starts_with(builder, ctx, outcome)? {
        return Ok(Some(adopt));
    }
    if let Some(adopt) = try_int_to_str(builder, ctx, outcome)? {
        return Ok(Some(adopt));
    }
    if let Some(adopt) = try_escape_map(builder, ctx, outcome)? {
        return Ok(Some(adopt));
    }
    if let Some(adopt) = try_split_lines(builder, ctx, outcome)? {
        return Ok(Some(adopt));
    }
    if let Some(adopt) = try_skip_ws(builder, ctx, outcome)? {
        return Ok(Some(adopt));
    }
    if allow_generic_loop {
        if let Some(adopt) = try_generic_v1(builder, ctx, outcome)? {
            return Ok(Some(adopt));
        }
        if let Some(adopt) = try_generic_v0(builder, ctx, outcome)? {
            return Ok(Some(adopt));
        }
    }

    Ok(None)
}

pub(in crate::mir::builder) fn strict_nested_loop_guard(
    outcome: &PlanBuildOutcome,
    ctx: &LoopPatternContext,
) -> Option<String> {
    if joinir_dev::debug_enabled() {
        let features = flowbox_tags::features_from_facts(outcome.facts.as_ref());
        let features_csv = features.join(",");
        let plan_state = if outcome.plan.is_some() { "Some" } else { "None" };
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[plan/trace:nested_loop_guard_entry] ctx=shadow_adopt features={} plan={}",
            features_csv,
            plan_state
        ));
    }
    let facts_present = outcome.facts.is_some();
    let (v0_present, v1_shape, v1_exit_allowed) = outcome.facts.as_ref().map_or(
        (false, false, false),
        |facts| {
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
        },
    );
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
            "[plan/freeze:nested_loop_guard] func={} span={} plan={:?} pattern={:?} depth={:?}",
            ctx.func_name,
            ctx.condition.span(),
            outcome.plan,
            ctx.pattern_kind,
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
    let plan_repr = crate::mir::builder::control_flow::plan::diagnostics::span_format::normalize_span_line_col(
        &plan_repr_raw,
    );
    let freeze = Freeze::unstructured(&format!(
        "nested loop requires plan/composer support: {} not in strict_nested_loop_guard allowlist",
        plan_repr
    ));
    Some(freeze.to_string())
}

fn allow_strict_nested_pattern4_min1(
    outcome: &PlanBuildOutcome,
    ctx: &LoopPatternContext,
) -> bool {
    if outcome.plan.is_some() {
        return false;
    }
    if ctx.pattern_kind != LoopPatternKind::Pattern4Continue {
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
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
    allow_generic_loop: bool,
) -> Result<Option<PrePlanShadowOutcome>, String> {
    let strict_or_dev =
        joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
    let planner_required = strict_or_dev && joinir_dev::planner_required_enabled();
    if planner_required {
        let freeze = Freeze::contract(
            "shadow_adopt pre-plan is disallowed in planner_required (verified recipe boundary)",
        )
        .to_string();
        return Ok(Some(PrePlanShadowOutcome::GuardError(freeze)));
    }
    if let Some(adopt) = try_shadow_adopt_nested_minimal(builder, ctx, outcome)? {
        return Ok(Some(PrePlanShadowOutcome::Adopt(adopt)));
    }

    if let Some(err) = strict_nested_loop_guard(outcome, ctx) {
        return Ok(Some(PrePlanShadowOutcome::GuardError(err)));
    }

    if let Some(adopt) = try_adopt_pre_plan_sequence(
        builder,
        ctx,
        outcome,
        allow_generic_loop,
        try_shadow_adopt_is_integer,
        try_shadow_adopt_starts_with,
        try_shadow_adopt_int_to_str,
        try_shadow_adopt_escape_map,
        try_shadow_adopt_split_lines,
        try_shadow_adopt_skip_ws,
        try_shadow_adopt_generic_loop_v1,
        try_shadow_adopt_generic_loop_v0,
    )? {
        return Ok(Some(PrePlanShadowOutcome::Adopt(adopt)));
    }

    Ok(None)
}

pub(in crate::mir::builder) fn try_release_adopt_pre_plan(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
    allow_generic_loop: bool,
) -> Result<Option<LoweredRecipe>, String> {
    if let Some(core_plan) = try_release_adopt_nested_minimal(builder, ctx, outcome)? {
        return Ok(Some(core_plan));
    }

    if let Some(core_plan) = try_adopt_pre_plan_sequence(
        builder,
        ctx,
        outcome,
        allow_generic_loop,
        try_release_adopt_is_integer,
        try_release_adopt_starts_with,
        try_release_adopt_int_to_str,
        try_release_adopt_escape_map,
        try_release_adopt_split_lines,
        try_release_adopt_skip_ws,
        try_release_adopt_generic_loop_v1,
        try_release_adopt_generic_loop_v0,
    )? {
        return Ok(Some(core_plan));
    }

    Ok(None)
}

pub(in crate::mir::builder) fn try_shadow_adopt_nested_minimal(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
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

pub(in crate::mir::builder) fn try_shadow_adopt_is_integer(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<ShadowAdoptOutcome>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    let Some(is_integer) = facts.facts.pattern_is_integer.as_ref() else {
        return Ok(None);
    };

    let core_plan = normalize_is_integer_minimal(builder, is_integer, ctx)
        .map_err(|e| format!("is_integer strict/dev adopt failed: {}", e))?;
    Ok(Some(ShadowAdoptOutcome {
        core_plan,
        emit_flowbox_adopt_tag: facts.exit_usage.has_continue,
    }))
}

pub(in crate::mir::builder) fn try_shadow_adopt_starts_with(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<ShadowAdoptOutcome>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    let Some(starts_with) = facts.facts.pattern_starts_with.as_ref() else {
        return Ok(None);
    };

    let core_plan = normalize_starts_with_minimal(builder, starts_with, ctx)
        .map_err(|e| format!("starts_with strict/dev adopt failed: {}", e))?;
    Ok(Some(ShadowAdoptOutcome {
        core_plan,
        emit_flowbox_adopt_tag: true,
    }))
}

pub(in crate::mir::builder) fn try_shadow_adopt_int_to_str(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<ShadowAdoptOutcome>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    let Some(int_to_str) = facts.facts.pattern_int_to_str.as_ref() else {
        return Ok(None);
    };

    let core_plan = normalize_int_to_str_minimal(builder, int_to_str, ctx)
        .map_err(|e| format!("int_to_str strict/dev adopt failed: {}", e))?;
    Ok(Some(ShadowAdoptOutcome {
        core_plan,
        emit_flowbox_adopt_tag: true,
    }))
}

pub(in crate::mir::builder) fn try_shadow_adopt_escape_map(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<ShadowAdoptOutcome>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    let Some(escape_map) = facts.facts.pattern_escape_map.as_ref() else {
        return Ok(None);
    };

    let core_plan = normalize_escape_map_minimal(builder, escape_map, ctx)
        .map_err(|e| format!("escape_map strict/dev adopt failed: {}", e))?;
    Ok(Some(ShadowAdoptOutcome {
        core_plan,
        emit_flowbox_adopt_tag: true,
    }))
}

pub(in crate::mir::builder) fn try_shadow_adopt_split_lines(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<ShadowAdoptOutcome>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    let Some(split_lines) = facts.facts.pattern_split_lines.as_ref() else {
        return Ok(None);
    };

    let core_plan = normalize_split_lines_minimal(builder, split_lines, ctx)
        .map_err(|e| format!("split_lines strict/dev adopt failed: {}", e))?;
    Ok(Some(ShadowAdoptOutcome {
        core_plan,
        emit_flowbox_adopt_tag: true,
    }))
}

pub(in crate::mir::builder) fn try_shadow_adopt_skip_ws(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<ShadowAdoptOutcome>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    let Some(skip_ws) = facts.facts.pattern_skip_ws.as_ref() else {
        return Ok(None);
    };

    let core_plan = normalize_skip_ws_minimal(builder, skip_ws, ctx)
        .map_err(|e| format!("skip_ws strict/dev adopt failed: {}", e))?;
    Ok(Some(ShadowAdoptOutcome {
        core_plan,
        emit_flowbox_adopt_tag: true,
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

pub(in crate::mir::builder) fn try_shadow_adopt_generic_loop_v0(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
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
    let emit_flowbox_adopt_tag = facts.exit_usage.has_continue
        || matches!(
            classify_step_placement(
                generic.body.as_ref(),
                &generic.loop_var,
                &generic.loop_increment
            )
            .placement,
            Some(StepPlacement::InBody(_))
        );
    Ok(Some(ShadowAdoptOutcome {
        core_plan,
        emit_flowbox_adopt_tag,
    }))
}

pub(in crate::mir::builder) fn try_shadow_adopt_generic_loop_v1(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
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
    let emit_flowbox_adopt_tag = facts.exit_usage.has_continue
        || matches!(
            classify_step_placement(
                generic.body.as_ref(),
                &generic.loop_var,
                &generic.loop_increment
            )
            .placement,
            Some(StepPlacement::InBody(_))
        );
    Ok(Some(ShadowAdoptOutcome {
        core_plan,
        emit_flowbox_adopt_tag,
    }))
}

pub(in crate::mir::builder) fn try_release_adopt_is_integer(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<LoweredRecipe>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    let Some(is_integer) = facts.facts.pattern_is_integer.as_ref() else {
        return Ok(None);
    };

    match normalize_is_integer_minimal(builder, is_integer, ctx) {
        Ok(core) => Ok(Some(core)),
        Err(_) => Ok(None),
    }
}

pub(in crate::mir::builder) fn try_release_adopt_int_to_str(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<LoweredRecipe>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    let Some(int_to_str) = facts.facts.pattern_int_to_str.as_ref() else {
        return Ok(None);
    };

    match normalize_int_to_str_minimal(builder, int_to_str, ctx) {
        Ok(core) => Ok(Some(core)),
        Err(_) => Ok(None),
    }
}

pub(in crate::mir::builder) fn try_release_adopt_starts_with(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<LoweredRecipe>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    let Some(starts_with) = facts.facts.pattern_starts_with.as_ref() else {
        return Ok(None);
    };

    match normalize_starts_with_minimal(builder, starts_with, ctx) {
        Ok(core) => Ok(Some(core)),
        Err(_) => Ok(None),
    }
}

pub(in crate::mir::builder) fn try_release_adopt_escape_map(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<LoweredRecipe>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    let Some(escape_map) = facts.facts.pattern_escape_map.as_ref() else {
        return Ok(None);
    };

    match normalize_escape_map_minimal(builder, escape_map, ctx) {
        Ok(core) => Ok(Some(core)),
        Err(_) => Ok(None),
    }
}

pub(in crate::mir::builder) fn try_release_adopt_split_lines(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<LoweredRecipe>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    let Some(split_lines) = facts.facts.pattern_split_lines.as_ref() else {
        return Ok(None);
    };

    match normalize_split_lines_minimal(builder, split_lines, ctx) {
        Ok(core) => Ok(Some(core)),
        Err(_) => Ok(None),
    }
}

pub(in crate::mir::builder) fn try_release_adopt_skip_ws(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<LoweredRecipe>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    let Some(skip_ws) = facts.facts.pattern_skip_ws.as_ref() else {
        return Ok(None);
    };

    match normalize_skip_ws_minimal(builder, skip_ws, ctx) {
        Ok(core) => Ok(Some(core)),
        Err(_) => Ok(None),
    }
}

pub(in crate::mir::builder) fn try_release_adopt_generic_loop_v0(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<LoweredRecipe>, String> {
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

    match normalize_generic_loop_v0(builder, generic, ctx) {
        Ok(core) => Ok(Some(core)),
        Err(_) => Ok(None),
    }
}

pub(in crate::mir::builder) fn try_release_adopt_generic_loop_v1(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<LoweredRecipe>, String> {
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

    match normalize_generic_loop_v1(builder, generic, ctx) {
        Ok(core) => Ok(Some(core)),
        Err(_) => Ok(None),
    }
}

pub(in crate::mir::builder) fn try_release_adopt_nested_minimal(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    outcome: &PlanBuildOutcome,
) -> Result<Option<LoweredRecipe>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    if facts.facts.pattern6_nested_minimal.is_none() {
        return Ok(None);
    }

    match try_compose_core_loop_v2_nested_minimal(builder, facts, ctx) {
        Ok(Some(core)) => Ok(Some(core)),
        Ok(None) | Err(_) => Ok(None),
    }
}
