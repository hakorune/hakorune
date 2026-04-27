//! If-plan lowering for conditions.

use super::cond_lowering_freshen::clone_plans_with_fresh_loops;
use super::cond_lowering_prelude::lower_cond_prelude_stmts;
use super::cond_lowering_value_expr::lower_cond_value_expr;
use crate::ast::{ASTNode, BinaryOperator, UnaryOperator};
use crate::mir::builder::control_flow::cleanup::policies::cond_prelude_vocab::prelude_has_loop_like_stmt;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::parts::entry as parts_entry;
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreIfJoin, CoreIfPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{ConstValue, MirType, ValueId};
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
    let cond_plans = lower_cond_expr_to_if_plans(
        builder,
        &bindings,
        &cond.tail_expr,
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

    let mut cond_plans = lower_cond_expr_to_if_plans(
        builder,
        &bindings,
        &cond.tail_expr,
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
        | ASTNode::While { body, .. }
        | ASTNode::ForRange { body, .. }
        | ASTNode::ScopeBox { body, .. } => prelude_writes_outer_binding(body, outer_bindings),
        ASTNode::Program { statements, .. } => {
            prelude_writes_outer_binding(statements, outer_bindings)
        }
        _ => false,
    }
}

fn lower_cond_expr_to_if_plans(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    expr: &ASTNode,
    then_plans: Vec<LoweredRecipe>,
    else_plans: Option<Vec<LoweredRecipe>>,
    joins: Vec<CoreIfJoin>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let (then_plans, else_plans) = normalize_empty_branches(then_plans, else_plans);
    let else_plans = Some(else_plans.unwrap_or_else(|| vec![CorePlan::Seq(Vec::new())]));

    match expr {
        ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            operand,
            ..
        } => {
            let new_then = else_plans.unwrap_or_default();
            let new_else = Some(then_plans);
            lower_cond_expr_to_if_plans(
                builder,
                phi_bindings,
                operand,
                new_then,
                new_else,
                joins,
                error_prefix,
            )
        }
        ASTNode::BinaryOp {
            operator: BinaryOperator::And,
            left,
            right,
            ..
        } => {
            let (else_plans_for_right, joins_for_right) = match else_plans.as_ref() {
                Some(plans) => {
                    let fresh = clone_plans_with_fresh_loops(builder, plans)?;
                    (
                        Some(fresh.plans),
                        remap_joins_with_map(&joins, &fresh.value_map),
                    )
                }
                None => (None, joins.clone()),
            };

            let mut intermediate_vals = Vec::new();
            let mut right_joins = Vec::new();
            for j in &joins_for_right {
                let ty = builder
                    .type_ctx
                    .get_type(j.then_val)
                    .or_else(|| builder.type_ctx.get_type(j.else_val))
                    .cloned()
                    .unwrap_or(MirType::Unknown);
                let intermediate = builder.alloc_typed(ty);
                intermediate_vals.push((j.name.clone(), intermediate, j.else_val));

                // Create join for right's if-merge: intermediate = PHI(j.then_val, j.else_val)
                // This defines intermediate ONCE via PHI in right's merge block (SSA-compliant)
                right_joins.push(CoreIfJoin {
                    name: j.name.clone(),
                    dst: intermediate,
                    pre_val: None,
                    then_val: j.then_val,
                    else_val: j.else_val,
                });
            }

            // Pass original plans directly (no Copy augmentation)
            let inner_plans = lower_cond_expr_to_if_plans(
                builder,
                phi_bindings,
                right,
                then_plans,
                else_plans_for_right,
                right_joins,
                error_prefix,
            )?;

            // Outer joins use intermediate (preserves short-circuit semantics)
            let outer_joins: Vec<CoreIfJoin> = joins
                .iter()
                .zip(intermediate_vals.iter())
                .map(|(j, (_, intermediate, _))| CoreIfJoin {
                    name: j.name.clone(),
                    dst: j.dst,
                    pre_val: j.pre_val,
                    then_val: *intermediate,
                    else_val: j.else_val,
                })
                .collect();
            lower_cond_expr_to_if_plans(
                builder,
                phi_bindings,
                left,
                inner_plans,
                else_plans,
                outer_joins,
                error_prefix,
            )
        }
        ASTNode::BinaryOp {
            operator: BinaryOperator::Or,
            left,
            right,
            ..
        } => {
            let then_fresh = clone_plans_with_fresh_loops(builder, &then_plans)?;
            let else_fresh = match else_plans.as_ref() {
                Some(plans) => Some(clone_plans_with_fresh_loops(builder, plans)?),
                None => None,
            };
            let value_map_for_right = merge_value_maps(
                builder,
                &then_fresh.value_map,
                else_fresh.as_ref().map(|f| &f.value_map),
            )?;
            let joins_for_right = remap_joins_with_map(&joins, &value_map_for_right);

            let then_plans_for_right = then_fresh.plans;
            let else_plans_for_right = else_fresh.map(|f| f.plans);

            let mut intermediate_vals = Vec::new();
            let mut right_joins = Vec::new();
            for j in &joins_for_right {
                let ty = builder
                    .type_ctx
                    .get_type(j.then_val)
                    .or_else(|| builder.type_ctx.get_type(j.else_val))
                    .cloned()
                    .unwrap_or(MirType::Unknown);
                let intermediate = builder.alloc_typed(ty);
                intermediate_vals.push((j.name.clone(), intermediate, j.then_val));

                // Create join for right's if-merge: intermediate = PHI(j.then_val, j.else_val)
                // This defines intermediate ONCE via PHI in right's merge block (SSA-compliant)
                right_joins.push(CoreIfJoin {
                    name: j.name.clone(),
                    dst: intermediate,
                    pre_val: None,
                    then_val: j.then_val,
                    else_val: j.else_val,
                });
            }

            // Pass original plans directly (no Copy augmentation)
            let inner_plans = lower_cond_expr_to_if_plans(
                builder,
                phi_bindings,
                right,
                then_plans_for_right,
                else_plans_for_right,
                right_joins,
                error_prefix,
            )?;

            // Outer joins use intermediate (preserves short-circuit semantics)
            let outer_joins: Vec<CoreIfJoin> = joins
                .iter()
                .zip(intermediate_vals.iter())
                .map(|(j, (_, intermediate, _))| CoreIfJoin {
                    name: j.name.clone(),
                    dst: j.dst,
                    pre_val: j.pre_val,
                    then_val: j.then_val,
                    else_val: *intermediate,
                })
                .collect();
            lower_cond_expr_to_if_plans(
                builder,
                phi_bindings,
                left,
                then_plans,
                Some(inner_plans),
                outer_joins,
                error_prefix,
            )
        }
        _ => {
            let (cond_id, cond_effects) =
                lower_cond_value_expr(builder, phi_bindings, expr, error_prefix)?;
            debug_log_cond_if_lit3_origin(builder, &cond_effects);
            let mut plans = effects_to_plans(cond_effects);
            plans.push(CorePlan::If(CoreIfPlan {
                condition: cond_id,
                then_plans,
                else_plans,
                joins,
            }));
            Ok(plans)
        }
    }
}

