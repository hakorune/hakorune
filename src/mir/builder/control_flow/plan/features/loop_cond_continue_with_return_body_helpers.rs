//! Route-local body helpers for `LoopCondContinueWithReturn`.
//!
//! Scope:
//! - item dispatch inside the verified route
//! - `continue_if` / `hetero_return_if` lowering
//! - recursive statement lowering for this route only

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::features::body_view::BodyView;
use crate::mir::builder::control_flow::plan::features::carrier_merge::{
    lower_assignment_stmt, lower_local_init_stmt,
};
use crate::mir::builder::control_flow::plan::features::if_branch_lowering;
use crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_recipe::ContinueWithReturnItem;
use crate::mir::builder::control_flow::plan::normalizer::{loop_body_lowering, PlanNormalizer};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::steps::{effects_to_plans, lower_stmt_block};
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::{Effect, EffectMask};
use std::collections::BTreeMap;

#[inline]
fn trace_collection_len(tag: &str, len: usize) {
    if crate::config::env::is_joinir_debug() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug(&format!("[plan/trace] {}: len={}", tag, len));
    }
}

pub(in crate::mir::builder) fn lower_continue_with_return_block(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    body: &BodyView<'_>,
    items: &[ContinueWithReturnItem],
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let mut plans = Vec::new();
    for item in items {
        plans.extend(lower_continue_with_return_item(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            carrier_updates,
            body,
            item,
            error_prefix,
        )?);
    }
    Ok(plans)
}

fn get_body_stmt<'a>(
    body: &BodyView<'a>,
    stmt_ref: StmtRef,
    error_prefix: &str,
) -> Result<&'a ASTNode, String> {
    body.get_stmt(stmt_ref)
        .ok_or_else(|| format!("{error_prefix}: missing stmt idx={}", stmt_ref.index()))
}

fn lower_continue_with_return_item(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    body: &BodyView<'_>,
    item: &ContinueWithReturnItem,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    match item {
        ContinueWithReturnItem::Stmt(stmt_ref) => {
            let stmt = get_body_stmt(body, *stmt_ref, error_prefix)?;
            lower_stmt_ast(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                stmt,
                error_prefix,
            )
        }
        ContinueWithReturnItem::ContinueIf {
            if_stmt,
            prelude_span,
            prelude_items,
        } => {
            let stmt = get_body_stmt(body, *if_stmt, error_prefix)?;
            let ASTNode::If {
                condition,
                then_body,
                else_body: _,
                ..
            } = stmt
            else {
                return Err(format!("{error_prefix}: continue_if is not if"));
            };
            let (prelude_start, prelude_end) = prelude_span.indices();
            if prelude_end > then_body.len() || prelude_start > prelude_end {
                return Err(format!(
                    "{error_prefix}: continue_if prelude span out of range"
                ));
            }
            let prelude_body = &then_body[prelude_start..prelude_end];
            lower_continue_if(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                condition,
                prelude_body,
                prelude_items,
                error_prefix,
            )
        }
        ContinueWithReturnItem::HeteroReturnIf { if_stmt } => {
            let stmt = get_body_stmt(body, *if_stmt, error_prefix)?;
            let ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } = stmt
            else {
                return Err(format!("{error_prefix}: hetero_return_if is not if"));
            };
            let then_assignment = then_body
                .first()
                .ok_or_else(|| format!("{error_prefix}: hetero_return_if empty then_body"))?;
            let Some(else_body) = else_body.as_ref() else {
                return Err(format!(
                    "{error_prefix}: hetero_return_if missing else_body"
                ));
            };
            lower_hetero_return_if(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                condition,
                then_assignment,
                else_body,
                error_prefix,
            )
        }
        ContinueWithReturnItem::IfAny(stmt_ref) => {
            let stmt = get_body_stmt(body, *stmt_ref, error_prefix)?;
            let ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } = stmt
            else {
                return Err(format!("{error_prefix}: if_any is not if"));
            };
            if let Some(plans) = parts::entry::lower_conditional_update_if(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                condition,
                then_body,
                else_body.as_ref(),
                error_prefix,
            )? {
                return Ok(plans);
            }

            let saved_bindings = current_bindings.clone();
            let then_plans = lower_stmt_block(then_body, |stmt| {
                lower_stmt_ast(
                    builder,
                    current_bindings,
                    carrier_phis,
                    carrier_step_phis,
                    carrier_updates,
                    stmt,
                    error_prefix,
                )
            })?;
            *current_bindings = saved_bindings.clone();
            let else_plans = match else_body {
                Some(body) => Some(lower_stmt_block(body, |stmt| {
                    lower_stmt_ast(
                        builder,
                        current_bindings,
                        carrier_phis,
                        carrier_step_phis,
                        carrier_updates,
                        stmt,
                        error_prefix,
                    )
                })?),
                None => None,
            };
            *current_bindings = saved_bindings;

            let cond_view = CondBlockView::from_expr(condition);
            let mut then_plans_once = Some(then_plans);
            let mut else_plans_once = else_plans;
            let has_else = else_plans_once.is_some();
            let mut lower_else =
                |_builder: &mut MirBuilder,
                 _bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
                    Ok(else_plans_once.take().ok_or_else(|| {
                        format!("{error_prefix}: internal error: else_plans consumed twice")
                    })?)
                };
            let lower_else: Option<
                &mut dyn FnMut(
                    &mut MirBuilder,
                    &mut BTreeMap<String, crate::mir::ValueId>,
                ) -> Result<Vec<LoweredRecipe>, String>,
            > = if has_else {
                Some(&mut lower_else)
            } else {
                None
            };

            let should_update_binding =
                |name: &str, bindings: &BTreeMap<String, crate::mir::ValueId>| {
                    carrier_phis.contains_key(name) || bindings.contains_key(name)
                };
            parts::entry::lower_if_join_with_branch_lowerers(
                builder,
                current_bindings,
                &cond_view,
                error_prefix,
                &mut |_builder, _bindings| {
                    Ok(then_plans_once.take().ok_or_else(|| {
                        format!("{error_prefix}: internal error: then_plans consumed twice")
                    })?)
                },
                lower_else,
                &should_update_binding,
            )
        }
    }
}

