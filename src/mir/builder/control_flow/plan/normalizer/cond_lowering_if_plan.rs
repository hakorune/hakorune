//! If-plan lowering for conditions.

use super::cond_lowering_freshen::clone_plans_with_fresh_loops;
use super::cond_lowering_if_plan_port::lower_cond_expr_to_if_plans_input;
use super::cond_lowering_prelude::lower_cond_prelude_stmts;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::cleanup::policies::cond_prelude_vocab::prelude_has_loop_like_stmt;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::parts::entry as parts_entry;
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreIfJoin, CorePlan, LoweredRecipe, RawLoopPlanExpressionPortV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{ConstValue, ValueId};
use std::collections::BTreeMap;

pub(super) fn lower_cond_to_if_plans(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    cond: &CondBlockView,
    then_plans: Vec<LoweredRecipe>,
    else_plans: Option<Vec<LoweredRecipe>>,
    joins: Vec<CoreIfJoin>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    if prelude_has_loop_like_stmt(&cond.prelude_stmts) {
        return lower_cond_to_if_plans_with_plan_prelude(
            builder,
            phi_bindings,
            cond,
            then_plans,
            else_plans,
            joins,
            error_prefix,
        );
    }

    let (bindings, prelude_effects) =
        lower_cond_prelude_stmts(builder, phi_bindings, &cond.prelude_stmts, error_prefix)?;
    let port = RawLoopPlanExpressionPortV1::new();
    let cond_plans = lower_cond_expr_to_if_plans_input(
        &port,
        port.expr(&cond.tail_expr),
        builder,
        &bindings,
        then_plans,
        else_plans,
        joins,
        error_prefix,
    )?;

    if prelude_effects.is_empty() {
        return Ok(cond_plans);
    }

    let mut plans = effects_to_plans(prelude_effects);
    plans.extend(cond_plans);
    Ok(plans)
}

fn lower_cond_to_if_plans_with_plan_prelude(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    cond: &CondBlockView,
    then_plans: Vec<LoweredRecipe>,
    else_plans: Option<Vec<LoweredRecipe>>,
    joins: Vec<CoreIfJoin>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    if prelude_writes_outer_binding(&cond.prelude_stmts, phi_bindings) {
        return Err(format!(
            "[freeze:contract][cond_prelude] {error_prefix}: loop-like prelude cannot write outer bindings in branch-plan route"
        ));
    }

    let mut bindings = phi_bindings.clone();
    let mut prelude_plans = Vec::new();
    for stmt in &cond.prelude_stmts {
        if stmt.contains_non_local_exit_outside_loops() {
            return Err(
                "[freeze:contract][cond_prelude] exit stmt is forbidden in condition prelude"
                    .to_string(),
            );
        }
        let mut stmt_plans = parts_entry::lower_cond_prelude_stmt_as_plan(
            builder,
            &mut bindings,
            stmt,
            error_prefix,
        )?;
        prelude_plans.append(&mut stmt_plans);
    }

    let port = RawLoopPlanExpressionPortV1::new();
    let mut cond_plans = lower_cond_expr_to_if_plans_input(
        &port,
        port.expr(&cond.tail_expr),
        builder,
        &bindings,
        then_plans,
        else_plans,
        joins,
        error_prefix,
    )?;
    prelude_plans.append(&mut cond_plans);
    Ok(prelude_plans)
}

fn prelude_writes_outer_binding(
    stmts: &[ASTNode],
    outer_bindings: &BTreeMap<String, ValueId>,
) -> bool {
    stmts
        .iter()
        .any(|stmt| stmt_writes_outer_binding(stmt, outer_bindings))
}

fn stmt_writes_outer_binding(stmt: &ASTNode, outer_bindings: &BTreeMap<String, ValueId>) -> bool {
    match stmt {
        ASTNode::Assignment { target, .. } => matches!(
            target.as_ref(),
            ASTNode::Variable { name, .. } if outer_bindings.contains_key(name)
        ),
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            prelude_writes_outer_binding(then_body, outer_bindings)
                || else_body
                    .as_ref()
                    .is_some_and(|body| prelude_writes_outer_binding(body, outer_bindings))
        }
        ASTNode::Loop { body, .. }
        | ASTNode::LoopRange { body, .. }
        | ASTNode::ScopeBox { body, .. } => prelude_writes_outer_binding(body, outer_bindings),
        ASTNode::Program { statements, .. } => {
            prelude_writes_outer_binding(statements, outer_bindings)
        }
        _ => false,
    }
}

pub(super) fn clone_branch_plans_for_shortcircuit(
    builder: &mut MirBuilder,
    plans: &[LoweredRecipe],
) -> Result<Vec<LoweredRecipe>, String> {
    // Short-circuit lowering may place the same branch template under multiple
    // leaf tests. Even "simple" non-loop branches can contain Const/BinOp/Call
    // definitions, so a plain Vec clone would emit the same ValueId in multiple
    // blocks. Branches without definition sites are safe to clone directly.
    if plans_have_definition_sites(plans) {
        Ok(clone_plans_with_fresh_loops(builder, plans)?.plans)
    } else {
        Ok(plans.to_vec())
    }
}

fn plans_have_definition_sites(plans: &[LoweredRecipe]) -> bool {
    plans.iter().any(plan_has_definition_site)
}