fn debug_log_cond_if_lit3_origin(builder: &MirBuilder, effects: &[CoreEffectPlan]) {
    if !crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
        return;
    }

    let mut lit3_spans: Vec<String> = Vec::new();
    let mut lit3_dsts: Vec<ValueId> = Vec::new();
    for effect in effects {
        if let CoreEffectPlan::Const { dst, value } = effect {
            if matches!(value, ConstValue::Integer(3)) {
                if let Some(span) = builder.metadata_ctx.value_span(*dst) {
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
    let span_list = lit3_spans.join(",");
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[cond_if/effects:lit3_origin] fn={} bb={:?} effects_len={} const_int3_dsts=[{}] origin_spans=[{}]",
        fn_name,
        builder.current_block,
        effects.len(),
        const_int3_dsts,
        span_list
    ));
}

fn normalize_empty_branches(
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

fn remap_joins_with_map(
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

fn merge_value_maps(
    builder: &MirBuilder,
    primary: &BTreeMap<ValueId, ValueId>,
    secondary: Option<&BTreeMap<ValueId, ValueId>>,
) -> Result<BTreeMap<ValueId, ValueId>, String> {
    let mut merged = primary.clone();
    let strict_planner_required = crate::config::env::joinir_dev::strict_enabled()
        && crate::config::env::joinir_dev::planner_required_enabled();
    let fn_name = builder
        .scope_ctx
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