fn lower_stmt_ast(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            let effects = lower_assignment_stmt(
                builder,
                current_bindings,
                carrier_phis,
                carrier_updates,
                target,
                value,
                error_prefix,
            )?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::Local {
            variables,
            initial_values,
            ..
        } => {
            let effects = lower_local_init_stmt(
                builder,
                current_bindings,
                variables,
                initial_values,
                error_prefix,
            )?;

            if crate::config::env::is_joinir_debug() {
                let mut found = 0usize;
                for name in variables {
                    if current_bindings.contains_key(name) {
                        found += 1;
                    }
                }
                trace_collection_len("local_init_bindings_found", found);
            }

            Ok(effects_to_plans(effects))
        }
        ASTNode::MethodCall { .. } => {
            let effects = loop_body_lowering::lower_method_call_stmt(
                builder,
                current_bindings,
                stmt,
                error_prefix,
            )?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::FunctionCall { .. } => {
            let effects = loop_body_lowering::lower_function_call_stmt(
                builder,
                current_bindings,
                stmt,
                error_prefix,
            )?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::Print { expression, .. } => {
            let (value_id, mut effects) =
                PlanNormalizer::lower_value_ast(expression, builder, current_bindings)?;
            effects.push(CoreEffectPlan::ExternCall {
                dst: None,
                iface_name: "env.console".to_string(),
                method_name: "log".to_string(),
                args: vec![value_id],
                effects: EffectMask::PURE.add(Effect::Io),
            });
            Ok(effects_to_plans(effects))
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            if let Some(plans) = parts::entry::lower_conditional_update_if(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                condition,
                then_body,
                else_body.as_ref(),
                error_prefix,
            )? {
                return Ok(plans);
            }

            if_branch_lowering::lower_if_with_branch_lowerers_and_updates(
                builder,
                current_bindings,
                carrier_phis,
                carrier_updates,
                condition,
                then_body,
                else_body.as_deref(),
                error_prefix,
                |builder, bindings, carrier_updates, stmt| {
                    lower_stmt_ast(
                        builder,
                        bindings,
                        carrier_phis,
                        carrier_step_phis,
                        carrier_updates,
                        stmt,
                        error_prefix,
                    )
                },
            )
        }
        ASTNode::Return { value, .. } => parts::entry::lower_return_with_effects(
            builder,
            value.as_deref(),
            current_bindings,
            error_prefix,
        ),
        _ => {
            let (_value_id, effects) =
                PlanNormalizer::lower_value_ast(stmt, builder, current_bindings)?;
            Ok(effects_to_plans(effects))
        }
    }
}

fn lower_hetero_return_if(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_assignment: &ASTNode,
    else_chain: &[ASTNode],
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    use crate::mir::builder::control_flow::plan::features::conditional_update_join::collect_conditional_update_branch;

    let pre_if_map = builder.variable_ctx.variable_map.clone();
    let pre_bindings = current_bindings.clone();

    let then_body = std::slice::from_ref(then_assignment);
    let then_branch =
        collect_conditional_update_branch(builder, current_bindings, then_body, error_prefix)?;

    let mut then_plans = Vec::new();
    then_plans.extend(effects_to_plans(then_branch.effects));
    then_plans.extend(lower_stmt_ast(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        then_assignment,
        error_prefix,
    )?);
    let then_map = builder.variable_ctx.variable_map.clone();

    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();

    let mut else_plans = Vec::new();
    for stmt in else_chain {
        else_plans.extend(lower_stmt_ast(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            carrier_updates,
            stmt,
            error_prefix,
        )?);
    }
    let else_map = builder.variable_ctx.variable_map.clone();

    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings;

    if crate::config::env::is_joinir_debug() {
        let mut then_changed = Vec::new();
        let mut else_changed = Vec::new();
        for (name, pre_val) in &pre_if_map {
            let then_val = then_map.get(name).copied().unwrap_or(*pre_val);
            if then_val != *pre_val {
                then_changed.push(name.as_str());
            }
            let else_val = else_map.get(name).copied().unwrap_or(*pre_val);
            if else_val != *pre_val {
                else_changed.push(name.as_str());
            }
        }
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[plan/trace] hetero_return_if maps then_changed_count={} else_changed_count={}",
            then_changed.len(),
            else_changed.len(),
        ));
    }

    let carrier_vars_for_join: Vec<&String> = carrier_phis.keys().collect();
    let cond_view = CondBlockView::from_expr(condition);
    let should_update_binding = |name: &str, bindings: &BTreeMap<String, crate::mir::ValueId>| {
        carrier_phis.contains_key(name) || bindings.contains_key(name)
    };
    let plans = parts::entry::lower_value_cond_if_with_filtered_joins(
        builder,
        current_bindings,
        &cond_view,
        &pre_if_map,
        &then_map,
        &else_map,
        carrier_vars_for_join.into_iter(),
        then_plans,
        else_plans,
        error_prefix,
        &should_update_binding,
        |name, dst| {
            carrier_updates.insert(name.to_owned(), dst);
        },
    )?;

    if crate::config::env::is_joinir_debug() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[plan/trace] hetero_return_if: CoreIfJoin merge + if-else"
        ));
    }

    Ok(plans)
}

