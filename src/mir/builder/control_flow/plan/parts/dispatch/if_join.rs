//! If-join lowering - join-related if handling.
//!
//! Contains:
//! - lower_if_join_with_stmt_lowerer
//! - lower_if_join_with_branch_lowerers
//! - lower_value_cond_if_with_filtered_joins

use crate::mir::builder::control_flow::plan::normalizer::lower_cond_branch;
use crate::mir::builder::control_flow::plan::normalizer::lower_cond_value;
use crate::mir::builder::control_flow::plan::steps::build_join_payload;
use crate::mir::builder::control_flow::plan::steps::build_join_payload_filtered;
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeBodies;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeBlock;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::ConstValue;
use std::collections::BTreeMap;

use super::block::{
    lower_block_internal, plans_exit_on_all_paths, BlockKindInternal, BoxedLowerStmtFn,
};
use crate::mir::builder::control_flow::plan::parts::join_scope::{
    collect_branch_local_vars_from_block_recursive, collect_branch_local_vars_from_maps,
    filter_branch_locals_from_maps,
};

fn snapshot_branch_map(
    builder: &MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
) -> BTreeMap<String, crate::mir::ValueId> {
    let mut map = builder.variable_ctx.variable_map.clone();
    for (name, value_id) in current_bindings.iter() {
        map.insert(name.clone(), *value_id);
    }
    map
}

fn branch_is_terminal(plans: &[LoweredRecipe]) -> bool {
    plans_exit_on_all_paths(plans)
}

#[derive(Clone, Copy)]
struct BranchExitShape {
    then_exits: bool,
    one_sided_exit: bool,
    both_sides_exit: bool,
    no_join_continuation: bool,
}

fn analyze_branch_exit_shape(
    then_plans: &[LoweredRecipe],
    else_plans: Option<&[LoweredRecipe]>,
) -> BranchExitShape {
    let then_exits = branch_is_terminal(then_plans);
    let else_exits = else_plans.is_some_and(branch_is_terminal);
    let one_sided_exit = then_exits ^ else_exits;
    let both_sides_exit = then_exits && else_exits;
    BranchExitShape {
        then_exits,
        one_sided_exit,
        both_sides_exit,
        no_join_continuation: one_sided_exit || both_sides_exit,
    }
}

/// Lower if with join payload, using injected stmt lowerer (NoExit context).
pub(super) fn lower_if_join_with_stmt_lowerer<'a>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    arena: &RecipeBodies,
    cond_view: &crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView,
    then_block: &RecipeBlock,
    else_block: Option<&RecipeBlock>,
    error_prefix: &str,
    make_lower_stmt: &mut dyn FnMut() -> BoxedLowerStmtFn<'a>,
    should_update_binding: &dyn Fn(&str, &BTreeMap<String, crate::mir::ValueId>) -> bool,
) -> Result<Vec<LoweredRecipe>, String> {
    // NoExit join branch contract is recursive (Option A).
    // Lower then/else as NoExit blocks so nested joins are supported without re-checking in features.
    let pre_if_map = snapshot_branch_map(builder, current_bindings);
    let pre_bindings = current_bindings.clone();

    let then_plans = {
        let mut make_lower_stmt_boxed = || -> BoxedLowerStmtFn<'_> { make_lower_stmt() };
        lower_block_internal(
            builder,
            current_bindings,
            carrier_step_phis,
            arena,
            then_block,
            error_prefix,
            BlockKindInternal::NoExit {
                break_phi_dsts,
                make_lower_stmt: &mut make_lower_stmt_boxed,
                should_update_binding,
            },
        )?
    };
    let then_map = snapshot_branch_map(builder, current_bindings);

    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();

    let else_plans = match else_block {
        Some(eb) => {
            let mut make_lower_stmt_boxed = || -> BoxedLowerStmtFn<'_> { make_lower_stmt() };
            Some(lower_block_internal(
                builder,
                current_bindings,
                carrier_step_phis,
                arena,
                eb,
                error_prefix,
                BlockKindInternal::NoExit {
                    break_phi_dsts,
                    make_lower_stmt: &mut make_lower_stmt_boxed,
                    should_update_binding,
                },
            )?)
        }
        None => None,
    };
    let else_map = snapshot_branch_map(builder, current_bindings);

    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings;

    let mut branch_locals = collect_branch_local_vars_from_block_recursive(arena, then_block);
    if let Some(else_block) = else_block {
        branch_locals.extend(collect_branch_local_vars_from_block_recursive(
            arena, else_block,
        ));
    }
    let (then_map, else_map) = filter_branch_locals_from_maps(
        &pre_if_map,
        &then_map,
        &else_map,
        &branch_locals,
    );

    // This path lowers branches under NoExit join contract, but keep the same
    // exit-shape signal handling as the general join path to avoid binding
    // join dsts that have no dominating definition.
    let exit_shape = analyze_branch_exit_shape(&then_plans, else_plans.as_deref());
    let joins = if exit_shape.no_join_continuation {
        Vec::new()
    } else {
        build_join_payload(builder, &pre_if_map, &then_map, &else_map)?
    };

    let plans = lower_cond_branch(
        builder,
        current_bindings,
        cond_view,
        then_plans,
        else_plans,
        joins.clone(),
        error_prefix,
    )?;
    debug_log_if_join_lit3_origin(builder, &plans, exit_shape.one_sided_exit);

    if exit_shape.one_sided_exit {
        let continuing_map = if exit_shape.then_exits {
            &else_map
        } else {
            &then_map
        };
        builder.variable_ctx.variable_map = continuing_map.clone();
        for (name, value_id) in continuing_map {
            if should_update_binding(name, current_bindings) {
                current_bindings.insert(name.clone(), *value_id);
            }
        }
    } else if exit_shape.both_sides_exit {
        // Both branches terminate: no continuation bindings to apply.
    } else {
        for join in &joins {
            builder
                .variable_ctx
                .variable_map
                .insert(join.name.clone(), join.dst);
            if should_update_binding(&join.name, current_bindings) {
                current_bindings.insert(join.name.clone(), join.dst);
            }
        }
    }

    Ok(plans)
}

