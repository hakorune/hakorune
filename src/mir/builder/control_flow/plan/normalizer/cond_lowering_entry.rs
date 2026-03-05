//! Phase 2a entry points for condition lowering (SSOT).

use super::cond_lowering_if_plan::lower_cond_to_if_plans;
use super::cond_lowering_value_expr::lower_cond_to_value_impl;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreIfJoin, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{ConstValue, ValueId};
use std::collections::BTreeMap;

/// Branch context: condition -> then/else paths (no value created).
///
/// Use this when lowering a condition for control flow branching (if/loop header).
/// Phase 2b will change signature to `lower_cond(expr, on_true, on_false)`.
///
/// ## Join Contract
///
/// - `joins` is computed by caller (via `build_join_payload`)
/// - Short-circuit expansion (&&/||) creates 3 paths, but joins assumes 2-state model
/// - This function handles the 3-path -> 2-state mapping via intermediate `Copy` values
#[track_caller]
pub fn lower_cond_branch(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    cond: &CondBlockView,
    then_plans: Vec<LoweredRecipe>,
    else_plans: Option<Vec<LoweredRecipe>>,
    joins: Vec<CoreIfJoin>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let plans = lower_cond_to_if_plans(
        builder,
        phi_bindings,
        cond,
        then_plans,
        else_plans,
        joins,
        error_prefix,
    )?;
    debug_log_cond_branch_lit3_origin(builder, &plans, std::panic::Location::caller());
    Ok(plans)
}

/// Value context: condition -> ValueId (creates bool value).
///
/// Use this when lowering a condition to obtain its boolean result as a value.
/// Phase 2b will change to `lower_bool(expr) -> ValueId` with single join.
pub fn lower_cond_value(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    cond: &CondBlockView,
    error_prefix: &str,
) -> Result<(ValueId, Vec<CoreEffectPlan>), String> {
    lower_cond_to_value_impl(builder, phi_bindings, cond, error_prefix)
}

fn debug_log_cond_branch_lit3_origin(
    builder: &MirBuilder,
    plans: &[LoweredRecipe],
    caller: &'static std::panic::Location<'static>,
) {
    if !crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
        return;
    }

    let mut lit3_dsts = Vec::new();
    let mut lit3_spans = Vec::new();
    let mut origin_missing = 0usize;
    collect_lit3_from_plans(
        builder,
        plans,
        &mut lit3_dsts,
        &mut lit3_spans,
        &mut origin_missing,
    );

    if lit3_dsts.is_empty() {
        return;
    }

    let fn_name = builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|f| f.signature.name.as_str())
        .unwrap_or("<none>");
    let const_int3_dsts = lit3_dsts
        .iter()
        .map(|v| format!("%{}", v.0))
        .collect::<Vec<_>>()
        .join(",");
    let origin_spans = lit3_spans.join(",");
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[cond_branch/plans:lit3_origin] fn={} bb={:?} plans_len={} const_int3_dsts=[{}] origin_spans=[{}] origin_missing={} caller={}",
        fn_name,
        builder.current_block,
        plans.len(),
        const_int3_dsts,
        origin_spans,
        origin_missing,
        caller
    ));
}

fn collect_lit3_from_plans(
    builder: &MirBuilder,
    plans: &[LoweredRecipe],
    lit3_dsts: &mut Vec<ValueId>,
    lit3_spans: &mut Vec<String>,
    origin_missing: &mut usize,
) {
    for plan in plans {
        match plan {
            CorePlan::Seq(items) => {
                collect_lit3_from_plans(builder, items, lit3_dsts, lit3_spans, origin_missing);
            }
            CorePlan::If(if_plan) => {
                collect_lit3_from_plans(
                    builder,
                    &if_plan.then_plans,
                    lit3_dsts,
                    lit3_spans,
                    origin_missing,
                );
                if let Some(else_plans) = &if_plan.else_plans {
                    collect_lit3_from_plans(
                        builder,
                        else_plans,
                        lit3_dsts,
                        lit3_spans,
                        origin_missing,
                    );
                }
            }
            CorePlan::Loop(loop_plan) => {
                collect_lit3_from_plans(
                    builder,
                    &loop_plan.body,
                    lit3_dsts,
                    lit3_spans,
                    origin_missing,
                );
                for (_, effects) in &loop_plan.block_effects {
                    collect_lit3_from_effects(
                        builder,
                        effects,
                        lit3_dsts,
                        lit3_spans,
                        origin_missing,
                    );
                }
            }
            CorePlan::BranchN(branch_plan) => {
                for arm in &branch_plan.arms {
                    collect_lit3_from_plans(
                        builder,
                        &arm.plans,
                        lit3_dsts,
                        lit3_spans,
                        origin_missing,
                    );
                }
                if let Some(else_plans) = &branch_plan.else_plans {
                    collect_lit3_from_plans(
                        builder,
                        else_plans,
                        lit3_dsts,
                        lit3_spans,
                        origin_missing,
                    );
                }
            }
            CorePlan::Effect(effect) => {
                collect_lit3_from_effect(effect, builder, lit3_dsts, lit3_spans, origin_missing);
            }
            CorePlan::Exit(_) => {}
        }
    }
}

fn collect_lit3_from_effects(
    builder: &MirBuilder,
    effects: &[CoreEffectPlan],
    lit3_dsts: &mut Vec<ValueId>,
    lit3_spans: &mut Vec<String>,
    origin_missing: &mut usize,
) {
    for effect in effects {
        collect_lit3_from_effect(effect, builder, lit3_dsts, lit3_spans, origin_missing);
    }
}

fn collect_lit3_from_effect(
    effect: &CoreEffectPlan,
    builder: &MirBuilder,
    lit3_dsts: &mut Vec<ValueId>,
    lit3_spans: &mut Vec<String>,
    origin_missing: &mut usize,
) {
    if let CoreEffectPlan::Const { dst, value } = effect {
        if matches!(value, ConstValue::Integer(3)) {
            lit3_dsts.push(*dst);
            if let Some(span) = builder.metadata_ctx.value_span(*dst) {
                lit3_spans.push(span.to_string());
            } else {
                *origin_missing += 1;
            }
        }
    }
}

/// Bool-condition SSOT helper: lower a boolean condition expression to a `ValueId`.
///
/// Contract:
/// - Callers should use this helper for boolean conditions (`==`, `<`, `&&`, `||`, etc.).
/// - Value lowering (`PlanNormalizer::lower_value_ast`) intentionally does not support comparisons.
///
/// This is a thin wrapper around `lower_cond_value()` to make the intended entrypoint explicit.
pub fn lower_bool_expr_value_id(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    cond: &CondBlockView,
    error_prefix: &str,
) -> Result<(ValueId, Vec<CoreEffectPlan>), String> {
    lower_cond_value(builder, phi_bindings, cond, error_prefix)
}
