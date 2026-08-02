//! Phase 29ao P30: shadow adopt pre-plan guard entrypoints.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::config::env::joinir_dev;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::facts::feature_facts::detect_nested_loop;
use crate::mir::builder::control_flow::plan::planner::{Freeze, PlanBuildOutcome};
use crate::mir::builder::control_flow::verify::diagnostics::span_format::normalize_span_line_col;
use crate::mir::builder::control_flow::verify::observability::flowbox_tags;
use crate::mir::loop_route_detection::LoopRouteKind;

pub(in crate::mir::builder) fn strict_nested_loop_guard(
    outcome: &PlanBuildOutcome,
    ctx: &LoopRouteContext,
) -> Option<String> {
    strict_nested_loop_guard_from_observations(
        outcome.facts.as_ref(),
        outcome.recipe_contract.is_some(),
        ctx,
    )
}

/// Applies the strict nested-loop guard to already-observed planner inputs.
///
/// This keeps registry handlers independent of `PlanBuildOutcome`: they receive
/// compose facts explicitly and read contract disposition from their execution
/// witness.
pub(in crate::mir::builder) fn strict_nested_loop_guard_from_observations(
    facts: Option<&CanonicalLoopFacts>,
    recipe_contract_present: bool,
    ctx: &LoopRouteContext,
) -> Option<String> {
    if joinir_dev::debug_enabled() {
        let features = flowbox_tags::features_from_facts(facts);
        let features_csv = features.join(",");
        let recipe_contract_state = if recipe_contract_present {
            "Some"
        } else {
            "None"
        };
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[plan/trace:nested_loop_guard_entry] ctx=shadow_adopt features={} recipe_contract={}",
            features_csv, recipe_contract_state
        ));
    }
    let facts_present = facts.is_some();
    let nested_loop = facts
        .map(|facts| facts.nested_loop)
        .unwrap_or_else(|| detect_nested_loop(ctx.body));
    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[plan/trace:nested_loop_guard] func={} nested_loop={} facts_present={}",
            ctx.func_name, nested_loop, facts_present,
        ));
    }
    if !nested_loop {
        return None;
    }
    if allow_strict_nested_loop_continue_min1(facts, ctx) {
        return None;
    }

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        let _ = ring0.io.stderr_write(
            format!(
                "[plan/trace:nested_loop_guard_func] func={} span={} route_kind={} depth={:?}\n",
                ctx.func_name,
                ctx.condition.span(),
                ctx.route_kind.semantic_label(),
                ctx.step_tree_max_loop_depth
            )
            .as_bytes(),
        );
        ring0.log.debug(&format!(
            "[plan/freeze:nested_loop_guard] func={} span={} recipe_contract={} route_kind={} depth={:?}",
            ctx.func_name,
            ctx.condition.span(),
            recipe_contract_present,
            ctx.route_kind.semantic_label(),
            ctx.step_tree_max_loop_depth
        ));
    }

    let plan_repr_raw = facts
        .and_then(|facts| {
            facts.facts.loop_continue_only().map(|loop_continue| {
                let mut carrier_vars: Vec<String> =
                    loop_continue.carrier_updates.keys().cloned().collect();
                carrier_vars.sort();
                format!(
                    "Some(loop_continue_only(loop_var={:?}, carrier_vars={:?}, condition={:?}, continue_condition={:?}, carrier_updates={:?}, loop_increment={:?}))",
                    loop_continue.loop_var,
                    carrier_vars,
                    loop_continue.condition,
                    loop_continue.continue_condition,
                    loop_continue.carrier_updates,
                    loop_continue.loop_increment
                )
            })
        })
        .unwrap_or_else(|| "None".to_string());
    let plan_repr = normalize_span_line_col(&plan_repr_raw);
    let freeze = Freeze::unstructured(&format!(
        "nested loop requires plan/composer support: {} not in strict_nested_loop_guard allowlist",
        plan_repr
    ));
    Some(freeze.to_string())
}

fn allow_strict_nested_loop_continue_min1(
    facts: Option<&CanonicalLoopFacts>,
    ctx: &LoopRouteContext,
) -> bool {
    if ctx.route_kind != LoopRouteKind::LoopContinueOnly {
        return false;
    }
    let Some(facts) = facts else {
        return false;
    };
    if !facts.nested_loop {
        return false;
    }
    if !facts.exit_usage.has_continue || facts.exit_usage.has_break || facts.exit_usage.has_return {
        return false;
    }

    let Some(loop_continue) = facts.facts.loop_continue_only() else {
        return false;
    };
    if loop_continue.carrier_updates.len() != 1 {
        return false;
    }

    let Some(loop_upper_bound) =
        extract_lt_var_int(&loop_continue.condition, &loop_continue.loop_var)
    else {
        return false;
    };
    let Some(continue_lower_bound) =
        extract_ge_var_int(&loop_continue.continue_condition, &loop_continue.loop_var)
    else {
        return false;
    };

    continue_lower_bound > loop_upper_bound
        && is_add_one_of_var(&loop_continue.loop_increment, &loop_continue.loop_var)
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

pub(in crate::mir::builder) fn shadow_pre_plan_guard_error(
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
) -> Option<String> {
    let strict_or_dev = joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
    let planner_required = strict_or_dev && joinir_dev::planner_required_enabled();
    if planner_required {
        let freeze = Freeze::contract(
            "shadow_adopt pre-plan is disallowed in planner_required (verified recipe boundary)",
        )
        .to_string();
        return Some(freeze);
    }

    strict_nested_loop_guard(outcome, ctx)
}