fn debug_log_if_join_lit3_origin(
    builder: &MirBuilder,
    plans: &[LoweredRecipe],
    one_sided_exit: bool,
) {
    if !crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
        return;
    }

    let mut lit3_dsts = Vec::new();
    let mut lit3_spans = Vec::new();
    let mut origin_missing = 0usize;
    collect_lit3_from_plans(builder, plans, &mut lit3_dsts, &mut lit3_spans, &mut origin_missing);

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
        "[if_join/lowered_plans:lit3_origin] fn={} bb={:?} one_sided_exit={} plans_len={} const_int3_dsts=[{}] origin_spans=[{}] origin_missing={}",
        fn_name,
        builder.current_block,
        if one_sided_exit { 1 } else { 0 },
        plans.len(),
        const_int3_dsts,
        origin_spans,
        origin_missing
    ));
}

fn collect_lit3_from_plans(
    builder: &MirBuilder,
    plans: &[LoweredRecipe],
    lit3_dsts: &mut Vec<crate::mir::ValueId>,
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
                    collect_lit3_from_effects(builder, effects, lit3_dsts, lit3_spans, origin_missing);
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
    lit3_dsts: &mut Vec<crate::mir::ValueId>,
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
    lit3_dsts: &mut Vec<crate::mir::ValueId>,
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

/// Lower an if with join payload, using injected then/else branch lowerers.
///
/// SSOT for:
/// - `pre_if_map`/`then_map`/`else_map` snapshots
/// - `build_join_payload(...)`
/// - `lower_cond_branch(..., joins)`
/// - Applying join results to `variable_map` and `current_bindings`
pub(in crate::mir::builder) fn lower_if_join_with_branch_lowerers<
    ShouldUpdateBinding,
>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    cond_view: &crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView,
    error_prefix: &str,
    lower_then: &mut dyn FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, crate::mir::ValueId>,
    ) -> Result<Vec<LoweredRecipe>, String>,
    mut lower_else: Option<
        &mut dyn FnMut(
            &mut MirBuilder,
            &mut BTreeMap<String, crate::mir::ValueId>,
        ) -> Result<Vec<LoweredRecipe>, String>,
    >,
    should_update_binding: &ShouldUpdateBinding,
) -> Result<Vec<LoweredRecipe>, String>
where
    ShouldUpdateBinding: Fn(&str, &BTreeMap<String, crate::mir::ValueId>) -> bool,
{
    let pre_if_map = snapshot_branch_map(builder, current_bindings);
    let pre_bindings = current_bindings.clone();

    let then_plans = lower_then(builder, current_bindings)?;
    let then_map = snapshot_branch_map(builder, current_bindings);

    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();

    let else_plans = match lower_else.as_deref_mut() {
        Some(lower_else) => Some(lower_else(builder, current_bindings)?),
        None => None,
    };
    let else_map = snapshot_branch_map(builder, current_bindings);

    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings;

    // If exactly one branch exits, the `if` has no join point on that side.
    // If both branches exit, there is no continuation join point either.
    // In both cases, join payload must be empty (otherwise cond lowering may append
    // join Copy plans after an Exit, violating V11).
    //
    // After lowering, bindings must reflect the *continuing* branch map.
    let exit_shape = analyze_branch_exit_shape(&then_plans, else_plans.as_deref());
    let branch_locals = collect_branch_local_vars_from_maps(&pre_if_map, &then_map, &else_map);
    let (then_map, else_map) = filter_branch_locals_from_maps(
        &pre_if_map,
        &then_map,
        &else_map,
        &branch_locals,
    );
    let joins = if exit_shape.no_join_continuation {
        Vec::new()
    } else {
        build_join_payload(builder, &pre_if_map, &then_map, &else_map)?
    };

    let plans = lower_cond_branch(
        builder,
        current_bindings,
        cond_view,
        then_plans,
        else_plans,
        joins.clone(),
        error_prefix,
    )?;

    debug_log_if_join_lit3_origin(builder, &plans, exit_shape.one_sided_exit);

    if exit_shape.one_sided_exit {
        let continuing_map = if exit_shape.then_exits {
            &else_map
        } else {
            &then_map
        };
        builder.variable_ctx.variable_map = continuing_map.clone();
        for (name, value_id) in continuing_map {
            if should_update_binding(name, current_bindings) {
                current_bindings.insert(name.clone(), *value_id);
            }
        }
    } else if exit_shape.both_sides_exit {
        // Both branches terminate: no continuation bindings to apply.
    } else {
        for join in &joins {
            builder
                .variable_ctx
                .variable_map
                .insert(join.name.clone(), join.dst);
            if should_update_binding(&join.name, current_bindings) {
                current_bindings.insert(join.name.clone(), join.dst);
            }
        }
    }

    Ok(plans)
}