fn lower_continue_if(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    prelude_body: &[ASTNode],
    prelude_items: &[ContinueWithReturnItem],
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    trace_collection_len(
        "continue_if_before_prelude:current_bindings",
        current_bindings.len(),
    );

    let mut then_plans = Vec::new();
    let prelude_view = BodyView::Slice(prelude_body);

    let saved_bindings = current_bindings.clone();
    let saved_map = builder.variable_ctx.variable_map.clone();
    then_plans.extend(lower_continue_with_return_block(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        &prelude_view,
        prelude_items,
        error_prefix,
    )?);
    then_plans.push(CorePlan::Exit(parts::exit::build_continue_with_phi_args(
        builder,
        carrier_step_phis,
        current_bindings,
        error_prefix,
    )?));

    *current_bindings = saved_bindings;
    builder.variable_ctx.variable_map = saved_map;

    let cond_view = CondBlockView::from_expr(condition);
    let mut then_plans_once = Some(then_plans);
    parts::entry::lower_if_join_with_branch_lowerers(
        builder,
        current_bindings,
        &cond_view,
        error_prefix,
        &mut |_builder, _bindings| {
            Ok(then_plans_once.take().ok_or_else(|| {
                format!("{error_prefix}: internal error: then_plans consumed twice")
            })?)
        },
        None,
        &|_name, _bindings| false,
    )
}