fn plan_has_definition_site(plan: &LoweredRecipe) -> bool {
    match plan {
        CorePlan::Seq(inner) => plans_have_definition_sites(inner),
        CorePlan::If(if_plan) => {
            plans_have_definition_sites(&if_plan.then_plans)
                || if_plan
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| plans_have_definition_sites(plans))
                || !if_plan.joins.is_empty()
        }
        CorePlan::Loop(_) => true,
        CorePlan::BranchN(branch_plan) => {
            branch_plan
                .arms
                .iter()
                .any(|arm| plans_have_definition_sites(&arm.plans))
                || branch_plan
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| plans_have_definition_sites(plans))
        }
        CorePlan::Effect(effect) => effect_defines_value(effect),
        CorePlan::Exit(_) => false,
    }
}

fn effect_defines_value(effect: &CoreEffectPlan) -> bool {
    match effect {
        CoreEffectPlan::MethodCall { dst, .. }
        | CoreEffectPlan::GlobalCall { dst, .. }
        | CoreEffectPlan::ValueCall { dst, .. }
        | CoreEffectPlan::ExternCall { dst, .. } => dst.is_some(),
        CoreEffectPlan::NewBox { .. }
        | CoreEffectPlan::VariantMake { .. }
        | CoreEffectPlan::BinOp { .. }
        | CoreEffectPlan::Compare { .. }
        | CoreEffectPlan::Select { .. }
        | CoreEffectPlan::Const { .. }
        | CoreEffectPlan::Copy { .. }
        | CoreEffectPlan::LocalContractWrite { .. }
        | CoreEffectPlan::FieldGet { .. } => true,
        CoreEffectPlan::FieldSet { .. }
        | CoreEffectPlan::ExitIf { .. }
        | CoreEffectPlan::IfEffect { .. } => false,
    }
}

pub(super) fn debug_log_cond_if_lit3_origin(builder: &MirBuilder, effects: &[CoreEffectPlan]) {
    if !crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
        return;
    }

    let mut lit3_spans: Vec<String> = Vec::new();
    let mut lit3_dsts: Vec<ValueId> = Vec::new();
    for effect in effects {
        if let CoreEffectPlan::Const { dst, value } = effect {
            if matches!(value, ConstValue::Integer(3)) {
                if let Some(span) = builder.value_origin_span(*dst) {
                    lit3_spans.push(span.to_string());
                    lit3_dsts.push(*dst);
                }
            }
        }
    }

    if lit3_dsts.is_empty() {
        return;
    }

    let fn_name = builder
        .function_state
        .current_function
        .as_ref()
        .map(|f| f.signature.name.as_str())
        .unwrap_or("<none>");
    let const_int3_dsts = lit3_dsts
        .iter()
        .map(|v| format!("%{}", v.0))
        .collect::<Vec<_>>()
        .join(",");
    let span_list = lit3_spans.join(",");
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[cond_if/effects:lit3_origin] fn={} bb={:?} effects_len={} const_int3_dsts=[{}] origin_spans=[{}]",
        fn_name,
        builder.function_state.current_block,
        effects.len(),
        const_int3_dsts,
        span_list
    ));
}

pub(super) fn normalize_empty_branches(
    then_plans: Vec<LoweredRecipe>,
    else_plans: Option<Vec<LoweredRecipe>>,
) -> (Vec<LoweredRecipe>, Option<Vec<LoweredRecipe>>) {
    let then_plans = if then_plans.is_empty() {
        vec![CorePlan::Seq(Vec::new())]
    } else {
        then_plans
    };
    let else_plans = else_plans.map(|plans| {
        if plans.is_empty() {
            vec![CorePlan::Seq(Vec::new())]
        } else {
            plans
        }
    });
    (then_plans, else_plans)
}

pub(super) fn remap_joins_with_map(
    joins: &[CoreIfJoin],
    value_map: &BTreeMap<ValueId, ValueId>,
) -> Vec<CoreIfJoin> {
    joins
        .iter()
        .map(|j| CoreIfJoin {
            name: j.name.clone(),
            dst: value_map.get(&j.dst).copied().unwrap_or(j.dst),
            pre_val: j.pre_val.map(|v| value_map.get(&v).copied().unwrap_or(v)),
            then_val: value_map.get(&j.then_val).copied().unwrap_or(j.then_val),
            else_val: value_map.get(&j.else_val).copied().unwrap_or(j.else_val),
        })
        .collect()
}

pub(super) fn merge_value_maps(
    builder: &MirBuilder,
    primary: &BTreeMap<ValueId, ValueId>,
    secondary: Option<&BTreeMap<ValueId, ValueId>>,
) -> Result<BTreeMap<ValueId, ValueId>, String> {
    let mut merged = primary.clone();
    let strict_planner_required = crate::config::env::joinir_dev::strict_enabled()
        && crate::config::env::joinir_dev::planner_required_enabled();
    let fn_name = builder
        .function_state
        .current_function
        .as_ref()
        .map(|f| f.signature.name.as_str())
        .unwrap_or("<none>");
    if let Some(secondary) = secondary {
        for (old, new) in secondary {
            if let Some(existing) = merged.get(old) {
                if existing != new && strict_planner_required {
                    return Err(format!(
                        "[freeze:contract][cond_freshen/merge_map_conflict] fn={} old=%{} new1=%{} new2=%{}",
                        fn_name,
                        old.0,
                        existing.0,
                        new.0
                    ));
                }
            } else {
                merged.insert(*old, *new);
            }
        }
    }
    Ok(merged)
}