/// Lower an if with a value condition (`lower_cond_value`) and a filtered join payload.
///
/// SSOT for:
/// - `build_join_payload_filtered(...)`
/// - `lower_cond_value(...)` (effects + condition value)
/// - Applying join results to `variable_map` and `current_bindings`
/// - Constructing `CorePlan::If(CoreIfPlan { .. })`
///
/// Contract:
/// - Caller provides `pre_if_map`/`then_map`/`else_map` snapshots and the already-lowered branch plans.
/// - Join application happens *after* condition lowering (so the condition sees pre-if bindings).
pub(in crate::mir::builder) fn lower_value_cond_if_with_filtered_joins<
    'a,
    I,
    ShouldUpdateBinding,
    OnJoinApplied,
>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    cond_view: &crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView,
    pre_if_map: &BTreeMap<String, crate::mir::ValueId>,
    then_map: &BTreeMap<String, crate::mir::ValueId>,
    else_map: &BTreeMap<String, crate::mir::ValueId>,
    filter_vars: I,
    then_plans: Vec<LoweredRecipe>,
    else_plans: Vec<LoweredRecipe>,
    error_prefix: &str,
    should_update_binding: &ShouldUpdateBinding,
    mut on_join_applied: OnJoinApplied,
) -> Result<Vec<LoweredRecipe>, String>
where
    I: IntoIterator<Item = &'a String>,
    ShouldUpdateBinding: Fn(&str, &BTreeMap<String, crate::mir::ValueId>) -> bool,
    OnJoinApplied: FnMut(&str, crate::mir::ValueId),
{
    let joins = build_join_payload_filtered(builder, pre_if_map, then_map, else_map, filter_vars);

    let (cond_id, cond_effects) =
        lower_cond_value(builder, current_bindings, cond_view, error_prefix)?;

    for join in &joins {
        builder
            .variable_ctx
            .variable_map
            .insert(join.name.clone(), join.dst);
        if should_update_binding(&join.name, current_bindings) {
            current_bindings.insert(join.name.clone(), join.dst);
            on_join_applied(&join.name, join.dst);
        }
    }

    let mut plans = effects_to_plans(cond_effects);
    plans.push(crate::mir::builder::control_flow::plan::CorePlan::If(
        crate::mir::builder::control_flow::plan::CoreIfPlan {
            condition: cond_id,
            then_plans,
            else_plans: Some(else_plans),
            joins,
        },
    ));

    Ok(plans)
}
